use crate::state_machine::{StateMachineEventHandler, LoveLetterInstanceState, MIN_PLAYERS};
use crate::types::GameData;

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
                // TODO notify caller of err?
                // TODO idempotency?
                return from_state;
            }
        }

        // Not enough players
        if self.players.count() < MIN_PLAYERS {
            // TODO notify caller of err?
            return from_state;
        }

        // Player not in match
        if !self.players.contains(&player_id) {
            // TODO notify caller of err?
            return from_state;
        }

        LoveLetterInstanceState::InProgress(GameData::new(self.players.player_ids()))
    }
}