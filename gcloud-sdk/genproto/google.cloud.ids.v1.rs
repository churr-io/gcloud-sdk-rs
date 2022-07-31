///  Endpoint describes a single IDS endpoint. It defines a forwarding rule to
///  which packets can be sent for IDS inspection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Endpoint {
    ///  Output only. The name of the endpoint.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Output only. The create time timestamp.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The update time timestamp.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  The labels of the endpoint.
    #[prost(map="string, string", tag="4")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  Required. The fully qualified URL of the network to which the IDS Endpoint is
    ///  attached.
    #[prost(string, tag="5")]
    pub network: ::prost::alloc::string::String,
    ///  Output only. The fully qualified URL of the endpoint's ILB Forwarding Rule.
    #[prost(string, tag="6")]
    pub endpoint_forwarding_rule: ::prost::alloc::string::String,
    ///  Output only. The IP address of the IDS Endpoint's ILB.
    #[prost(string, tag="7")]
    pub endpoint_ip: ::prost::alloc::string::String,
    ///  User-provided description of the endpoint
    #[prost(string, tag="8")]
    pub description: ::prost::alloc::string::String,
    ///  Required. Lowest threat severity that this endpoint will alert on.
    #[prost(enumeration="endpoint::Severity", tag="9")]
    pub severity: i32,
    ///  Output only. Current state of the endpoint.
    #[prost(enumeration="endpoint::State", tag="12")]
    pub state: i32,
    ///  Whether the endpoint should report traffic logs in addition to threat logs.
    #[prost(bool, tag="13")]
    pub traffic_logs: bool,
}
/// Nested message and enum types in `Endpoint`.
pub mod endpoint {
    ///  Threat severity levels.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Severity {
        ///  Not set.
        Unspecified = 0,
        ///  Informational alerts.
        Informational = 1,
        ///  Low severity alerts.
        Low = 2,
        ///  Medium severity alerts.
        Medium = 3,
        ///  High severity alerts.
        High = 4,
        ///  Critical severity alerts.
        Critical = 5,
    }
    impl Severity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Severity::Unspecified => "SEVERITY_UNSPECIFIED",
                Severity::Informational => "INFORMATIONAL",
                Severity::Low => "LOW",
                Severity::Medium => "MEDIUM",
                Severity::High => "HIGH",
                Severity::Critical => "CRITICAL",
            }
        }
    }
    ///  Endpoint state
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        ///  Not set.
        Unspecified = 0,
        ///  Being created.
        Creating = 1,
        ///  Active and ready for traffic.
        Ready = 2,
        ///  Being deleted.
        Deleting = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Creating => "CREATING",
                State::Ready => "READY",
                State::Deleting => "DELETING",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEndpointsRequest {
    ///  Required. The parent, which owns this collection of endpoints.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Optional. The maximum number of endpoints to return. The service may return fewer
    ///  than this value.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    ///  Optional. A page token, received from a previous `ListEndpoints` call.
    ///  Provide this to retrieve the subsequent page.
    ///
    ///  When paginating, all other parameters provided to `ListEndpoints` must
    ///  match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    ///  Optional. The filter expression, following the syntax outlined in
    ///  <https://google.aip.dev/160.>
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    ///  Optional. One or more fields to compare and use to sort the output.
    ///  See <https://google.aip.dev/132#ordering.>
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEndpointsResponse {
    ///  The list of endpoints response.
    #[prost(message, repeated, tag="1")]
    pub endpoints: ::prost::alloc::vec::Vec<Endpoint>,
    ///  A token, which can be sent as `page_token` to retrieve the next page.
    ///  If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    ///  Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndpointRequest {
    ///  Required. The name of the endpoint to retrieve.
    ///  Format: `projects/{project}/locations/{location}/endpoints/{endpoint}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEndpointRequest {
    ///  Required. The endpoint's parent.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Required. The endpoint identifier. This will be part of the endpoint's
    ///  resource name.
    ///  This value must start with a lowercase letter followed by up to 62
    ///  lowercase letters, numbers, or hyphens, and cannot end with a hyphen.
    ///  Values that do not match this pattern will trigger an INVALID_ARGUMENT
    ///  error.
    #[prost(string, tag="2")]
    pub endpoint_id: ::prost::alloc::string::String,
    ///  Required. The endpoint to create.
    #[prost(message, optional, tag="3")]
    pub endpoint: ::core::option::Option<Endpoint>,
    ///  An optional request ID to identify requests. Specify a unique request ID
    ///  so that if you must retry your request, the server will know to ignore
    ///  the request if it has already been completed. The server will guarantee
    ///  that for at least 60 minutes since the first request.
    ///
    ///  For example, consider a situation where you make an initial request and t
    ///  he request times out. If you make the request again with the same request
    ///  ID, the server can check if original operation with the same request ID
    ///  was received, and if so, will ignore the second request. This prevents
    ///  clients from accidentally creating duplicate commitments.
    ///
    ///  The request ID must be a valid UUID with the exception that zero UUID is
    ///  not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag="4")]
    pub request_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEndpointRequest {
    ///  Required. The name of the endpoint to delete.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  An optional request ID to identify requests. Specify a unique request ID
    ///  so that if you must retry your request, the server will know to ignore
    ///  the request if it has already been completed. The server will guarantee
    ///  that for at least 60 minutes after the first request.
    ///
    ///  For example, consider a situation where you make an initial request and t
    ///  he request times out. If you make the request again with the same request
    ///  ID, the server can check if original operation with the same request ID
    ///  was received, and if so, will ignore the second request. This prevents
    ///  clients from accidentally creating duplicate commitments.
    ///
    ///  The request ID must be a valid UUID with the exception that zero UUID is
    ///  not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag="2")]
    pub request_id: ::prost::alloc::string::String,
}
///  Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    ///  Output only. The time the operation was created.
    #[prost(message, optional, tag="1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The time the operation finished running.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag="3")]
    pub target: ::prost::alloc::string::String,
    ///  Output only. Name of the verb executed by the operation.
    #[prost(string, tag="4")]
    pub verb: ::prost::alloc::string::String,
    ///  Output only. Human-readable status of the operation, if any.
    #[prost(string, tag="5")]
    pub status_message: ::prost::alloc::string::String,
    ///  Output only. Identifies whether the user has requested cancellation
    ///  of the operation. Operations that have successfully been cancelled
    ///  have \[Operation.error][\] value with a \[google.rpc.Status.code][google.rpc.Status.code\] of 1,
    ///  corresponding to `Code.CANCELLED`.
    #[prost(bool, tag="6")]
    pub requested_cancellation: bool,
    ///  Output only. API version used to start the operation.
    #[prost(string, tag="7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod ids_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The IDS Service
    #[derive(Debug, Clone)]
    pub struct IdsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IdsClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> IdsClient<T>
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
        ) -> IdsClient<InterceptedService<T, F>>
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
            IdsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists Endpoints in a given project and location.
        pub async fn list_endpoints(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEndpointsRequest>,
        ) -> Result<tonic::Response<super::ListEndpointsResponse>, tonic::Status> {
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
                "/google.cloud.ids.v1.IDS/ListEndpoints",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Endpoint.
        pub async fn get_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEndpointRequest>,
        ) -> Result<tonic::Response<super::Endpoint>, tonic::Status> {
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
                "/google.cloud.ids.v1.IDS/GetEndpoint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Endpoint in a given project and location.
        pub async fn create_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEndpointRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.ids.v1.IDS/CreateEndpoint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Endpoint.
        pub async fn delete_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEndpointRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.ids.v1.IDS/DeleteEndpoint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
