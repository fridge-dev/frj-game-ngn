pub mod wrapper {
    use crate::wire_api::proto_frj_ngn::{ProtoHostGameReq, ProtoJoinGameReq, ProtoStartGameReq, ProtoStartGameReply, ProtoPreGameMessage, ProtoLoveLetterDataIn, ProtoLoveLetterDataOut};
    use crate::wire_api::proto_frj_ngn::proto_fridge_game_engine_client::ProtoFridgeGameEngineClient;
    use std::error::Error;
    use tokio::sync::mpsc;
    use tonic::transport::{Channel, Endpoint};
    use tonic::{Status, Streaming};

    #[derive(Clone)]
    pub struct GameClient {
        inner_client: ProtoFridgeGameEngineClient<Channel>
    }

    type DataStream<I, O> = (mpsc::UnboundedSender<I>, Streaming<O>);

    impl GameClient {

        pub async fn new(hostname: impl Into<String>, port: u16) -> Result<Self, Box<dyn Error>> {
            let url = format!("http://{}:{}", hostname.into(), port);
            println!("Connecting to {}", url);
            let endpoint = Endpoint::from_shared(url)?;

            let connection = endpoint.connect().await?;

            Ok(GameClient {
                inner_client: ProtoFridgeGameEngineClient::new(connection)
            })
        }

        // TODO:3 abstract away the tonic dependency
        pub async fn host_game(&mut self, req: ProtoHostGameReq) -> Result<Streaming<ProtoPreGameMessage>, Status> {
            self.inner_client
                .host_game(req)
                .await
                .map(|response| response.into_inner())
        }

        pub async fn join_game(&mut self, req: ProtoJoinGameReq) -> Result<Streaming<ProtoPreGameMessage>, Status> {
            self.inner_client
                .join_game(req)
                .await
                .map(|response| response.into_inner())
        }

        pub async fn start_game(&mut self, req: ProtoStartGameReq) -> Result<ProtoStartGameReply, Status> {
            self.inner_client
                .start_game(req)
                .await
                .map(|response| response.into_inner())
        }

        pub async fn open_love_letter_stream(&mut self) -> Result<DataStream<ProtoLoveLetterDataIn, ProtoLoveLetterDataOut>, Status> {
            let (snd, rcv) = mpsc::unbounded_channel();

            self.inner_client
                .open_love_letter_data_stream(rcv)
                .await
                .map(|response| (snd, response.into_inner()))
        }
    }
}