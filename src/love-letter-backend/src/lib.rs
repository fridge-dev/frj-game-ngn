pub mod events;

mod deck;
mod state_machine;
mod types;
mod type_converters;

use crate::events::{LoveLetterEvent, LoveLetterEventType};
use crate::state_machine::{LoveLetterState, LoveLetterStateMachine};
use crate::types::RoundData;
use backend_framework::holder::Holder;

/// This is the top level class for managing a single game of LoveLetter.
///
/// State is encapsulated here, separately from the StateMachine, so that we can enforce the
/// invariant that state is present at the end of handling the event. StateMachine implementation
/// cannot interact with the `Holder`, it directly operates on the owned state and returns the new
/// state.
pub struct LoveLetterInstanceManager {
    state: Holder<LoveLetterState>,
    state_machine: LoveLetterStateMachine,
}

impl LoveLetterInstanceManager {

    pub fn new(player_ids: Vec<String>) -> Self {
        LoveLetterInstanceManager {
            state: Holder::new(LoveLetterState::PlayPending(RoundData::new(&player_ids))),
            state_machine: LoveLetterStateMachine::new(player_ids),
        }
    }

    /// This is the single entry point for manipulating the state of the game.
    ///
    /// Logic:
    /// 1. Take ownership of current state from game instance
    /// 2. Unwrap the incoming event (i.e. request)
    /// 3. Route event payload to the correct state machine method
    /// 4. Put current state back into game instance
    pub fn handle_event(&mut self, event: LoveLetterEvent) {
        let from_state = self.state.take();
        let to_state = self.route_event_to_state_machine(from_state, event);
        self.state.put(to_state);
    }

    /// State machine logic:
    ///
    /// Move from FROM_STATE to TO_STATE and mutate internal data as needed.
    fn route_event_to_state_machine(
        &mut self,
        from_state: LoveLetterState,
        event: LoveLetterEvent,
    ) -> LoveLetterState {
        let player_id = event.client_info.player_id;

        // This will be a PITA to add Result<> to. Unless Err means game is in corrupt state
        // and we drop the game instance.
        match event.payload {
            LoveLetterEventType::GetGameState => {
                self.state_machine.send_game_state(&from_state, &player_id);
                from_state
            },
            LoveLetterEventType::RegisterDataStream(stream_out) => {
                self.state_machine.add_stream(player_id.clone(), stream_out);
                self.state_machine.send_game_state(&from_state, &player_id);
                from_state
            },
            LoveLetterEventType::PlayCardStaged(card_source) => {
                self.state_machine.play_card_staged(from_state, player_id, card_source)
            },
            LoveLetterEventType::SelectTargetPlayer(target_player_id) => {
                self.state_machine.select_target_player(from_state, player_id, target_player_id)
            },
            LoveLetterEventType::SelectTargetCard(target_card) => {
                self.state_machine.select_target_card(from_state, player_id, target_card)
            },
            LoveLetterEventType::PlayCardCommit => {
                self.state_machine.play_card_commit(from_state, player_id)
            },
            LoveLetterEventType::ReadyUp => {
                self.state_machine.ready_up(from_state, player_id)
            },
        }
    }
}
