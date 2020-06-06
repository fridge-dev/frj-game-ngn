use crate::state_machine::{MastermindStateMachineImpl, BoardState};
use crate::state_machine::data::{PregameData, ActiveData};
use crate::types::PlayerSide;

impl MastermindStateMachineImpl {
    pub fn submit_password(&self, from_state: BoardState, player: PlayerSide) -> BoardState {
        // Happy case
        if let BoardState::Pregame(mut data) = from_state {
            if set_my_ready(&mut data, player) && data.op_board(player).ready {
                BoardState::Active(ActiveData::from(data))
            } else {
                BoardState::Pregame(data)
            }
        } else {
            // TODO:1.5 notify caller bad state
            from_state
        }
    }
}

/// return true if we successfully ready-ed up.
fn set_my_ready(data: &mut PregameData, player: PlayerSide) -> bool {
    let my_board = data.my_board_mut(player);
    if !my_board.sparse_password.is_complete() {
        // TODO:1.5 notify player of error
        return false;
    }

    my_board.ready = true;
    true
}
