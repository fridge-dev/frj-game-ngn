//! There are two "data holders" introduced here:
//! 1. `LoveLetterStateMachine` - holds data that is present for the entirety of the game.
//! 2. `LoveLetterState` - holds data that is optionally present depending on the current game state.
mod handler;

use crate::types::{StagedPlay, GameData, RoundData, RoundResult, UnreadyPlayers};
use backend_framework::data_stream::PlayerDataStreams;
use backend_framework::streaming::StreamSender;
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
    PlayPending(RoundData),
    PlayStaging(RoundData, StagedPlay),
    TurnIntermission(RoundData, UnreadyPlayers),
    RoundIntermission(RoundResult, UnreadyPlayers),
    // TODO:3 - `GameComplete(())`, or we could just let players play rounds endlessly :D
}

/// A state machine executor. It operates on states as inputs/outputs, not owned data.
/// Although it does own some data specific to a game instance.
pub struct LoveLetterStateMachine {
    streams: PlayerDataStreams<ProtoLoveLetterDataOut>,
    game_data: GameData,
}

impl LoveLetterStateMachine {
    pub fn new(player_ids: Vec<String>) -> Self {
        let streams = PlayerDataStreams::new(player_ids.clone());
        let game_data = GameData::new(player_ids);

        LoveLetterStateMachine {
            streams,
            game_data,
        }
    }

    pub fn add_stream(&mut self, player_id: String, stream: StreamSender<ProtoLoveLetterDataOut>) {
        self.streams.add_stream(player_id, stream);
    }

    pub fn all_player_ids(&self) -> &Vec<String> {
        &self.game_data.player_id_turn_order
    }
}
