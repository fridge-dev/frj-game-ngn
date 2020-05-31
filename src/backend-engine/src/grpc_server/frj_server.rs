use crate::task;
use crate::grpc_server::love_letter_stream::LoveLetterStreamInitializer;
use crate::game_manager::api::GameRepositoryClient;
use crate::game_manager::types::{GameType, GameIdentifier};
use backend_framework::wire_api::proto_frj_ngn::proto_fridge_game_engine_server::ProtoFridgeGameEngine;
use backend_framework::wire_api::proto_frj_ngn::{ProtoPreGameMessage, ProtoHostGameReq, ProtoJoinGameReq, ProtoGameType, ProtoStartGameReq, ProtoStartGameReply, ProtoLoveLetterDataIn, ProtoLoveLetterDataOut};
use backend_framework::streaming::StreamSender;
use std::convert::TryFrom;
use std::error::Error;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tonic::{Request, Response, Status, Streaming, Code};

/// Backend server is the entry point which will implement the gRPC server type.
pub struct FrjServer {
    game_repo_client: Box<dyn GameRepositoryClient + Send + Sync>,
    love_letter_stream_opener: LoveLetterStreamInitializer,
}

impl FrjServer {

    pub fn start() -> Result<Self, Box<dyn Error>> {
        let game_repo_client = task::start_repository_instance();
        let love_letter_stream_opener = LoveLetterStreamInitializer::new(game_repo_client.unsized_clone());

        Ok(FrjServer::new(
            game_repo_client,
            love_letter_stream_opener
        ))
    }

    fn new(
        game_repo_client: Box<dyn GameRepositoryClient + Send + Sync>,
        love_letter_stream_opener: LoveLetterStreamInitializer,
    ) -> Self {
        FrjServer {
            game_repo_client,
            love_letter_stream_opener,
        }
    }
}

type PreGameStream = mpsc::UnboundedReceiver<Result<ProtoPreGameMessage, Status>>;
pub type GameDataStream<T> = mpsc::UnboundedReceiver<Result<T, Status>>;

#[tonic::async_trait]
impl ProtoFridgeGameEngine for FrjServer {

    type HostGameStream = PreGameStream;

    async fn host_game(&self, request: Request<ProtoHostGameReq>) -> Result<Response<Self::HostGameStream>, Status> {
        let req = request.into_inner();

        let (tx, rx) = mpsc::unbounded_channel();
        let client_out = StreamSender::new(tx);

        let game_type = GameType::try_from(ProtoGameType::try_from(req.game_type)?)?;
        let game = GameIdentifier {
            game_id: req.game_id,
            game_type
        };

        // This currently relies on the assumption of serialized access, which I'm only like
        // 90% sure will always work as expected. Might have to properly synchronize this later.
        self.game_repo_client.create_pregame(game.clone());
        self.game_repo_client.register_pregame_stream(req.player_id, game, client_out);

        Ok(Response::new(rx))
    }

    type JoinGameStream = PreGameStream;

    async fn join_game(&self, request: Request<ProtoJoinGameReq>) -> Result<Response<Self::JoinGameStream>, Status> {
        let req = request.into_inner();

        let (tx, rx) = mpsc::unbounded_channel();
        let client_out = StreamSender::new(tx);

        let game_type = GameType::try_from(ProtoGameType::try_from(req.game_type)?)?;
        let game = GameIdentifier {
            game_id: req.game_id,
            game_type
        };

        self.game_repo_client.register_pregame_stream(req.player_id, game, client_out);

        Ok(Response::new(rx))
    }

    async fn start_game(&self, request: Request<ProtoStartGameReq>) -> Result<Response<ProtoStartGameReply>, Status> {
        let req = request.into_inner();

        let (tx, rx) = oneshot::channel::<Result<ProtoStartGameReply, Status>>();

        let game_type = GameType::try_from(ProtoGameType::try_from(req.game_type)?)?;
        let game = GameIdentifier {
            game_id: req.game_id,
            game_type
        };

        self.game_repo_client.start_game(req.player_id, game, tx);

        rx.await
            .map_err(|e| {
                println!("ERROR: Failed to start game. Oneshot sender dropped before sending the reply; Debug: {:?}, Display: {}", e, e);
                Status::new(Code::Internal, "Failed to start the game")
            })?
            .map(|reply| Response::new(reply))
    }

    type OpenLoveLetterDataStreamStream = GameDataStream<ProtoLoveLetterDataOut>;

    async fn open_love_letter_data_stream(&self, request: Request<Streaming<ProtoLoveLetterDataIn>>) -> Result<Response<Self::OpenLoveLetterDataStreamStream>, Status> {
        let stream_in = request.into_inner();
        self.love_letter_stream_opener
            .handle_new_stream(stream_in)
            .await
            .map(|stream_out| Response::new(stream_out))
    }
}