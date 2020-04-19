use crate::lost_cities_placeholder::{LostCitiesEvent, LostCitiesInstanceManager};
use love_letter_backend::{LoveLetterEvent, LoveLetterInstanceManager};
use std::collections::HashMap;

#[derive(Debug)]
pub enum GameEvent {
    LoveLetter(LoveLetterEvent),
    #[allow(dead_code)]
    LostCities(LostCitiesEvent),
}

pub(crate) struct GameManager {
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
