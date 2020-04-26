mod types;
mod state_machine;
mod deck;

use crate::types::{Card, GameData};
use crate::state_machine::{LoveLetterState, LoveLetterStateMachine};
use backend_framework::holder::Holder;
use backend_framework::streaming::StreamSender;
use backend_framework::wire_api::proto_frj_ngn::ProtoLoveLetterDataOut;

// ================= Actor =================

pub struct LoveLetterInstanceManager {
    state: Holder<LoveLetterState>,
    state_machine: LoveLetterStateMachine,
}

impl LoveLetterInstanceManager {

    pub fn new(player_ids: Vec<String>) -> Self {
        LoveLetterInstanceManager {
            state: Holder::new(LoveLetterState::InProgress(GameData::new(player_ids.clone()))),
            state_machine: LoveLetterStateMachine::new(player_ids),
        }
    }

    pub fn handle_event(&mut self, event: LoveLetterEvent) {
        let from_state = self.state.take();
        let to_state = self.state_machine.transition_state(from_state, event);
        self.state.put(to_state);
    }
}

// ================= Inputs =================

#[derive(Debug)]
pub enum LoveLetterEvent {
    // Common
    RegisterDataStream(String, StreamSender<ProtoLoveLetterDataOut>),
    GetGameState(String),

    // Game-specific
    PlayCardStaged(String, PlayCardSource),
    SelectTargetPlayer(String, String),
    SelectTargetCard(String, Card),
    PlayCardCommit(String),
}

#[derive(Debug)]
pub enum PlayCardSource {
    Hand,
    TopDeck,
}

