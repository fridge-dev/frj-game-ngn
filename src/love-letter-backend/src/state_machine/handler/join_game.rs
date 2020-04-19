use crate::state_machine::{LoveLetterInstanceState, StateMachineEventHandler, MAX_PLAYERS};
use backend_framework::{ClientOut, MessageErrType};
use backend_framework::wire_api::proto_frj_ngn::proto_pre_game_message::ProtoJoinGameAck;
use backend_framework::wire_api::proto_frj_ngn::ProtoGameType;

impl StateMachineEventHandler {

    pub fn join_game(&mut self, player_id: String, client_out: Box<dyn ClientOut + Send>, from_state: &LoveLetterInstanceState) {
        // Reconnect
        if self.players.contains(&player_id) {
            self.add_player_and_send_ack(player_id, client_out);
            return;
        }

        // Game in progress
        match from_state {
            LoveLetterInstanceState::WaitingForStart => { /* continue below */ },
            _ => {
                client_out.send_error_message("Can't join, game has started".into(), MessageErrType::InvalidReq);
                return;
            }
        }

        // Check max players
        if self.players.count() >= MAX_PLAYERS {
            client_out.send_error_message("Can't join, game has max players".into(), MessageErrType::InvalidReq);
            return;
        }

        self.add_player_and_send_ack(player_id, client_out);
    }

    fn add_player_and_send_ack(&mut self, player_id: String, client_out: Box<dyn ClientOut + Send>) {
        self.players.add(player_id.clone(), client_out);

        let (party_leader_index_opt, mut player_ids) = self.players.party_leader_and_all_player_ids();
        let party_leader_index = party_leader_index_opt.expect("BUG: Just added a player, but no party leader present");

        let party_leader_player_id = player_ids.remove(party_leader_index);

        self.players.send_pre_game_message(&player_id, ProtoJoinGameAck {
            game_type: ProtoGameType::LoveLetter as i32,
            host_player_id: party_leader_player_id,
            other_player_ids: player_ids,
        })
    }
}
