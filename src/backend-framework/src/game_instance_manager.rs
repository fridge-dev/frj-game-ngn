use std::time::Duration;

/// The generic trait which acts as a manager for a single instance of the game.
/// A game instance comes into play only *after* the pre-game phase, and games
/// are assumed, once created and started, to have an immutable set of players.
pub trait GameInstanceManager<T> {

    /// Create a new instance of a game from the set of players.
    /// TODO:3 add `Random` to input, which is seeded with game_id on caller side.
    fn create_new_game(player_ids: Vec<String>) -> Self;

    /// This is the single entry point for manipulating the state of the game.
    fn handle_event(&mut self, event: T);

    /// Accessor to get a reference to the players in the game.
    fn player_ids(&self) -> &Vec<String>;

    /// Check if we can delete game
    fn is_game_stale(&self, expiry_duration: Duration) -> bool;
}
