use crate::events::Card;
use crate::state_machine::{LoveLetterStateMachine, LoveLetterState};
use crate::types::RoundResult;

impl LoveLetterStateMachine {

    pub fn play_card_commit(
        &mut self,
        from_state: LoveLetterState,
        player_id: String
    ) -> LoveLetterState {
        match from_state {
            LoveLetterState::PlayPending(round_data) => {
                // TODO if selection not-needed, auto-commit
                LoveLetterState::PlayPending(round_data)
            },
            LoveLetterState::TurnIntermission(round_data) => {
                // TODO inform caller of bad request
                LoveLetterState::TurnIntermission(round_data)
            },
            LoveLetterState::RoundIntermission(round_result) => {
                // TODO inform caller of bad request
                LoveLetterState::RoundIntermission(round_result)
            },
            LoveLetterState::PlayStaging(mut round_data, staged_play) => {
                let mut player_to_eliminate = PlayerToEliminate::none();

                // Perform action
                match staged_play.played_card {
                    Card::Guard => {
                        // TODO more robust way of expecting staging (micro states?)
                        let target_player = staged_play.target_player.expect("Rule");
                        let guessed_card = staged_play.target_card.expect("Rule");
                        let actual_card = round_data
                            .players
                            .get_card(&target_player)
                            .expect("bug with input validation");
                        if guessed_card == actual_card {
                            // Player is out!
                            player_to_eliminate.set(target_player);
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

                // Eliminate player and increment turn cursor
                match player_to_eliminate.into() {
                    None => {
                        round_data.players.increment_turn()
                    },
                    Some(player_id) => {
                        // TODO use card
                        let card = round_data.players.eliminate_and_increment_turn(&player_id);
                    },
                }

                // Update current-player hand
                // TODO do this during staging

                // TODO check if 1 player remaining

                // Send next card to next player
                let next_card_opt = round_data.deck.last();
                match next_card_opt {
                    None => {
                        // Round over
                        let final_card_by_player_id = round_data.players.into_player_card_map();
                        let mut player_cards = final_card_by_player_id.clone().into_iter();

                        let (winner, mut high_card) = player_cards.next()
                            .expect("LoveLetter round ended with no players remaining");
                        let mut winners: Vec<String> = vec![winner];

                        for (player_id, card) in player_cards {
                            if card > high_card {
                                winners.clear();
                                winners.push(player_id);
                                high_card = card;
                            } else if card == high_card {
                                winners.push(player_id);
                            }
                        }

                        for player_id in winners {
                            *self.game_data.wins_per_player.entry(player_id).or_insert(0) += 1;
                        }

                        // TODO notify players

                        // TODO new API to start next round
                        LoveLetterState::RoundIntermission(RoundResult::new(final_card_by_player_id))
                    },
                    Some(next_card) => {
                        let next_player = round_data.players.current_turn_player_id();
                        // TODO notify players
                        LoveLetterState::PlayPending(round_data)
                    },
                }
            },
        }
    }
}

struct PlayerToEliminate {
    player_id: Option<String>,
}

impl PlayerToEliminate {
    pub fn none() -> Self {
        PlayerToEliminate {
            player_id: None,
        }
    }

    pub fn set(&mut self, player_id: String) {
        let replaced = self.player_id.replace(player_id);

        if replaced.is_some() {
            panic!("Rule: tried to eliminate 2 players in one turn.");
        }
    }

    pub fn into(self) -> Option<String> {
        self.player_id
    }
}