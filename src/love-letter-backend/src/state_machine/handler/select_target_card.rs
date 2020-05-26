use crate::events::Card;
use crate::state_machine::{LoveLetterStateMachine, LoveLetterState};
use crate::types::{StagedPlay, RoundData};
use tonic::Status;

impl LoveLetterStateMachine {

    pub fn select_target_card(
        &self,
        from_state: LoveLetterState,
        client_player_id: String,
        target_card: Card
    ) -> LoveLetterState {
        match from_state {
            LoveLetterState::PlayStaging(round_data, staged_play) => {
                self.handle_card_selection(&client_player_id, target_card, round_data, staged_play)
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

    fn handle_card_selection(
        &self,
        client_player_id: &String,
        target_card: Card,
        round_data: RoundData,
        staged_play: StagedPlay,
    ) -> LoveLetterState {
        // Is my turn
        if client_player_id != round_data.players.current_turn_player_id() {
            self.streams.send_err(&client_player_id, Status::failed_precondition("Can't select target card, not your turn"));
            return LoveLetterState::PlayStaging(round_data, staged_play);
        }

        // Staged card needs a card selection
        if staged_play.played_card != Card::Guard {
            self.streams.send_err(&client_player_id, Status::failed_precondition("The card you played doesn't require selecting a target card"));
            return LoveLetterState::PlayStaging(round_data, staged_play);
        }

        // Guard-specific validation
        if staged_play.played_card == Card::Guard && target_card == Card::Guard {
            self.streams.send_err(&client_player_id, Status::failed_precondition("You cannot guess another player has 'Guard' for the Guard action."));
            return LoveLetterState::PlayStaging(round_data, staged_play);
        }

        // Apply update
        let mut staged_play = staged_play;
        staged_play.set_target_card(target_card);

        // Notify state change
        let to_state = LoveLetterState::PlayStaging(round_data, staged_play);
        self.send_game_state_to_all(&to_state);

        to_state
    }
}