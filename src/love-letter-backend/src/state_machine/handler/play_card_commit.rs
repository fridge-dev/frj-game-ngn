use crate::events::Card;
use crate::state_machine::{LoveLetterStateMachine, LoveLetterState};
use crate::types::{RoundResult, RoundData, StagedPlay, CommittedPlay, CommittedPlayOutcome, UnreadyPlayers};
use tonic::Status;

impl LoveLetterStateMachine {

    pub fn play_card_commit(
        &mut self,
        from_state: LoveLetterState,
        client_player_id: String
    ) -> LoveLetterState {
        match from_state {
            LoveLetterState::PlayStaging(round_data, staged_play) => {
                self.handle_commit(round_data, staged_play, client_player_id)
            },
            _ => {
                self.send_game_state(&from_state, &client_player_id);
                from_state
            },
        }
    }

    /// Assumed pre-requisites:
    /// * Players hand has been "cycled" (drawn new card)
    /// * Players played card has been appended to play history
    /// * Handmaid status is cleared
    ///
    /// Logic:
    /// 1: Validate action has selections
    /// 2. Update player hands (if needed)
    /// 3. Create and set most recent play details
    /// 4. Eliminate player
    /// 5. Increment turn counter
    /// 6: State transition: TurnIntermission OR RoundIntermission
    fn handle_commit(
        &mut self,
        mut round_data: RoundData,
        staged_play: StagedPlay,
        client_player_id: String,
    ) -> LoveLetterState {
        let failed_precondition = |message, round_data, staged_play| {
            self.streams.send_err(&client_player_id, Status::failed_precondition(message));
            LoveLetterState::PlayStaging(round_data, staged_play)
        };

        // Check: Is my turn
        if &client_player_id != round_data.players.current_turn_player_id() {
            return failed_precondition("Can't commit play, not your turn", round_data, staged_play);
        }

        // Do the following:
        // 0. Validate staged play
        // 1. Create new commit snapshot of the outcome
        // 2. Mutate player hands
        //
        // Do NOT do the following:
        // * Eliminate player
        // * increment turn cursor
        let committed_play_outcome: CommittedPlayOutcome = match staged_play.played_card {
            Card::Guard => {
                let (target_player_id, guessed_card) = {
                    // A small deficiency (unnecessary clone of 1 string) for a big readability gain
                    let staged_play_clone = staged_play.clone();
                    match (staged_play.target_player, staged_play.target_card) {
                        (Some(target_player_id), Some(target_card)) => (target_player_id, target_card),
                        (None, _) => return failed_precondition("To play Guard, you must select a target player", round_data, staged_play_clone),
                        (_, None) => return failed_precondition("To play Guard, you must select a target card", round_data, staged_play_clone),
                    }
                };

                // Check guess
                let actual_card = round_data.players
                    .get_card(&target_player_id)
                    .expect("Game is in unrecoverable, invalid state: Player selected is not in round.");
                let correct = guessed_card == actual_card;

                CommittedPlayOutcome::Guard {
                    target_player_id,
                    guessed_card,
                    correct,
                }
            },
            Card::Priest => {
                let target_player_id = match staged_play.target_player {
                    Some(target_player_id) => target_player_id,
                    None => return failed_precondition("To play Priest, you must select a target player", round_data, staged_play),
                };

                let opponent_card = round_data.players.get_card(&target_player_id)
                    .expect("Game is in unrecoverable, invalid state: Player selected is not in round.");

                CommittedPlayOutcome::Priest {
                    target_player_id,
                    opponent_card,
                }
            },
            Card::Baron => {
                let target_player_id = match staged_play.target_player {
                    Some(target_player_id) => target_player_id,
                    None => return failed_precondition("To play Baron, you must select a target player", round_data, staged_play),
                };

                // Eliminate player
                let client_card = round_data.players.get_card(&client_player_id)
                    .expect("Game is in unrecoverable, invalid state: Committing player did not have a card.");
                let other_card = round_data.players.get_card(&target_player_id)
                    .expect("Game is in unrecoverable, invalid state: Player targeted another player who isn't in round.");

                let eliminated_player_id_and_card = if client_card > other_card {
                    Some((target_player_id.clone(), other_card))
                } else if client_card < other_card {
                    Some((client_player_id.clone(), client_card))
                } else {
                    None
                };

                CommittedPlayOutcome::Baron {
                    target_player_id,
                    eliminated_player_id_and_card,
                }
            },
            Card::Handmaid => {
                round_data.handmaid_immunity_player_ids.insert(client_player_id.clone());
                CommittedPlayOutcome::Handmaid
            },
            Card::Prince => {
                let target_player_id = match staged_play.target_player {
                    Some(target_player_id) => target_player_id,
                    None => return failed_precondition("To play Prince, you must select a target player", round_data, staged_play),
                };

                // Discard and draw new card
                let new_card = round_data.deck.pop()
                    .expect("Round was incorrectly not ended when <= 1 card remained in the deck");
                let discarded_card = round_data.players.replace_card(target_player_id.clone(), new_card);

                CommittedPlayOutcome::Prince {
                    target_player_id,
                    discarded_card,
                }
            },
            Card::King => {
                let target_player_id = match staged_play.target_player {
                    Some(target_player_id) => target_player_id,
                    None => return failed_precondition("To play King, you must select a target player", round_data, staged_play),
                };
                CommittedPlayOutcome::King {
                    target_player_id,
                }
            },
            Card::Countess => CommittedPlayOutcome::Countess,
            Card::Princess => CommittedPlayOutcome::Princess,
        };

        // Update most recent commit
        let committed_play = CommittedPlay {
            committer_player_id: client_player_id,
            outcome: committed_play_outcome,
        };

        // Eliminate player and increment turn cursor
        match committed_play.player_id_to_eliminate() {
            None => round_data.players.increment_turn(),
            Some(player_id) => {
                let discarded = round_data.players.eliminate_and_increment_turn(&player_id);
                round_data.play_history.push(discarded);
            },
        }
        round_data.most_recent_play_details.replace(committed_play);

        // Clear next turn player from Handmaids (played from previous turn).
        round_data.handmaid_immunity_player_ids.remove(round_data.players.current_turn_player_id());

        // State transition
        let to_state = if round_data.players.remaining_player_ids().len() < 2 {
            LoveLetterState::RoundIntermission(
                self.complete_round(round_data),
                self.unready_player_list()
            )
        } else if round_data.deck.len() < 2 {
            LoveLetterState::RoundIntermission(
                self.complete_round(round_data),
                self.unready_player_list()
            )
        } else {
            LoveLetterState::TurnIntermission(
                round_data,
                self.unready_player_list()
            )
        };

        // Last thing: send result to all players
        self.send_game_state_to_all(&to_state);
        to_state
    }

    fn unready_player_list(&self) -> UnreadyPlayers {
        // As a later optimization (to UX), we can make only subset of players ready-up. For now,
        // we just require all players to ready up.
        UnreadyPlayers::new(self.game_data.player_id_turn_order.clone())
    }

    fn complete_round(&mut self, round_data: RoundData) -> RoundResult {
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

        RoundResult::new(final_card_by_player_id)
    }
}

impl CommittedPlay {
    pub fn player_id_to_eliminate(&self) -> Option<&String> {
        match &self.outcome {
            CommittedPlayOutcome::Guard { target_player_id, correct, .. } => {
                if *correct {
                    Some(target_player_id)
                } else {
                    None
                }
            },
            CommittedPlayOutcome::Baron { eliminated_player_id_and_card, .. } => {
                eliminated_player_id_and_card.as_ref().map(|x| &x.0)
            },
            CommittedPlayOutcome::Prince { target_player_id, discarded_card: Card::Princess, } => {
                Some(&target_player_id)
            },
            CommittedPlayOutcome::Princess => Some(&self.committer_player_id),
            _ => None,
        }
    }
}
