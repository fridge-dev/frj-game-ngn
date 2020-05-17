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
            LoveLetterState::PlayPending(_, _) => from_state,
            LoveLetterState::PlayStaging(game_data, round_data, mut staged_play) => {
                // TODO check if player is still in game
                staged_play.set_target_player(target_player_id);
                LoveLetterState::PlayStaging(game_data, round_data, staged_play)
            }
            LoveLetterState::TurnIntermission(_, _) => {
                unimplemented!("LoveLetterStateMachineEventHandler.select_target_player(TurnIntermission)");
            }
            LoveLetterState::RoundIntermission(_, _) => {
                unimplemented!("LoveLetterStateMachineEventHandler.select_target_player(RoundIntermission)");
            }
        }
    }

}