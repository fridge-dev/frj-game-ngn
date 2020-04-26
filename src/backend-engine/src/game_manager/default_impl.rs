use crate::game_manager::api::GameRepository;
use crate::game_manager::pre_game::PreGameInstanceManager;
use crate::game_manager::types::{GameIdentifier, GameType};
use crate::lost_cities_placeholder::{LostCitiesInstanceManager, LostCitiesEvent};
use backend_framework::streaming::StreamSender;
use backend_framework::wire_api::proto_frj_ngn::{ProtoPreGameMessage, ProtoStartGameReply};
use love_letter_backend::{LoveLetterInstanceManager, LoveLetterEvent};
use std::collections::HashMap;
use tokio::sync::oneshot;
use tonic::Status;

/// Repository for holding instances of games.
pub(crate) struct DefaultGameRepository {
    unstarted_games: HashMap<GameIdentifier, PreGameInstanceManager>,
    love_letter_instances: HashMap<String, LoveLetterInstanceManager>,
    lost_cities_instances: HashMap<String, LostCitiesInstanceManager>,
}

impl DefaultGameRepository {

    pub fn new() -> Self {
        DefaultGameRepository {
            unstarted_games: HashMap::new(),
            love_letter_instances: HashMap::new(),
            lost_cities_instances: HashMap::new(),
        }
    }

    fn create_typed_game(&mut self, game: GameIdentifier, player_ids: Vec<String>) {
        match game.game_type {
            GameType::LoveLetter => {
                self.love_letter_instances.insert(game.game_id, LoveLetterInstanceManager::new(player_ids));
            },
            GameType::LostCities => {
                self.lost_cities_instances.insert(game.game_id, LostCitiesInstanceManager::new());
            },
        }
    }
}

impl GameRepository for DefaultGameRepository {

    /// Idempotent-ly creates a new instance manager of a game.
    fn create_game(&mut self, game: GameIdentifier) {
        let game_type = game.game_type;
        self.unstarted_games
            .entry(game)
            .or_insert_with(|| PreGameInstanceManager::new(game_type));
    }

    fn register_pregame_stream(&mut self, player_id: String, game: GameIdentifier, stream_out: StreamSender<ProtoPreGameMessage>) {
        match self.unstarted_games.get_mut(&game) {
            None => {
                // TODO check race condition if game started while player in match disconnected
                // TODO notify caller of NotFound
            },
            Some(pre_game_instance_manager) => {
                pre_game_instance_manager.add_player(player_id, stream_out);
            },
        }
    }

    fn start_game(&mut self, player_id: String, game_id: GameIdentifier, response_sender: oneshot::Sender<Result<ProtoStartGameReply, Status>>) {
        // Pop the GIM out
        let pre_game_instance_manager = match self.unstarted_games.remove(&game_id) {
            Some(instance_manager) => instance_manager,
            None => {
                // TODO idempotency check
                let _ = response_sender.send(Err(Status::not_found(format!(
                    "{} Game ID '{}' does not exist or is already in progress.",
                    game_id.game_type,
                    game_id.game_id
                ))));
                return;
            },
        };

        // Attempt to start the game, put the GIM back in the map if failed.
        let player_ids = match pre_game_instance_manager.try_start_game(player_id, response_sender) {
            Ok(player_ids) => player_ids,
            Err(pre_game_instance_manager) => {
                self.unstarted_games.insert(game_id, pre_game_instance_manager);
                return;
            },
        };

        // Create the specific type of game instance.
        self.create_typed_game(game_id, player_ids);
    }

    fn notify_game_state(&mut self, _player_id: String, _game: GameIdentifier) {
        unimplemented!()
    }

    fn handle_event_love_letter(&mut self, _event: LoveLetterEvent) {
        unimplemented!()
    }

    fn handle_event_lost_cities(&mut self, _event: LostCitiesEvent) {
        unimplemented!()
    }
}