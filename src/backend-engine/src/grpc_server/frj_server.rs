use crate::task;
use crate::task::{GameTaskClient};
use crate::game_manager::GameEvent;
use crate::grpc_server::love_letter_stream::LoveLetterStreamOpener;
use backend_framework::wire_api::proto_frj_ngn::proto_fridge_game_engine_server::ProtoFridgeGameEngine;
use backend_framework::wire_api::proto_frj_ngn::{ProtoPreGameMessage, ProtoHostGameReq, ProtoJoinGameReq, ProtoGameType, ProtoStartGameReq, ProtoStartGameReply, ProtoLoveLetterDataIn, ProtoLoveLetterDataOut};
use backend_framework::streaming::StreamSender;
use love_letter_backend::LoveLetterEvent;
use std::convert::TryFrom;
use std::error::Error;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tonic::{Request, Response, Status, Streaming, Code};

/// Backend server is the entry point which will implement the gRPC server type.
pub struct FrjServer {
    game_task_client: GameTaskClient,
    love_letter_stream_opener: LoveLetterStreamOpener,
}

impl FrjServer {

    pub fn start() -> Result<Self, Box<dyn Error>> {
        let task_client = task::start_backend();
        let love_letter_stream_opener = LoveLetterStreamOpener::new(task_client.clone());

        Ok(FrjServer::new(
            task_client,
            love_letter_stream_opener
        ))
    }

    fn new(
        game_task_client: GameTaskClient,
        love_letter_stream_opener: LoveLetterStreamOpener,
    ) -> Self {
        FrjServer {
            game_task_client,
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

        let event = match ProtoGameType::try_from(req.game_type)? {
            ProtoGameType::UnspecifiedGameType => unimplemented!(),
            ProtoGameType::LoveLetter => {
                GameEvent::LoveLetter(LoveLetterEvent::JoinGame(req.player_id, client_out))
            },
            ProtoGameType::LostCities => unimplemented!(),
        };

        self.game_task_client.send(req.game_id, event);

        Ok(Response::new(rx))
    }

    type JoinGameStream = PreGameStream;

    async fn join_game(&self, request: Request<ProtoJoinGameReq>) -> Result<Response<Self::JoinGameStream>, Status> {
        let req = request.into_inner();

        let (tx, rx) = mpsc::unbounded_channel();
        let client_out = StreamSender::new(tx);

        let event = match ProtoGameType::try_from(req.game_type)? {
            ProtoGameType::UnspecifiedGameType => unimplemented!(),
            ProtoGameType::LoveLetter => {
                GameEvent::LoveLetter(LoveLetterEvent::JoinGame(req.player_id, client_out))
            },
            ProtoGameType::LostCities => unimplemented!(),
        };

        self.game_task_client.send(req.game_id, event);

        Ok(Response::new(rx))
    }

    async fn start_game(&self, request: Request<ProtoStartGameReq>) -> Result<Response<ProtoStartGameReply>, Status> {
        let req = request.into_inner();

        let (tx, rx) = oneshot::channel::<ProtoStartGameReply>();

        let event = match ProtoGameType::try_from(req.game_type)? {
            ProtoGameType::UnspecifiedGameType => unimplemented!(),
            ProtoGameType::LoveLetter => {
                GameEvent::LoveLetter(LoveLetterEvent::StartGame(req.player_id, tx))
            },
            ProtoGameType::LostCities => unimplemented!(),
        };

        self.game_task_client.send(req.game_id, event);

        rx.await
            .map(|reply| Response::new(reply))
            .map_err(|e| {
                println!("ERROR: Failed to start game. Oneshot sender dropped before sending the reply; Debug: {:?}, Display: {}", e, e);
                Status::new(Code::Internal, "Failed to start the game")
            })
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