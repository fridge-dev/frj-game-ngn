use crate::state_machine::{LoveLetterInstanceState, StateMachineEventHandler, MAX_PLAYERS};
use backend_framework::{ClientOut, MessageErrType};

impl StateMachineEventHandler {

    pub fn join(&mut self, player_id: String, client_out: Box<dyn ClientOut + Send>, state: &LoveLetterInstanceState) {
        // Reconnect
        if self.players.contains(&player_id) {
            self.players.add(player_id, client_out);
            return;
        }

        // Game in progress
        match state {
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

        self.players.add(player_id, client_out);
    }

    pub fn get_game_state(&self, player_id: String) {
        unimplemented!()
    }
}