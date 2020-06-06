use crate::state_machine::{MastermindStateMachineImpl, BoardState};
use crate::types::PlayerSide;

impl MastermindStateMachineImpl {
    pub fn send_game_state(&self, _state: &BoardState, _player: PlayerSide) {
        unimplemented!() // TODO:1.5 convert to proto model
    }

    pub fn send_game_state_to_all(&self, _state: &BoardState) {
        unimplemented!() // TODO:1.5 convert to proto model
    }
}