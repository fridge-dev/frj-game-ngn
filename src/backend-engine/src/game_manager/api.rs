use crate::game_manager::types::GameIdentifier;
use crate::lost_cities_placeholder::LostCitiesEvent;
use backend_framework::wire_api::proto_frj_ngn::{ProtoPreGameMessage, ProtoStartGameReply};
use backend_framework::streaming::StreamSender;
use love_letter_backend::LoveLetterEvent;
use tokio::sync::oneshot;
use tonic::Status;

/// The "backend" of managing all game instances. It has all the same methods as below, with
/// `&mut self` because we mutate data internally.
pub trait GameRepository {

    // Pre-game APIs

    fn create_game(&mut self, game: GameIdentifier);
    fn register_pregame_stream(&mut self, player_id: String, game: GameIdentifier, stream_out: StreamSender<ProtoPreGameMessage>);
    fn start_game(&mut self, player_id: String, game: GameIdentifier, response_sender: oneshot::Sender<Result<ProtoStartGameReply, Status>>);

    // Data-stream common APIs

    fn notify_game_state(&mut self, player_id: String, game: GameIdentifier);

    // Data-stream game-specific APIs

    fn handle_event_love_letter(&mut self, event: LoveLetterEvent);
    fn handle_event_lost_cities(&mut self, event: LostCitiesEvent);
}

/// The "client" or caller of the repository. It has all the same methods as above, just with
/// `&self` instead of `&mut self`.
pub trait GameRepositoryClient {

    /// Implementing `Clone` will not work, because `Clone: Sized` and a
    /// `Box<dyn GameRepositoryClient>` trait object's methods cannot be
    /// invoked. FRICK ME because this is super weird.
    ///
    /// Correct solution should be later, when I'm looking to optimize,
    /// change the usage to be static dispatch.
    fn unsized_clone(&self) -> Box<dyn GameRepositoryClient + Send + Sync>;

    // Pre-game APIs

    fn create_game(&self, game: GameIdentifier);
    fn register_pregame_stream(&self, player_id: String, game: GameIdentifier, stream_out: StreamSender<ProtoPreGameMessage>);
    fn start_game(&self, player_id: String, game: GameIdentifier, response_sender: oneshot::Sender<Result<ProtoStartGameReply, Status>>);

    // Data-stream common APIs

    fn notify_game_state(&self, player_id: String, game: GameIdentifier);

    // Data-stream game-specific APIs

    fn handle_event_love_letter(&self, event: LoveLetterEvent);
    fn handle_event_lost_cities(&self, event: LostCitiesEvent);
}
