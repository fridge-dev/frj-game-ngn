use crate::wire_api::proto_frj_ngn::ProtoPreGameMessage;
use std::fmt::{Debug, Formatter};
use tonic::{Code, Status};
use std::fmt;
use tokio::sync::mpsc;

pub struct PlayerStreams {
    inner: Vec<PlayerData>,
    party_leader_index: usize,
}

struct PlayerData {
    pub player_id: String,
    pub pre_game_stream: StreamSender<ProtoPreGameMessage>,
}

impl PlayerStreams {

    pub fn new() -> Self {
        PlayerStreams {
            inner: Vec::new(),
            party_leader_index: 0,
        }
    }

    pub fn add_player(&mut self, player_id: String, pre_game_stream: StreamSender<ProtoPreGameMessage>) {
        self.remove_player(&player_id);
        self.inner.push(PlayerData { player_id, pre_game_stream });
    }

    // O(n), could be O(1), but n will always be less than 10.
    pub fn remove_player(&mut self, player_id: &String) {
        self.inner.retain(|player| &player.player_id != player_id)
    }

    pub fn contains_player(&self, player_id: &String) -> bool {
        self.find_player(player_id).is_some()
    }

    // O(n), could be O(1), but n will always be less than 10.
    fn find_player(&self, player_id: &String) -> Option<&PlayerData> {
        self.inner
            .iter()
            .find(|&player| &player.player_id == player_id)
//        for player in self.inner.iter() {
//            if player_id == &player.player_id {
//                return Some(player);
//            }
//        }
//
//        None
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

    pub fn send_pre_game_message(
        &mut self,
        player_id: &String,
        message: impl Into<ProtoPreGameMessage>
    ) {
        self.out_stream(player_id, |out| out.send_message(message.into()))
    }

    pub fn send_pre_game_error(
        &mut self,
        player_id: &String,
        message: impl Into<String>,
        err_type: MessageErrType
    ) {
        self.out_stream(player_id, |out| out.send_error_message(message.into(), err_type))
    }

    fn out_stream<F>(
        &mut self,
        player_id: &String,
        send_func: F
    ) where
        F: FnOnce(&StreamSender<ProtoPreGameMessage>) -> Result<(), ()>
    {
        if let Some(player) = self.find_player(player_id) {
            if let Err(_) = send_func(&player.pre_game_stream) {
                self.remove_player(&player_id);
            }
        } else {
            println!("ERROR: Cannot send message, Player '{}' not found.", player_id);
        }
    }
}

pub struct StreamSender<M: prost::Message> {
    sender: mpsc::UnboundedSender<Result<M, Status>>,
}

impl<M: prost::Message> StreamSender<M> {
    pub fn new(sender: mpsc::UnboundedSender<Result<M, Status>>) -> Self {
        StreamSender {
            sender
        }
    }

    pub fn send_message(&self, message: M) -> Result<(), ()> {
        self.sender.send(Ok(message))
            .map_err(|msg| println!("WARN: Client stream dropped. We failed to send message: {:?}", msg))
    }

    pub fn send_error_message(&self, message: String, err_type: MessageErrType) -> Result<(), ()> {
        let status = Status::new(
            Code::from(err_type),
            message
        );
        self.sender.send(Err(status))
            .map_err(|msg| println!("WARN: Client stream dropped. We failed to send message: {:?}", msg))
    }
}

impl<M: prost::Message> Debug for StreamSender<M> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "StreamSender {{...}}")
    }
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
