use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct GameIdentifier {
    pub game_id: String,
    pub game_type: GameType,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
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
    use crate::game_manager::types::GameType;
    use backend_framework::wire_api::proto_frj_ngn::ProtoGameType;
    use std::convert::TryFrom;
    use tonic::Status;

    impl From<GameType> for ProtoGameType {
        fn from(game_type: GameType) -> Self {
            match game_type {
                GameType::LoveLetter => ProtoGameType::LoveLetter,
                GameType::LostCities => ProtoGameType::LostCities,
            }
        }
    }

    impl TryFrom<ProtoGameType> for GameType {
        type Error = Status;

        fn try_from(proto_game_type: ProtoGameType) -> Result<Self, Self::Error> {
            match proto_game_type {
                ProtoGameType::UnspecifiedGameType => Err(Status::invalid_argument("Unspecified game type")),
                ProtoGameType::LoveLetter => Ok(GameType::LoveLetter),
                ProtoGameType::LostCities => Ok(GameType::LostCities),
            }
        }
    }
}
