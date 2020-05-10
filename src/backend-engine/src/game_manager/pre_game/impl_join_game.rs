use crate::game_manager::pre_game::PreGameInstanceManager;
use backend_framework::streaming::StreamSender;
use backend_framework::wire_api::proto_frj_ngn::{ProtoPreGameMessage, ProtoGameType};
use backend_framework::wire_api::proto_frj_ngn::proto_pre_game_message::{ProtoJoinGameAck, ProtoPlayerJoinMsg};
use tonic::Status;

impl PreGameInstanceManager {

    pub fn add_player(
        &mut self,
        player_id: String,
        client_stream: StreamSender<ProtoPreGameMessage>,
    ) {
        // Reconnect
        if self.players.contains_player(&player_id) {
            self.add_player_and_send_ack(player_id, client_stream);
            return;
        }

        // Check max players
        if self.players.count() >= self.max_players {
            if let Err(_) = client_stream.send_error_message(Status::failed_precondition("Can't join, game has max players")) {
                println!("INFO: Client dropped before we sent join rejection response.");
            }
            return;
        }

        self.add_player_and_send_ack(player_id.clone(), client_stream);
        self.notify_other_players(player_id);
    }

    fn add_player_and_send_ack(&mut self, player_id: String, client_stream: StreamSender<ProtoPreGameMessage>) {
        self.players.add_player(player_id.clone(), client_stream);
        let host_player_id = self.players.party_leader()
            .expect("Party leader should always exist immediately after an add.")
            .to_owned();

        let mut other_player_ids = self.players.player_ids();
        other_player_ids.retain(|id| { id != &host_player_id });

        self.players.send_pre_game_message(&player_id, ProtoJoinGameAck {
            game_type: ProtoGameType::from(self.game_type).into(),
            host_player_id,
            other_player_ids,
        })
    }

    fn notify_other_players(&mut self, new_player_id: String) {
        let mut players_to_notify = self.players.player_ids();
        players_to_notify.retain(|pid| pid != &new_player_id);

        let msg = ProtoPlayerJoinMsg {
            player_id: new_player_id
        };

        for existing_player_id in players_to_notify.iter() {
            self.players.send_pre_game_message(existing_player_id, msg.clone());
        }
    }
}
