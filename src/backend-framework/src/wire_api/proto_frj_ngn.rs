/// Every Data Stream should begin by client sending a handshake message to server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoGameDataHandshake {
    #[prost(string, tag = "1")]
    pub player_id: std::string::String,
    /// Game type not needed, since it can be inferred from the stream
    /// type that the message is contained in.
    #[prost(string, tag = "2")]
    pub game_id: std::string::String,
}
/// Empty: This means "send me the latest state for the game stream I have opened".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoGameDataStateReq {}
/// Empty: Player clicks "ready" button on various screen.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoGameDataReadyUpClick {}
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
        tags = "2, 3, 4, 5, 6, 7, 8"
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
        #[prost(message, tag = "8")]
        ReadyUp(super::ProtoGameDataReadyUpClick),
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
        /// TODO:2.5 remove other possible game states
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
    /// TODO:2.5 remove redundant field
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
        #[prost(message, optional, tag = "6")]
        pub staged_play: ::std::option::Option<super::ProtoLvLeCardSelection>,
        #[prost(message, optional, tag = "7")]
        pub most_recent_committed_play: ::std::option::Option<super::ProtoLvLeCommittedPlay>,
        #[prost(enumeration = "super::ProtoLvLeCard", repeated, tag = "8")]
        pub play_history: ::std::vec::Vec<i32>,
        /// TODO:3 `turn` should really include a distinction between pending-play and pending-commit. Until then, the
        /// API model must closely resemble the backend state (yuck!).
        #[prost(string, repeated, tag = "9")]
        pub handmaid_player_ids: ::std::vec::Vec<std::string::String>,
        #[prost(oneof = "proto_lv_le_round_state::Turn", tags = "3, 4, 5")]
        pub turn: ::std::option::Option<proto_lv_le_round_state::Turn>,
    }
    pub mod proto_lv_le_round_state {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Turn {
            #[prost(enumeration = "super::super::ProtoLvLeCard", tag = "3")]
            MyDrawnCard(i32),
            #[prost(string, tag = "4")]
            CurrentTurnPlayerId(std::string::String),
            #[prost(message, tag = "5")]
            TurnIntermission(super::ProtoLvLeTurnIntermissionState),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoLvLeTurnIntermissionState {
        #[prost(string, repeated, tag = "1")]
        pub unready_player_ids: ::std::vec::Vec<std::string::String>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoLvLeResultState {
        /// Sparse map, missing value => player eliminated
        #[prost(map = "string, enumeration(super::ProtoLvLeCard)", tag = "1")]
        pub final_cards: ::std::collections::HashMap<std::string::String, i32>,
        #[prost(string, repeated, tag = "2")]
        pub unready_player_ids: ::std::vec::Vec<std::string::String>,
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
#[doc = r" Generated server implementations."]
pub mod proto_fridge_game_engine_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ProtoFridgeGameEngineServer."]
    #[async_trait]
    pub trait ProtoFridgeGameEngine: Send + Sync + 'static {
        #[doc = "Server streaming response type for the HostGame method."]
        type HostGameStream: Stream<Item = Result<super::ProtoPreGameMessage, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn host_game(
            &self,
            request: tonic::Request<super::ProtoHostGameReq>,
        ) -> Result<tonic::Response<Self::HostGameStream>, tonic::Status>;
        #[doc = "Server streaming response type for the JoinGame method."]
        type JoinGameStream: Stream<Item = Result<super::ProtoPreGameMessage, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn join_game(
            &self,
            request: tonic::Request<super::ProtoJoinGameReq>,
        ) -> Result<tonic::Response<Self::JoinGameStream>, tonic::Status>;
        async fn start_game(
            &self,
            request: tonic::Request<super::ProtoStartGameReq>,
        ) -> Result<tonic::Response<super::ProtoStartGameReply>, tonic::Status>;
        #[doc = "Server streaming response type for the OpenLoveLetterDataStream method."]
        type OpenLoveLetterDataStreamStream: Stream<Item = Result<super::ProtoLoveLetterDataOut, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn open_love_letter_data_stream(
            &self,
            request: tonic::Request<tonic::Streaming<super::ProtoLoveLetterDataIn>>,
        ) -> Result<tonic::Response<Self::OpenLoveLetterDataStreamStream>, tonic::Status>;
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ProtoFridgeGameEngineServer<T: ProtoFridgeGameEngine> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ProtoFridgeGameEngine> ProtoFridgeGameEngineServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for ProtoFridgeGameEngineServer<T>
    where
        T: ProtoFridgeGameEngine,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/proto_frj_ngn.ProtoFridgeGameEngine/HostGame" => {
                    #[allow(non_camel_case_types)]
                    struct HostGameSvc<T: ProtoFridgeGameEngine>(pub Arc<T>);
                    impl<T: ProtoFridgeGameEngine>
                        tonic::server::ServerStreamingService<super::ProtoHostGameReq>
                        for HostGameSvc<T>
                    {
                        type Response = super::ProtoPreGameMessage;
                        type ResponseStream = T::HostGameStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProtoHostGameReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.host_game(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = HostGameSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto_frj_ngn.ProtoFridgeGameEngine/JoinGame" => {
                    #[allow(non_camel_case_types)]
                    struct JoinGameSvc<T: ProtoFridgeGameEngine>(pub Arc<T>);
                    impl<T: ProtoFridgeGameEngine>
                        tonic::server::ServerStreamingService<super::ProtoJoinGameReq>
                        for JoinGameSvc<T>
                    {
                        type Response = super::ProtoPreGameMessage;
                        type ResponseStream = T::JoinGameStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProtoJoinGameReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.join_game(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = JoinGameSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto_frj_ngn.ProtoFridgeGameEngine/StartGame" => {
                    #[allow(non_camel_case_types)]
                    struct StartGameSvc<T: ProtoFridgeGameEngine>(pub Arc<T>);
                    impl<T: ProtoFridgeGameEngine>
                        tonic::server::UnaryService<super::ProtoStartGameReq> for StartGameSvc<T>
                    {
                        type Response = super::ProtoStartGameReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProtoStartGameReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.start_game(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = StartGameSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto_frj_ngn.ProtoFridgeGameEngine/OpenLoveLetterDataStream" => {
                    #[allow(non_camel_case_types)]
                    struct OpenLoveLetterDataStreamSvc<T: ProtoFridgeGameEngine>(pub Arc<T>);
                    impl<T: ProtoFridgeGameEngine>
                        tonic::server::StreamingService<super::ProtoLoveLetterDataIn>
                        for OpenLoveLetterDataStreamSvc<T>
                    {
                        type Response = super::ProtoLoveLetterDataOut;
                        type ResponseStream = T::OpenLoveLetterDataStreamStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::ProtoLoveLetterDataIn>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.open_love_letter_data_stream(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = OpenLoveLetterDataStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ProtoFridgeGameEngine> Clone for ProtoFridgeGameEngineServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ProtoFridgeGameEngine> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ProtoFridgeGameEngine> tonic::transport::NamedService for ProtoFridgeGameEngineServer<T> {
        const NAME: &'static str = "proto_frj_ngn.ProtoFridgeGameEngine";
    }
}
