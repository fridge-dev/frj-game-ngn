use crate::game_manager::api::GameRepositoryClient;
use crate::grpc_server::frj_server::GameDataStream;
use crate::grpc_server::stream_reader::StreamDriver;
use crate::grpc_server::stream_reader::StreamMessageHandler;
use backend_framework::common_types::ClientInfo;
use backend_framework::streaming::StreamSender;
use backend_framework::wire_api::proto_frj_ngn::{ProtoLoveLetterDataIn, ProtoLoveLetterDataOut, ProtoGameDataHandshake, ProtoLvLeCard};
use backend_framework::wire_api::proto_frj_ngn::proto_love_letter_data_in::ProtoLvLeIn;
use backend_framework::wire_api::proto_frj_ngn::proto_lv_le_play_card_req::ProtoLvLeCardSource;
use love_letter_backend::events::{LoveLetterEventType, LoveLetterEvent, PlayCardSource, Card};
use std::convert::TryFrom;
use tokio::sync::mpsc;
use tonic::{Streaming, Status, Code};

/// This struct is responsible for handling newly opened streams to the backend.
pub struct LoveLetterStreamInitializer {
    game_repo_client: Box<dyn GameRepositoryClient + Send + Sync>,
}

impl LoveLetterStreamInitializer {

    pub fn new(game_repo_client: Box<dyn GameRepositoryClient + Send + Sync>) -> Self {
        LoveLetterStreamInitializer {
            game_repo_client
        }
    }

    /// This method is called when the server receives a request to open a LoveLetter data stream.
    pub async fn handle_new_stream(&self, stream_in_rcv: Streaming<ProtoLoveLetterDataIn>) -> Result<GameDataStream<ProtoLoveLetterDataOut>, Status> {
        let (tx, rx) = mpsc::unbounded_channel();
        let game_repo_client = self.game_repo_client.unsized_clone();

        tokio::spawn(Self::initialize_bi_stream_processors(game_repo_client, tx, stream_in_rcv));

        Ok(rx)
    }

    // Here, we have the 2 "server" halves of a bidirectional stream.
    async fn initialize_bi_stream_processors(
        game_repo_client: Box<dyn GameRepositoryClient + Send + Sync>,
        stream_out: mpsc::UnboundedSender<Result<ProtoLoveLetterDataOut, Status>>,
        mut stream_in: Streaming<ProtoLoveLetterDataIn>,
    ) {
        // 1. Poll receiver for handshake
        let handshake_result = wait_for_handshake_message(&mut stream_in).await;
        let handshake = match handshake_result {
            Ok(handshake) => handshake,
            Err(e) => {
                let _ = stream_out.send(Err(e));
                return;
            },
        };
        let client_info = ClientInfo::from(handshake);

        // 2. Register sender to backend
        let payload = LoveLetterEventType::RegisterDataStream(StreamSender::new(stream_out));
        game_repo_client.handle_event_love_letter(LoveLetterEvent {
            payload,
            client_info: client_info.clone(),
        });

        // 3. Spawn task to poll receiver
        spawn_stream_driver_task(game_repo_client, stream_in, client_info);
    }
}

async fn wait_for_handshake_message(stream_in_recv: &mut Streaming<ProtoLoveLetterDataIn>) -> Result<ProtoGameDataHandshake, Status> {
    match stream_in_recv.message().await {
        Err(status) => {
            println!("WARN: LoveLetterStreamInitializer Received Status err when expected Handshake. Err: {:?}", status);
            Err(Status::new(Code::FailedPrecondition, "Failed to read message from stream upon opening."))
        },
        Ok(None) => {
            println!("INFO: LoveLetterStreamInitializer Stream closed as soon as it was opened. wtf!");
            Err(Status::new(Code::FailedPrecondition, "Read empty message from stream upon opening."))
        },
        Ok(Some(message)) => {
            println!(
                "DEBUG: ({}) LoveLetterStreamInitializer Received initial stream message: {:?}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.6f"), // TODO remove?
                message,
            );
            match message.proto_lv_le_in {
                Some(ProtoLvLeIn::Handshake(handshake)) => Ok(handshake),
                None => {
                    println!("INFO: LoveLetterStreamInitializer Stream initial message is missing data.");
                    Err(Status::new(Code::FailedPrecondition, "Expected data stream message to have data."))
                }
                Some(_) => {
                    println!("INFO: LoveLetterStreamInitializer Stream initial message is not Handshake.");
                    Err(Status::new(Code::FailedPrecondition, "Expected first stream message to be Handshake message."))
                },
            }
        },
    }
}

fn spawn_stream_driver_task(
    game_repo_client: Box<dyn GameRepositoryClient + Send + Sync>,
    stream_in: Streaming<ProtoLoveLetterDataIn>,
    client: ClientInfo
) {
    let stream_id = format!("{}--{}", client.game_id, client.player_id);
    let handler = LoveLetterStreamMessageHandler {
        game_repo_client,
        client,
    };

    let stream_driver = StreamDriver::new(stream_id, stream_in, handler);
    tokio::spawn(stream_driver.run());
}

/// This struct is responsible for handling individual messages from the client stream.
struct LoveLetterStreamMessageHandler {
    game_repo_client: Box<dyn GameRepositoryClient + Send + Sync>,
    client: ClientInfo,
}

impl LoveLetterStreamMessageHandler {

    fn convert_and_send_message(&self, payload: ProtoLvLeIn) {
        match self.convert_message(payload) {
            Err(status) => self.notify_client_invalid_message(status),
            Ok(event_type) => {
                let event = LoveLetterEvent {
                    client_info: self.client.clone(),
                    payload: event_type
                };
                self.game_repo_client.handle_event_love_letter(event);
            },
        }
    }

    fn convert_message(&self, payload: ProtoLvLeIn) -> Result<LoveLetterEventType, Status> {
        match payload {
            ProtoLvLeIn::Handshake(_) => {
                println!("INFO: Client stream sent Handshake message after handshake is done.");
                Err(Status::failed_precondition("Client sent handshake twice."))
            },
            ProtoLvLeIn::GameState(_) => Ok(LoveLetterEventType::GetGameState),
            ProtoLvLeIn::PlayCard(req) => {
                let proto_card_source = ProtoLvLeCardSource::try_from(req.card_source)?;
                let card_source = PlayCardSource::try_from(proto_card_source)
                    .map_err(|_| Status::invalid_argument("Unspecified ProtoLvLeCardSource"))?;
                Ok(LoveLetterEventType::PlayCardStaged(card_source))
            },
            ProtoLvLeIn::SelectTargetPlayer(req) => {
                Ok(LoveLetterEventType::SelectTargetPlayer(req.target_player_id))
            },
            ProtoLvLeIn::SelectTargetCard(req) => {
                let proto_card = ProtoLvLeCard::try_from(req.target_card)?;
                let card = Card::try_from(proto_card)
                    .map_err(|_| Status::invalid_argument("Unspecified ProtoLvLeCard"))?;
                Ok(LoveLetterEventType::SelectTargetCard(card))
            },
            ProtoLvLeIn::CommitSelection(_) => {
                Ok(LoveLetterEventType::PlayCardCommit)
            },
        }
    }

    fn notify_client_invalid_message(&self, status: Status) {
        // TODO: notify client that `messageId` was invalid.
        // Close stream? Drop message? Idk.
        println!("Client sent invalid message to data stream. Sending err {:?}", status);
        unimplemented!("TODO notify client of invalid message")
    }
}

impl StreamMessageHandler<ProtoLoveLetterDataIn> for LoveLetterStreamMessageHandler {

    fn handle_message(&self, message: ProtoLoveLetterDataIn) {
        match message.proto_lv_le_in {
            None => {
                self.notify_client_invalid_message(Status::invalid_argument("Missing proto_lv_le_in field."));
            },
            Some(payload) => {
                self.convert_and_send_message(payload)
            },
        }
    }
}