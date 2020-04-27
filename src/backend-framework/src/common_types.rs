/// A "client" is the combination of a single player playing in a single game.
#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub player_id: String,
    pub game_id: String,
}