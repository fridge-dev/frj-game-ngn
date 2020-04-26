mod handler;

use crate::LoveLetterEvent;
use crate::types::{StagedPlay, GameData};
use backend_framework::data_stream::PlayerDataStreams;
use backend_framework::wire_api::proto_frj_ngn::ProtoLoveLetterDataOut;

/// The possible states of an instance of the game.
pub enum LoveLetterState {
    InProgress(GameData),
    InProgressStaged(GameData, StagedPlay),
}

/// A state machine executor. It operates on states as inputs/outputs, not owned data.
/// Although it does own some data specific to a game instance.
pub struct LoveLetterStateMachine {
    handler: LoveLetterStateMachineEventHandler,
}

impl LoveLetterStateMachine {
    pub fn new(player_ids: Vec<String>) -> Self {
        LoveLetterStateMachine {
            handler: LoveLetterStateMachineEventHandler::new(player_ids),
        }
    }

    /// State machine logic:
    ///
    /// Move from FROM_STATE to TO_STATE and mutate internal data as needed.
    ///
    /// This will be a PITA to add Result<> to. Unless Err means game is in corrupt state
    /// and we drop the game instance.
    pub fn transition_state(
        &mut self,
        from_state: LoveLetterState,
        event: LoveLetterEvent,
    ) -> LoveLetterState {
        match event {
            LoveLetterEvent::GetGameState(player_id) => {
                self.handler.get_game_state(player_id);
                from_state
            },
            LoveLetterEvent::RegisterDataStream(player_id, stream_out) => {
                self.handler.players.add_stream(player_id, stream_out);
                from_state
            },
            LoveLetterEvent::PlayCardStaged(player_id, card_source) => {
                self.handler.play_card_staged(from_state, player_id, card_source)
            },
            LoveLetterEvent::SelectTargetPlayer(client_player_id, target_player_id) => {
                self.handler.select_target_player(from_state, client_player_id, target_player_id)
            },
            LoveLetterEvent::SelectTargetCard(client_player_id, target_card) => {
                self.handler.select_target_card(from_state, client_player_id, target_card)
            },
            LoveLetterEvent::PlayCardCommit(player_id) => {
                self.handler.play_card_commit(from_state, player_id)
            },
        }
    }
}

struct LoveLetterStateMachineEventHandler {
    players: PlayerDataStreams<ProtoLoveLetterDataOut>,
}

impl LoveLetterStateMachineEventHandler {
    pub fn new(player_ids: Vec<String>) -> Self {
        LoveLetterStateMachineEventHandler {
            players: PlayerDataStreams::new(player_ids)
        }
    }
}
