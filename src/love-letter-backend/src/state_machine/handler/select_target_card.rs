use crate::events::Card;
use crate::state_machine::{LoveLetterStateMachine, LoveLetterState};

impl LoveLetterStateMachine {

    pub fn select_target_card(
        &self,
        from_state: LoveLetterState,
        client_player_id: String,
        target_card: Card
    ) -> LoveLetterState {
        // TODO being lazy, fill out full match statement... Only happy path for now
        match from_state {
            LoveLetterState::PlayPending(_) => from_state,
            LoveLetterState::PlayStaging(round_data, mut staged_play) => {
                staged_play.set_target_card(target_card);
                LoveLetterState::PlayStaging(round_data, staged_play)
            }
            LoveLetterState::TurnIntermission(_) => {
                unimplemented!("LoveLetterStateMachineEventHandler.select_target_card(TurnIntermission)");
            }
            LoveLetterState::RoundIntermission(_) => {
                unimplemented!("LoveLetterStateMachineEventHandler.select_target_card(RoundIntermission)");
            }
        }
    }

}