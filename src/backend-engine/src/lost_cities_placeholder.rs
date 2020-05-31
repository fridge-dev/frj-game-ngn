use backend_framework::game_instance_manager::GameInstanceManager;

/// I want a place-holder of a 2nd game type, to show how multiple games will be hosted.
#[derive(Debug)]
pub struct LostCitiesEvent;

pub struct LostCitiesInstanceManager;

impl GameInstanceManager<LostCitiesEvent> for LostCitiesInstanceManager {
    fn create_new_game(_player_ids: Vec<String>) -> Self {
        LostCitiesInstanceManager
    }

    fn handle_event(&mut self, _event: LostCitiesEvent) {
        unimplemented!()
    }

    fn player_ids(&self) -> &Vec<String> {
        unimplemented!()
    }
}