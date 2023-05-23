#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(bool, tag = "1")]
    pub ok: bool,
    #[prost(string, tag = "2")]
    pub err: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmResponse {
    #[prost(bool, tag = "1")]
    pub ok: bool,
    #[prost(fixed64, tag = "2")]
    pub mbus_id: u64,
    #[prost(message, optional, tag = "3")]
    pub wait: ::core::option::Option<::prost_types::Duration>,
    #[prost(string, tag = "4")]
    pub err: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Content {
    #[prost(bytes = "vec", tag = "1")]
    pub entity: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Supply {
    #[prost(fixed64, tag = "1")]
    pub id: u64,
    #[prost(fixed64, tag = "2")]
    pub sender_id: u64,
    /// for message id (not for sender_id)
    #[prost(fixed64, tag = "3")]
    pub target_id: u64,
    /// channel type
    #[prost(uint32, tag = "4")]
    pub channel_type: u32,
    #[prost(string, tag = "5")]
    pub supply_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub ts: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "7")]
    pub arg_json: ::prost::alloc::string::String,
    /// new mbus id for select demand.
    #[prost(fixed64, tag = "8")]
    pub mbus_id: u64,
    /// content data
    #[prost(message, optional, tag = "9")]
    pub cdata: ::core::option::Option<Content>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Demand {
    #[prost(fixed64, tag = "1")]
    pub id: u64,
    #[prost(fixed64, tag = "2")]
    pub sender_id: u64,
    /// if set with message id (not for sender_id) (select for supply)
    #[prost(fixed64, tag = "3")]
    pub target_id: u64,
    /// channel type
    #[prost(uint32, tag = "4")]
    pub channel_type: u32,
    #[prost(string, tag = "5")]
    pub demand_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub ts: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "7")]
    pub arg_json: ::prost::alloc::string::String,
    /// new mbus id for select supply...
    #[prost(fixed64, tag = "8")]
    pub mbus_id: u64,
    /// content data
    #[prost(message, optional, tag = "9")]
    pub cdata: ::core::option::Option<Content>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Target {
    #[prost(fixed64, tag = "1")]
    pub id: u64,
    #[prost(fixed64, tag = "2")]
    pub sender_id: u64,
    /// for target
    #[prost(fixed64, tag = "3")]
    pub target_id: u64,
    /// channel type
    #[prost(uint32, tag = "4")]
    pub channel_type: u32,
    #[prost(message, optional, tag = "5")]
    pub wait: ::core::option::Option<::prost_types::Duration>,
    /// if you need message bus, set Mbus with mbus_id = 1
    #[prost(fixed64, tag = "6")]
    pub mbus_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {
    #[prost(fixed64, tag = "1")]
    pub client_id: u64,
    /// channel type
    #[prost(uint32, tag = "2")]
    pub channel_type: u32,
    /// for Channel Argument
    #[prost(string, tag = "3")]
    pub arg_json: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mbus {
    #[prost(fixed64, tag = "1")]
    pub client_id: u64,
    #[prost(fixed64, tag = "2")]
    pub mbus_id: u64,
    /// for mbus description
    #[prost(string, tag = "3")]
    pub arg_json: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MbusMsg {
    /// if 0 for close message
    #[prost(fixed64, tag = "1")]
    pub msg_id: u64,
    #[prost(fixed64, tag = "2")]
    pub sender_id: u64,
    /// for target // if 0 for broadcast in mbus
    #[prost(fixed64, tag = "3")]
    pub target_id: u64,
    #[prost(fixed64, tag = "4")]
    pub mbus_id: u64,
    /// for message type
    #[prost(uint32, tag = "5")]
    pub msg_type: u32,
    /// for abstract information
    #[prost(string, tag = "6")]
    pub msg_info: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub arg_json: ::prost::alloc::string::String,
    /// content data (enbedded from v0.4.0)
    #[prost(message, optional, tag = "8")]
    pub cdata: ::core::option::Option<Content>,
}
/// options for creating Mbus from v0.4.0
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MbusOpt {
    #[prost(enumeration = "mbus_opt::MbusType", tag = "1")]
    pub mbus_type: i32,
    /// use this for limiting subscribers
    #[prost(fixed64, repeated, tag = "2")]
    pub subscribers: ::prost::alloc::vec::Vec<u64>,
}
/// Nested message and enum types in `MbusOpt`.
pub mod mbus_opt {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum MbusType {
        Public = 0,
        Private = 1,
    }
    impl MbusType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MbusType::Public => "PUBLIC",
                MbusType::Private => "PRIVATE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PUBLIC" => Some(Self::Public),
                "PRIVATE" => Some(Self::Private),
                _ => None,
            }
        }
    }
}
/// message for obtaining mbus state from 0.4.0
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MbusState {
    #[prost(fixed64, tag = "1")]
    pub mbus_id: u64,
    #[prost(enumeration = "mbus_state::MbusStatus", tag = "2")]
    pub status: i32,
    /// subscriber count  (only with status= SUBSCRIBERS)
    #[prost(fixed64, repeated, tag = "3")]
    pub subscribers: ::prost::alloc::vec::Vec<u64>,
}
/// Nested message and enum types in `MbusState`.
pub mod mbus_state {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum MbusStatus {
        /// just created (no subscribers)
        Intialized = 0,
        /// there are some subscribers
        Subscribers = 1,
        /// closed mbus
        Closed = 2,
        /// no mbus is available with the mbus_id or hidden.
        Invalid = 3,
    }
    impl MbusStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MbusStatus::Intialized => "INTIALIZED",
                MbusStatus::Subscribers => "SUBSCRIBERS",
                MbusStatus::Closed => "CLOSED",
                MbusStatus::Invalid => "INVALID",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INTIALIZED" => Some(Self::Intialized),
                "SUBSCRIBERS" => Some(Self::Subscribers),
                "CLOSED" => Some(Self::Closed),
                "INVALID" => Some(Self::Invalid),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GatewayInfo {
    /// client_id (snowflake) of gateway
    #[prost(fixed64, tag = "1")]
    pub client_id: u64,
    #[prost(enumeration = "GatewayType", tag = "2")]
    pub gateway_type: i32,
    /// which channel for forward
    #[prost(uint32, repeated, tag = "3")]
    pub channels: ::prost::alloc::vec::Vec<u32>,
}
/// Subscribe from Gateway to SynerexServer
///
/// how to prevent loop!
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GatewayMsg {
    #[prost(fixed64, tag = "1")]
    pub src_synerex_id: u64,
    /// massage type
    #[prost(enumeration = "MsgType", tag = "2")]
    pub msg_type: i32,
    #[prost(oneof = "gateway_msg::MsgOneof", tags = "3, 4, 5, 6, 7")]
    pub msg_oneof: ::core::option::Option<gateway_msg::MsgOneof>,
}
/// Nested message and enum types in `GatewayMsg`.
pub mod gateway_msg {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MsgOneof {
        #[prost(message, tag = "3")]
        Demand(super::Demand),
        #[prost(message, tag = "4")]
        Supply(super::Supply),
        #[prost(message, tag = "5")]
        Target(super::Target),
        #[prost(message, tag = "6")]
        Mbus(super::Mbus),
        #[prost(message, tag = "7")]
        MbusMsg(super::MbusMsg),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProviderId {
    #[prost(fixed64, tag = "1")]
    pub client_id: u64,
    /// for Any Argument
    #[prost(string, tag = "3")]
    pub arg_json: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GatewayType {
    /// normal gateway
    Bidirectional = 0,
    /// no need to receive
    WriteOnly = 1,
    ///
    ReadOnly = 2,
}
impl GatewayType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GatewayType::Bidirectional => "BIDIRECTIONAL",
            GatewayType::WriteOnly => "WRITE_ONLY",
            GatewayType::ReadOnly => "READ_ONLY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BIDIRECTIONAL" => Some(Self::Bidirectional),
            "WRITE_ONLY" => Some(Self::WriteOnly),
            "READ_ONLY" => Some(Self::ReadOnly),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MsgType {
    Demand = 0,
    Supply = 1,
    /// target for select/confirm
    Target = 2,
    /// mbus id for subscribe
    Mbus = 3,
    Mbusmsg = 4,
}
impl MsgType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MsgType::Demand => "DEMAND",
            MsgType::Supply => "SUPPLY",
            MsgType::Target => "TARGET",
            MsgType::Mbus => "MBUS",
            MsgType::Mbusmsg => "MBUSMSG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEMAND" => Some(Self::Demand),
            "SUPPLY" => Some(Self::Supply),
            "TARGET" => Some(Self::Target),
            "MBUS" => Some(Self::Mbus),
            "MBUSMSG" => Some(Self::Mbusmsg),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod synerex_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SynerexClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SynerexClient<tonic::transport::Channel> {
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
    impl<T> SynerexClient<T>
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
        ) -> SynerexClient<InterceptedService<T, F>>
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
            SynerexClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn notify_demand(
            &mut self,
            request: impl tonic::IntoRequest<super::Demand>,
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
            let path = http::uri::PathAndQuery::from_static("/api.Synerex/NotifyDemand");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.Synerex", "NotifyDemand"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn notify_supply(
            &mut self,
            request: impl tonic::IntoRequest<super::Supply>,
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
            let path = http::uri::PathAndQuery::from_static("/api.Synerex/NotifySupply");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.Synerex", "NotifySupply"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn propose_demand(
            &mut self,
            request: impl tonic::IntoRequest<super::Demand>,
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
                "/api.Synerex/ProposeDemand",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.Synerex", "ProposeDemand"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn propose_supply(
            &mut self,
            request: impl tonic::IntoRequest<super::Supply>,
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
                "/api.Synerex/ProposeSupply",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.Synerex", "ProposeSupply"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn select_supply(
            &mut self,
            request: impl tonic::IntoRequest<super::Target>,
        ) -> std::result::Result<
            tonic::Response<super::ConfirmResponse>,
            tonic::Status,
        > {
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
            let path = http::uri::PathAndQuery::from_static("/api.Synerex/SelectSupply");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.Synerex", "SelectSupply"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn select_modified_supply(
            &mut self,
            request: impl tonic::IntoRequest<super::Supply>,
        ) -> std::result::Result<
            tonic::Response<super::ConfirmResponse>,
            tonic::Status,
        > {
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
                "/api.Synerex/SelectModifiedSupply",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api.Synerex", "SelectModifiedSupply"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn select_demand(
            &mut self,
            request: impl tonic::IntoRequest<super::Target>,
        ) -> std::result::Result<
            tonic::Response<super::ConfirmResponse>,
            tonic::Status,
        > {
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
            let path = http::uri::PathAndQuery::from_static("/api.Synerex/SelectDemand");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.Synerex", "SelectDemand"));
            self.inner.unary(req, path, codec).await
        }
        ///    rpc SelectModifiedDemand(Demand) returns (ConfirmResponse) {} // select with modification(since 0.5.1)
        pub async fn confirm(
            &mut self,
            request: impl tonic::IntoRequest<super::Target>,
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
            let path = http::uri::PathAndQuery::from_static("/api.Synerex/Confirm");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.Synerex", "Confirm"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn subscribe_demand(
            &mut self,
            request: impl tonic::IntoRequest<super::Channel>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::Demand>>,
            tonic::Status,
        > {
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
                "/api.Synerex/SubscribeDemand",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api.Synerex", "SubscribeDemand"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn subscribe_supply(
            &mut self,
            request: impl tonic::IntoRequest<super::Channel>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::Supply>>,
            tonic::Status,
        > {
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
                "/api.Synerex/SubscribeSupply",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api.Synerex", "SubscribeSupply"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn create_mbus(
            &mut self,
            request: impl tonic::IntoRequest<super::MbusOpt>,
        ) -> std::result::Result<tonic::Response<super::Mbus>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/api.Synerex/CreateMbus");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.Synerex", "CreateMbus"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn close_mbus(
            &mut self,
            request: impl tonic::IntoRequest<super::Mbus>,
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
            let path = http::uri::PathAndQuery::from_static("/api.Synerex/CloseMbus");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.Synerex", "CloseMbus"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn subscribe_mbus(
            &mut self,
            request: impl tonic::IntoRequest<super::Mbus>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::MbusMsg>>,
            tonic::Status,
        > {
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
                "/api.Synerex/SubscribeMbus",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.Synerex", "SubscribeMbus"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn send_mbus_msg(
            &mut self,
            request: impl tonic::IntoRequest<super::MbusMsg>,
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
            let path = http::uri::PathAndQuery::from_static("/api.Synerex/SendMbusMsg");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.Synerex", "SendMbusMsg"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_mbus_state(
            &mut self,
            request: impl tonic::IntoRequest<super::Mbus>,
        ) -> std::result::Result<tonic::Response<super::MbusState>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/api.Synerex/GetMbusState");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.Synerex", "GetMbusState"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn subscribe_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::GatewayInfo>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::GatewayMsg>>,
            tonic::Status,
        > {
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
                "/api.Synerex/SubscribeGateway",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api.Synerex", "SubscribeGateway"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn forward_to_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::GatewayMsg>,
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
                "/api.Synerex/ForwardToGateway",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api.Synerex", "ForwardToGateway"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn close_demand_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::Channel>,
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
                "/api.Synerex/CloseDemandChannel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api.Synerex", "CloseDemandChannel"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn close_supply_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::Channel>,
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
                "/api.Synerex/CloseSupplyChannel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api.Synerex", "CloseSupplyChannel"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn close_all_channels(
            &mut self,
            request: impl tonic::IntoRequest<super::ProviderId>,
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
                "/api.Synerex/CloseAllChannels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api.Synerex", "CloseAllChannels"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod synerex_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SynerexServer.
    #[async_trait]
    pub trait Synerex: Send + Sync + 'static {
        async fn notify_demand(
            &self,
            request: tonic::Request<super::Demand>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status>;
        async fn notify_supply(
            &self,
            request: tonic::Request<super::Supply>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status>;
        async fn propose_demand(
            &self,
            request: tonic::Request<super::Demand>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status>;
        async fn propose_supply(
            &self,
            request: tonic::Request<super::Supply>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status>;
        async fn select_supply(
            &self,
            request: tonic::Request<super::Target>,
        ) -> std::result::Result<tonic::Response<super::ConfirmResponse>, tonic::Status>;
        async fn select_modified_supply(
            &self,
            request: tonic::Request<super::Supply>,
        ) -> std::result::Result<tonic::Response<super::ConfirmResponse>, tonic::Status>;
        async fn select_demand(
            &self,
            request: tonic::Request<super::Target>,
        ) -> std::result::Result<tonic::Response<super::ConfirmResponse>, tonic::Status>;
        ///    rpc SelectModifiedDemand(Demand) returns (ConfirmResponse) {} // select with modification(since 0.5.1)
        async fn confirm(
            &self,
            request: tonic::Request<super::Target>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status>;
        /// Server streaming response type for the SubscribeDemand method.
        type SubscribeDemandStream: futures_core::Stream<
                Item = std::result::Result<super::Demand, tonic::Status>,
            >
            + Send
            + 'static;
        async fn subscribe_demand(
            &self,
            request: tonic::Request<super::Channel>,
        ) -> std::result::Result<
            tonic::Response<Self::SubscribeDemandStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the SubscribeSupply method.
        type SubscribeSupplyStream: futures_core::Stream<
                Item = std::result::Result<super::Supply, tonic::Status>,
            >
            + Send
            + 'static;
        async fn subscribe_supply(
            &self,
            request: tonic::Request<super::Channel>,
        ) -> std::result::Result<
            tonic::Response<Self::SubscribeSupplyStream>,
            tonic::Status,
        >;
        async fn create_mbus(
            &self,
            request: tonic::Request<super::MbusOpt>,
        ) -> std::result::Result<tonic::Response<super::Mbus>, tonic::Status>;
        async fn close_mbus(
            &self,
            request: tonic::Request<super::Mbus>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status>;
        /// Server streaming response type for the SubscribeMbus method.
        type SubscribeMbusStream: futures_core::Stream<
                Item = std::result::Result<super::MbusMsg, tonic::Status>,
            >
            + Send
            + 'static;
        async fn subscribe_mbus(
            &self,
            request: tonic::Request<super::Mbus>,
        ) -> std::result::Result<
            tonic::Response<Self::SubscribeMbusStream>,
            tonic::Status,
        >;
        async fn send_mbus_msg(
            &self,
            request: tonic::Request<super::MbusMsg>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status>;
        async fn get_mbus_state(
            &self,
            request: tonic::Request<super::Mbus>,
        ) -> std::result::Result<tonic::Response<super::MbusState>, tonic::Status>;
        /// Server streaming response type for the SubscribeGateway method.
        type SubscribeGatewayStream: futures_core::Stream<
                Item = std::result::Result<super::GatewayMsg, tonic::Status>,
            >
            + Send
            + 'static;
        async fn subscribe_gateway(
            &self,
            request: tonic::Request<super::GatewayInfo>,
        ) -> std::result::Result<
            tonic::Response<Self::SubscribeGatewayStream>,
            tonic::Status,
        >;
        async fn forward_to_gateway(
            &self,
            request: tonic::Request<super::GatewayMsg>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status>;
        async fn close_demand_channel(
            &self,
            request: tonic::Request<super::Channel>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status>;
        async fn close_supply_channel(
            &self,
            request: tonic::Request<super::Channel>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status>;
        async fn close_all_channels(
            &self,
            request: tonic::Request<super::ProviderId>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SynerexServer<T: Synerex> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Synerex> SynerexServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SynerexServer<T>
    where
        T: Synerex,
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
                "/api.Synerex/NotifyDemand" => {
                    #[allow(non_camel_case_types)]
                    struct NotifyDemandSvc<T: Synerex>(pub Arc<T>);
                    impl<T: Synerex> tonic::server::UnaryService<super::Demand>
                    for NotifyDemandSvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Demand>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).notify_demand(request).await
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
                        let method = NotifyDemandSvc(inner);
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
                "/api.Synerex/NotifySupply" => {
                    #[allow(non_camel_case_types)]
                    struct NotifySupplySvc<T: Synerex>(pub Arc<T>);
                    impl<T: Synerex> tonic::server::UnaryService<super::Supply>
                    for NotifySupplySvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Supply>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).notify_supply(request).await
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
                        let method = NotifySupplySvc(inner);
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
                "/api.Synerex/ProposeDemand" => {
                    #[allow(non_camel_case_types)]
                    struct ProposeDemandSvc<T: Synerex>(pub Arc<T>);
                    impl<T: Synerex> tonic::server::UnaryService<super::Demand>
                    for ProposeDemandSvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Demand>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).propose_demand(request).await
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
                        let method = ProposeDemandSvc(inner);
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
                "/api.Synerex/ProposeSupply" => {
                    #[allow(non_camel_case_types)]
                    struct ProposeSupplySvc<T: Synerex>(pub Arc<T>);
                    impl<T: Synerex> tonic::server::UnaryService<super::Supply>
                    for ProposeSupplySvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Supply>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).propose_supply(request).await
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
                        let method = ProposeSupplySvc(inner);
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
                "/api.Synerex/SelectSupply" => {
                    #[allow(non_camel_case_types)]
                    struct SelectSupplySvc<T: Synerex>(pub Arc<T>);
                    impl<T: Synerex> tonic::server::UnaryService<super::Target>
                    for SelectSupplySvc<T> {
                        type Response = super::ConfirmResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Target>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).select_supply(request).await
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
                        let method = SelectSupplySvc(inner);
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
                "/api.Synerex/SelectModifiedSupply" => {
                    #[allow(non_camel_case_types)]
                    struct SelectModifiedSupplySvc<T: Synerex>(pub Arc<T>);
                    impl<T: Synerex> tonic::server::UnaryService<super::Supply>
                    for SelectModifiedSupplySvc<T> {
                        type Response = super::ConfirmResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Supply>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).select_modified_supply(request).await
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
                        let method = SelectModifiedSupplySvc(inner);
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
                "/api.Synerex/SelectDemand" => {
                    #[allow(non_camel_case_types)]
                    struct SelectDemandSvc<T: Synerex>(pub Arc<T>);
                    impl<T: Synerex> tonic::server::UnaryService<super::Target>
                    for SelectDemandSvc<T> {
                        type Response = super::ConfirmResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Target>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).select_demand(request).await
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
                        let method = SelectDemandSvc(inner);
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
                "/api.Synerex/Confirm" => {
                    #[allow(non_camel_case_types)]
                    struct ConfirmSvc<T: Synerex>(pub Arc<T>);
                    impl<T: Synerex> tonic::server::UnaryService<super::Target>
                    for ConfirmSvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Target>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).confirm(request).await };
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
                        let method = ConfirmSvc(inner);
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
                "/api.Synerex/SubscribeDemand" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeDemandSvc<T: Synerex>(pub Arc<T>);
                    impl<
                        T: Synerex,
                    > tonic::server::ServerStreamingService<super::Channel>
                    for SubscribeDemandSvc<T> {
                        type Response = super::Demand;
                        type ResponseStream = T::SubscribeDemandStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Channel>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).subscribe_demand(request).await
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
                        let method = SubscribeDemandSvc(inner);
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
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Synerex/SubscribeSupply" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeSupplySvc<T: Synerex>(pub Arc<T>);
                    impl<
                        T: Synerex,
                    > tonic::server::ServerStreamingService<super::Channel>
                    for SubscribeSupplySvc<T> {
                        type Response = super::Supply;
                        type ResponseStream = T::SubscribeSupplyStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Channel>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).subscribe_supply(request).await
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
                        let method = SubscribeSupplySvc(inner);
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
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Synerex/CreateMbus" => {
                    #[allow(non_camel_case_types)]
                    struct CreateMbusSvc<T: Synerex>(pub Arc<T>);
                    impl<T: Synerex> tonic::server::UnaryService<super::MbusOpt>
                    for CreateMbusSvc<T> {
                        type Response = super::Mbus;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MbusOpt>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).create_mbus(request).await };
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
                        let method = CreateMbusSvc(inner);
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
                "/api.Synerex/CloseMbus" => {
                    #[allow(non_camel_case_types)]
                    struct CloseMbusSvc<T: Synerex>(pub Arc<T>);
                    impl<T: Synerex> tonic::server::UnaryService<super::Mbus>
                    for CloseMbusSvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Mbus>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).close_mbus(request).await };
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
                        let method = CloseMbusSvc(inner);
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
                "/api.Synerex/SubscribeMbus" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeMbusSvc<T: Synerex>(pub Arc<T>);
                    impl<T: Synerex> tonic::server::ServerStreamingService<super::Mbus>
                    for SubscribeMbusSvc<T> {
                        type Response = super::MbusMsg;
                        type ResponseStream = T::SubscribeMbusStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Mbus>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).subscribe_mbus(request).await
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
                        let method = SubscribeMbusSvc(inner);
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
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Synerex/SendMbusMsg" => {
                    #[allow(non_camel_case_types)]
                    struct SendMbusMsgSvc<T: Synerex>(pub Arc<T>);
                    impl<T: Synerex> tonic::server::UnaryService<super::MbusMsg>
                    for SendMbusMsgSvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MbusMsg>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).send_mbus_msg(request).await
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
                        let method = SendMbusMsgSvc(inner);
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
                "/api.Synerex/GetMbusState" => {
                    #[allow(non_camel_case_types)]
                    struct GetMbusStateSvc<T: Synerex>(pub Arc<T>);
                    impl<T: Synerex> tonic::server::UnaryService<super::Mbus>
                    for GetMbusStateSvc<T> {
                        type Response = super::MbusState;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Mbus>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_mbus_state(request).await
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
                        let method = GetMbusStateSvc(inner);
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
                "/api.Synerex/SubscribeGateway" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeGatewaySvc<T: Synerex>(pub Arc<T>);
                    impl<
                        T: Synerex,
                    > tonic::server::ServerStreamingService<super::GatewayInfo>
                    for SubscribeGatewaySvc<T> {
                        type Response = super::GatewayMsg;
                        type ResponseStream = T::SubscribeGatewayStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GatewayInfo>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).subscribe_gateway(request).await
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
                        let method = SubscribeGatewaySvc(inner);
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
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Synerex/ForwardToGateway" => {
                    #[allow(non_camel_case_types)]
                    struct ForwardToGatewaySvc<T: Synerex>(pub Arc<T>);
                    impl<T: Synerex> tonic::server::UnaryService<super::GatewayMsg>
                    for ForwardToGatewaySvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GatewayMsg>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).forward_to_gateway(request).await
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
                        let method = ForwardToGatewaySvc(inner);
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
                "/api.Synerex/CloseDemandChannel" => {
                    #[allow(non_camel_case_types)]
                    struct CloseDemandChannelSvc<T: Synerex>(pub Arc<T>);
                    impl<T: Synerex> tonic::server::UnaryService<super::Channel>
                    for CloseDemandChannelSvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Channel>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).close_demand_channel(request).await
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
                        let method = CloseDemandChannelSvc(inner);
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
                "/api.Synerex/CloseSupplyChannel" => {
                    #[allow(non_camel_case_types)]
                    struct CloseSupplyChannelSvc<T: Synerex>(pub Arc<T>);
                    impl<T: Synerex> tonic::server::UnaryService<super::Channel>
                    for CloseSupplyChannelSvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Channel>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).close_supply_channel(request).await
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
                        let method = CloseSupplyChannelSvc(inner);
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
                "/api.Synerex/CloseAllChannels" => {
                    #[allow(non_camel_case_types)]
                    struct CloseAllChannelsSvc<T: Synerex>(pub Arc<T>);
                    impl<T: Synerex> tonic::server::UnaryService<super::ProviderId>
                    for CloseAllChannelsSvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProviderId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).close_all_channels(request).await
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
                        let method = CloseAllChannelsSvc(inner);
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
    impl<T: Synerex> Clone for SynerexServer<T> {
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
    impl<T: Synerex> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Synerex> tonic::server::NamedService for SynerexServer<T> {
        const NAME: &'static str = "api.Synerex";
    }
}
