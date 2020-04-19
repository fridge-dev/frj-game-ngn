use crate::task;
use crate::task::{GameTaskClient};
use crate::game_manager::GameEvent;
use backend_framework::wire_api::proto_frj_ngn::proto_fridge_game_engine_server::ProtoFridgeGameEngine;
use backend_framework::wire_api::proto_frj_ngn::{ProtoPreGameMessage, ProtoHostGameReq, ProtoJoinGameReq, ProtoGameType, ProtoGetGameStateReq, ProtoGetGameStateReply, ProtoGameDataIn, ProtoGameDataOut};
use backend_framework::{ClientOut, MessageErrType};
use love_letter_backend::LoveLetterEvent;
use std::error::Error;
use tokio::sync::mpsc;
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
        let client_out = make_client_out(tx);

        let event = match hack_type_converters::game_type(req.game_type) {
            ProtoGameType::UnspecifiedGameType => unimplemented!(),
            ProtoGameType::LoveLetter => {
                GameEvent::LoveLetter(LoveLetterEvent::Join(req.player_id, client_out))
            },
            ProtoGameType::LostCities => unimplemented!(),
        };

        self.games.send(req.game_id, event);

        Ok(Response::new(rx))
    }

    type JoinGameStream = PreGameStream;

    async fn join_game(&self, _request: Request<ProtoJoinGameReq>) -> Result<Response<Self::JoinGameStream>, Status> {
        unimplemented!()
    }

    async fn get_game_state(&self, _request: Request<ProtoGetGameStateReq>) -> Result<Response<ProtoGetGameStateReply>, Status> {
        unimplemented!()
    }

    type OpenGameDataStreamStream = GameDataStream;

    async fn open_game_data_stream(&self, _request: Request<Streaming<ProtoGameDataIn>>) -> Result<Response<Self::OpenGameDataStreamStream>, Status> {
        unimplemented!()
    }
}

fn make_client_out(tx: mpsc::UnboundedSender<Result<ProtoPreGameMessage, Status>>) -> Box<StreamClientOut> {
    Box::new(StreamClientOut {
        sender: tx
    })
}

#[derive(Debug)]
struct StreamClientOut {
    sender: mpsc::UnboundedSender<Result<ProtoPreGameMessage, Status>>,
}

impl ClientOut for StreamClientOut {

    fn send_pre_game_message(&self, message: ProtoPreGameMessage) {
        if let Err(msg) = self.sender.send(Ok(message)) {
            println!("WARN: Client stream dropped. We failed to send message: {:?}", msg);
        }
    }

    fn send_error_message(&self, message: String, err_type: MessageErrType) {
        if let Err(msg) = self.sender.send(Err(Status::new(convert_err(err_type), message))) {
            println!("WARN: Client stream dropped. We failed to send message: {:?}", msg);
        }
    }
}

fn convert_err(err_type: MessageErrType) -> Code {
    match err_type {
        MessageErrType::ServerFault => Code::Internal,
        MessageErrType::InvalidReq => Code::InvalidArgument,
    }
}

mod hack_type_converters {
    use backend_framework::wire_api::proto_frj_ngn::ProtoGameType;

    pub fn game_type(proto_type: i32) -> ProtoGameType {
        match proto_type {
            1 => ProtoGameType::LoveLetter,
            2 => ProtoGameType::LostCities,
            _ => unimplemented!()
        }
    }
}
