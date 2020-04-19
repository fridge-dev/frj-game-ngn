pub mod prng;
pub mod wire_api;
pub mod shuffler;

use crate::wire_api::proto_frj_ngn::ProtoPreGameMessage;
use std::fmt::Debug;
use tonic::Code;

pub struct Players {
    inner: Vec<PlayerData>,
    party_leader_index: usize,
}

struct PlayerData {
    pub player_id: String,
    pub client_out: Box<dyn ClientOut + Send>,
}

impl Players {

    pub fn new() -> Self {
        Players {
            inner: Vec::new(),
            party_leader_index: 0,
        }
    }

    pub fn add(&mut self, player_id: String, client_out: Box<dyn ClientOut + Send>) {
        self.inner.push(PlayerData { player_id, client_out });
    }

    pub fn contains(&self, player_id: &String) -> bool {
        self.find_player(player_id).is_some()
    }

    // O(n), could be O(1), but n will always be less than 10.
    fn find_player(&self, player_id: &String) -> Option<&PlayerData> {
        for player in self.inner.iter() {
            if player_id == &player.player_id {
                return Some(player);
            }
        }

        None
    }

    pub fn count(&self) -> usize {
        self.inner.len()
    }

    pub fn player_ids(&self) -> Vec<String> {
        self.inner
            .iter()
            .map(|player| player.player_id.clone())
            .collect()
    }

    pub fn party_leader(&self) -> Option<&String> {
        match self.inner.get(self.party_leader_index) {
            None => None,
            Some(player) => Some(&player.player_id)
        }
    }

    pub fn party_leader_and_all_player_ids(&self) -> (usize, Vec<String>) {
        (self.party_leader_index, self.player_ids())
    }

    pub fn send_pre_game_message(
        &self,
        player_id: &String,
        message: impl Into<ProtoPreGameMessage>
    ) {
        self.out_stream(player_id, |out| out.send_pre_game_message(message.into()))
    }

    pub fn send_error_message(
        &self,
        player_id: &String,
        message: impl Into<String>,
        err_type: MessageErrType
    ) {
        self.out_stream(player_id, |out| out.send_error_message(message.into(), err_type))
    }

    fn out_stream<F>(
        &self,
        player_id: &String,
        send_func: F
    ) where
        F: FnOnce(&Box<dyn ClientOut + Send>) -> ()
    {
        if let Some(player) = self.find_player(player_id) {
            send_func(&player.client_out);
        } else {
            println!("ERROR: Cannot send message, Player '{}' not found.", player_id);
        }
    }
}

pub trait ClientOut: Debug {

    fn send_pre_game_message(&self, message: ProtoPreGameMessage);

    fn send_error_message(&self, message: String, err_type: MessageErrType);
}

#[derive(Copy, Clone, Debug)]
pub enum MessageErrType {
    ServerFault,
    InvalidReq,
}

impl From<MessageErrType> for Code {
    fn from(err_type: MessageErrType) -> Self {
        match err_type {
            MessageErrType::ServerFault => Code::Internal,
            MessageErrType::InvalidReq => Code::InvalidArgument,
        }
    }
}

/// A simplified interface for an Option where you expect to
/// alternate between `Some` and `None` by taking the inner
/// value out.
pub struct Holder<T>(Option<T>);

impl<T> Holder<T> {
    pub fn new(item: T) -> Self {
        Holder(Some(item))
    }

    pub fn take(&mut self) -> T {
        self.0.take().expect("Invalid state: Holder.take() called when it was empty")
    }

    pub fn put(&mut self, item: T) {
        if self.0.is_some() {
            panic!("Invalid state: Holder.put() called when it was full");
        }
        self.0.replace(item);
    }
}
