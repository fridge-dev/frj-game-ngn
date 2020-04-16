mod handler;

use crate::LoveLetterEvent;
use crate::types::{StagedPlay, GameData};
use backend_framework::Players;

const MIN_PLAYERS: usize = 2;
const MAX_PLAYERS: usize = 4;

/// The possible states of an instance of the game.
pub enum LoveLetterInstanceState {
    WaitingForStart,
    InProgress(GameData),
    InProgressStaged(GameData, StagedPlay),
}

/// A state machine executor. It operates on states as inputs/outputs, not owned data.
/// Although it does own some data specific to a game instance.
pub struct LoveLetterStateMachine {
    handler: StateMachineEventHandler,
}

struct StateMachineEventHandler {
    players: Players,
}

impl StateMachineEventHandler {
    pub fn new(players: Players) -> Self {
        StateMachineEventHandler {
            players,
        }
    }
}

impl LoveLetterStateMachine {
    pub fn new(players: Players) -> Self {
        LoveLetterStateMachine {
            handler: StateMachineEventHandler::new(players),
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
        from_state: LoveLetterInstanceState,
        event: LoveLetterEvent,
    ) -> LoveLetterInstanceState {
        match event {
            LoveLetterEvent::Join(player_id, client_out) => {
                self.handler.join(player_id, client_out, &from_state);
                from_state
            },
            LoveLetterEvent::GetGameState(player_id) => {
                self.handler.get_game_state(player_id);
                from_state
            },
            LoveLetterEvent::StartGame(player_id) => {
                self.handler.start_game(from_state, player_id)
            },
            LoveLetterEvent::PlayCardStaged(player_id, card_source) => {
                self.handler.play_card_staged(from_state, player_id, card_source)
            },
            LoveLetterEvent::SelectTargetPlayer(client_player_id, target_player_id) => {
                self.handler.select_target_player(from_state, client_player_id, target_player_id)
            }
            LoveLetterEvent::SelectTargetCard(client_player_id, target_card) => {
                self.handler.select_target_card(from_state, client_player_id, target_card)
            }
            LoveLetterEvent::PlayCardCommit(player_id) => {
                self.handler.play_card_commit(from_state, player_id)
            }
        }
    }
}