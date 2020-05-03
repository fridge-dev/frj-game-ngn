use crate::events::Card;
use crate::state_machine::{LoveLetterStateMachineEventHandler, LoveLetterState};

impl LoveLetterStateMachineEventHandler {

    pub fn select_target_card(
        &self,
        from_state: LoveLetterState,
        client_player_id: String,
        target_card: Card
    ) -> LoveLetterState {
        // TODO being lazy, fill out full match statement... Only happy path for now
        match from_state {
            LoveLetterState::InProgress(_) => from_state,
            LoveLetterState::InProgressStaged(game_data, mut staged_play) => {
                staged_play.set_target_card(target_card);
                LoveLetterState::InProgressStaged(game_data, staged_play)
            },
        }
    }

}