/// information for synerex servers and providers, gateways (nodes)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeInfo {
    #[prost(string, tag = "1")]
    pub node_name: ::prost::alloc::string::String,
    /// node is provider/server/gateway?
    #[prost(enumeration = "NodeType", tag = "2")]
    pub node_type: i32,
    /// for synerex servers/gateways
    #[prost(string, tag = "3")]
    pub server_info: ::prost::alloc::string::String,
    /// which protocol base version
    #[prost(string, tag = "4")]
    pub node_pbase_version: ::prost::alloc::string::String,
    /// for reconnection with previous node_id (usually -1)
    #[prost(int32, tag = "5")]
    pub with_node_id: i32,
    ///
    #[prost(int32, tag = "6")]
    pub cluster_id: i32,
    /// for area definition
    #[prost(string, tag = "7")]
    pub area_id: ::prost::alloc::string::String,
    /// used channel list
    #[prost(uint32, repeated, tag = "8")]
    pub channel_types: ::prost::alloc::vec::Vec<u32>,
    /// for gateway information
    #[prost(string, tag = "9")]
    pub gw_info: ::prost::alloc::string::String,
    /// for information for controller
    ///
    /// version of binary
    #[prost(string, tag = "10")]
    pub bin_version: ::prost::alloc::string::String,
    /// keepalive update count
    #[prost(int32, tag = "11")]
    pub count: i32,
    #[prost(message, optional, tag = "12")]
    pub last_alive_time: ::core::option::Option<::prost_types::Timestamp>,
    /// keepalive argument
    #[prost(string, tag = "13")]
    pub keepalive_arg: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeId {
    /// unique id for each node in current node server.
    #[prost(int32, tag = "1")]
    pub node_id: i32,
    /// secret id with node_server (Not used for Query)
    #[prost(fixed64, tag = "2")]
    pub secret: u64,
    /// synerex server address (only for registration of Server/Gateway)
    #[prost(string, tag = "3")]
    pub server_info: ::prost::alloc::string::String,
    /// at least make keep alive less than this time.
    #[prost(int32, tag = "4")]
    pub keepalive_duration: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerStatus {
    /// cpu load average
    #[prost(double, tag = "1")]
    pub cpu: f64,
    /// memory usage rate
    #[prost(double, tag = "2")]
    pub memory: f64,
    /// message count
    #[prost(uint64, tag = "3")]
    pub msg_count: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeUpdate {
    /// unique id for each node in current node server.
    #[prost(int32, tag = "1")]
    pub node_id: i32,
    /// secret number for each provider (for auth)
    #[prost(fixed64, tag = "2")]
    pub secret: u64,
    /// sequential counter for nodes
    #[prost(int32, tag = "3")]
    pub update_count: i32,
    /// running state (health check)
    #[prost(int32, tag = "4")]
    pub node_status: i32,
    #[prost(string, tag = "5")]
    pub node_arg: ::prost::alloc::string::String,
    /// server status (load average)
    #[prost(message, optional, tag = "6")]
    pub status: ::core::option::Option<ServerStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(bool, tag = "1")]
    pub ok: bool,
    #[prost(enumeration = "KeepAliveCommand", tag = "2")]
    pub command: i32,
    #[prost(string, tag = "3")]
    pub err: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NodeType {
    /// most of node is normal provider
    Provider = 0,
    /// node is synerex server.
    Server = 1,
    /// node is synerex gateway. (for inter synerex/outer synerex)
    Gateway = 2,
}
impl NodeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NodeType::Provider => "PROVIDER",
            NodeType::Server => "SERVER",
            NodeType::Gateway => "GATEWAY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROVIDER" => Some(Self::Provider),
            "SERVER" => Some(Self::Server),
            "GATEWAY" => Some(Self::Gateway),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KeepAliveCommand {
    None = 0,
    /// node server is now restarted. please reconnect.
    Reconnect = 1,
    /// There is change in Synerex Server. Obtain server information.
    ServerChange = 2,
    /// only for Synerex-Server. Provider is disconnected. Remove Channels
    ProviderDisconnect = 3,
}
impl KeepAliveCommand {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            KeepAliveCommand::None => "NONE",
            KeepAliveCommand::Reconnect => "RECONNECT",
            KeepAliveCommand::ServerChange => "SERVER_CHANGE",
            KeepAliveCommand::ProviderDisconnect => "PROVIDER_DISCONNECT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NONE" => Some(Self::None),
            "RECONNECT" => Some(Self::Reconnect),
            "SERVER_CHANGE" => Some(Self::ServerChange),
            "PROVIDER_DISCONNECT" => Some(Self::ProviderDisconnect),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod node_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct NodeClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl NodeClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> NodeClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> NodeClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            NodeClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn register_node(
            &mut self,
            request: impl tonic::IntoRequest<super::NodeInfo>,
        ) -> std::result::Result<tonic::Response<super::NodeId>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/nodeapi.Node/RegisterNode",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("nodeapi.Node", "RegisterNode"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_node(
            &mut self,
            request: impl tonic::IntoRequest<super::NodeId>,
        ) -> std::result::Result<tonic::Response<super::NodeInfo>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nodeapi.Node/QueryNode");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("nodeapi.Node", "QueryNode"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn keep_alive(
            &mut self,
            request: impl tonic::IntoRequest<super::NodeUpdate>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/nodeapi.Node/KeepAlive");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("nodeapi.Node", "KeepAlive"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn un_register_node(
            &mut self,
            request: impl tonic::IntoRequest<super::NodeId>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/nodeapi.Node/UnRegisterNode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("nodeapi.Node", "UnRegisterNode"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod node_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with NodeServer.
    #[async_trait]
    pub trait Node: Send + Sync + 'static {
        async fn register_node(
            &self,
            request: tonic::Request<super::NodeInfo>,
        ) -> std::result::Result<tonic::Response<super::NodeId>, tonic::Status>;
        async fn query_node(
            &self,
            request: tonic::Request<super::NodeId>,
        ) -> std::result::Result<tonic::Response<super::NodeInfo>, tonic::Status>;
        async fn keep_alive(
            &self,
            request: tonic::Request<super::NodeUpdate>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status>;
        async fn un_register_node(
            &self,
            request: tonic::Request<super::NodeId>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct NodeServer<T: Node> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Node> NodeServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for NodeServer<T>
    where
        T: Node,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/nodeapi.Node/RegisterNode" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterNodeSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::NodeInfo>
                    for RegisterNodeSvc<T> {
                        type Response = super::NodeId;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NodeInfo>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).register_node(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterNodeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/nodeapi.Node/QueryNode" => {
                    #[allow(non_camel_case_types)]
                    struct QueryNodeSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::NodeId>
                    for QueryNodeSvc<T> {
                        type Response = super::NodeInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NodeId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).query_node(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryNodeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/nodeapi.Node/KeepAlive" => {
                    #[allow(non_camel_case_types)]
                    struct KeepAliveSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::NodeUpdate>
                    for KeepAliveSvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NodeUpdate>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).keep_alive(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = KeepAliveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/nodeapi.Node/UnRegisterNode" => {
                    #[allow(non_camel_case_types)]
                    struct UnRegisterNodeSvc<T: Node>(pub Arc<T>);
                    impl<T: Node> tonic::server::UnaryService<super::NodeId>
                    for UnRegisterNodeSvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NodeId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).un_register_node(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnRegisterNodeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Node> Clone for NodeServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Node> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Node> tonic::server::NamedService for NodeServer<T> {
        const NAME: &'static str = "nodeapi.Node";
    }
}
