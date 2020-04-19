pub mod prng;
pub mod wire_api;
pub mod shuffler;

use crate::wire_api::proto_frj_ngn::ProtoPreGameMessage;
use std::collections::HashMap;
use std::fmt::Debug;
use tonic::Code;

pub struct Players {
    out_by_player_id: HashMap<String, Box<dyn ClientOut + Send>>,
    // TODO lifetimes learning opportunity:
    //      use reference here, because the string is always a key in the map.
    // TODO If Playsers was only instantiated at the time when a new player joins,
    //      then we could avoid Option.
    party_leader_player_id: Option<String>,
}

impl Players {

    pub fn new() -> Self {
        Players {
            out_by_player_id: HashMap::new(),
            party_leader_player_id: None
        }
    }

    pub fn add(&mut self, player_id: String, client_out: Box<dyn ClientOut + Send>) {
        if self.party_leader_player_id.is_none() {
            self.party_leader_player_id = Some(player_id.clone());
        }
        self.out_by_player_id.insert(player_id, client_out);
    }

    pub fn contains(&self, player_id: &String) -> bool {
        self.out_by_player_id.contains_key(player_id)
    }

    pub fn count(&self) -> usize {
        self.out_by_player_id.len()
    }

    pub fn player_ids(&self) -> Vec<String> {
        self.out_by_player_id
            .keys()
            .map(|k| k.to_owned())
            .collect()
    }

    pub fn party_leader(&self) -> &Option<String> {
        &self.party_leader_player_id
    }

    pub fn party_leader_and_all_player_ids(&self) -> (Option<usize>, Vec<String>) {
        let player_ids = self.player_ids();
        let index_of_party_leader = match &self.party_leader_player_id {
            None => None,
            Some(party_leader) => player_ids.iter().position(|pid| pid == party_leader),
        };

        (index_of_party_leader, player_ids)
    }

    fn out_stream<F: FnOnce(&Box<dyn ClientOut + Send>) -> ()>(&self, player_id: &String, send_func: F) {
        match self.out_by_player_id.get(player_id) {
            None => println!("ERROR: Cannot send message, Player '{}' not found.", player_id),
            Some(out) => send_func(out),
        }
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
