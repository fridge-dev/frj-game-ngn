use crate::state_machine::{LoveLetterStateMachine, LoveLetterState};
use crate::types::RoundData;

impl LoveLetterStateMachine {
    pub fn ready_up(&self, from_state: LoveLetterState, client_player_id: String) -> LoveLetterState {
        match from_state {
            LoveLetterState::TurnIntermission(round_data, mut unready_players) => {
                unready_players.ready_up(&client_player_id);

                let to_state = if unready_players.all_ready() {
                    LoveLetterState::PlayPending(round_data)
                } else {
                    LoveLetterState::TurnIntermission(round_data, unready_players)
                };

                self.send_game_state_to_all(&to_state);
                to_state
            },
            LoveLetterState::RoundIntermission(round_result, mut unready_players) => {
                unready_players.ready_up(&client_player_id);

                let to_state = if unready_players.all_ready() {
                    LoveLetterState::PlayPending(self.new_round())
                } else {
                    LoveLetterState::RoundIntermission(round_result, unready_players)
                };

                self.send_game_state_to_all(&to_state);
                to_state
            },
            _ => {
                // Do nothing and drop message
                from_state
            },
        }
    }

    fn new_round(&self) -> RoundData {
        RoundData::new(&self.game_data.player_id_turn_order)
    }
}
