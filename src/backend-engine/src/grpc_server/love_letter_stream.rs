use crate::game_manager::api::GameRepositoryClient;
use crate::grpc_server::frj_server::GameDataStream;
use crate::grpc_server::stream_reader::StreamDriver;
use crate::grpc_server::stream_reader::StreamMessageHandler;
use backend_framework::streaming::StreamSender;
use backend_framework::wire_api::proto_frj_ngn::{ProtoLoveLetterDataIn, ProtoLoveLetterDataOut, ProtoGameType, ProtoGameDataHandshake};
use love_letter_backend::{LoveLetterEventType, LoveLetterEvent};
use tonic::{Streaming, Status, Code};
use tokio::sync::mpsc;
use backend_framework::wire_api::proto_frj_ngn::proto_love_letter_data_in::proto_data_in::PayloadIn;
use backend_framework::common_types::ClientInfo;

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
        self.game_repo_client.handle_event_love_letter(LoveLetterEvent {
            payload: LoveLetterEventType::RegisterDataStream(stream_out),
            client: ClientInfo {
                player_id: handshake.player_id.clone(),
                game_id: handshake.game_id.clone()
            },
        });

        self.start_stream_driver(stream_in, handshake);

        Ok(rx)
    }

    fn start_stream_driver(&self, stream_in: Streaming<ProtoLoveLetterDataIn>, handshake: ProtoGameDataHandshake) {
        let handler = LoveLetterStreamHandler {
            game_repo_client: self.game_repo_client.unsized_clone(),
            client: ClientInfo {
                player_id: handshake.player_id,
                game_id: handshake.game_id
            },
        };
        let stream_driver = StreamDriver::new(stream_in, handler);
        tokio::spawn(stream_driver.run());
    }

    async fn wait_for_handshake_message(&self, stream_in: &mut Streaming<ProtoLoveLetterDataIn>) -> Result<ProtoGameDataHandshake, Status> {
        match stream_in.message().await {
            Err(status) => {
                println!("WARN: Received Status err when expected Handshake. Err: {:?}", status);
                Err(Status::new(Code::FailedPrecondition, "Failed to read message from stream upon opening."))
            },
            Ok(None) => {
                println!("INFO: Stream closed as soon as it was opened. wtf!");
                Err(Status::new(Code::FailedPrecondition, "Read empty message from stream upon opening."))
            },
            Ok(Some(message)) => {
                println!("DEBUG: Received initial stream message: {:?}", message);
                match message.data {
                    None => {
                        println!("INFO: Stream initial message is missing data.");
                        Err(Status::new(Code::FailedPrecondition, "Expected data stream message to have data."))
                    }
                    Some(data) => match data.payload_in {
                        Some(PayloadIn::Handshake(handshake)) => Ok(handshake),
                        _ => {
                            println!("INFO: Stream initial message is not Handshake.");
                            Err(Status::new(Code::FailedPrecondition, "Expected first stream message to be Handshake message."))
                        },
                    },
                }
            },
        }
    }
}

struct LoveLetterStreamHandler {
    game_repo_client: Box<dyn GameRepositoryClient + Send + Sync>,
    client: ClientInfo,
}

impl LoveLetterStreamHandler {

    fn convert_and_send_message(&self, payload: PayloadIn) {
        let event_type = match payload {
            PayloadIn::Handshake(_) => {
                println!("INFO: Client stream sent Handshake message after handshake is done.");
                self.notify_client_invalid_message();
                return;
            },
            PayloadIn::GameState(_) => LoveLetterEventType::GetGameState,
            _ => {
                // TODO TODONEXT start here
                unimplemented!()
            }
        };

        self.game_repo_client.handle_event_love_letter(LoveLetterEvent {
            client: self.client.clone(),
            payload: event_type
        });
    }

    fn notify_client_invalid_message(&self) {
        // TODO: notify client that `messageId` was invalid.
        unimplemented!()
    }
}

impl StreamMessageHandler<ProtoLoveLetterDataIn> for LoveLetterStreamHandler {

    fn handle_message(&self, message: ProtoLoveLetterDataIn) {
        match message.data {
            Some(data) => match data.payload_in {
                Some(payload_in) => {
                    self.convert_and_send_message(payload_in);
                },
                None => {
                    println!("INFO: Client sent data message with missing payload.");
                    self.notify_client_invalid_message();
                },
            },
            None => {
                println!("INFO: Client sent data message with missing data_in.");
                self.notify_client_invalid_message();
            },
        }
    }
}