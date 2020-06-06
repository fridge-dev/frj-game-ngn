use crate::state_machine::{MastermindStateMachineImpl, BoardState};
use crate::types::{PlayerSide, CompletedBoard, ActiveBoard, Row, ResultPegs, Color};
use crate::state_machine::data::{LDoneRActiveData, LActiveRDoneData, DoneData};
use std::collections::HashMap;

impl MastermindStateMachineImpl {
    pub fn commit_row(&self, from_state: BoardState, player: PlayerSide) -> BoardState {
        match from_state {
            BoardState::Active(mut data) => {
                let mut my_board = data.my_board_mut(player);
                if my_board.current_guess.is_complete() {
                    // TODO:1.5 notify caller
                    return BoardState::Active(data);
                }

                let is_board_done = commit_active_board(&mut my_board);

                match (player, is_board_done) {
                    (PlayerSide::Left, true) => {
                        BoardState::LDoneRActive(LDoneRActiveData {
                            left: CompletedBoard::from(data.left),
                            right: data.right,
                        })
                    },
                    (PlayerSide::Right, true) => {
                        BoardState::LActiveRDone(LActiveRDoneData {
                            left: data.left,
                            right: CompletedBoard::from(data.right),
                        })
                    }
                    (_, false) => {
                        BoardState::Active(data)
                    }
                }
            },
            BoardState::LActiveRDone(mut data) => {
                if player == PlayerSide::Right {
                    // TODO:1.5 notify caller already done
                    return BoardState::LActiveRDone(data);
                }

                if !data.left.current_guess.is_complete() {
                    // TODO:1.5 notify caller
                    return BoardState::LActiveRDone(data);
                }

                let is_board_done = commit_active_board(&mut data.left);
                if !is_board_done {
                    BoardState::LActiveRDone(data)
                } else {
                    BoardState::Done(DoneData {
                        left: CompletedBoard::from(data.left),
                        right: data.right,
                    })
                }
            },
            BoardState::LDoneRActive(mut data) => {
                if player == PlayerSide::Left {
                    // TODO:1.5 notify caller already done
                    return BoardState::LDoneRActive(data);
                }

                if !data.right.current_guess.is_complete() {
                    // TODO:1.5 notify caller
                    return BoardState::LDoneRActive(data);
                }

                let is_board_done = commit_active_board(&mut data.right);
                if !is_board_done {
                    BoardState::LDoneRActive(data)
                } else {
                    BoardState::Done(DoneData {
                        left: data.left,
                        right: CompletedBoard::from(data.right)
                    })
                }
            },
            _ => {
                // TODO:1.5 notify caller unhappy case
                from_state
            },
        }
    }
}

fn commit_active_board(board: &mut ActiveBoard) -> bool {
    let result_pegs = compare(&board.current_guess, &board.password_to_guess);
    let is_board_done = result_pegs.correct == board.password_to_guess.len() as u8;
    board.completed_rows.push((board.current_guess.clone(), result_pegs));
    board.current_guess = Row::new(8); // TODO:1.5 param
    is_board_done
}

/// This is the part of the game the is super helpful to automate and kind of annoying and
/// error prone to calculate in human head during actualy board game.
///
/// For `c` color choices and `p` pegs in a row, this algo is `O(p)`.
///
/// `bool` in return is if password was guessed correctly.
fn compare(guess: &Row, password: &Row) -> ResultPegs {
    assert_eq!(guess.len(), password.len());

    // `O(p)`, but could be pre-computed
    let mut password_colors_count: HashMap<Color, u8> = HashMap::new();
    for i in 0..password.len() {
        *password_colors_count
            .entry(password.peg(i))
            .or_insert(0) += 1;
    }
    let password_colors_count = password_colors_count;

    // `O(p)`, could just be cloned
    let mut password_colors: HashMap<Color, ResultPegs> = HashMap::new();
    for i in 0..password.len() {
        password_colors
            .entry(password.peg(i))
            .or_insert_with(|| ResultPegs::default());
    }

    // `O(p)`
    for i in 0..guess.len() {
        let guessed_color = guess.peg(i);
        if let Some(result_pegs) = password_colors.get_mut(&guessed_color) {
            // Guessed a correct color. Was it correct location?
            if guessed_color == password.peg(i) {
                result_pegs.correct += 1; // Yes
            } else {
                result_pegs.correct_color_wrong_slot += 1; // No
            }
        }
    }

    let mut correct = 0;
    let mut correct_color_wrong_slot = 0;

    // `O(p)` - technically `O(c)`, but the `c` here is always bounded by `p`.
    for (color, mut result) in password_colors.iter_mut() {
        let num_occurrences_in_password = password_colors_count.get(&color)
            .expect("UNREACHABLE BUG MOTHER FUCKER.");

        let hits = result.correct + result.correct_color_wrong_slot;
        if hits > *num_occurrences_in_password {
            assert!(*num_occurrences_in_password >= result.correct);
            let correction = hits - *num_occurrences_in_password;
            result.correct_color_wrong_slot -= correction;
        }

        correct += result.correct;
        correct_color_wrong_slot += result.correct_color_wrong_slot;
    }

    ResultPegs {
        correct,
        correct_color_wrong_slot,
    }
}
