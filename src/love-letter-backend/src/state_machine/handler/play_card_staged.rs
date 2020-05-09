use crate::events::PlayCardSource;
use crate::state_machine::{LoveLetterStateMachineEventHandler, LoveLetterState};
use crate::types::StagedPlay;
use backend_framework::streaming::MessageErrType;

impl LoveLetterStateMachineEventHandler {

    pub fn play_card_staged(
        &self,
        from_state: LoveLetterState,
        player_id: String,
        card_source: PlayCardSource
    ) -> LoveLetterState {
        match from_state {
            LoveLetterState::InProgressStaged(game_data, staged_play) => {
                // Is my turn
                if &player_id != game_data.current_player_turn() {
                    self.streams.send_err(&player_id, "Can't play card, not your turn", MessageErrType::InvalidReq);
                    return LoveLetterState::InProgressStaged(game_data, staged_play)
                }

                // Idempotent check
                let card_to_stage = game_data.current_round.get_card_to_play(&player_id, &card_source);
                if card_to_stage == staged_play.card {
                    // TODO send ACK to only requesting player
                    // Or send player some type of message telling
                    // them to re-get state
                } else {
                    self.streams.send_err(&player_id, "Can't play card while pending commit", MessageErrType::InvalidReq);
                }

                // No state change
                LoveLetterState::InProgressStaged(game_data, staged_play)
            },
            LoveLetterState::InProgress(game_data) => {
                if game_data.current_player_turn() != &player_id {
                    self.streams.send_err(&player_id, "Can't play card, not your turn", MessageErrType::InvalidReq);

                    // No state change
                    return LoveLetterState::InProgress(game_data);
                }

                let card_to_stage = game_data.current_round.get_card_to_play(&player_id, &card_source);

                // TODO if selection not-needed, auto-commit

                LoveLetterState::InProgressStaged(game_data, StagedPlay::new(card_to_stage))
            },
        }
    }

}