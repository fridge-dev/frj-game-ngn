use crate::state_machine::{StateMachineEventHandler, LoveLetterInstanceState};
use crate::types::Card;

impl StateMachineEventHandler {

    pub fn select_target_card(
        &self,
        from_state: LoveLetterInstanceState,
        client_player_id: String,
        target_card: Card
    ) -> LoveLetterInstanceState {
        // TODO being lazy, fill out full match statement... Only happy path for now
        match from_state {
            LoveLetterInstanceState::WaitingForStart => from_state,
            LoveLetterInstanceState::InProgress(_) => from_state,
            LoveLetterInstanceState::InProgressStaged(game_data, mut staged_play) => {
                staged_play.set_target_card(target_card);
                LoveLetterInstanceState::InProgressStaged(game_data, staged_play)
            },
        }
    }

}