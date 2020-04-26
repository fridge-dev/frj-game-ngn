use crate::lost_cities_placeholder::LostCitiesEvent;
use backend_framework::wire_api::proto_frj_ngn::{ProtoPreGameMessage, ProtoStartGameReply};
use backend_framework::streaming::StreamSender;
use love_letter_backend::LoveLetterEvent;
use tokio::sync::oneshot;
use tonic::Status;
use std::fmt::{Display, Formatter};
use std::fmt;

pub trait GameRepository {
    // Pre-game APIs

    fn create_game(&mut self, game: GameIdentifier);
    fn register_pregame_stream(&mut self, player_id: String, game: GameIdentifier, stream_out: StreamSender<ProtoPreGameMessage>);
    fn start_game(&mut self, player_id: String, game: GameIdentifier, response_sender: oneshot::Sender<Result<ProtoStartGameReply, Status>>);

    // Data-stream common APIs

    fn notify_game_state(&mut self, player_id: String, game: GameIdentifier);

    // Data-stream game-specific APIs

    fn handle_event_love_letter(&mut self, event: LoveLetterEvent);
    fn handle_event_lost_cities(&mut self, event: LostCitiesEvent);
}

#[derive(Hash, PartialEq, Eq)]
pub struct GameIdentifier {
    pub game_id: String,
    pub game_type: GameType,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
pub enum GameType {
    LoveLetter,
    LostCities,
}

impl Display for GameType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            GameType::LoveLetter => write!(f, "Love Letter"),
            GameType::LostCities => write!(f, "Lost Cities"),
        }
    }
}

mod converters {
    use crate::game_manager::api::GameType;
    use backend_framework::wire_api::proto_frj_ngn::ProtoGameType;

    impl From<GameType> for ProtoGameType {
        fn from(game_type: GameType) -> Self {
            match game_type {
                GameType::LoveLetter => ProtoGameType::LoveLetter,
                GameType::LostCities => ProtoGameType::LostCities,
            }
        }
    }
}
