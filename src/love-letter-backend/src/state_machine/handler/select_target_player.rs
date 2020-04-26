use crate::state_machine::{LoveLetterStateMachineEventHandler, LoveLetterState};

impl LoveLetterStateMachineEventHandler {

    pub fn select_target_player(
        &self,
        from_state: LoveLetterState,
        client_player_id: String,
        target_player_id: String
    ) -> LoveLetterState {
        // TODO being lazy, fill out full match statement... Only happy path for now
        match from_state {
            LoveLetterState::InProgress(_) => from_state,
            LoveLetterState::InProgressStaged(game_data, mut staged_play) => {
                // TODO check if player is still in game
                staged_play.set_target_player(target_player_id);
                LoveLetterState::InProgressStaged(game_data, staged_play)
            },
        }
    }

}