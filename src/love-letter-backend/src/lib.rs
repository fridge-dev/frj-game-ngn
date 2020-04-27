mod types;
mod state_machine;
mod deck;

use crate::types::{Card, GameData};
use crate::state_machine::{LoveLetterState, LoveLetterStateMachine};
use backend_framework::common_types::ClientInfo;
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
pub struct LoveLetterEvent {
    // TODO this unnecessarily leaks `game_id` into individual instance managers
    pub client: ClientInfo,
    pub payload: LoveLetterEventType,
}

#[derive(Debug)]
pub enum LoveLetterEventType {
    // Common
    RegisterDataStream(StreamSender<ProtoLoveLetterDataOut>),
    GetGameState,

    // Game-specific
    PlayCardStaged(PlayCardSource),
    SelectTargetPlayer(String),
    SelectTargetCard(Card),
    PlayCardCommit,
}

#[derive(Debug)]
pub enum PlayCardSource {
    Hand,
    TopDeck,
}
