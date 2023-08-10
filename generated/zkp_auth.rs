#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterRequest {
    #[prost(string, tag = "1")]
    pub user: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub y1: i64,
    #[prost(int64, tag = "3")]
    pub y2: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationChallengeRequest {
    #[prost(string, tag = "1")]
    pub user: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub r1: i64,
    #[prost(int64, tag = "3")]
    pub r2: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationChallengeResponse {
    #[prost(string, tag = "1")]
    pub auth_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub c: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationAnswerRequest {
    #[prost(string, tag = "1")]
    pub auth_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub s: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationAnswerResponse {
    #[prost(string, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod auth_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct AuthClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AuthClient<tonic::transport::Channel> {
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
    impl<T> AuthClient<T>
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
        pub async fn register(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterRequest>,
        ) -> Result<tonic::Response<super::RegisterResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/zkp_auth.Auth/Register");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_authentication_challenge(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthenticationChallengeRequest>,
        ) -> Result<tonic::Response<super::AuthenticationChallengeResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zkp_auth.Auth/CreateAuthenticationChallenge",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn verify_authentication(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthenticationAnswerRequest>,
        ) -> Result<tonic::Response<super::AuthenticationAnswerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/zkp_auth.Auth/VerifyAuthentication");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AuthClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AuthClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AuthClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod auth_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with AuthServer."]
    #[async_trait]
    pub trait Auth: Send + Sync + 'static {
        async fn register(
            &self,
            request: tonic::Request<super::RegisterRequest>,
        ) -> Result<tonic::Response<super::RegisterResponse>, tonic::Status>;
        async fn create_authentication_challenge(
            &self,
            request: tonic::Request<super::AuthenticationChallengeRequest>,
        ) -> Result<tonic::Response<super::AuthenticationChallengeResponse>, tonic::Status>;
        async fn verify_authentication(
            &self,
            request: tonic::Request<super::AuthenticationAnswerRequest>,
        ) -> Result<tonic::Response<super::AuthenticationAnswerResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct AuthServer<T: Auth> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Auth> AuthServer<T> {
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
    impl<T, B> Service<http::Request<B>> for AuthServer<T>
    where
        T: Auth,
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
                "/zkp_auth.Auth/Register" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterSvc<T: Auth>(pub Arc<T>);
                    impl<T: Auth> tonic::server::UnaryService<super::RegisterRequest> for RegisterSvc<T> {
                        type Response = super::RegisterResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegisterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).register(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RegisterSvc(inner);
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
                "/zkp_auth.Auth/CreateAuthenticationChallenge" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAuthenticationChallengeSvc<T: Auth>(pub Arc<T>);
                    impl<T: Auth> tonic::server::UnaryService<super::AuthenticationChallengeRequest>
                        for CreateAuthenticationChallengeSvc<T>
                    {
                        type Response = super::AuthenticationChallengeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuthenticationChallengeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_authentication_challenge(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateAuthenticationChallengeSvc(inner);
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
                "/zkp_auth.Auth/VerifyAuthentication" => {
                    #[allow(non_camel_case_types)]
                    struct VerifyAuthenticationSvc<T: Auth>(pub Arc<T>);
                    impl<T: Auth> tonic::server::UnaryService<super::AuthenticationAnswerRequest>
                        for VerifyAuthenticationSvc<T>
                    {
                        type Response = super::AuthenticationAnswerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuthenticationAnswerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).verify_authentication(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = VerifyAuthenticationSvc(inner);
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
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Auth> Clone for AuthServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Auth> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Auth> tonic::transport::NamedService for AuthServer<T> {
        const NAME: &'static str = "zkp_auth.Auth";
    }
}
