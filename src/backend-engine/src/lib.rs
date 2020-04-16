use std::collections::HashMap;
use love_letter_backend::{LoveLetterInstanceManager, LoveLetterEvent};
use crate::lost_cities_placeholder::{LostCitiesEvent, LostCitiesInstanceManager};

pub struct GameManager {
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

pub enum GameEvent {
    LoveLetter(LoveLetterEvent),
    LostCities(LostCitiesEvent),
}

/// I want a place-holder of a 2nd game type, to show how multiple games will be hosted.
mod lost_cities_placeholder {
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