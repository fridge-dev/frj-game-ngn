mod transitions;
pub(crate) mod data;

use crate::types::PlayerSide;
use crate::events::MastermindEventType;
use crate::state_machine::data::{PregameData, ActiveData, LActiveRDoneData, LDoneRActiveData, DoneData};

// --------------------- State ---------------------

pub enum BoardState {
    Pregame(PregameData),
    Active(ActiveData),
    LActiveRDone(LActiveRDoneData),
    LDoneRActive(LDoneRActiveData),
    Done(DoneData),
}

// --------------------- State Machine ---------------------

struct MastermindStateMachineImpl {
    // nothing
}

pub struct MastermindStateMachine {
    inner: MastermindStateMachineImpl,
}

impl MastermindStateMachine {
    pub fn new() -> Self {
        MastermindStateMachine {
            inner: MastermindStateMachineImpl {}
        }
    }

    pub fn handle_transition(
        &self,
        mut from_state: BoardState,
        player: PlayerSide,
        event: MastermindEventType,
    ) -> BoardState {
        match event {
            MastermindEventType::RegisterDataStream(_) => {
                unimplemented!("MastermindEventType::RegisterDataStream")
            },
            MastermindEventType::GetGameState => {
                self.inner.send_game_state(&from_state, player);
                from_state
            },
            MastermindEventType::SubmitPassword => {
                self.inner.submit_password(from_state, player)
            }
            MastermindEventType::PutPeg { peg, color } => {
                self.inner.put_peg(&mut from_state, player, peg, color);
                from_state
            },
            MastermindEventType::CommitRow => {
                self.inner.commit_row(from_state, player)
            },
        }
    }
}
