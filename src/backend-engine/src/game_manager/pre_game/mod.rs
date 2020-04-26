use crate::game_manager::types::GameType;
use backend_framework::streaming::PlayerPreGameStreams;

mod impl_join_game;
mod impl_start_game;

pub(crate) struct PreGameInstanceManager {
    pub game_type: GameType,
    min_players: usize,
    max_players: usize,
    players: PlayerPreGameStreams,
}

impl PreGameInstanceManager {

    pub fn new(game_type: GameType) -> Self {
        let (min, max) = player_count_min_max(&game_type);

        PreGameInstanceManager {
            game_type,
            min_players: min,
            max_players: max,
            players: PlayerPreGameStreams::new(),
        }
    }
}

fn player_count_min_max(game_type: &GameType) -> (usize, usize) {
    match game_type {
        GameType::LoveLetter => (2, 4),
        GameType::LostCities => (2, 2),
    }
}