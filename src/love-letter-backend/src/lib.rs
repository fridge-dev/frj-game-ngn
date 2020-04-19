mod types;
mod state_machine;
mod deck;

use crate::types::Card;
use crate::state_machine::{LoveLetterInstanceState, LoveLetterStateMachine};
use backend_framework::{Holder, Players, ClientOut};

// ================= Actor =================

pub struct LoveLetterInstanceManager {
    state: Holder<LoveLetterInstanceState>,
    state_machine: LoveLetterStateMachine,
}

impl LoveLetterInstanceManager {
    pub fn new() -> Self {
        let players = Players::new();

        LoveLetterInstanceManager {
            state: Holder::new(LoveLetterInstanceState::WaitingForStart),
            state_machine: LoveLetterStateMachine::new(players),
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
    // Common(?)
    Join(String, Box<dyn ClientOut + Send>),
    StartGame(String),
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

