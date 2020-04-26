use crate::game_manager::pre_game::PreGameInstanceManager;
use backend_framework::wire_api::proto_frj_ngn::ProtoStartGameReply;
use backend_framework::wire_api::proto_frj_ngn::proto_pre_game_message::ProtoGameStartMsg;
use tokio::sync::oneshot;
use tonic::Status;

impl PreGameInstanceManager {

    /// Handle all actions to complete the "pre game" phase of the game, so
    /// we can move the game to in progress.
    ///
    /// Consumes `self` but passes back `Err(self)` if there was some reason
    /// the game couldn't be started. Notifying clients is taken care of.
    pub fn try_start_game(
        mut self,
        player_id: String,
        response_sender: oneshot::Sender<Result<ProtoStartGameReply, Status>>,
    ) -> Result<Vec<String>, Self> {
        if !self.is_party_leader(&player_id) {
            println!("INFO: Non-party leader '{}' attempted to start game. Rejecting the call.", player_id);
            let _ = response_sender.send(Err(Status::failed_precondition("You are not party leader.")));
            return Err(self);
        }

        // TODO check for disconnects

        // Not enough players
        if self.players.count() < self.min_players {
            let _ = response_sender.send(Err(Status::failed_precondition("Not enough players.")));
            return Err(self);
        }

        // Notify all players' streams that game is starting
        let player_ids = self.players.player_ids();
        if let Err(_) = self.notify_all_players(response_sender, &player_ids) {
            return Err(self);
        }

        // Notice: `self` will drop causing all streams to close.
        Ok(player_ids)
    }

    fn is_party_leader(
        &self,
        player_id: &String,
    ) -> bool {
        match self.players.party_leader() {
            None => false,
            Some(party_leader_id) => party_leader_id == player_id,
        }
    }

    fn notify_all_players(
        &mut self,
        response_sender: oneshot::Sender<Result<ProtoStartGameReply, Status>>,
        player_ids: &Vec<String>
    ) -> Result<(), ()> {
        let reply = ProtoStartGameReply {
            player_ids: player_ids.clone()
        };
        if let Err(_) = response_sender.send(Ok(reply)) {
            println!("ERROR: Failed to send reply to oneshot channel. Receiver dropped.");
            return Err(());
        } else {
            println!("DEBUG: Sent req-reply callback for StartGame API.");
        }

        for player_id in player_ids.iter() {
            println!("DEBUG: Sending GameStart stream message to '{}'", player_id);
            self.players.send_pre_game_message(player_id, ProtoGameStartMsg {})
        }

        println!("DEBUG: Done notifying all players of game start.");

        Ok(())
    }
}