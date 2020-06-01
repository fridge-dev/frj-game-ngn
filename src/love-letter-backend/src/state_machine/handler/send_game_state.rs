use crate::state_machine::{LoveLetterStateMachine, LoveLetterState};
use crate::types::{GameData, RoundData, RoundResult, UnreadyPlayers};
use backend_framework::wire_api::proto_frj_ngn::{ProtoLvLeGameState, ProtoLvLeCard, ProtoLvLeCardSelection};
use backend_framework::wire_api::proto_frj_ngn::proto_lv_le_game_state::{ProtoLvLeRoundState, ProtoLvLePlayer, Stage, ProtoLvLeResultState, proto_lv_le_round_state, ProtoLvLeTurnIntermissionState};
use std::collections::HashMap;

impl LoveLetterStateMachine {
    pub fn send_game_state(&self, state: &LoveLetterState, player_id: &String) {
        let proto_all_players = get_proto_all_players(&self.game_data);

        self.send_game_state_to_player(player_id, state, proto_all_players);
    }

    pub fn send_game_state_to_all(&self, state: &LoveLetterState) {
        let proto_all_players = get_proto_all_players(&self.game_data);

        for player_id in self.game_data.player_id_turn_order.iter() {
            self.send_game_state_to_player(player_id, state, proto_all_players.clone());
        }
    }

    fn send_game_state_to_player(
        &self,
        player_id: &String,
        state: &LoveLetterState,
        proto_all_players: Vec<ProtoLvLePlayer>
    ) {
        let proto_state = ProtoLvLeGameState {
            clock: 0,
            players: proto_all_players,
            stage: Some(into_proto_stage(state, player_id)),
        };
        self.streams.send_msg(player_id, proto_state);
    }
}

fn get_proto_all_players(game_data: &GameData) -> Vec<ProtoLvLePlayer> {
    let mut proto_game_players: Vec<ProtoLvLePlayer> = Vec::with_capacity(game_data.player_id_turn_order.len());

    for player_id in game_data.player_id_turn_order.iter() {
        let proto_player_state = ProtoLvLePlayer {
            player_id: player_id.to_string(),
            round_wins: game_data.wins_per_player.get(player_id).map(|n| *n as u32).unwrap_or(0),
        };
        proto_game_players.push(proto_player_state);
    }

    proto_game_players
}

fn into_proto_stage(state: &LoveLetterState, player_id: &String) -> Stage {
    match state {
        LoveLetterState::PlayPending(round_data) => Stage::RoundInProgress(
            into_proto_round_state(
                round_data,
                player_id,
                None,
                None,
            )
        ),
        LoveLetterState::PlayStaging(round_data, staged) => Stage::RoundInProgress(
            into_proto_round_state(
                round_data,
                player_id,
                Some(ProtoLvLeCardSelection::from(staged.clone())),
                None,
            )
        ),
        LoveLetterState::TurnIntermission(round_data, unready_players) => Stage::RoundInProgress(
            into_proto_round_state(
                round_data,
                player_id,
                None,
                Some(unready_players.clone())
            )
        ),
        LoveLetterState::RoundIntermission(round_result, unready_players) => Stage::RoundIntermission(
            into_proto_result_state(round_result.clone(), unready_players.clone().into_inner())
        ),
    }
}

fn into_proto_round_state(
    round_data: &RoundData,
    my_player_id: &String,
    staged_play: Option<ProtoLvLeCardSelection>,
    opt_unready_players: Option<UnreadyPlayers>,
) -> ProtoLvLeRoundState {
    let remaining_player_ids = round_data.players.remaining_player_ids().clone();
    let my_hand: i32 = match round_data.players.get_card(my_player_id) {
        None => 0,
        Some(card) => ProtoLvLeCard::from(card) as i32,
    };

    let most_recent_committed_play = round_data.most_recent_play_details
        .clone()
        .map(|play| play.into_proto(my_player_id));

    let play_history: Vec<i32> = round_data.play_history
        .iter()
        .map(|card| ProtoLvLeCard::from(*card) as i32)
        .collect();

    let mut handmaid_player_ids = Vec::with_capacity(round_data.handmaid_immunity_player_ids.len());
    for player_id in round_data.handmaid_immunity_player_ids.clone() {
        handmaid_player_ids.push(player_id);
    }

    let is_my_turn = my_player_id == round_data.players.current_turn_player_id();
    let turn: Option<proto_lv_le_round_state::Turn> = match (opt_unready_players, is_my_turn) {
        (Some(unready_players), _) => {
            Some(proto_lv_le_round_state::Turn::TurnIntermission(ProtoLvLeTurnIntermissionState {
                unready_player_ids: unready_players.into_inner()
            }))
        }
        (None, true) => {
            round_data.deck
                .last()
                .map(|c| ProtoLvLeCard::from(*c) as i32)
                .map(|i| proto_lv_le_round_state::Turn::MyDrawnCard(i))
        },
        (None, false) => {
            Some(proto_lv_le_round_state::Turn::CurrentTurnPlayerId(
                round_data.players.current_turn_player_id().to_string()
            ))
        },
    };

    ProtoLvLeRoundState {
        remaining_player_ids,
        my_hand,
        staged_play,
        most_recent_committed_play,
        play_history,
        handmaid_player_ids,
        turn,
    }
}

fn into_proto_result_state(
    round_result: RoundResult,
    unready_player_ids: Vec<String>
) -> ProtoLvLeResultState {
    let mut final_cards = HashMap::new();
    for (player_id, card) in round_result.final_card_by_player_id {
        final_cards.insert(player_id, ProtoLvLeCard::from(card) as i32);
    }

    ProtoLvLeResultState {
        final_cards,
        unready_player_ids,
    }
}
