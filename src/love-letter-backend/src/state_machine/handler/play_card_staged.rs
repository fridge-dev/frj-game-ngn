use crate::state_machine::{StateMachineEventHandler, LoveLetterInstanceState};
use crate::PlayCardSource;
use crate::types::StagedPlay;
use backend_framework::streaming::MessageErrType;

impl StateMachineEventHandler {

    pub fn play_card_staged(
        &mut self,
        from_state: LoveLetterInstanceState,
        player_id: String,
        card_source: PlayCardSource
    ) -> LoveLetterInstanceState {
        if !self.players.contains_player(&player_id) {
            // TODO notify caller of err?
            return from_state;
        }

        match from_state {
            LoveLetterInstanceState::WaitingForStart => {
                // TODO idempotency?
                self.players.send_pre_game_error(&player_id, "Can't play before game has started", MessageErrType::InvalidReq);

                // No state change
                LoveLetterInstanceState::WaitingForStart
            },
            LoveLetterInstanceState::InProgressStaged(game_data, staged_play) => {
                // Is my turn
                if &player_id != game_data.current_player_turn() {
                    self.players.send_pre_game_error(&player_id, "Can't play card, not your turn", MessageErrType::InvalidReq);
                    return LoveLetterInstanceState::InProgressStaged(game_data, staged_play)
                }

                // Idempotent check
                let card_to_stage = game_data.current_round.get_card_to_play(&player_id, &card_source);
                if card_to_stage == staged_play.card {
                    // TODO send ACK to only requesting player
                    // Or send player some type of message telling
                    // them to re-get state
                } else {
                    self.players.send_pre_game_error(&player_id, "Can't play card while pending commit", MessageErrType::InvalidReq);
                }

                // No state change
                LoveLetterInstanceState::InProgressStaged(game_data, staged_play)
            },
            LoveLetterInstanceState::InProgress(game_data) => {
                if game_data.current_player_turn() != &player_id {
                    self.players.send_pre_game_error(&player_id, "Can't play card, not your turn", MessageErrType::InvalidReq);

                    // No state change
                    return LoveLetterInstanceState::InProgress(game_data);
                }

                let card_to_stage = game_data.current_round.get_card_to_play(&player_id, &card_source);

                // TODO if selection not-needed, auto-commit

                LoveLetterInstanceState::InProgressStaged(game_data, StagedPlay::new(card_to_stage))
            },
        }
    }

}