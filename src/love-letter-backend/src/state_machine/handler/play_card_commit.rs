use crate::events::Card;
use crate::state_machine::{LoveLetterStateMachineEventHandler, LoveLetterState};
use crate::types::RoundData;

impl LoveLetterStateMachineEventHandler {

    pub fn play_card_commit(
        &mut self,
        from_state: LoveLetterState,
        player_id: String
    ) -> LoveLetterState {
        match from_state {
            LoveLetterState::InProgress(game_data) => {
                // TODO if selection not-needed, auto-commit
                LoveLetterState::InProgress(game_data)
            },
            LoveLetterState::InProgressStaged(mut game_data, staged_play) => {
                let mut player_to_eliminate = PlayerToEliminate::none();

                // Perform action
                match staged_play.card {
                    Card::Guard => {
                        // TODO more robust way of expecting staging (micro states?)
                        let target_player = staged_play.target_player.expect("Rule");
                        let guessed_card = staged_play.target_card.expect("Rule");
                        let actual_card = game_data
                            .current_round
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
                        game_data.current_round.players.increment_turn()
                    },
                    Some(player_id) => {
                        // TODO use card
                        let card = game_data.current_round.players.eliminate_and_increment_turn(&player_id);
                    },
                }

                // Update current-player hand
                // TODO do this during staging

                // Send next card to next player
                let next_card_opt = game_data.current_round.deck.last();
                match next_card_opt {
                    None => {
                        // Round over
                        let mut player_cards = game_data.current_round.players.into_iter();

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
                            *game_data.wins_per_player.entry(player_id).or_insert(0) += 1;
                        }

                        // TODO notify players

                        // Re-deal
                        game_data.current_round = RoundData::new(&game_data.player_id_turn_order)
                    },
                    Some(next_card) => {
                        let next_player = game_data.current_round.players.current_turn_player_id();
                        unimplemented!("Send next_card to player")
                    },
                }

                LoveLetterState::InProgress(game_data)
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