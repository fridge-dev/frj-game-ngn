mod handler;

use crate::events::{LoveLetterEvent, LoveLetterEventType};
use crate::types::{StagedPlay, GameData, RoundData, RoundResult};
use backend_framework::data_stream::PlayerDataStreams;
use backend_framework::wire_api::proto_frj_ngn::ProtoLoveLetterDataOut;

/// The possible states of an instance of the game.
///
/// ```text
/// +-------------------+    +-------------------+    +-------------------+
/// |      (start)      |    |                   |    |                   |
/// |    PlayPending    |--->|    PlayStaging    |--->|  TurnIntermission |
/// |                   |    |                   |    |                   |
/// +-------------------+    +-------------------+    +-------------------+
///         ^    ^                                       |    |    |
///         |    +----<----<----<----<----<----<----<----+    |    |
///         |                                                 |    |
///         |                +-------------------+            |    |
///         |                |                   |            |    |
///         +----<-----<-----| RoundIntermission |<-----<-----+    |
///                          |                   |                 V
///                          +-------------------+    +-------------------+
///                                                   |       (fin)       |
///                                                   |    GameComplete   |
///                                                   |                   |
///                                                   +-------------------+
/// ```
pub enum LoveLetterState {
    PlayPending(GameData, RoundData),
    PlayStaging(GameData, RoundData, StagedPlay),
    TurnIntermission(GameData, RoundData),
    RoundIntermission(GameData, RoundResult),
// TODO something like this:
//    GameComplete{
//        wins_per_player: HashMap<String, u8>,
//    },
}

impl LoveLetterState {
    pub fn initial_game_state(player_ids: Vec<String>) -> Self {
        let round_data = RoundData::new(&player_ids);
        let game_data = GameData::new(player_ids);
        LoveLetterState::PlayPending(game_data, round_data)
    }
}

/// A state machine executor. It operates on states as inputs/outputs, not owned data.
/// Although it does own some data specific to a game instance.
pub struct LoveLetterStateMachine {
    handler: LoveLetterStateMachineEventHandler,
}

impl LoveLetterStateMachine {
    pub fn new(player_ids: Vec<String>) -> Self {
        LoveLetterStateMachine {
            handler: LoveLetterStateMachineEventHandler::new(player_ids),
        }
    }

    /// State machine logic:
    ///
    /// Move from FROM_STATE to TO_STATE and mutate internal data as needed.
    ///
    /// This will be a PITA to add Result<> to. Unless Err means game is in corrupt state
    /// and we drop the game instance.
    pub fn transition_state(
        &mut self,
        from_state: LoveLetterState,
        event: LoveLetterEvent,
    ) -> LoveLetterState {
        let player_id = event.client_info.player_id;

        match event.payload {
            LoveLetterEventType::GetGameState => {
                self.handler.send_game_state(&from_state, &player_id);
                from_state
            },
            LoveLetterEventType::RegisterDataStream(stream_out) => {
                self.handler.streams.add_stream(player_id.clone(), stream_out);
                self.handler.send_game_state(&from_state, &player_id);
                from_state
            },
            LoveLetterEventType::PlayCardStaged(card_source) => {
                self.handler.play_card_staged(from_state, player_id, card_source)
            },
            LoveLetterEventType::SelectTargetPlayer(target_player_id) => {
                self.handler.select_target_player(from_state, player_id, target_player_id)
            },
            LoveLetterEventType::SelectTargetCard(target_card) => {
                self.handler.select_target_card(from_state, player_id, target_card)
            },
            LoveLetterEventType::PlayCardCommit => {
                self.handler.play_card_commit(from_state, player_id)
            },
        }
    }
}

struct LoveLetterStateMachineEventHandler {
    streams: PlayerDataStreams<ProtoLoveLetterDataOut>,
}

impl LoveLetterStateMachineEventHandler {
    pub fn new(player_ids: Vec<String>) -> Self {
        LoveLetterStateMachineEventHandler {
            streams: PlayerDataStreams::new(player_ids)
        }
    }
}
