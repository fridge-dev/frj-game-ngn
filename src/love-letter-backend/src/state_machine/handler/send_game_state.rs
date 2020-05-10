use crate::state_machine::{LoveLetterStateMachineEventHandler, LoveLetterState};
use crate::types::GameData;
use backend_framework::wire_api::proto_frj_ngn::{ProtoLvLeGameState, ProtoLvLeCard, proto_lv_le_game_state};
use tonic::Status;
use backend_framework::wire_api::proto_frj_ngn::proto_lv_le_game_state::{ProtoLvLeRoundState, ProtoLvLePlayer};

impl LoveLetterStateMachineEventHandler {
    pub fn send_game_state(&self, state: &LoveLetterState, player_id: String) {
        match convert_state(state, &player_id) {
            Ok(proto_state) => self.streams.send_msg(&player_id, proto_state),
            Err(status) => self.streams.send_err(&player_id, status),
        }
    }
}

// TODO Internal state and API state have inconsistent (disjoint) data model. Needs thoughtful review and refactor.
fn convert_state(state: &LoveLetterState, player_id: &String) -> Result<ProtoLvLeGameState, Status> {
    let (
        proto_game_players,
        proto_round_players,
        opt_my_hand,
    ) = match state {
        LoveLetterState::InProgress(data) => convert_game_data(data, player_id),
        LoveLetterState::InProgressStaged(data, _staged) => convert_game_data(data, player_id),
    }?;

    let my_hand = opt_my_hand
        .map(|c| c as i32)
        .unwrap_or(0);

    let round_state = ProtoLvLeRoundState {
        remaining_player_ids: proto_round_players,
        my_hand,
        staged_play: None,
        most_recent_committed_play: None,
        play_history: vec![],
        turn: None
    };

    Ok(ProtoLvLeGameState {
        clock: 0,
        players: proto_game_players,
        stage: Some(proto_lv_le_game_state::Stage::RoundInProgress(round_state)),
    })
}

type ConvertGameDataResult = Result<
    (
        Vec<ProtoLvLePlayer>,
        Vec<String>,
        Option<ProtoLvLeCard>,
    ),
    Status
>;

fn convert_game_data(data: &GameData, my_player_id: &String) -> ConvertGameDataResult {
    let num_players = data.player_id_turn_order.len();
    let mut proto_game_players: Vec<ProtoLvLePlayer> = Vec::with_capacity(num_players);
    let mut proto_round_players: Vec<String> = Vec::with_capacity(num_players);
    let mut opt_my_hand: Option<ProtoLvLeCard> = None;

    for player_id in data.player_id_turn_order.iter() {
        // Game state
        let proto_player_state = ProtoLvLePlayer {
            player_id: player_id.to_string(),
            round_wins: 0,
        };
        proto_game_players.push(proto_player_state);

        // Round state
        let opt_player_card = data.current_round.player_cards.get(player_id);
        if opt_player_card.is_some() {
            proto_round_players.push(player_id.to_string());
        }

        // My state
        if my_player_id == player_id {
            opt_my_hand = opt_player_card.map(|c| ProtoLvLeCard::from(*c));
        }
    }

    Ok((
        proto_game_players,
        proto_round_players,
        opt_my_hand,
    ))
}