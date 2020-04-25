use crate::state_machine::{StateMachineEventHandler, LoveLetterInstanceState, MIN_PLAYERS};
use crate::types::GameData;
use backend_framework::streaming::{MessageErrType, PlayerStreams};
use backend_framework::wire_api::proto_frj_ngn::proto_pre_game_message::ProtoGameStartMsg;
use backend_framework::wire_api::proto_frj_ngn::ProtoStartGameReply;
use tokio::sync::oneshot;

impl StateMachineEventHandler {

    pub fn start_game(
        &mut self,
        from_state: LoveLetterInstanceState,
        player_id: String,
        response_sender: oneshot::Sender<ProtoStartGameReply>,
    ) -> LoveLetterInstanceState {
        // Game in progress
        match from_state {
            LoveLetterInstanceState::WaitingForStart => { /* continue below */ },
            _ => {
                // TODO idempotency?
                println!("INFO: Attempted to start game while not in correct state.");
                self.players.send_pre_game_error(&player_id, "Cannot start game. It's already started.", MessageErrType::InvalidReq);
                return from_state;
            }
        }

        // Player not party leader
        let is_party_leader = match self.players.party_leader() {
            None => false,
            Some(party_leader_id) => party_leader_id == &player_id,
        };
        if !is_party_leader {
            println!("INFO: Non-party leader attempted to start game. Rejecting the call.");
            self.players.send_pre_game_error(&player_id, "Cannot start game. You are not party leader.", MessageErrType::InvalidReq);
            return from_state;
        }

        // TODO check for disconnects

        // Not enough players
        if self.players.count() < MIN_PLAYERS {
            self.players.send_pre_game_error(&player_id, "Cannot start game. Not enough players", MessageErrType::InvalidReq);
            return from_state;
        }

        // to be returned
        let to_state = LoveLetterInstanceState::InProgress(GameData::new(self.players.player_ids()));
        self.notify_all_players(response_sender);

        to_state
    }

    fn notify_all_players(&mut self, response_sender: oneshot::Sender<ProtoStartGameReply>) {
        let reply = ProtoStartGameReply {
            player_ids: self.players.player_ids()
        };
        if let Err(_) = response_sender.send(reply) {
            println!("ERROR: Failed to send reply to oneshot channel. Receiver dropped.");
        }

        println!("DEBUG: Sent req-reply callback for StartGame API.");
        // TODO unnecessary clone of player_ids array. We're about to drop it. Also, we only need ref.
        for player_id in self.players.player_ids() {
            println!("DEBUG: Sending GameStart stream message to '{}'", player_id);
            self.players.send_pre_game_message(&player_id, ProtoGameStartMsg {})
        }
        self.players = PlayerStreams::new();

        println!("DEBUG: Done notifying all players of game start.");
    }
}