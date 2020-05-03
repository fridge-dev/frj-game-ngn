mod deck;
mod state_machine;
mod types;
mod type_converters;
pub mod events;

use crate::events::LoveLetterEvent;
use crate::state_machine::{LoveLetterState, LoveLetterStateMachine};
use crate::types::GameData;
use backend_framework::holder::Holder;

pub struct LoveLetterInstanceManager {
    state: Holder<LoveLetterState>,
    state_machine: LoveLetterStateMachine,
}

impl LoveLetterInstanceManager {

    pub fn new(player_ids: Vec<String>) -> Self {
        LoveLetterInstanceManager {
            state: Holder::new(LoveLetterState::InProgress(GameData::new(player_ids.clone()))),
            state_machine: LoveLetterStateMachine::new(player_ids),
        }
    }

    pub fn handle_event(&mut self, event: LoveLetterEvent) {
        let from_state = self.state.take();
        let to_state = self.state_machine.transition_state(from_state, event);
        self.state.put(to_state);
    }
}
