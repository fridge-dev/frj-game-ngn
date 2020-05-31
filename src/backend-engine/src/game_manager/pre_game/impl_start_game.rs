use crate::game_manager::pre_game::PreGameInstanceManager;
use backend_framework::wire_api::proto_frj_ngn::proto_pre_game_message::ProtoGameStartMsg;
use tonic::Status;

impl PreGameInstanceManager {

    /// Handle all validation checks to complete the "pre-game" phase of the game, so
    /// we can move the game to in progress.
    pub fn start_game_pre_check(&self, requesting_player_id: &String) -> Result<Vec<String>, Status> {
        if !self.is_party_leader(requesting_player_id) {
            println!("INFO: Non-party leader '{}' attempted to start game. Rejecting the call.", requesting_player_id);
            return Err(Status::failed_precondition("You are not party leader."));
        }

        // Not enough players
        if self.players.count() < self.min_players {
            println!(
                "INFO: Attempted to start game '{:?}' with only '{}' players. Rejecting the call.",
                self.game_type,
                self.players.count()
            );
            return Err(Status::failed_precondition("Not enough players."));
        }

        // If some of the already-joined players disconnected (fatally) without leaving the game,
        // then this game state will be doomed.
        // TODO:3 Check for disconnects before starting.

        Ok(self.players.player_ids())
    }

    fn is_party_leader(&self, player_id: &String) -> bool {
        match self.players.party_leader() {
            None => false,
            Some(party_leader_id) => party_leader_id == player_id,
        }
    }

    /// Notify all players' streams that game is starting.
    ///
    /// Consumes `self` since this is intended to be the terminal action of a pre-game.
    pub fn start_game_notify_players(mut self) {
        for player_id in self.players.player_ids() {
            println!("DEBUG: Sending GameStart stream message to '{}'", player_id);
            self.players.send_pre_game_message(&player_id, ProtoGameStartMsg {})
        }

        // Notice: `self` will drop causing all streams to close.
    }

    /// Notify all players' streams that "pre-game" is being deleted.
    ///
    /// Consumes `self` since this is intended to be the terminal action of a pre-game.
    pub fn drop_game_notify_players(mut self, status: Status) {
        for player_id in self.players.player_ids() {
            self.players.send_pre_game_message_err(&player_id, status.clone());
        }

        // Notice: `self` will drop causing all streams to close.
    }
}