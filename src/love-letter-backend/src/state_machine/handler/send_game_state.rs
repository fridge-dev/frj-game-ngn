use crate::state_machine::{LoveLetterStateMachineEventHandler, LoveLetterState};
use crate::types::{GameData, RoundData, RoundResult};
use backend_framework::wire_api::proto_frj_ngn::{ProtoLvLeGameState, ProtoLvLeCard, ProtoLvLeCommittedPlay, ProtoLvLeCardSelection};
use tonic::Status;
use backend_framework::wire_api::proto_frj_ngn::proto_lv_le_game_state::{ProtoLvLeRoundState, ProtoLvLePlayer, Stage, ProtoLvLeResultState, proto_lv_le_round_state};
use std::collections::HashMap;

impl LoveLetterStateMachineEventHandler {
    pub fn send_game_state(&self, state: &LoveLetterState, player_id: &String) {
        let proto_state = convert_state(state, &player_id);
        self.streams.send_msg(&player_id, proto_state);
    }

    pub fn send_game_state_to_all(&self, state: &LoveLetterState) {
        unimplemented!()
    }
}

fn convert_state(state: &LoveLetterState, player_id: &String) -> ProtoLvLeGameState {
    let (
        players,
        stage,
    ) = match state {
        LoveLetterState::PlayPending(game_data, round_data) => (
            get_proto_players(game_data),
            Stage::RoundInProgress(into_proto_round_state(
                round_data,
                None,
                player_id
            )),
        ),
        LoveLetterState::PlayStaging(game_data, round_data, staged) => (
            get_proto_players(game_data),
            Stage::RoundInProgress(into_proto_round_state(
                round_data,
                Some(ProtoLvLeCardSelection::from(staged.clone())),
                player_id
            )),
        ),
        LoveLetterState::TurnIntermission(game_data, round_data) => (
            get_proto_players(game_data),
            Stage::RoundInProgress(into_proto_round_state(
                round_data,
                None,
                player_id
            )),
        ),
        LoveLetterState::RoundIntermission(game_data, round_result) => (
            get_proto_players(game_data),
            Stage::RoundIntermission(into_proto_result_state(round_result)),
        ),
    };

    ProtoLvLeGameState {
        clock: 0,
        players,
        stage: Some(stage),
    }
}

fn get_proto_players(game_data: &GameData) -> Vec<ProtoLvLePlayer> {
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

fn into_proto_round_state(round_data: &RoundData, staged_play: Option<ProtoLvLeCardSelection>, my_player_id: &String) -> ProtoLvLeRoundState {
    let remaining_player_ids = round_data.players.remaining_player_ids().clone();
    let my_hand: i32 = match round_data.players.get_card(my_player_id) {
        None => 0,
        Some(card) => ProtoLvLeCard::from(card) as i32,
    };

    let most_recent_committed_play = round_data.most_recent_play_details
        .clone()
        .map(|play| ProtoLvLeCommittedPlay::from(play));

    let play_history: Vec<i32> = round_data.play_history
        .iter()
        .map(|card| ProtoLvLeCard::from(*card) as i32)
        .collect();

    let turn: Option<proto_lv_le_round_state::Turn> = if my_player_id == round_data.players.current_turn_player_id() {
        round_data.deck
            .last()
            .map(|c| ProtoLvLeCard::from(*c) as i32)
            .map(|i| proto_lv_le_round_state::Turn::MyDrawnCard(i))
    } else {
        Some(proto_lv_le_round_state::Turn::CurrentTurnPlayerId(
            round_data.players.current_turn_player_id().to_string()
        ))
    };

    ProtoLvLeRoundState {
        remaining_player_ids,
        my_hand,
        staged_play,
        most_recent_committed_play,
        play_history,
        turn,
    }
}

fn into_proto_result_state(round_result: &RoundResult) -> ProtoLvLeResultState {
    let mut final_cards = HashMap::new();
    for (player_id, card) in round_result.final_card_by_player_id.iter() {
        final_cards.insert(player_id.clone(), ProtoLvLeCard::from(*card) as i32);
    }

    ProtoLvLeResultState {
        final_cards,
    }
}
