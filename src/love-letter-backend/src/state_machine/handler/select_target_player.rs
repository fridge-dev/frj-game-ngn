use crate::state_machine::{LoveLetterStateMachine, LoveLetterState};
use tonic::Status;
use crate::types::{RoundData, StagedPlay};
use crate::events::Card;

impl LoveLetterStateMachine {

    pub fn select_target_player(
        &self,
        from_state: LoveLetterState,
        client_player_id: String,
        target_player_id: String
    ) -> LoveLetterState {
        match from_state {
            LoveLetterState::PlayStaging(round_data, staged_play) => {
                self.handle_player_selection(&client_player_id, target_player_id, round_data, staged_play)
            },
            _ => {
                // Missing: Player ID validation
                // Missing: Card validation
                // But this doesn't matter, we just drop the event and proactively update the client's state.
                self.send_game_state(&from_state, &client_player_id);
                from_state
            },
        }
    }

    fn handle_player_selection(
        &self,
        client_player_id: &String,
        target_player_id: String,
        round_data: RoundData,
        staged_play: StagedPlay,
    ) -> LoveLetterState {
        // Check: Is my turn
        if client_player_id != round_data.players.current_turn_player_id() {
            self.streams.send_err(&client_player_id, Status::failed_precondition("Can't select target player, not your turn"));
            return LoveLetterState::PlayStaging(round_data, staged_play);
        }

        // Check: selected player is still in game
        if !round_data.players.remaining_player_ids().contains(&target_player_id) {
            self.streams.send_err(&client_player_id, Status::failed_precondition("Selected player is not in the round."));
            return LoveLetterState::PlayStaging(round_data, staged_play);
        }

        // 1. Staged card needs a player selection
        // 2. Card-specific validation:
        //    a. Selected self
        match staged_play.played_card {
            Card::Guard | Card::Priest | Card::Baron | Card::King => {
                if client_player_id == &target_player_id {
                    if there_exists_a_non_self_targetable_player(&round_data, client_player_id) {
                        self.streams.send_err(&client_player_id, Status::failed_precondition("Cannot select self"));
                        return LoveLetterState::PlayStaging(round_data, staged_play);
                    }
                    // else, valid
                }
            },
            Card::Prince => { /* No card-specific validation */ },
            _ => {
                self.streams.send_err(&client_player_id, Status::failed_precondition("The card you played doesn't require selecting a target player"));
                return LoveLetterState::PlayStaging(round_data, staged_play);
            }
        }

        // Check: selected player is not Handmaid
        if round_data.handmaid_immunity_player_ids.contains(&target_player_id) {
            self.streams.send_err(&client_player_id, Status::failed_precondition("Selected player is Handmaid."));
            return LoveLetterState::PlayStaging(round_data, staged_play);
        }

        // Apply update
        let mut staged_play = staged_play;
        staged_play.set_target_player(target_player_id);

        // Notify state change
        let to_state = LoveLetterState::PlayStaging(round_data, staged_play);
        self.send_game_state_to_all(&to_state);

        to_state
    }
}

fn there_exists_a_non_self_targetable_player(round_data: &RoundData, client_player_id: &String) -> bool {
    for p in round_data.players.remaining_player_ids() {
        if p != client_player_id && !round_data.handmaid_immunity_player_ids.contains(p) {
            return true;
        }
    };

    false
}
