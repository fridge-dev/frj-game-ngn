use crate::events::PlayCardSource;
use crate::state_machine::{LoveLetterStateMachine, LoveLetterState};
use crate::types::StagedPlay;
use tonic::Status;

impl LoveLetterStateMachine {

    pub fn play_card_staged(
        &self,
        from_state: LoveLetterState,
        player_id: String,
        card_source: PlayCardSource
    ) -> LoveLetterState {
        match from_state {
            LoveLetterState::PlayStaging(round_data, staged_play) => {
                // Is my turn
                if &player_id != round_data.players.current_turn_player_id() {
                    self.streams.send_err(&player_id, Status::failed_precondition("Can't play card, not your turn"));
                    return LoveLetterState::PlayStaging(round_data, staged_play)
                }

                // Idempotent check
                let card_to_stage = round_data.get_card_to_stage(&player_id, &card_source);
                if card_to_stage != staged_play.played_card {
                    self.streams.send_err(&player_id, Status::failed_precondition("Can't play card while pending commit"));
                }

                // No state change
                let to_state = LoveLetterState::PlayStaging(round_data, staged_play);

                // Notify caller of latest game state
                self.send_game_state(&to_state, &player_id);

                to_state
            },
            LoveLetterState::PlayPending(round_data) => {
                // Is my turn
                if &player_id != round_data.players.current_turn_player_id() {
                    self.streams.send_err(&player_id, Status::failed_precondition("Can't play card, not your turn"));
                    return LoveLetterState::PlayPending(round_data);
                }

                let card_to_stage = round_data.get_card_to_stage(&player_id, &card_source);

                // TODO:2 if selection not-needed, auto-commit

                LoveLetterState::PlayStaging(round_data, StagedPlay::new(card_to_stage, card_source))
            },
            _ => {
                self.streams.send_err(&player_id, Status::failed_precondition("Can't play card while in current state"));
                from_state
            },
        }
    }

}
