use crate::state_machine::{StateMachineEventHandler, LoveLetterInstanceState, MIN_PLAYERS};
use crate::types::GameData;
use backend_framework::MessageErrType;
use backend_framework::wire_api::proto_frj_ngn::proto_pre_game_message::ProtoGameStartMsg;

impl StateMachineEventHandler {

    pub fn start_game(
        &mut self,
        from_state: LoveLetterInstanceState,
        player_id: String,
    ) -> LoveLetterInstanceState {
        // Game in progress
        match from_state {
            LoveLetterInstanceState::WaitingForStart => { /* continue below */ },
            _ => {
                // TODO idempotency?
                println!("INFO: Attempted to start game while not in correct state.");
                self.players.send_error_message(&player_id, "Cannot start game. It's already started.", MessageErrType::InvalidReq);
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
            self.players.send_error_message(&player_id, "Cannot start game. You are not party leader.", MessageErrType::InvalidReq);
            return from_state;
        }

        // TODO check for disconnects

        // Not enough players
        if self.players.count() < MIN_PLAYERS {
            self.players.send_error_message(&player_id, "Cannot start game. Not enough players", MessageErrType::InvalidReq);
            return from_state;
        }

        // TODO unnecessary clone of player_ids array. We're about to drop it. Also, we only need ref.
        for player_id in self.players.player_ids() {
            self.players.send_pre_game_message(&player_id, ProtoGameStartMsg {})
        }
        // TODO drop player map, close all streams.

        LoveLetterInstanceState::InProgress(GameData::new(self.players.player_ids()))
    }
}