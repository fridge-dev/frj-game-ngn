use crate::wire_api::proto_frj_ngn::proto_fridge_game_engine_server::ProtoFridgeGameEngine;
use crate::wire_api::proto_frj_ngn::{ProtoPreGameMessage, ProtoHostGameReq, ProtoJoinGameReq, ProtoGameType};
use crate::server::hack_type_converters::game_type;
use std::error::Error;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};
use backend_engine::{GameTaskClient, GameEvent};
use love_letter_backend::LoveLetterEvent;
use backend_framework::ClientOut;

/// Backend server is the entry point which will implement the gRPC server type.
pub struct FrjServer {
    games: GameTaskClient,
}

impl FrjServer {
    pub fn start() -> Result<Self, Box<dyn Error>> {
        let task_client = backend_engine::start_backend();
        Ok(FrjServer::new(task_client))
    }

    fn new(games: GameTaskClient) -> Self {
        FrjServer {
            games,
        }
    }
}

type PreGameStream = mpsc::Receiver<Result<ProtoPreGameMessage, Status>>;

#[tonic::async_trait]
impl ProtoFridgeGameEngine for FrjServer {
    type HostGameStream = PreGameStream;

    async fn host_game(&self, request: Request<ProtoHostGameReq>) -> Result<Response<Self::HostGameStream>, Status> {
        let req = request.into_inner();

        let (tx, rx) = mpsc::channel(4);
        let client_out = make_client_out(tx);

        let event = match game_type(req.game_type) {
            ProtoGameType::Unspecified => unimplemented!(),
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
}

fn make_client_out(_tx: mpsc::Sender<Result<ProtoPreGameMessage, Status>>) -> ClientOut {
    unimplemented!()
}

mod hack_type_converters {
    use crate::wire_api::proto_frj_ngn::ProtoGameType;

    pub fn game_type(proto_type: i32) -> ProtoGameType {
        match proto_type {
            1 => ProtoGameType::LoveLetter,
            2 => ProtoGameType::LostCities,
            _ => unimplemented!()
        }
    }
}
