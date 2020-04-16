use crate::state_machine::{StateMachineEventHandler, LoveLetterInstanceState};

impl StateMachineEventHandler {

    pub fn select_target_player(
        &self,
        from_state: LoveLetterInstanceState,
        client_player_id: String,
        target_player_id: String
    ) -> LoveLetterInstanceState {
        // TODO being lazy, fill out full match statement... Only happy path for now
        match from_state {
            LoveLetterInstanceState::WaitingForStart => from_state,
            LoveLetterInstanceState::InProgress(_) => from_state,
            LoveLetterInstanceState::InProgressStaged(game_data, mut staged_play) => {
                // TODO check if player is still in game
                staged_play.set_target_player(target_player_id);
                LoveLetterInstanceState::InProgressStaged(game_data, staged_play)
            },
        }
    }

}