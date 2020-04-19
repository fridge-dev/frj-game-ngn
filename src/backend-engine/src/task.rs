use crate::game_manager::{GameEvent, GameManager};
use tokio::sync::mpsc;

pub fn start_backend() -> GameTaskClient {
    let (tx, rx) = mpsc::unbounded_channel();
    let task = GameManagerTask::new(rx);

    tokio::spawn(task.event_loop());

    GameTaskClient::new(tx)
}

/// This is a mpsc Sender (immutable) for accessing a GameManager (mutable).
#[derive(Clone)]
pub struct GameTaskClient {
    sender: mpsc::UnboundedSender<(String, GameEvent)>
}

impl GameTaskClient {
    fn new(sender: mpsc::UnboundedSender<(String, GameEvent)>) -> Self {
        GameTaskClient {
            sender,
        }
    }

    pub fn send(&self, game_id: String, event: GameEvent) {
        self.sender.send((game_id, event)).expect("Game task stopped - this should never happen");
    }
}

/// This is a mpsc Receiver wrapped around an instance of a GameManager.
struct GameManagerTask {
    receiver: mpsc::UnboundedReceiver<(String, GameEvent)>,
    game_manager: GameManager,
}

impl GameManagerTask {
    fn new(receiver: mpsc::UnboundedReceiver<(String, GameEvent)>) -> Self {
        GameManagerTask {
            receiver,
            game_manager: GameManager::new(),
        }
    }

    async fn event_loop(mut self) {
        println!("INFO: Starting event loop.");

        while let Some((game_id, event)) = self.receiver.recv().await {
            println!("INFO: Received event for game {}: {:?}", game_id, event);
            self.game_manager.handle(game_id, event);
        }

        println!("INFO: Exiting event loop.");
    }
}