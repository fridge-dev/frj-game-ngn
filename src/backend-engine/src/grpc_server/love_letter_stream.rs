use crate::game_manager::api::GameRepositoryClient;
use crate::grpc_server::frj_server::GameDataStream;
use crate::grpc_server::stream_reader::StreamDriver;
use crate::grpc_server::stream_reader::StreamMessageHandler;
use backend_framework::streaming::StreamSender;
use backend_framework::wire_api::proto_frj_ngn::{ProtoLoveLetterDataIn, ProtoLoveLetterDataOut, ProtoGameType, ProtoGameDataHeader};
use backend_framework::wire_api::proto_frj_ngn::proto_love_letter_data_in::Inner;
use love_letter_backend::LoveLetterEvent;
use tonic::{Streaming, Status, Code};
use tokio::sync::mpsc;

pub struct LoveLetterStreamOpener {
    game_repo_client: Box<dyn GameRepositoryClient + Send + Sync>,
}

impl LoveLetterStreamOpener {
    pub fn new(game_repo_client: Box<dyn GameRepositoryClient + Send + Sync>) -> Self {
        LoveLetterStreamOpener {
            game_repo_client
        }
    }

    pub async fn handle_new_stream(&self, mut stream_in: Streaming<ProtoLoveLetterDataIn>) -> Result<GameDataStream<ProtoLoveLetterDataOut>, Status> {
        let handshake = self.wait_for_handshake_message(&mut stream_in).await?;

        if handshake.game_type != ProtoGameType::LoveLetter as i32 {
            println!("INFO: Invalid game type in Handshake message.");
            return Err(Status::new(Code::FailedPrecondition, "Invalid game type in Handshake message."));
        }

        let (tx, rx) = mpsc::unbounded_channel();
        let stream_out = StreamSender::new(tx);
        self.game_repo_client.handle_event_love_letter(LoveLetterEvent::RegisterDataStream(
            handshake.player_id.clone(),
            stream_out
        ));

        self.start_stream_driver(stream_in, handshake);

        Ok(rx)
    }

    fn start_stream_driver(&self, stream_in: Streaming<ProtoLoveLetterDataIn>, handshake: ProtoGameDataHeader) {
        let handler = LoveLetterStreamHandler {
            game_repo_client: self.game_repo_client.unsized_clone(),
            game_id: handshake.game_id,
            player_id: handshake.player_id,
        };
        let stream_driver = StreamDriver::new(stream_in, handler);
        tokio::spawn(stream_driver.run());
    }

    async fn wait_for_handshake_message(&self, stream_in: &mut Streaming<ProtoLoveLetterDataIn>) -> Result<ProtoGameDataHeader, Status> {
        match stream_in.message().await {
            Err(status) => {
                println!("WARN: Received Status err when expected DataStreamHeader. Err: {:?}", status);
                Err(Status::new(Code::FailedPrecondition, "Failed to read message from stream upon opening."))
            },
            Ok(None) => {
                println!("INFO: Stream closed as soon as it was opened. wtf!");
                Err(Status::new(Code::FailedPrecondition, "Read empty message from stream upon opening."))
            },
            Ok(Some(message)) => {
                println!("DEBUG: Received open stream header message: {:?}", message);
                match message.inner {
                    Some(Inner::Header(header)) => {
                        Ok(header)
                    },
                    _ => {
                        Err(Status::new(Code::FailedPrecondition, "Expected first stream message to be Header handshake message."))
                    }
                }
            },
        }
    }
}

struct LoveLetterStreamHandler {
    game_repo_client: Box<dyn GameRepositoryClient + Send + Sync>,
    #[allow(dead_code)] // TODO is this needed?
    game_id: String,
    #[allow(dead_code)] // TODO is this needed?
    player_id: String,
}

impl LoveLetterStreamHandler {

    fn fwd(&self, event: LoveLetterEvent) {
        self.game_repo_client.handle_event_love_letter(event);
    }

    fn convert_message(&self, inner: Inner) -> Option<LoveLetterEvent> {
        match inner {
            Inner::GameStateReq(_) => {
                Some(LoveLetterEvent::GetGameState(self.player_id.clone()))
            },
            Inner::ExMsg(_msg) => {
                Some(LoveLetterEvent::PlayCardCommit(self.player_id.clone()))
            },
            Inner::Header(_header) => {
                println!("INFO: Client sent header after stream handshake.");
                self.notify_client_invalid_message();
                None
            },
        }
    }

    fn notify_client_invalid_message(&self) {
        // TODO: notify client that `messageId` was invalid.
        unimplemented!()
    }
}

impl StreamMessageHandler<ProtoLoveLetterDataIn> for LoveLetterStreamHandler {

    fn handle_message(&self, message: ProtoLoveLetterDataIn) {
        match message.inner {
            Some(inner) => {
                if let Some(event) = self.convert_message(inner) {
                    self.fwd(event);
                }
            },
            None => {
                println!("INFO: Client sent data message with missing payload.");
                self.notify_client_invalid_message();
            },
        }
    }
}