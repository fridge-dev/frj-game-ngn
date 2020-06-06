use crate::types::Color;

#[derive(Debug)]
pub struct MastermindEvent {
    pub client_player_id: String,
    pub payload: MastermindEventType,
}

/// TODO:1.5 should be `backend_framework::streaming::StreamSender`
#[derive(Debug)]
pub struct FakeStreamSender;

#[derive(Debug)]
pub enum MastermindEventType {
    // Common
    RegisterDataStream(FakeStreamSender),
    GetGameState,
    SubmitPassword,

    // Game-specific
    PutPeg {
        peg: usize,
        color: Color
    },
    CommitRow,
}
