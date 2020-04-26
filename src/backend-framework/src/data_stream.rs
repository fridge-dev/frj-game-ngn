use crate::data_stream::immutable::PlayerIds;
use crate::streaming::{StreamSender, MessageErrType};
use tonic::Status;
use std::collections::HashMap;

pub struct PlayerDataStreams<M: prost::Message> {
    player_ids: PlayerIds,
    streams: HashMap<String, StreamSender<M>>,
}

impl<M: prost::Message> PlayerDataStreams<M> {

    pub fn new(player_ids: Vec<String>) -> Self {
        PlayerDataStreams {
            player_ids: PlayerIds::new(player_ids),
            streams: HashMap::new(),
        }
    }

    pub fn contains(&self, player_id: &String) -> bool {
        self.player_ids.contains(player_id)
    }

    pub fn add_stream(&mut self, player_id: String, stream: StreamSender<M>) {
        if self.player_ids.contains(&player_id) {
            // Idempotent reconnect
            self.streams.insert(player_id, stream);
        } else {
            stream.disconnect_with_err(Status::permission_denied("You are not a player in this game."));
        }
    }

    /// Intentionally avoiding to update state when a disconnected stream is detected
    /// because it results in a cascading `mut` up the call chain, that's otherwise not
    /// required.
    pub fn send_msg(&self, player_id: &String, message: impl Into<M>) {
        if let Some(stream) = self.streams.get(player_id) {
            let _ = stream.send_message(message.into());
        }
    }

    /// Intentionally avoiding to update state when a disconnected stream is detected
    /// because it results in a cascading `mut` up the call chain, that's otherwise not
    /// required.
    pub fn send_err(&self, player_id: &String, message: impl Into<String>, err_type: MessageErrType) {
        if let Some(stream) = self.streams.get(player_id) {
            let _ = stream.send_error_message(message.into(), err_type);
        }
    }
}

mod immutable {
    pub(crate) struct PlayerIds(Vec<String>);

    impl PlayerIds {
        pub fn new(player_ids: Vec<String>) -> Self {
            PlayerIds(player_ids)
        }

        pub fn contains(&self, player_id: &String) -> bool {
            self.0.contains(player_id)
        }
    }
}