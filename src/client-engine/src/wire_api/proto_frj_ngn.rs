#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoGameDataHeader {
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoLoveLetterDataIn {
    #[prost(oneof = "proto_love_letter_data_in::Inner", tags = "1, 2, 3")]
    pub inner: ::std::option::Option<proto_love_letter_data_in::Inner>,
}
pub mod proto_love_letter_data_in {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Inner {
        #[prost(message, tag = "1")]
        Header(super::ProtoGameDataHeader),
        #[prost(message, tag = "2")]
        GameStateReq(super::ProtoGameDataStateReq),
        #[prost(message, tag = "3")]
        ExMsg(super::ProtoLoLeExample),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoLoveLetterDataOut {
    #[prost(oneof = "proto_love_letter_data_out::Inner", tags = "1")]
    pub inner: ::std::option::Option<proto_love_letter_data_out::Inner>,
}
pub mod proto_love_letter_data_out {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Inner {
        #[prost(message, tag = "1")]
        GameState(super::ProtoLoLeGameState),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoLoLeExample {
    #[prost(string, tag = "1")]
    pub ex_field: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoLoLeGameState {
    #[prost(string, tag = "1")]
    pub ex_field: std::string::String,
}
// =======================================
// API Request and Reply messages
// =======================================

// --- StageCard

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
// --- SelectTargetPlayer

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
// --- SelectTargetCard

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
// --- PlayCardCommit

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoPlayCardCommitReq {
    #[prost(string, tag = "1")]
    pub player_id: std::string::String,
}
/// Simple ACK
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoPlayCardCommitReply {}
// =======================================
// Sub types
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
