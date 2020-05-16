use crate::events::PlayCardSource;
use crate::state_machine::{LoveLetterStateMachineEventHandler, LoveLetterState};
use crate::types::StagedPlay;
use tonic::Status;

impl LoveLetterStateMachineEventHandler {

    pub fn play_card_staged(
        &self,
        from_state: LoveLetterState,
        player_id: String,
        card_source: PlayCardSource
    ) -> LoveLetterState {
        match from_state {
            LoveLetterState::PlayStaging(game_data, staged_play) => {
                // Is my turn
                if &player_id != game_data.current_round.players.current_turn_player_id() {
                    self.streams.send_err(&player_id, Status::failed_precondition("Can't play card, not your turn"));
                    return LoveLetterState::PlayStaging(game_data, staged_play)
                }

                // Idempotent check
                let card_to_stage = game_data.current_round.get_card_to_stage(&player_id, &card_source);
                if card_to_stage == staged_play.card {
                    // TODO send ACK to only requesting player
                    // Or send player some type of message telling them to re-get state
                } else {
                    self.streams.send_err(&player_id, Status::failed_precondition("Can't play card while pending commit"));
                }

                // No state change
                LoveLetterState::PlayStaging(game_data, staged_play)
            },
            LoveLetterState::PlayPending(game_data) => {
                if &player_id != game_data.current_round.players.current_turn_player_id() {
                    self.streams.send_err(&player_id, Status::failed_precondition("Can't play card, not your turn"));

                    // No state change
                    return LoveLetterState::PlayPending(game_data);
                }

                let card_to_stage = game_data.current_round.get_card_to_stage(&player_id, &card_source);

                // TODO if selection not-needed, auto-commit

                LoveLetterState::PlayStaging(game_data, StagedPlay::new(card_to_stage))
            }
            LoveLetterState::TurnIntermission(_) => {
                // TODO handle staging
                unimplemented!("LoveLetterStateMachineEventHandler.play_card_staged(TurnIntermission)");
            }
            LoveLetterState::RoundIntermission(_) => {
                // TODO handle staging
                unimplemented!("LoveLetterStateMachineEventHandler.play_card_staged(RoundIntermission)");
            }
        }
    }

}