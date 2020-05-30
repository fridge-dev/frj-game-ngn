use crate::events::{Card, PlayCardSource};
use crate::state_machine::{LoveLetterStateMachine, LoveLetterState};
use crate::types::{RoundResult, RoundData, StagedPlay, CommittedPlay, CommittedPlayOutcome};
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

    /// 1: Mutate game state
    ///     a. Perform action
    ///     b. Eliminate player
    /// 2: State transition:
    ///     * TurnIntermission, OR
    ///     * RoundIntermission, OR
    ///     * GameEnd
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

        // Clear self from Handmaids (played from previous turn) after we validate it's our turn.
        // This is idempotent, so it's fine to happen before input validation. Also it's required
        // to have it before input validation, because round state is mutated during input
        // validation.
        round_data.handmaid_immunity_player_ids.remove(&client_player_id);

        // Sanity check: Deck is not empty
        let top_deck = round_data.deck.pop()
            .expect("Player should not be able to commit if round is over.");

        // Sanity check: Played card source is valid
        assert_eq!(
            staged_play.played_card,
            round_data.get_card_to_stage(&client_player_id, &staged_play.source),
            "Illegal game state: Staged play and card source have different."
        );

        // Discard current player's card and cycle new card
        // TODO:1 do this during staging
        let played_card = match staged_play.source {
            PlayCardSource::Hand => round_data.players.replace_card(client_player_id.clone(), top_deck),
            PlayCardSource::TopDeck => top_deck,
        };

        // Append to play history
        round_data.play_history.push(played_card);

        // Do the following:
        // 0. Validate staged play
        // 1. Create new commit snapshot of the outcome
        // 2. Mutate round state (player hands, play history)
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

                let eliminated_player_id = if client_card > other_card {
                    Some(target_player_id.clone())
                } else if client_card < other_card {
                    Some(client_player_id.clone())
                } else {
                    None
                };

                CommittedPlayOutcome::Baron {
                    target_player_id,
                    eliminated_player_id,
                    committer_card: client_card,
                    opponent_card: other_card,
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
        round_data.most_recent_play_details.replace(committed_play.clone());

        // Eliminate player and increment turn cursor
        match committed_play.player_id_to_eliminate() {
            None => round_data.players.increment_turn(),
            Some(player_id) => {
                let discarded = round_data.players.eliminate_and_increment_turn(&player_id);
                round_data.play_history.push(discarded);
            },
        }

        let to_state = if round_data.players.remaining_player_ids().len() < 2 {
            LoveLetterState::RoundIntermission(self.complete_round(round_data))
            // TODO:1 new API to start next round
        } else if round_data.deck.len() < 2 {
            LoveLetterState::RoundIntermission(self.complete_round(round_data))
            // TODO:1 new API to start next round
        } else {
            LoveLetterState::TurnIntermission(round_data, committed_play)
        };

        // Last thing: send result to all players
        self.send_game_state_to_all(&to_state);
        to_state
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
            CommittedPlayOutcome::Baron { eliminated_player_id, .. } => {
                eliminated_player_id.as_ref()
            },
            CommittedPlayOutcome::Prince { target_player_id, discarded_card: Card::Princess, } => {
                Some(&target_player_id)
            },
            CommittedPlayOutcome::Princess => Some(&self.committer_player_id),
            _ => None,
        }
    }
}
