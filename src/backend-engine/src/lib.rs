use crate::lost_cities_placeholder::{LostCitiesEvent, LostCitiesInstanceManager};
use love_letter_backend::{LoveLetterInstanceManager, LoveLetterEvent};
use std::collections::HashMap;
use tokio::sync::mpsc;
use crate::task::GameTask;

struct GameManager {
    love_letter_instances: HashMap<String, LoveLetterInstanceManager>,
    lost_cities_instances: HashMap<String, LostCitiesInstanceManager>,
}

impl GameManager {
    pub fn new() -> Self {
        GameManager {
            love_letter_instances: HashMap::new(),
            lost_cities_instances: HashMap::new(),
        }
    }

    pub fn handle(&mut self, game_id: String, event: GameEvent) {
        match event {
            GameEvent::LoveLetter(inner) => {
                self.love_letter_instances
                    .entry(game_id)
                    .or_insert_with(|| LoveLetterInstanceManager::new())
                    .handle_event(inner);
            },
            GameEvent::LostCities(inner) => {
                self.lost_cities_instances
                    .entry(game_id)
                    .or_insert_with(|| LostCitiesInstanceManager::new())
                    .handle_event(inner);
            }
        }
    }
}

#[derive(Debug)]
pub enum GameEvent {
    LoveLetter(LoveLetterEvent),
    LostCities(LostCitiesEvent),
}

#[derive(Clone)]
pub struct GameTaskClient {
    sender: mpsc::UnboundedSender<(String, GameEvent)>
}

impl GameTaskClient {
    pub(crate) fn new(
        sender: mpsc::UnboundedSender<(String, GameEvent)>,
    ) -> Self {
        GameTaskClient {
            sender,
        }
    }

    pub fn send(&self, game_id: String, event: GameEvent) {
        self.sender.send((game_id, event)).expect("Game task stopped - this should never happen");
    }
}

pub fn start_backend() -> GameTaskClient {
    let (tx, rx) = mpsc::unbounded_channel();
    let task = GameTask::new(rx);

    tokio::spawn(task.event_loop());

    GameTaskClient::new(tx)
}

mod task {
    use tokio::sync::mpsc;
    use crate::{GameEvent, GameManager};

    pub(crate) struct GameTask {
        receiver: mpsc::UnboundedReceiver<(String, GameEvent)>,
        game_manager: GameManager,
    }

    impl GameTask {
        pub fn new(
            receiver: mpsc::UnboundedReceiver<(String, GameEvent)>,
        ) -> Self {
            GameTask {
                receiver,
                game_manager: GameManager::new(),
            }
        }

        pub async fn event_loop(mut self) {
            println!("Starting event loop.");

            while let Some((game_id, event)) = self.receiver.recv().await {
                println!("Received event for game {}: {:?}", game_id, event);
                self.game_manager.handle(game_id, event);
            }

            println!("Exiting event loop.");
        }
    }
}

/// I want a place-holder of a 2nd game type, to show how multiple games will be hosted.
mod lost_cities_placeholder {
    #[derive(Debug)]
    pub struct LostCitiesEvent;
    pub struct LostCitiesInstanceManager {}
    impl LostCitiesInstanceManager {
        pub fn new() -> Self {
            LostCitiesInstanceManager {}
        }
        pub fn handle_event(&mut self, _: LostCitiesEvent) {
            unimplemented!()
        }
    }
}