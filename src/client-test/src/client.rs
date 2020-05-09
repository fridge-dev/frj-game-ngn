use client_engine::game_client::wrapper::GameClient;
use client_engine::wire_api::proto_frj_ngn::{ProtoHostGameReq, ProtoJoinGameReq, ProtoStartGameReq, ProtoStartGameReply, ProtoPreGameMessage, ProtoLoveLetterDataIn, ProtoLoveLetterDataOut};
use client_engine::wire_api::proto_frj_ngn::proto_love_letter_data_in::ProtoLvLeIn;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Formatter};
use tokio::sync::mpsc;
use tonic::{Status, Streaming};

fn time() -> String {
    format!("{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.6f"))
}

// ------- LoggingStreamRecv --------

pub struct LoggingStreamRecv<T: prost::Message> {
    inner: Streaming<T>,
    player_id: String,
}

pub enum StreamMsg<T: prost::Message> {
    Data(T),
    Err(Status),
    Closed,
}

impl<T: prost::Message> LoggingStreamRecv<T> {
    pub fn new(tonic_stream: Streaming<T>, player_id: String) -> Self {
        LoggingStreamRecv {
            inner: tonic_stream,
            player_id
        }
    }

    pub async fn recv(&mut self) -> StreamMsg<T> {
        let message = self.inner.message().await;

        println!("STREAM_RECV ({}) [{}]: {:?}", time(), self.player_id, message);

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

impl<T: prost::Message> Debug for LoggingStreamRecv<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "LoggingStreamRecv")
    }
}

// ------- LoggingStreamSender --------

pub struct LoggingStreamSender<T: prost::Message> {
    inner: mpsc::UnboundedSender<T>,
    player_id: String,
}

impl<T: prost::Message> LoggingStreamSender<T> {
    pub fn new(tx: mpsc::UnboundedSender<T>, player_id: String) -> Self {
        LoggingStreamSender {
            inner: tx,
            player_id
        }
    }

    pub fn send(&self, message: T) -> Result<(), ()> {
        println!("STREAM_SEND ({}) [{}]: {:?}", time(), self.player_id, message);
        self.inner.send(message)
            .map_err(|_| ())
    }
}

impl<T: prost::Message> Debug for LoggingStreamSender<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "LoggingStreamSender")
    }
}

impl LoggingStreamSender<ProtoLoveLetterDataIn> {
    pub fn send_lvle(&self, message_payload: ProtoLvLeIn) {
        let message = ProtoLoveLetterDataIn {
            clock: 0, // field unused, as of now. It's an OCC premature optimization :P
            proto_lv_le_in: Some(message_payload)
        };

        self.send(message)
            .expect(&format!("gRPC mpsc Receiver (that tunnels to sending to server) dropped."));
    }
}

// ------- LoggingGameClient --------

#[derive(Clone)]
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

    pub async fn host_game(&mut self, req: ProtoHostGameReq) -> Result<LoggingStreamRecv<ProtoPreGameMessage>, Status> {
        self.log_request(&req);
        let result = self.inner.host_game(req).await;
        self.log_result(result)
            .map(|stream| self.make_stream_recv(stream))
    }

    pub async fn join_game(&mut self, req: ProtoJoinGameReq) -> Result<LoggingStreamRecv<ProtoPreGameMessage>, Status> {
        self.log_request(&req);
        let result = self.inner.join_game(req).await;
        self.log_result(result)
            .map(|stream| self.make_stream_recv(stream))
    }

    pub async fn start_game(&mut self, req: ProtoStartGameReq) -> Result<ProtoStartGameReply, Status> {
        self.log_request(&req);
        let result = self.inner.start_game(req).await;
        self.log_result(result)
    }

    pub async fn open_love_letter_stream(&mut self) -> Result<
        (LoggingStreamSender<ProtoLoveLetterDataIn>, LoggingStreamRecv<ProtoLoveLetterDataOut>),
        Status
    > {
        self.log_request(&"OpenLoveLetterDataStream");
        let result = self.inner.open_love_letter_stream().await;
        self.log_result(result.map(|(snd, rcv)| (self.make_stream_sender(snd), self.make_stream_recv(rcv))))
    }

    fn log_request<I: Debug>(&self, req: &I) {
        println!("REQUEST ({}) [{}]: {:?}", time(), &self.player_id, req);
    }

    fn log_result<O: Debug, E: Debug>(&self, result: Result<O, E>) -> Result<O, E> {
        match &result {
            Ok(response) => println!("RESPONSE ({}) [{}]: {:?}", time(), &self.player_id, response),
            Err(status) => println!("ERROR ({}) [{}]: {:?}", time(), &self.player_id, status),
        }

        result
    }

    fn make_stream_recv<T: prost::Message>(&self, tonic_stream: Streaming<T>) -> LoggingStreamRecv<T> {
        LoggingStreamRecv::new(tonic_stream, self.player_id.clone())
    }

    fn make_stream_sender<T: prost::Message>(&self, tx: mpsc::UnboundedSender<T>) -> LoggingStreamSender<T> {
        LoggingStreamSender::new(tx, self.player_id.clone())
    }
}
