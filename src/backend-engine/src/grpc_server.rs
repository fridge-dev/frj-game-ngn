use crate::task;
use crate::task::{GameTaskClient};
use crate::game_manager::GameEvent;
use backend_framework::wire_api::proto_frj_ngn::proto_fridge_game_engine_server::ProtoFridgeGameEngine;
use backend_framework::wire_api::proto_frj_ngn::{ProtoPreGameMessage, ProtoHostGameReq, ProtoJoinGameReq, ProtoGameType, ProtoGetGameStateReq, ProtoGetGameStateReply, ProtoGameDataIn, ProtoGameDataOut, ProtoStartGameReq, ProtoStartGameReply};
use backend_framework::streaming::StreamSender;
use love_letter_backend::LoveLetterEvent;
use std::convert::TryFrom;
use std::error::Error;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tonic::{Request, Response, Status, Streaming, Code};

/// Backend server is the entry point which will implement the gRPC server type.
pub struct FrjServer {
    games: GameTaskClient,
}

impl FrjServer {
    pub fn start() -> Result<Self, Box<dyn Error>> {
        let task_client = task::start_backend();
        Ok(FrjServer::new(task_client))
    }

    fn new(games: GameTaskClient) -> Self {
        FrjServer {
            games,
        }
    }
}

type PreGameStream = mpsc::UnboundedReceiver<Result<ProtoPreGameMessage, Status>>;
type GameDataStream = mpsc::UnboundedReceiver<Result<ProtoGameDataOut, Status>>;

#[tonic::async_trait]
impl ProtoFridgeGameEngine for FrjServer {
    type HostGameStream = PreGameStream;

    async fn host_game(&self, request: Request<ProtoHostGameReq>) -> Result<Response<Self::HostGameStream>, Status> {
        let req = request.into_inner();

        let (tx, rx) = mpsc::unbounded_channel();
        let client_out = make_pre_game_stream(tx);

        let event = match ProtoGameType::try_from(req.game_type)? {
            ProtoGameType::UnspecifiedGameType => unimplemented!(),
            ProtoGameType::LoveLetter => {
                GameEvent::LoveLetter(LoveLetterEvent::JoinGame(req.player_id, client_out))
            },
            ProtoGameType::LostCities => unimplemented!(),
        };

        self.games.send(req.game_id, event);

        Ok(Response::new(rx))
    }

    type JoinGameStream = PreGameStream;

    async fn join_game(&self, request: Request<ProtoJoinGameReq>) -> Result<Response<Self::JoinGameStream>, Status> {
        let req = request.into_inner();

        let (tx, rx) = mpsc::unbounded_channel();
        let client_out = make_pre_game_stream(tx);

        let event = match ProtoGameType::try_from(req.game_type)? {
            ProtoGameType::UnspecifiedGameType => unimplemented!(),
            ProtoGameType::LoveLetter => {
                GameEvent::LoveLetter(LoveLetterEvent::JoinGame(req.player_id, client_out))
            },
            ProtoGameType::LostCities => unimplemented!(),
        };

        self.games.send(req.game_id, event);

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

        self.games.send(req.game_id, event);

        rx.await
            .map(|reply| Response::new(reply))
            .map_err(|e| {
                println!("ERROR: Failed to start game. Oneshot sender dropped before sending the reply; Debug: {:?}, Display: {}", e, e);
                Status::new(Code::Internal, "Failed to start the game")
            })
    }

    async fn get_game_state(&self, _request: Request<ProtoGetGameStateReq>) -> Result<Response<ProtoGetGameStateReply>, Status> {
        unimplemented!()
    }

    type OpenGameDataStreamStream = GameDataStream;

    async fn open_game_data_stream(&self, _request: Request<Streaming<ProtoGameDataIn>>) -> Result<Response<Self::OpenGameDataStreamStream>, Status> {
        unimplemented!()
    }
}

fn make_pre_game_stream(tx: mpsc::UnboundedSender<Result<ProtoPreGameMessage, Status>>) -> StreamSender<ProtoPreGameMessage> {
    StreamSender::new(tx)
}
