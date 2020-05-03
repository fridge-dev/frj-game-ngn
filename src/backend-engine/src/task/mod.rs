
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

pub fn start_repository_instance() -> Box<dyn GameRepositoryClient + Send + Sync> {
    let (tx, rx) = mpsc::unbounded_channel();
    let task = GameRepoTask::new(rx);

    tokio::spawn(task.event_loop());

    Box::new(GameRepoTaskClientAdapter::new(tx))
}

/// A 1:1 enumeration of GameRepository API methods.
#[derive(Debug)]
enum GameRepoTaskEvent {
    // Pre-game APIs
    CreateGame(GameIdentifier),
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
            .expect("Game task stopped - this should never happen");
    }
}

impl GameRepositoryClient for GameRepoTaskClientAdapter {

    fn unsized_clone(&self) -> Box<dyn GameRepositoryClient + Send + Sync> {
        Box::new(self.clone())
    }

    fn create_game(&self, game: GameIdentifier) {
        self.send(GameRepoTaskEvent::CreateGame(game))
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
    fn new(receiver: mpsc::UnboundedReceiver<GameRepoTaskEvent>) -> Self {
        GameRepoTask {
            receiver,
            game_repo: DefaultGameRepository::new(),
        }
    }

    async fn event_loop(mut self) {
        println!("INFO: Starting event loop.");

        while let Some(event) = self.receiver.recv().await {
            self.route_event(event);
        }

        println!("INFO: Exiting event loop.");
    }

    fn route_event(&mut self, event: GameRepoTaskEvent) {
        match event {
            GameRepoTaskEvent::CreateGame(game) => {
                self.game_repo.create_game(game)
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
