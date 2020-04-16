use crate::state_machine::{LoveLetterInstanceState, StateMachineEventHandler, MAX_PLAYERS};
use backend_framework::ClientOut;

impl StateMachineEventHandler {

    pub fn join(&mut self, player_id: String, client_out: ClientOut, state: &LoveLetterInstanceState) {
        // Reconnect
        if self.players.contains(&player_id) {
            self.players.add(player_id, client_out);
            return;
        }

        // Game in progress
        match state {
            LoveLetterInstanceState::WaitingForStart => { /* continue below */ },
            _ => {
                client_out.send_err("Can't join, game has started");
                return;
            }
        }

        // Check max players
        if self.players.count() >= MAX_PLAYERS {
            client_out.send_err("Can't join, game has max players");
            return;
        }

        self.players.add(player_id, client_out);
    }

    pub fn get_game_state(&self, player_id: String) {
        self.players.send_msg(&player_id, format!("TODO msg"));
    }
}