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
    /// Logical clock for this game instance
    #[prost(uint64, tag = "1")]
    pub clock: u64,
    /// The actual message
    #[prost(message, optional, tag = "2")]
    pub data: ::std::option::Option<proto_love_letter_data_in::ProtoDataIn>,
}
pub mod proto_love_letter_data_in {
    /// It works?
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoDataIn {
        #[prost(oneof = "proto_data_in::PayloadIn", tags = "1, 2, 3, 4, 5, 6")]
        pub payload_in: ::std::option::Option<proto_data_in::PayloadIn>,
    }
    pub mod proto_data_in {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum PayloadIn {
            #[prost(message, tag = "1")]
            Handshake(super::super::ProtoGameDataHandshake),
            #[prost(message, tag = "2")]
            GameState(super::super::ProtoGameDataStateReq),
            #[prost(message, tag = "3")]
            PlayCard(super::super::ProtoLvLePlayCardReq),
            #[prost(message, tag = "4")]
            SelectTargetPlayer(super::super::ProtoLvLeSelectTargetPlayer),
            #[prost(message, tag = "5")]
            SelectTargetCard(super::super::ProtoLvLeSelectTargetCard),
            #[prost(message, tag = "6")]
            CommitSelection(super::super::ProtoLvLeCommitSelectionReq),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoLoveLetterDataOut {
    /// Logical clock for this game instance
    #[prost(uint64, tag = "1")]
    pub clock: u64,
    /// The actual message
    #[prost(message, optional, tag = "2")]
    pub data: ::std::option::Option<proto_love_letter_data_out::ProtoDataOut>,
}
pub mod proto_love_letter_data_out {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoDataOut {
        #[prost(oneof = "proto_data_out::PayloadOut", tags = "1, 2, 3, 4, 5, 6, 7")]
        pub payload_out: ::std::option::Option<proto_data_out::PayloadOut>,
    }
    pub mod proto_data_out {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum PayloadOut {
            #[prost(message, tag = "1")]
            GameState(super::super::ProtoLvLeGameState),
            #[prost(message, tag = "2")]
            TurnIndicator(super::super::ProtoLvLeTurnIndicatorRepl),
            #[prost(message, tag = "3")]
            PlayCard(super::super::ProtoLvLePlayCardRepl),
            #[prost(message, tag = "4")]
            StageCard(super::super::ProtoLvLeStageCardRepl),
            #[prost(message, tag = "5")]
            SelectTargetPlayer(super::super::ProtoLvLeSelectTargetPlayer),
            #[prost(message, tag = "6")]
            SelectTargetCard(super::super::ProtoLvLeSelectTargetCard),
            #[prost(message, tag = "7")]
            CommitSelection(super::super::ProtoLvLeCommitSelectionRepl),
        }
    }
}
// =======================================
// API Request and Reply messages
// =======================================

// --- GameState

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoLvLeGameState {
    /// Logical clock for this game instance
    #[prost(uint64, tag = "1")]
    pub clock: u64,
    #[prost(message, repeated, tag = "2")]
    pub players: ::std::vec::Vec<proto_lv_le_game_state::ProtoLvLePlayerState>,
    #[prost(enumeration = "ProtoLvLeCard", tag = "3")]
    pub my_card: i32,
    #[prost(string, tag = "4")]
    pub current_turn_player_id: std::string::String,
    #[prost(enumeration = "ProtoLvLeCard", repeated, tag = "5")]
    pub play_history: ::std::vec::Vec<i32>,
}
pub mod proto_lv_le_game_state {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoLvLePlayerState {
        #[prost(string, tag = "1")]
        pub player_id: std::string::String,
        #[prost(bool, tag = "2")]
        pub in_play: bool,
        #[prost(uint32, tag = "3")]
        pub round_wins: u32,
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
    #[prost(enumeration = "proto_lv_le_play_card_req::CardSource", tag = "1")]
    pub card_source: i32,
}
pub mod proto_lv_le_play_card_req {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CardSource {
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
/// Output data of effect:
/// 1 - Guard    : `(bool)` - was guess correct
/// 2 - Priest   : `()`
/// 3 - Baron    : `(String, Card)` - the player+card that was knocked out
/// 4 - Handmaid : `()`
/// 5 - Prince   : `(Card)` - the discarded card
/// 6 - King     : `(Card)` - new card received by each player
/// 7 - Countess : `()`
/// 8 - Princess : `()`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoLvLeCardOutcome {
    /// Set to null for 2,4,7,8
    #[prost(oneof = "proto_lv_le_card_outcome::Inner", tags = "1, 3, 5, 6")]
    pub inner: ::std::option::Option<proto_lv_le_card_outcome::Inner>,
}
pub mod proto_lv_le_card_outcome {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoGuardOutcome {
        #[prost(bool, tag = "1")]
        pub correct: bool,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoBaronOutcome {
        #[prost(string, tag = "1")]
        pub losing_player_id: std::string::String,
        #[prost(enumeration = "super::ProtoLvLeCard", tag = "2")]
        pub losing_player_card: i32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoPrinceOutcome {
        #[prost(enumeration = "super::ProtoLvLeCard", tag = "1")]
        pub discarded_card: i32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoKingOutcome {
        #[prost(enumeration = "super::ProtoLvLeCard", tag = "1")]
        pub new_card: i32,
    }
    /// Set to null for 2,4,7,8
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Inner {
        #[prost(message, tag = "1")]
        Guard(ProtoGuardOutcome),
        #[prost(message, tag = "3")]
        Baron(ProtoBaronOutcome),
        #[prost(message, tag = "5")]
        Prince(ProtoPrinceOutcome),
        #[prost(message, tag = "6")]
        King(ProtoKingOutcome),
    }
}
// =======================================
// Common sub types
// =======================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoLvLeCard {
    UnspecifiedLoveLetterCard = 0,
    Guard = 1,
    Priest = 2,
    Baron = 3,
    Handmaid = 4,
    Prince = 5,
    King = 6,
    Countess = 7,
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
