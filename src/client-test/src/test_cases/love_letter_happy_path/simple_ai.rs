use crate::client::{LoggingBiStream, LoggingStreamRecv};
use client_engine::wire_api::proto_frj_ngn::{ProtoLoveLetterDataIn, ProtoLoveLetterDataOut, ProtoLvLeGameState};
use client_engine::wire_api::proto_frj_ngn::proto_love_letter_data_out::ProtoLvLeOut;

/// This AI has simple rules:
/// 1. Always keep higher value card (if possible, i.e. Countess)
/// 2. Always select first (allowed) player
/// 3. When playing Guard, always select Princess
/// 4. Disconnect after 5 rounds
pub async fn run_simple_game_ai(bi_stream: LoggingBiStream<ProtoLoveLetterDataIn, ProtoLoveLetterDataOut>) {
    let (sender, mut game_state_receiver) = {
        let (s, r) = bi_stream;
        (s, GameStateReceiver(r))
    };

    let payload = game_state_receiver.recv().await;

    // TODO:1 implement AI
    println!("FIRST FUCKING MESSAGE: {:#?}", payload);
}

struct GameStateReceiver(LoggingStreamRecv<ProtoLoveLetterDataOut>);
impl GameStateReceiver {
    pub async fn recv(&mut self) -> ProtoLvLeGameState {
        let msg = self.0
            .recv_data("AI stream")
            .await
            .proto_lv_le_out
            .expect("payload missing");

        match msg {
            ProtoLvLeOut::GameState(game_state) => game_state,
            _ => panic!("Received unexpected message: {:?}", msg),
        }
    }
}