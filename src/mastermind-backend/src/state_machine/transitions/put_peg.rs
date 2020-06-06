use crate::state_machine::{MastermindStateMachineImpl, BoardState};
use crate::types::{PlayerSide, Color};

impl MastermindStateMachineImpl {
    pub fn put_peg(
        &self,
        from_state: &mut BoardState,
        player: PlayerSide,
        peg: usize,
        color: Color,
    ) {
        // happy case only
        match from_state {
            BoardState::Pregame(ref mut data) => {
                let my_board = data.my_board_mut(player);
                if let Err(_e) = my_board.sparse_password.try_set(peg, color) {
                    // TODO:1.5 notify caller of error
                } else {
                    my_board.ready = false;
                }
            },
            BoardState::Active(ref mut data) => {
                let my_board = data.my_board_mut(player);
                if let Err(_e) = my_board.current_guess.try_set(peg, color) {
                    // TODO:1.5 notify caller of error
                }
            },
            BoardState::LActiveRDone(ref mut data) => {
                match player {
                    PlayerSide::Left => {
                        if let Err(_e) = data.left.current_guess.try_set(peg, color) {
                            // TODO:1.5 notify caller of error
                        }
                    },
                    PlayerSide::Right => {
                        // TODO:1.5 notify caller of not their turn
                    },
                }
            },
            BoardState::LDoneRActive(ref mut data) => {
                match player {
                    PlayerSide::Left => {
                        // TODO:1.5 notify caller of not their turn
                    },
                    PlayerSide::Right => {
                        if let Err(_e) = data.right.current_guess.try_set(peg, color) {
                            // TODO:1.5 notify caller of error
                        }
                    },
                }
            },
            BoardState::Done(_) => {
                // TODO:1.5 notify caller bad state
            },
        }
    }
}