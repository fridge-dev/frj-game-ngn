use crate::state_machine::{LoveLetterStateMachineEventHandler, LoveLetterState};
use crate::types::GameData;
use backend_framework::streaming::MessageErrType;
use backend_framework::wire_api::proto_frj_ngn::{ProtoLvLeGameState, ProtoLvLeCard};
use backend_framework::wire_api::proto_frj_ngn::proto_lv_le_game_state::ProtoLvLePlayerState;

impl LoveLetterStateMachineEventHandler {
    pub fn send_game_state(&self, state: &LoveLetterState, player_id: String) {
        match convert_state(state, &player_id) {
            Ok(proto_state) => self.streams.send_msg(&player_id, proto_state),
            Err((msg, e)) => self.streams.send_err(&player_id, msg, e),
        }
    }
}

// TODO Internal state and API state have inconsistent (disjoint) data model. Needs thoughtful review and refactor.
//
fn convert_state(state: &LoveLetterState, player_id: &String) -> Result<ProtoLvLeGameState, (String, MessageErrType)> {
    let (
        players,
        opt_my_card,
        current_turn_player_id,
    ) = match state {
        LoveLetterState::InProgress(data) => {
            convert_game_data(data, player_id)
        },
        LoveLetterState::InProgressStaged(data, staged) => {
            convert_game_data(data, player_id)
        },
    }?;

    let my_card = opt_my_card
        .map(|c| c as i32)
        .unwrap_or(0);

    Ok(ProtoLvLeGameState {
        clock: 0,
        players,
        my_card,
        current_turn_player_id,
        play_history: vec![],
    })
}

// TODO error modeling is getting yucky. Should soon figure out how to best model this. Maybe this
// is a premature optimization and I should just use Status.
type ConvertGameDataResult = Result<
    (
        Vec<ProtoLvLePlayerState>,
        Option<ProtoLvLeCard>,
        String,
    ),
    (
        String,
        MessageErrType,
    )
>;

fn convert_game_data(data: &GameData, my_player_id: &String) -> ConvertGameDataResult {
    let mut proto_players = Vec::with_capacity(data.player_id_turn_order.len());
    let mut opt_my_card: Option<ProtoLvLeCard> = None;

    for player_id in data.player_id_turn_order.iter() {
        let opt_player_card = data.current_round.player_cards.get(player_id);

        if my_player_id == player_id {
            opt_my_card = opt_player_card.map(|c| ProtoLvLeCard::from(*c));
        }

        let proto_player_state = ProtoLvLePlayerState {
            player_id: player_id.to_string(),
            in_play: opt_player_card.is_some(),
            round_wins: 0,
        };
        proto_players.push(proto_player_state);
    }

    let current_turn_player_id = data.player_id_turn_order
        .get(data.current_round.turn_cursor)
        .ok_or_else(|| ("Internal bug when determining player turn".to_string(), MessageErrType::ServerFault))?
        .to_string();

    Ok((
        proto_players,
        opt_my_card,
        current_turn_player_id,
    ))
}