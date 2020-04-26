mod types;
mod state_machine;
mod deck;

use crate::types::{Card, GameData};
use crate::state_machine::{LoveLetterInstanceState, LoveLetterStateMachine};
use backend_framework::holder::Holder;
use backend_framework::streaming::{PlayerPreGameStreams, StreamSender};
use backend_framework::wire_api::proto_frj_ngn::{ProtoStartGameReply, ProtoPreGameMessage};
use tokio::sync::oneshot;

// ================= Actor =================

pub struct LoveLetterInstanceManager {
    player_ids: Vec<String>, // TODO is needed?
    state: Holder<LoveLetterInstanceState>,
    state_machine: LoveLetterStateMachine,
}

impl LoveLetterInstanceManager {

    pub fn new2(player_ids: Vec<String>) -> Self {
        LoveLetterInstanceManager {
            player_ids: player_ids.clone(),
            state: Holder::new(LoveLetterInstanceState::InProgress(GameData::new(player_ids))),
            state_machine: LoveLetterStateMachine::new2(),
        }
    }

    pub fn new() -> Self {
        let players = PlayerPreGameStreams::new();

        LoveLetterInstanceManager {
            player_ids: Vec::new(),
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
    JoinGame(String, StreamSender<ProtoPreGameMessage>),
    StartGame(String, oneshot::Sender<ProtoStartGameReply>),
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

