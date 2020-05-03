use client_engine::game_client::wrapper::GameClient;
use client_engine::wire_api::proto_frj_ngn::{ProtoHostGameReq, ProtoJoinGameReq, ProtoStartGameReq, ProtoStartGameReply, ProtoPreGameMessage};
use std::error::Error;
use std::fmt::Debug;
use tonic::{Status, Streaming};

// ------- LoggingStream --------

pub struct LoggingStream<T: prost::Message> {
    inner: Streaming<T>,
    player_id: String,
}

pub enum StreamMsg<T: prost::Message> {
    Data(T),
    Err(Status),
    Closed,
}

impl<T: prost::Message> LoggingStream<T> {
    pub fn new(tonic_stream: Streaming<T>, player_id: String) -> Self {
        LoggingStream {
            inner: tonic_stream,
            player_id
        }
    }

    pub async fn recv(&mut self) -> StreamMsg<T> {
        let message = self.inner.message().await;

        println!("STREAM_RECV [{}]: {:?}", self.player_id, message);

        match message {
            Ok(Some(msg)) => StreamMsg::Data(msg),
            Ok(None) => StreamMsg::Closed,
            Err(status) => StreamMsg::Err(status),
        }
    }

    pub async fn recv_data(&mut self, stream_name: &'static str) -> T {
        match self.recv().await {
            StreamMsg::Data(data) => data,
            StreamMsg::Err(status) => {
                panic!("recv_data() on stream '{}', stream received error: {:?}", stream_name, status);
            },
            StreamMsg::Closed => {
                panic!("recv_data() on stream '{}', stream closed by server.", stream_name);
            },
        }
    }

    // Impl note: consume `self` so UTs don't attempt to use self after closed stream.
    pub async fn recv_closed(mut self, stream_name: &'static str) {
        match self.recv().await {
            StreamMsg::Data(data) => {
                panic!("recv_closed() on stream '{}', stream received data: {:?}", stream_name, data);
            },
            StreamMsg::Err(status) => {
                panic!("recv_closed() on stream '{}', stream received error: {:?}", stream_name, status);
            },
            StreamMsg::Closed => (),
        }
    }

    pub async fn recv_err(&mut self, stream_name: &'static str) -> Status {
        match self.recv().await {
            StreamMsg::Data(data) => {
                panic!("recv_err() on stream '{}', stream received data: {:?}", stream_name, data);
            },
            StreamMsg::Err(status) => status,
            StreamMsg::Closed => {
                panic!("recv_err() on stream '{}', stream closed by server.", stream_name);
            },
        }
    }
}

// ------- LoggingGameClient --------

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

    pub async fn host_game(&mut self, req: ProtoHostGameReq) -> Result<LoggingStream<ProtoPreGameMessage>, Status> {
        self.log_request(&req);
        let result = self.inner.host_game(req).await;
        self.log_result(result)
            .map(|stream| self.make_stream(stream))
    }

    pub async fn join_game(&mut self, req: ProtoJoinGameReq) -> Result<LoggingStream<ProtoPreGameMessage>, Status> {
        self.log_request(&req);
        let result = self.inner.join_game(req).await;
        self.log_result(result)
            .map(|stream| self.make_stream(stream))
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

    fn make_stream<T: prost::Message>(&self, tonic_stream: Streaming<T>) -> LoggingStream<T> {
        LoggingStream::new(tonic_stream, self.player_id.clone())
    }
}