use crate::state_machine::{StateMachineEventHandler, LoveLetterInstanceState};
use crate::types::{Card, RoundData};
use backend_framework::streaming::MessageErrType;

impl StateMachineEventHandler {

    pub fn play_card_commit(
        &mut self,
        from_state: LoveLetterInstanceState,
        player_id: String
    ) -> LoveLetterInstanceState {
        match from_state {
            LoveLetterInstanceState::WaitingForStart => {
                self.players.send_pre_game_error(&player_id, "Can't play card before game start", MessageErrType::InvalidReq);
                LoveLetterInstanceState::WaitingForStart
            },
            LoveLetterInstanceState::InProgress(game_data) => {
                // TODO if selection not-needed, auto-commit
                LoveLetterInstanceState::InProgress(game_data)
            },
            LoveLetterInstanceState::InProgressStaged(mut game_data, staged_play) => {
                // Perform action
                match staged_play.card {
                    Card::Guard => {
                        // TODO more robust way of expecting staging (micro states?)
                        let target_player = staged_play.target_player.expect("Rule");
                        let guessed_card = staged_play.target_card.expect("Rule");
                        let actual_card = game_data.current_round.player_cards.get(&target_player)
                            .expect("bug with input validation");
                        if guessed_card == *actual_card {
                            // Player is out!
                            game_data.current_round.player_cards.remove(&target_player);
                            // TODO send result to all players
                        } else {
                            // TODO send result to all players
                        }
                    },
                    Card::Priest => {},
                    Card::Baron => {},
                    Card::Handmaid => {},
                    Card::Prince => {},
                    Card::King => {},
                    Card::Countess => {},
                    Card::Princess => {},
                }

                // Update current-player hand
                // TODO do this during staging

                // Send next card to next player
                let next_card_opt = game_data.current_round.remaining_cards.last();
                match next_card_opt {
                    None => {
                        // Round over
                        let (winner, mut high_card) = game_data.current_round
                            .player_cards
                            .remove_entry(&player_id)
                            .expect("impossible");
                        let mut winners = vec![winner];
                        for (player_id, card) in game_data.current_round.player_cards {
                            if card > high_card {
                                winners.clear();
                                winners.push(player_id);
                                high_card = card;
                            } else if card == high_card {
                                winners.push(player_id);
                            }
                        }

                        for player_id in winners {
                            *game_data.wins_per_player.entry(player_id).or_insert(0) += 1;
                        }

                        // TODO notify players

                        // Re-deal
                        game_data.current_round = RoundData::new(&game_data.player_id_turn_order)
                    },
                    Some(next_card) => {
                        game_data.current_round.turn_cursor = (game_data.current_round.turn_cursor + 1) % game_data.player_id_turn_order.len();
                        let next_player = game_data.current_player_turn();
                        unimplemented!("Send next_card to player")
                    },
                }

                LoveLetterInstanceState::InProgress(game_data)
            },
        }
    }
}