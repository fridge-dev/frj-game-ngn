use crate::events::MastermindEvent;
use crate::state_machine::{BoardState, MastermindStateMachine};
use crate::types::Players;
use backend_framework::activity_timer::ActivityTracker;
use backend_framework::game_instance_manager::GameInstanceManager;
use std::time::Duration;
use backend_framework::holder::Holder;
use crate::state_machine::data::PregameData;

const NUM_COLORS: u8 = 8; // TODO:2 parameterize colors

pub struct MastermindInstanceManager {
    state: Holder<BoardState>,
    state_machine: MastermindStateMachine,
    players: Players,
    activity_tracker: ActivityTracker,
}

impl GameInstanceManager<MastermindEvent> for MastermindInstanceManager {
    fn create_new_game(mut player_ids: Vec<String>) -> Self {
        assert_eq!(player_ids.len(), 2, "Mastermind validation bug: should've validated 2 players");
        let p2 = player_ids.pop().expect("Mastermind validation bug: should've validated 2 players");
        let p1 = player_ids.pop().expect("Mastermind validation bug: should've validated 2 players");

        MastermindInstanceManager {
            state: Holder::new(BoardState::Pregame(PregameData::new(NUM_COLORS))),
            state_machine: MastermindStateMachine::new(),
            players: Players::new(p1, p2),
            activity_tracker: ActivityTracker::new(),
        }
    }

    fn handle_event(&mut self, event: MastermindEvent) {
        let player = match self.players.get_side(&event.client_player_id) {
            Some(player) => player,
            None => {
                // Player not in match.
                // No way to notify player :P oh well.
                return;
            },
        };

        // Transition
        let from_state = self.state.take();
        let to_state = self.state_machine.handle_transition(from_state, player, event.payload);
        self.state.put(to_state);
        self.activity_tracker.ping();
    }

    fn player_ids(&self) -> &Vec<String> {
        &self.players.as_vec
    }

    fn is_game_stale(&self, expiry_duration: Duration) -> bool {
        self.activity_tracker.has_inactivity_elapsed(expiry_duration)
    }
}