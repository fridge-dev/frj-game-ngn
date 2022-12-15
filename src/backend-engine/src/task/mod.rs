use crate::game_manager::api::{GameRepositoryClient, GameRepository};
use crate::game_manager::default_impl::DefaultGameRepository;
use crate::game_manager::types::GameIdentifier;
use crate::lost_cities_placeholder::LostCitiesEvent;
use backend_framework::streaming::StreamSender;
use backend_framework::wire_api::proto_frj_ngn::ProtoPreGameMessage;
use backend_framework::wire_api::proto_frj_ngn::ProtoStartGameReply;
use love_letter_backend::events::LoveLetterEvent;
use tonic::Status;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use std::time::Duration;
use rand::Rng;

// TODO:1 consistent hashing onto parallel backend slots
pub fn start_repository_instance() -> Box<dyn GameRepositoryClient + Send + Sync> {
    let (tx, rx) = mpsc::unbounded_channel();
    let task = GameRepoTask::new(rx);

    tokio::spawn(task.event_loop());

    let task_client = GameRepoTaskClientAdapter::new(tx);
    tokio::task::spawn(garbage_collection_heartbeat(task_client.clone()));
    Box::new(task_client)
}

/// A 1:1 enumeration of GameRepository API methods.
#[derive(Debug)]
enum GameRepoTaskEvent {
    // Non-game APIs
    CleanupStaleGames,
    // Pre-game APIs
    CreatePregame(GameIdentifier),
    RegisterPregameStream {
        player_id: String,
        game: GameIdentifier,
        stream_out: StreamSender<ProtoPreGameMessage>,
    },
    StartGame {
        player_id: String,
        game: GameIdentifier,
        response_sender: oneshot::Sender<Result<ProtoStartGameReply, Status>>,
    },
    // Data-stream common APIs
    NotifyGameState {
        player_id: String,
        game: GameIdentifier,
    },
    // Data-stream game-specific APIs
    LoveLetter(LoveLetterEvent),
    LostCities(LostCitiesEvent),
}

/// This is a mpsc Sender (immutable) for accessing a GameRepository (mutable).
#[derive(Clone)]
struct GameRepoTaskClientAdapter {
    sender: mpsc::UnboundedSender<GameRepoTaskEvent>
}

impl GameRepoTaskClientAdapter {
    fn new(sender: mpsc::UnboundedSender<GameRepoTaskEvent>) -> Self {
        GameRepoTaskClientAdapter {
            sender,
        }
    }

    fn send(&self, event: GameRepoTaskEvent) {
        self.sender
            .send(event)
            .expect("GameRepo task stopped - this should never happen");
    }

    pub fn cleanup_stale_games(&self) {
        self.send(GameRepoTaskEvent::CleanupStaleGames)
    }
}

impl GameRepositoryClient for GameRepoTaskClientAdapter {

    fn unsized_clone(&self) -> Box<dyn GameRepositoryClient + Send + Sync> {
        Box::new(self.clone())
    }

    fn create_pregame(&self, game: GameIdentifier) {
        self.send(GameRepoTaskEvent::CreatePregame(game))
    }

    fn register_pregame_stream(&self, player_id: String, game: GameIdentifier, stream_out: StreamSender<ProtoPreGameMessage>) {
        self.send(GameRepoTaskEvent::RegisterPregameStream {
            player_id,
            game,
            stream_out
        })
    }

    fn start_game(&self, player_id: String, game: GameIdentifier, response_sender: oneshot::Sender<Result<ProtoStartGameReply, Status>>) {
        self.send(GameRepoTaskEvent::StartGame {
            player_id,
            game,
            response_sender
        })
    }

    fn notify_game_state(&self, player_id: String, game: GameIdentifier) {
        self.send(GameRepoTaskEvent::NotifyGameState {
            player_id,
            game
        })
    }

    fn handle_event_love_letter(&self, event: LoveLetterEvent) {
        self.send(GameRepoTaskEvent::LoveLetter(event))
    }

    fn handle_event_lost_cities(&self, event: LostCitiesEvent) {
        self.send(GameRepoTaskEvent::LostCities(event))
    }
}

/// This is a mpsc Receiver wrapped around an instance of a GameRepository.
struct GameRepoTask<T: GameRepository> {
    receiver: mpsc::UnboundedReceiver<GameRepoTaskEvent>,
    game_repo: T,
}

impl GameRepoTask<DefaultGameRepository> {
    pub fn new(receiver: mpsc::UnboundedReceiver<GameRepoTaskEvent>) -> Self {
        GameRepoTask {
            receiver,
            game_repo: DefaultGameRepository::new(),
        }
    }

    pub async fn event_loop(mut self) {
        println!("INFO: Starting event loop.");

        while let Some(event) = self.receiver.recv().await {
            self.route_event(event);
        }

        println!("INFO: Exiting event loop.");
    }

    fn route_event(&mut self, event: GameRepoTaskEvent) {
        match event {
            GameRepoTaskEvent::CleanupStaleGames => {
                self.game_repo.cleanup_stale_games()
            },
            GameRepoTaskEvent::CreatePregame(game) => {
                self.game_repo.create_pregame(game)
            },
            GameRepoTaskEvent::RegisterPregameStream { player_id, game, stream_out } => {
                self.game_repo.register_pregame_stream(player_id, game, stream_out)
            },
            GameRepoTaskEvent::StartGame { player_id, game, response_sender } => {
                self.game_repo.start_game(player_id, game, response_sender)
            },
            GameRepoTaskEvent::NotifyGameState { player_id, game } => {
                self.game_repo.notify_game_state(player_id, game)
            },
            GameRepoTaskEvent::LoveLetter(inner) => {
                self.game_repo.handle_event_love_letter(inner)
            },
            GameRepoTaskEvent::LostCities(inner) => {
                self.game_repo.handle_event_lost_cities(inner)
            },
        }
    }
}

/// Garbage collection heartbeat task that runs for entire app lifecycle.
/// There is 1 GC heartbeat task for each repo task.
async fn garbage_collection_heartbeat(task_client: GameRepoTaskClientAdapter) {
    let interval_time_min_seconds = 60u64;
    let interval_time_max_seconds = 180u64;

    loop {
        let jittered_interval_time_sec = rand::thread_rng().gen_range(
            interval_time_min_seconds,
            interval_time_max_seconds
        );
        tokio::time::delay_for(Duration::from_secs(jittered_interval_time_sec)).await;
        task_client.cleanup_stale_games();
    }
}
