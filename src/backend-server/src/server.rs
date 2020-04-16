use crate::wire_api::proto_frj_ngn::{ProtoHostGameReq, ProtoHostGameReply};
use std::error::Error;
use tonic::{Request, Response, Status};
use crate::wire_api::proto_frj_ngn::proto_fridge_game_engine_server::ProtoFridgeGameEngine;

/// Backend server is the entry point which will implement the gRPC server type.
pub struct FrjServer {
}

impl FrjServer {
    pub fn start() -> Result<Self, Box<dyn Error>> {
        Ok(FrjServer::new())
    }

    fn new() -> Self {
        FrjServer {
        }
    }
}

#[tonic::async_trait]
impl ProtoFridgeGameEngine for FrjServer {
    async fn host_game(&self, _request: Request<ProtoHostGameReq>) -> Result<Response<ProtoHostGameReply>, Status> {
        unimplemented!()
    }
}