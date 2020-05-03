use crate::game_manager::api::GameRepositoryClient;
use crate::grpc_server::frj_server::GameDataStream;
use crate::grpc_server::stream_reader::StreamDriver;
use crate::grpc_server::stream_reader::StreamMessageHandler;
use backend_framework::common_types::ClientInfo;
use backend_framework::streaming::StreamSender;
use backend_framework::wire_api::proto_frj_ngn::{ProtoLoveLetterDataIn, ProtoLoveLetterDataOut, ProtoGameType, ProtoGameDataHandshake, ProtoLvLeCard};
use backend_framework::wire_api::proto_frj_ngn::proto_love_letter_data_in::ProtoLvLeIn;
use backend_framework::wire_api::proto_frj_ngn::proto_lv_le_play_card_req::ProtoLvLeCardSource;
use love_letter_backend::events::{LoveLetterEventType, LoveLetterEvent, PlayCardSource, Card};
use std::convert::TryFrom;
use tokio::sync::mpsc;
use tonic::{Streaming, Status, Code};

/// This struct is responsible for handling newly opened streams to the backend.
pub struct LoveLetterStreamHandler {
    game_repo_client: Box<dyn GameRepositoryClient + Send + Sync>,
}

impl LoveLetterStreamHandler {

    pub fn new(game_repo_client: Box<dyn GameRepositoryClient + Send + Sync>) -> Self {
        LoveLetterStreamHandler {
            game_repo_client
        }
    }

    /// This method is called when the server receives a request to open a LoveLetter data stream.
    pub async fn handle_new_stream(&self, mut stream_in_rcv: Streaming<ProtoLoveLetterDataIn>) -> Result<GameDataStream<ProtoLoveLetterDataOut>, Status> {
        let handshake = self.wait_for_handshake_message(&mut stream_in_rcv).await?;
        let client_info = self.validate_and_convert_handshake(handshake)?;

        self.spawn_stream_driver_task(stream_in_rcv, client_info.clone());

        let (tx, rx) = mpsc::unbounded_channel();
        self.register_stream_sender(tx, client_info);
        Ok(rx)
    }

    async fn wait_for_handshake_message(&self, stream_in_recv: &mut Streaming<ProtoLoveLetterDataIn>) -> Result<ProtoGameDataHandshake, Status> {
        match stream_in_recv.message().await {
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
                match message.proto_lv_le_in {
                    Some(ProtoLvLeIn::Handshake(handshake)) => Ok(handshake),
                    None => {
                        println!("INFO: Stream initial message is missing data.");
                        Err(Status::new(Code::FailedPrecondition, "Expected data stream message to have data."))
                    }
                    Some(_) => {
                        println!("INFO: Stream initial message is not Handshake.");
                        Err(Status::new(Code::FailedPrecondition, "Expected first stream message to be Handshake message."))
                    },
                }
            },
        }
    }

    fn validate_and_convert_handshake(&self, handshake: ProtoGameDataHandshake) -> Result<ClientInfo, Status> {
        if handshake.game_type != ProtoGameType::LoveLetter as i32 {
            println!("INFO: Invalid game type in Handshake message.");
            return Err(Status::new(Code::FailedPrecondition, "Invalid game type in Handshake message."));
        }

        Ok(ClientInfo {
            player_id: handshake.player_id,
            game_id: handshake.game_id
        })
    }

    fn register_stream_sender(&self, tx: mpsc::UnboundedSender<Result<ProtoLoveLetterDataOut, Status>>, client: ClientInfo) {
        let stream_out_snd = StreamSender::new(tx);
        self.game_repo_client.handle_event_love_letter(LoveLetterEvent {
            payload: LoveLetterEventType::RegisterDataStream(stream_out_snd),
            client,
        });
    }

    fn spawn_stream_driver_task(&self, stream_in: Streaming<ProtoLoveLetterDataIn>, client: ClientInfo) {
        let handler = LoveLetterStreamMessageHandler {
            game_repo_client: self.game_repo_client.unsized_clone(),
            client,
        };

        let stream_driver = StreamDriver::new(stream_in, handler);
        tokio::spawn(stream_driver.run());
    }
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
                    client: self.client.clone(),
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