/// Every Data Stream should begin by client sending a handshake message to server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoGameDataHandshake {
    #[prost(string, tag = "1")]
    pub player_id: std::string::String,
    #[prost(string, tag = "2")]
    pub game_id: std::string::String,
    #[prost(enumeration = "ProtoGameType", tag = "3")]
    pub game_type: i32,
}
/// Empty: This means "send me the latest state for the game stream I have opened".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoGameDataStateReq {}
// ======================================================
// Common types needed for all games.
// ======================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoGameType {
    UnspecifiedGameType = 0,
    LoveLetter = 1,
    LostCities = 2,
}
// =======================================
// Data Stream Messages
// =======================================

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoLoveLetterDataIn {
    /// Logical clock for this game instance, provided for OCC if game needs it.
    #[prost(uint64, tag = "1")]
    pub clock: u64,
    /// The actual message
    #[prost(
        oneof = "proto_love_letter_data_in::ProtoLvLeIn",
        tags = "2, 3, 4, 5, 6, 7"
    )]
    pub proto_lv_le_in: ::std::option::Option<proto_love_letter_data_in::ProtoLvLeIn>,
}
pub mod proto_love_letter_data_in {
    /// The actual message
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ProtoLvLeIn {
        #[prost(message, tag = "2")]
        Handshake(super::ProtoGameDataHandshake),
        #[prost(message, tag = "3")]
        GameState(super::ProtoGameDataStateReq),
        #[prost(message, tag = "4")]
        PlayCard(super::ProtoLvLePlayCardReq),
        #[prost(message, tag = "5")]
        SelectTargetPlayer(super::ProtoLvLeSelectTargetPlayer),
        #[prost(message, tag = "6")]
        SelectTargetCard(super::ProtoLvLeSelectTargetCard),
        #[prost(message, tag = "7")]
        CommitSelection(super::ProtoLvLeCommitSelectionReq),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoLoveLetterDataOut {
    /// Logical clock for this game instance, provided for OCC if game needs it.
    #[prost(uint64, tag = "1")]
    pub clock: u64,
    /// The actual message
    #[prost(
        oneof = "proto_love_letter_data_out::ProtoLvLeOut",
        tags = "2, 3, 4, 5, 6, 7, 8"
    )]
    pub proto_lv_le_out: ::std::option::Option<proto_love_letter_data_out::ProtoLvLeOut>,
}
pub mod proto_love_letter_data_out {
    /// The actual message
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ProtoLvLeOut {
        #[prost(message, tag = "2")]
        GameState(super::ProtoLvLeGameState),
        #[prost(message, tag = "3")]
        TurnIndicator(super::ProtoLvLeTurnIndicatorRepl),
        #[prost(message, tag = "4")]
        PlayCard(super::ProtoLvLePlayCardRepl),
        #[prost(message, tag = "5")]
        StageCard(super::ProtoLvLeStageCardRepl),
        #[prost(message, tag = "6")]
        SelectTargetPlayer(super::ProtoLvLeSelectTargetPlayer),
        #[prost(message, tag = "7")]
        SelectTargetCard(super::ProtoLvLeSelectTargetCard),
        #[prost(message, tag = "8")]
        CommitSelection(super::ProtoLvLeCommitSelectionRepl),
    }
}
// =======================================
// API Request and Reply messages
// =======================================

// --- GameState

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoLvLeGameState {
    #[prost(uint64, tag = "1")]
    pub clock: u64,
    #[prost(message, repeated, tag = "2")]
    pub players: ::std::vec::Vec<proto_lv_le_game_state::ProtoLvLePlayer>,
    #[prost(oneof = "proto_lv_le_game_state::Stage", tags = "3, 4")]
    pub stage: ::std::option::Option<proto_lv_le_game_state::Stage>,
}
pub mod proto_lv_le_game_state {
    // -- nested message types

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoLvLePlayer {
        #[prost(string, tag = "1")]
        pub player_id: std::string::String,
        #[prost(uint32, tag = "2")]
        pub round_wins: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoLvLeRoundState {
        #[prost(string, repeated, tag = "1")]
        pub remaining_player_ids: ::std::vec::Vec<std::string::String>,
        #[prost(enumeration = "super::ProtoLvLeCard", tag = "2")]
        pub my_hand: i32,
        #[prost(message, optional, tag = "5")]
        pub staged_play: ::std::option::Option<super::ProtoLvLeCardSelection>,
        #[prost(message, optional, tag = "6")]
        pub most_recent_committed_play: ::std::option::Option<super::ProtoLvLeCommittedPlay>,
        #[prost(enumeration = "super::ProtoLvLeCard", repeated, tag = "7")]
        pub play_history: ::std::vec::Vec<i32>,
        #[prost(oneof = "proto_lv_le_round_state::Turn", tags = "3, 4")]
        pub turn: ::std::option::Option<proto_lv_le_round_state::Turn>,
    }
    pub mod proto_lv_le_round_state {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Turn {
            #[prost(enumeration = "super::super::ProtoLvLeCard", tag = "3")]
            MyDrawnCard(i32),
            #[prost(string, tag = "4")]
            CurrentTurnPlayerId(std::string::String),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoLvLeResultState {
        /// Sparse map, missing value => player eliminated
        #[prost(map = "string, enumeration(super::ProtoLvLeCard)", tag = "1")]
        pub final_cards: ::std::collections::HashMap<std::string::String, i32>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Stage {
        #[prost(message, tag = "3")]
        RoundInProgress(ProtoLvLeRoundState),
        #[prost(message, tag = "4")]
        RoundIntermission(ProtoLvLeResultState),
    }
}
// --- TurnIndicator

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoLvLeTurnIndicatorRepl {
    /// Current player's turn
    #[prost(string, tag = "1")]
    pub player_id: std::string::String,
    /// The new card drawn from top deck
    #[prost(enumeration = "ProtoLvLeCard", tag = "2")]
    pub your_card: i32,
}
// --- PlayCard

/// Req: First action taken during a turn
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoLvLePlayCardReq {
    #[prost(
        enumeration = "proto_lv_le_play_card_req::ProtoLvLeCardSource",
        tag = "1"
    )]
    pub card_source: i32,
}
pub mod proto_lv_le_play_card_req {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ProtoLvLeCardSource {
        UnspecifiedCardSource = 0,
        Hand = 1,
        TopDeck = 2,
    }
}
/// Repl: Sent when card has no selection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoLvLePlayCardRepl {
    #[prost(enumeration = "ProtoLvLeCard", tag = "1")]
    pub played_card: i32,
    #[prost(message, optional, tag = "2")]
    pub outcome: ::std::option::Option<ProtoLvLeCardOutcome>,
}
/// Repl: Sent when card requires selection
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoLvLeStageCardRepl {
    #[prost(enumeration = "ProtoLvLeCard", tag = "1")]
    pub played_card: i32,
}
// --- SelectTargetPlayer

/// Req & Repl
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoLvLeSelectTargetPlayer {
    #[prost(string, tag = "1")]
    pub target_player_id: std::string::String,
}
// --- SelectTargetCard

/// Req & Repl
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoLvLeSelectTargetCard {
    #[prost(enumeration = "ProtoLvLeCard", tag = "1")]
    pub target_card: i32,
}
// --- CommitSelection

/// Req: Signal completion of selection phase of a turn
///
/// Empty
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoLvLeCommitSelectionReq {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoLvLeCommitSelectionRepl {
    #[prost(message, optional, tag = "1")]
    pub outcome: ::std::option::Option<ProtoLvLeCardOutcome>,
}
/// Input selection for card:
/// 1 - Guard    : `(String, Card)` - the player+card that is guessed
/// 2 - Priest   : `(String)` - player to view card
/// 3 - Baron    : `(String)` - player to compare with
/// 4 - Handmaid : `()`
/// 5 - Prince   : `(String)` - player to discard/replace their card
/// 6 - King     : `(String)` - player to swap with
/// 7 - Countess : `()`
/// 8 - Princess : `()`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoLvLeCardSelection {
    #[prost(oneof = "proto_lv_le_card_selection::Inner", tags = "1, 2, 3, 5, 6")]
    pub inner: ::std::option::Option<proto_lv_le_card_selection::Inner>,
}
pub mod proto_lv_le_card_selection {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoGuardSelection {
        #[prost(string, tag = "1")]
        pub opt_player_id: std::string::String,
        #[prost(enumeration = "super::ProtoLvLeCard", tag = "2")]
        pub opt_card: i32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoPriestSelection {
        #[prost(string, tag = "1")]
        pub opt_player_id: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoBaronSelection {
        #[prost(string, tag = "1")]
        pub opt_player_id: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoPrinceSelection {
        #[prost(string, tag = "1")]
        pub opt_player_id: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoKingSelection {
        #[prost(string, tag = "1")]
        pub opt_player_id: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Inner {
        #[prost(message, tag = "1")]
        Guard(ProtoGuardSelection),
        #[prost(message, tag = "2")]
        Priest(ProtoPriestSelection),
        #[prost(message, tag = "3")]
        Baron(ProtoBaronSelection),
        #[prost(message, tag = "5")]
        Prince(ProtoPrinceSelection),
        #[prost(message, tag = "6")]
        King(ProtoKingSelection),
    }
}
/// Publicly broadcasted data after playing a card:
/// 1 - Guard    : `(bool)` - was guess correct
/// 2 - Priest   : `(String)` - opponent's card (player-specific)
/// 3 - Baron    : `(String, Card)` - the player+card that was knocked out
/// 4 - Handmaid : `()`
/// 5 - Prince   : `(Card)` - the discarded card
/// 6 - King     : `(Card)` - new card received by each player
/// 7 - Countess : `()`
/// 8 - Princess : `()`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoLvLeCardOutcome {
    #[prost(oneof = "proto_lv_le_card_outcome::Inner", tags = "1, 2, 3, 5")]
    pub inner: ::std::option::Option<proto_lv_le_card_outcome::Inner>,
}
pub mod proto_lv_le_card_outcome {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoGuardOutcome {
        #[prost(bool, tag = "1")]
        pub correct: bool,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoPriestOutcome {
        /// None => you are not allowed to see it
        /// Some => you are allowed to see it
        #[prost(enumeration = "super::ProtoLvLeCard", tag = "1")]
        pub opt_opponent_card: i32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoBaronOutcome {
        #[prost(message, optional, tag = "1")]
        pub opt_loser_info: ::std::option::Option<proto_baron_outcome::ProtoBaronLoserInfo>,
    }
    pub mod proto_baron_outcome {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ProtoBaronLoserInfo {
            #[prost(string, tag = "1")]
            pub losing_player_id: std::string::String,
            #[prost(enumeration = "super::super::ProtoLvLeCard", tag = "2")]
            pub losing_player_card: i32,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoPrinceOutcome {
        #[prost(enumeration = "super::ProtoLvLeCard", tag = "1")]
        pub discarded_card: i32,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Inner {
        #[prost(message, tag = "1")]
        Guard(ProtoGuardOutcome),
        #[prost(message, tag = "2")]
        Priest(ProtoPriestOutcome),
        #[prost(message, tag = "3")]
        Baron(ProtoBaronOutcome),
        #[prost(message, tag = "5")]
        Prince(ProtoPrinceOutcome),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoLvLeCommittedPlay {
    #[prost(message, optional, tag = "1")]
    pub selection: ::std::option::Option<ProtoLvLeCardSelection>,
    #[prost(message, optional, tag = "2")]
    pub outcome: ::std::option::Option<ProtoLvLeCardOutcome>,
}
// =======================================
// Common sub types
// =======================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoLvLeCard {
    UnspecifiedLoveLetterCard = 0,
    /// Guesses another player's card, if correct, other player is out. Can't guess Guard(1).
    Guard = 1,
    /// See another player's card.
    Priest = 2,
    /// Privately compare card with another player. Lower card is out.
    Baron = 3,
    /// Self cannot be targeted until the next turn.
    Handmaid = 4,
    /// Choose any player (including self) to discard their card and draw a new one.
    Prince = 5,
    /// Trade hands with any other player.
    King = 6,
    /// Must be discarded if other card is King(6) or Prince(5).
    Countess = 7,
    /// If this card is ever discarded, that player is out.
    Princess = 8,
}
// ======================================================
// API Request and Reply messages for Pre-game RPCs
// ======================================================

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoHostGameReq {
    #[prost(string, tag = "1")]
    pub player_id: std::string::String,
    #[prost(string, tag = "2")]
    pub game_id: std::string::String,
    #[prost(enumeration = "ProtoGameType", tag = "3")]
    pub game_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoJoinGameReq {
    #[prost(string, tag = "1")]
    pub player_id: std::string::String,
    #[prost(string, tag = "2")]
    pub game_id: std::string::String,
    #[prost(enumeration = "ProtoGameType", tag = "3")]
    pub game_type: i32,
}
/// Stream message type
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoPreGameMessage {
    #[prost(oneof = "proto_pre_game_message::Inner", tags = "1, 2, 3")]
    pub inner: ::std::option::Option<proto_pre_game_message::Inner>,
}
pub mod proto_pre_game_message {
    /// Initial response in PreGame stream
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoJoinGameAck {
        #[prost(enumeration = "super::ProtoGameType", tag = "1")]
        pub game_type: i32,
        #[prost(string, tag = "2")]
        pub host_player_id: std::string::String,
        #[prost(string, repeated, tag = "3")]
        pub other_player_ids: ::std::vec::Vec<std::string::String>,
    }
    /// N intermediate messages received in PreGame stream
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoPlayerJoinMsg {
        #[prost(string, tag = "1")]
        pub player_id: std::string::String,
    }
    /// Terminal message received in PreGame stream
    ///
    /// Empty
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoGameStartMsg {}
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Inner {
        #[prost(message, tag = "1")]
        JoinGameAck(ProtoJoinGameAck),
        #[prost(message, tag = "2")]
        PlayerJoinMsg(ProtoPlayerJoinMsg),
        #[prost(message, tag = "3")]
        GameStartMsg(ProtoGameStartMsg),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoStartGameReq {
    #[prost(string, tag = "1")]
    pub player_id: std::string::String,
    #[prost(string, tag = "2")]
    pub game_id: std::string::String,
    #[prost(enumeration = "ProtoGameType", tag = "3")]
    pub game_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoStartGameReply {
    #[prost(string, repeated, tag = "1")]
    pub player_ids: ::std::vec::Vec<std::string::String>,
}
#[doc = r" Generated client implementations."]
pub mod proto_fridge_game_engine_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct ProtoFridgeGameEngineClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProtoFridgeGameEngineClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ProtoFridgeGameEngineClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn host_game(
            &mut self,
            request: impl tonic::IntoRequest<super::ProtoHostGameReq>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ProtoPreGameMessage>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto_frj_ngn.ProtoFridgeGameEngine/HostGame",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn join_game(
            &mut self,
            request: impl tonic::IntoRequest<super::ProtoJoinGameReq>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ProtoPreGameMessage>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto_frj_ngn.ProtoFridgeGameEngine/JoinGame",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn start_game(
            &mut self,
            request: impl tonic::IntoRequest<super::ProtoStartGameReq>,
        ) -> Result<tonic::Response<super::ProtoStartGameReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto_frj_ngn.ProtoFridgeGameEngine/StartGame",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn open_love_letter_data_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::ProtoLoveLetterDataIn>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ProtoLoveLetterDataOut>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto_frj_ngn.ProtoFridgeGameEngine/OpenLoveLetterDataStream",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for ProtoFridgeGameEngineClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ProtoFridgeGameEngineClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ProtoFridgeGameEngineClient {{ ... }}")
        }
    }
}
