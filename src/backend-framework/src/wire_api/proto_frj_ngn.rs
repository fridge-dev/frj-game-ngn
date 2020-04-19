// =======================================
// API Request and Reply messages
// =======================================

// Convention: ALL messages should have prefix "Proto" so in the rust src, it's easy
// to understand which types are generated.

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
pub struct ProtoGetGameStateReq {
    #[prost(string, tag = "1")]
    pub player_id: std::string::String,
    #[prost(enumeration = "ProtoGameType", tag = "2")]
    pub game_type: i32,
    #[prost(string, tag = "3")]
    pub game_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoGetGameStateReply {
    #[prost(message, optional, tag = "1")]
    pub game_state: ::std::option::Option<ProtoJnGameState>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoGameDataIn {
    #[prost(oneof = "proto_game_data_in::Inner", tags = "1, 2")]
    pub inner: ::std::option::Option<proto_game_data_in::Inner>,
}
pub mod proto_game_data_in {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoGameDataHeader {
        #[prost(string, tag = "1")]
        pub player_id: std::string::String,
        #[prost(string, tag = "2")]
        pub game_id: std::string::String,
        #[prost(enumeration = "super::ProtoGameType", tag = "3")]
        pub game_type: i32,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Inner {
        #[prost(message, tag = "1")]
        Header(ProtoGameDataHeader),
        #[prost(message, tag = "2")]
        Data(super::ProtoJnGameDataIn),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoGameDataOut {
    #[prost(oneof = "proto_game_data_out::Inner", tags = "1, 2")]
    pub inner: ::std::option::Option<proto_game_data_out::Inner>,
}
pub mod proto_game_data_out {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Inner {
        #[prost(message, tag = "1")]
        GameState(super::ProtoJnGameState),
        #[prost(message, tag = "2")]
        Data(super::ProtoJnGameDataOut),
    }
}
// =======================================
// Jn types
// =======================================

/// TODO `Any` game-specific payload
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoJnGameState {}
/// TODO `Any` game-specific payload
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoJnGameDataIn {}
/// TODO `Any` game-specific payload
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoJnGameDataOut {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoStageCardReq {
    #[prost(string, tag = "1")]
    pub player_id: std::string::String,
    #[prost(enumeration = "ProtoPlayCardSource", tag = "2")]
    pub card_source: i32,
}
/// Simple ACK
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoStageCardReply {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoSelectTargetPlayerReq {
    #[prost(string, tag = "1")]
    pub player_id: std::string::String,
    #[prost(string, tag = "2")]
    pub target_player_id: std::string::String,
}
/// Simple ACK
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoSelectTargetPlayerReply {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoSelectTargetCardReq {
    #[prost(string, tag = "1")]
    pub player_id: std::string::String,
    #[prost(enumeration = "ProtoLoveLetterCard", tag = "2")]
    pub target_card: i32,
}
/// Simple ACK
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoSelectTargetCardReply {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoPlayCardCommitReq {
    #[prost(string, tag = "1")]
    pub player_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoPlayCardCommitReply {}
// =======================================
// Sub types
// =======================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoGameType {
    UnspecifiedGameType = 0,
    LoveLetter = 1,
    LostCities = 2,
}
// =======================================
// LoveLetter types
// =======================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoPlayCardSource {
    UnspecifiedPlayCardSource = 0,
    Hand = 1,
    TopDeck = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoLoveLetterCard {
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
        async fn get_game_state(
            &self,
            request: tonic::Request<super::ProtoGetGameStateReq>,
        ) -> Result<tonic::Response<super::ProtoGetGameStateReply>, tonic::Status>;
        #[doc = "Server streaming response type for the OpenGameDataStream method."]
        type OpenGameDataStreamStream: Stream<Item = Result<super::ProtoGameDataOut, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn open_game_data_stream(
            &self,
            request: tonic::Request<tonic::Streaming<super::ProtoGameDataIn>>,
        ) -> Result<tonic::Response<Self::OpenGameDataStreamStream>, tonic::Status>;
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
                "/proto_frj_ngn.ProtoFridgeGameEngine/GetGameState" => {
                    #[allow(non_camel_case_types)]
                    struct GetGameStateSvc<T: ProtoFridgeGameEngine>(pub Arc<T>);
                    impl<T: ProtoFridgeGameEngine>
                        tonic::server::UnaryService<super::ProtoGetGameStateReq>
                        for GetGameStateSvc<T>
                    {
                        type Response = super::ProtoGetGameStateReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProtoGetGameStateReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_game_state(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetGameStateSvc(inner);
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
                "/proto_frj_ngn.ProtoFridgeGameEngine/OpenGameDataStream" => {
                    #[allow(non_camel_case_types)]
                    struct OpenGameDataStreamSvc<T: ProtoFridgeGameEngine>(pub Arc<T>);
                    impl<T: ProtoFridgeGameEngine>
                        tonic::server::StreamingService<super::ProtoGameDataIn>
                        for OpenGameDataStreamSvc<T>
                    {
                        type Response = super::ProtoGameDataOut;
                        type ResponseStream = T::OpenGameDataStreamStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::ProtoGameDataIn>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.open_game_data_stream(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = OpenGameDataStreamSvc(inner);
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
