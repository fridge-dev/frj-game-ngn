use client_engine::game_client::wrapper::GameClient;
use client_engine::wire_api::proto_frj_ngn::{ProtoHostGameReq, ProtoJoinGameReq, ProtoStartGameReq, ProtoStartGameReply, ProtoPreGameMessage};
use std::error::Error;
use std::fmt::Debug;
use tonic::{Status, Streaming};

// TODO add a logging streamer
pub struct LoggingGameClient {
    inner: GameClient,
    player_id: String,
}

impl LoggingGameClient {
    pub async fn new(player_id: impl Into<String>) -> Result<Self, Box<dyn Error>> {
        let inner = GameClient::new("[::]", 8051).await?;

        Ok(LoggingGameClient {
            inner,
            player_id: player_id.into()
        })
    }

    pub fn player_id(&self) -> &String {
        &self.player_id
    }

    pub async fn host_game(&mut self, req: ProtoHostGameReq) -> Result<Streaming<ProtoPreGameMessage>, Status> {
        self.log_request(&req);
        let result = self.inner.host_game(req).await;
        self.log_result(result)
    }

    pub async fn join_game(&mut self, req: ProtoJoinGameReq) -> Result<Streaming<ProtoPreGameMessage>, Status> {
        self.log_request(&req);
        let result = self.inner.join_game(req).await;
        self.log_result(result)
    }

    pub async fn start_game(&mut self, req: ProtoStartGameReq) -> Result<ProtoStartGameReply, Status> {
        self.log_request(&req);
        let result = self.inner.start_game(req).await;
        self.log_result(result)
    }

    fn log_request<I: Debug>(&self, req: &I) {
        println!("REQUEST [{}]: {:?}", &self.player_id, req);
    }

    fn log_result<O, E>(&self, result: Result<O, E>) -> Result<O, E>
        where
            O: Debug,
            E: Debug,
    {
        match &result {
            Ok(response) => println!("RESPONSE [{}]: {:?}", &self.player_id, response),
            Err(status) => println!("ERROR [{}]: {:?}", &self.player_id, status),
        }

        result
    }

}