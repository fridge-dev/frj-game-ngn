use crate::game_manager::api::GameRepository;
use crate::game_manager::pre_game::PreGameInstanceManager;
use crate::game_manager::types::{GameIdentifier, GameType};
use crate::lost_cities_placeholder::{LostCitiesInstanceManager, LostCitiesEvent};
use backend_framework::game_instance_manager::GameInstanceManager;
use backend_framework::streaming::StreamSender;
use backend_framework::wire_api::proto_frj_ngn::{ProtoPreGameMessage, ProtoStartGameReply, ProtoGameType};
use love_letter_backend::LoveLetterInstanceManager;
use love_letter_backend::events::{LoveLetterEvent, LoveLetterEventType};
use std::collections::HashMap;
use tokio::sync::oneshot;
use tonic::Status;
use backend_framework::wire_api::proto_frj_ngn::proto_pre_game_message::{ProtoJoinGameAck, ProtoGameStartMsg};

/// Repository for holding instances of games.
//
/// TODO:2.5 the following refactor will help game_id collision logic:
/// ```
/// # use love_letter_backend::LoveLetterInstanceManager;
/// # use std::collections::HashMap;
/// enum GameInstanceManagerMode {
///     PreGame(PreGameInstanceManager),
///     LoveLetter(LoveLetterInstanceManager),
///     // ...
/// }
/// struct DefaultGameRepository2 {
///     games: HashMap<String, GameInstanceManagerMode>,
/// }
/// ```
pub(crate) struct DefaultGameRepository {
    // TODO:2.5 implement garbage collection
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

    fn insert_new_game(&mut self, game: GameIdentifier, player_ids: Vec<String>) -> Result<(), Status> {
        match game.game_type {
            GameType::LoveLetter => {
                if self.love_letter_instances.contains_key(&game.game_id) {
                    println!("ERROR: Pre-game was created while game with same ID was in progress. This should've been prevented internally, but wasn't. Game: {:?}", game);
                    return Err(Status::internal("Backend in illegal state, create a new game."));
                }
                self.love_letter_instances.insert(game.game_id, LoveLetterInstanceManager::create_new_game(player_ids));
            },
            GameType::LostCities => {
                if self.lost_cities_instances.contains_key(&game.game_id) {
                    println!("ERROR: Pre-game was created while game with same ID was in progress. This should've been prevented internally, but wasn't. Game: {:?}", game);
                    return Err(Status::internal("Backend in illegal state, create a new game."));
                }
                self.lost_cities_instances.insert(game.game_id, LostCitiesInstanceManager::create_new_game(player_ids));
            },
        }

        Ok(())
    }

    fn get_player_ids_if_game_exists(&self, game: &GameIdentifier) -> Option<&Vec<String>> {
        match game.game_type {
            GameType::LoveLetter => {
                self.love_letter_instances
                    .get(&game.game_id)
                    .map(|gim| gim.player_ids())
            },
            GameType::LostCities => {
                self.lost_cities_instances
                    .get(&game.game_id)
                    .map(|gim| gim.player_ids())
            },
        }
    }
}

impl GameRepository for DefaultGameRepository {

    /// Idempotent-ly creates a new generic "pre-game" instance manager for this game.
    fn create_pregame(&mut self, game: GameIdentifier) {
        // Ensure in-progress game doesn't exist with same ID. Don't actually notify client of
        // failure here, they'll get a failure below in `register_pregame_stream()`. See comments
        // on struct level above.
        if self.get_player_ids_if_game_exists(&game).is_some() {
            println!("WARN: Attempted to create pre-game with colliding game_id as in-progress game.");
            return;
        }

        println!("INFO: Creating game {:?}", game);
        let game_type = game.game_type;
        self.unstarted_games
            .entry(game)
            .or_insert_with(|| PreGameInstanceManager::new(game_type));
    }

    fn register_pregame_stream(
        &mut self,
        player_id: String,
        game_id: GameIdentifier,
        stream_out: StreamSender<ProtoPreGameMessage>
    ) {
        // Happy path
        if let Some(pre_game_instance_manager) = self.unstarted_games.get_mut(&game_id) {
            println!("INFO: Player '{}' joining game '{}'", player_id, game_id.game_id);
            pre_game_instance_manager.add_player(player_id, stream_out);
            return;
        }

        // Un-started game not found, check if game in-progress exists. This is possible if a player
        // disconnects while game is in "pre-game" phase, and game starts before player reconnects.
        match self.get_player_ids_if_game_exists(&game_id).filter(|p| p.contains(&player_id)) {
            None => {
                // Notify caller of NotFound.
                let _ = stream_out.send_error_message(Status::not_found(format!(
                    "{} Game ID '{}' does not exist.",
                    game_id.game_type,
                    game_id.game_id
                )));
            },
            Some(player_ids) => {
                let mut player_ids = player_ids.clone();
                let ack = ProtoJoinGameAck {
                    game_type: ProtoGameType::from(game_id.game_type) as i32,
                    host_player_id: player_ids.remove(0),
                    other_player_ids: player_ids,
                };
                // Notify caller that game started.
                let _ = stream_out.send_message(ack.into());
                let _ = stream_out.send_message(ProtoGameStartMsg {}.into());
            },
        }
    }

    fn start_game(
        &mut self,
        player_id: String,
        game_id: GameIdentifier,
        response_sender: oneshot::Sender<Result<ProtoStartGameReply, Status>>
    ) {
        let response_sender = StartGameReplySender(response_sender);

        // Pop the GIM out
        let pre_game_instance_manager = match self.unstarted_games.remove(&game_id) {
            Some(instance_manager) => instance_manager,
            None => {
                // Idempotency check
                let msg = self.get_player_ids_if_game_exists(&game_id)
                    .filter(|player_ids| player_ids.contains(&player_id))
                    // Re-send success without creating
                    .map(|player_ids| ProtoStartGameReply {
                        player_ids: player_ids.clone()
                    })
                    // Notify game not found
                    .ok_or_else(|| Status::not_found(format!(
                        "{} Game ID '{}' does not exist.",
                        game_id.game_type,
                        game_id.game_id
                    )));
                response_sender.send(msg);

                return;
            },
        };

        // Check pre-requisites
        let player_ids = match pre_game_instance_manager.start_game_pre_check(&player_id) {
            Ok(player_ids) => player_ids,
            Err(msg) => {
                response_sender.send(Err(msg));
                self.unstarted_games.insert(game_id, pre_game_instance_manager);
                return;
            },
        };

        // Create the specific type of game instance.
        match self.insert_new_game(game_id, player_ids.clone()) {
            Ok(_) => {
                // Notifying party leader of all player IDs when the game is going to start is redundant,
                // because the first game data-stream will include relevant game state including player IDs.
                // TODO:3 Remove unnecessary player_ids from ProtoStartGameReply.
                //
                // Also notice, if we fail to respond to sync request, we start the game anyway.
                response_sender.send(Ok(ProtoStartGameReply { player_ids }));
                println!("DEBUG: Sent req-reply callback for StartGame API.");
                pre_game_instance_manager.start_game_notify_players();
                println!("DEBUG: Done notifying all players of game start.");
            },
            Err(msg) => {
                response_sender.send(Err(msg.clone()));
                pre_game_instance_manager.drop_game_notify_players(msg);
            },
        }
    }

    fn notify_game_state(&mut self, _player_id: String, _game: GameIdentifier) {
        unimplemented!("DefaultGameRepository::notify_game_state()")
    }

    fn handle_event_love_letter(&mut self, event: LoveLetterEvent) {
        println!("DEBUG: DefaultGameRepository received {:?}", event);

        if let Some(game) = self.love_letter_instances.get_mut(&event.client_info.game_id) {
            // TODO:3 this unnecessarily leaks `game_id` into individual instance managers
            game.handle_event(event);
        } else if let LoveLetterEventType::RegisterDataStream(stream) = event.payload {
            let _ = stream.send_error_message(Status::not_found(format!("Game {} not found", event.client_info.game_id)));
        } else {
            // Can't notify client of game not found (because of how I modeled the code).
            // Should probably fix this at some point...
        }
    }

    fn handle_event_lost_cities(&mut self, _event: LostCitiesEvent) {
        unimplemented!("DefaultGameRepository::handle_event_lost_cities()")
    }
}

struct StartGameReplySender(oneshot::Sender<Result<ProtoStartGameReply, Status>>);
impl StartGameReplySender {
    pub fn send(self, message: Result<ProtoStartGameReply, Status>) {
        if let Err(_) = self.0.send(message) {
            println!("INFO: Failed to respond to StartGame call.");
        }
    }
}
