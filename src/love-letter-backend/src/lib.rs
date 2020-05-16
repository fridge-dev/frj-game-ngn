pub mod events;

mod deck;
mod state_machine;
mod types;
mod type_converters;

use crate::events::LoveLetterEvent;
use crate::state_machine::{LoveLetterState, LoveLetterStateMachine};
use crate::types::GameData;
use backend_framework::holder::Holder;

/// This is the top level class for managing a single game of LoveLetter.
pub struct LoveLetterInstanceManager {
    state: Holder<LoveLetterState>,
    state_machine: LoveLetterStateMachine,
}

impl LoveLetterInstanceManager {

    pub fn new(player_ids: Vec<String>) -> Self {
        LoveLetterInstanceManager {
            state: Holder::new(LoveLetterState::PlayPending(GameData::new(player_ids.clone()))),
            state_machine: LoveLetterStateMachine::new(player_ids),
        }
    }

    /// This is the single entry point for manipulating the state of the game.
    pub fn handle_event(&mut self, event: LoveLetterEvent) {
        let from_state = self.state.take();
        let to_state = self.state_machine.transition_state(from_state, event);
        self.state.put(to_state);
    }
}
