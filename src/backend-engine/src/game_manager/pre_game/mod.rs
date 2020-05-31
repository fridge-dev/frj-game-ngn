use crate::game_manager::types::GameType;

mod impl_join_game;
mod impl_start_game;

// ----------- PreGameInstanceManager -----------

pub(crate) struct PreGameInstanceManager {
    pub game_type: GameType,
    min_players: usize,
    max_players: usize,
    players: streaming::PlayerPreGameStreams,
}

impl PreGameInstanceManager {

    pub fn new(game_type: GameType) -> Self {
        let (min, max) = player_count_min_max(&game_type);

        PreGameInstanceManager {
            game_type,
            min_players: min,
            max_players: max,
            players: streaming::PlayerPreGameStreams::new(),
        }
    }
}

fn player_count_min_max(game_type: &GameType) -> (usize, usize) {
    match game_type {
        GameType::LoveLetter => (2, 4),
        GameType::LostCities => (2, 2),
    }
}

// ----------- PlayerPreGameStreams -----------

mod streaming {
    use backend_framework::streaming::StreamSender;
    use backend_framework::wire_api::proto_frj_ngn::ProtoPreGameMessage;
    use tonic::Status;

    pub(crate) struct PlayerPreGameStreams {
        inner: Vec<PlayerData>,
        party_leader_index: usize,
    }

    struct PlayerData {
        pub player_id: String,
        pub pre_game_stream: StreamSender<ProtoPreGameMessage>,
    }

    impl PlayerPreGameStreams {

        pub fn new() -> Self {
            PlayerPreGameStreams {
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

        pub fn send_pre_game_message_err(
            &mut self,
            player_id: &String,
            status: Status
        ) {
            self.out_stream(player_id, |out| out.send_error_message(status))
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

}