// =======================================
// API Request and Reply messages
// =======================================

// Convention: ALL messages should have prefix "Proto" so in the rust src, it's easy
// to understand which types are generated.

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoHostGameReq {
    #[prost(string, tag = "1")]
    pub game_id: std::string::String,
    #[prost(string, tag = "2")]
    pub player_id: std::string::String,
}
/// Nothing
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoHostGameReply {}
#[doc = r" Generated server implementations."]
pub mod proto_fridge_game_engine_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ProtoFridgeGameEngineServer."]
    #[async_trait]
    pub trait ProtoFridgeGameEngine: Send + Sync + 'static {
        async fn host_game(
            &self,
            request: tonic::Request<super::ProtoHostGameReq>,
        ) -> Result<tonic::Response<super::ProtoHostGameReply>, tonic::Status>;
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
    impl<T: ProtoFridgeGameEngine> Service<http::Request<HyperBody>>
        for ProtoFridgeGameEngineServer<T>
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<HyperBody>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/proto_frj_ngn.ProtoFridgeGameEngine/HostGame" => {
                    struct HostGameSvc<T: ProtoFridgeGameEngine>(pub Arc<T>);
                    impl<T: ProtoFridgeGameEngine>
                        tonic::server::UnaryService<super::ProtoHostGameReq> for HostGameSvc<T>
                    {
                        type Response = super::ProtoHostGameReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
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
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = HostGameSvc(inner);
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
