///  Feature represents the settings and status of any Hub Feature.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feature {
    ///  Output only. The full, unique name of this Feature resource in the format
    ///  `projects/*/locations/*/features/*`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  GCP labels for this Feature.
    #[prost(map="string, string", tag="2")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  Output only. State of the Feature resource itself.
    #[prost(message, optional, tag="3")]
    pub resource_state: ::core::option::Option<FeatureResourceState>,
    ///  Optional. Hub-wide Feature configuration. If this Feature does not support any
    ///  Hub-wide configuration, this field may be unused.
    #[prost(message, optional, tag="4")]
    pub spec: ::core::option::Option<CommonFeatureSpec>,
    ///  Optional. Membership-specific configuration for this Feature. If this Feature does
    ///  not support any per-Membership configuration, this field may be unused.
    ///
    ///  The keys indicate which Membership the configuration is for, in the form:
    ///
    ///      projects/{p}/locations/{l}/memberships/{m}
    ///
    ///  Where {p} is the project, {l} is a valid location and {m} is a valid
    ///  Membership in this project at that location. {p} WILL match the Feature's
    ///  project.
    ///
    ///  {p} will always be returned as the project number, but the project ID is
    ///  also accepted during input. If the same Membership is specified in the map
    ///  twice (using the project ID form, and the project number form), exactly
    ///  ONE of the entries will be saved, with no guarantees as to which. For this
    ///  reason, it is recommended the same format be used for all entries when
    ///  mutating a Feature.
    #[prost(map="string, message", tag="5")]
    pub membership_specs: ::std::collections::HashMap<::prost::alloc::string::String, MembershipFeatureSpec>,
    ///  Output only. The Hub-wide Feature state.
    #[prost(message, optional, tag="6")]
    pub state: ::core::option::Option<CommonFeatureState>,
    ///  Output only. Membership-specific Feature status. If this Feature does
    ///  report any per-Membership status, this field may be unused.
    ///
    ///  The keys indicate which Membership the state is for, in the form:
    ///
    ///      projects/{p}/locations/{l}/memberships/{m}
    ///
    ///  Where {p} is the project number, {l} is a valid location and {m} is a valid
    ///  Membership in this project at that location. {p} MUST match the Feature's
    ///  project number.
    #[prost(map="string, message", tag="7")]
    pub membership_states: ::std::collections::HashMap<::prost::alloc::string::String, MembershipFeatureState>,
    ///  Output only. When the Feature resource was created.
    #[prost(message, optional, tag="8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. When the Feature resource was last updated.
    #[prost(message, optional, tag="9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. When the Feature resource was deleted.
    #[prost(message, optional, tag="10")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
}
///  FeatureResourceState describes the state of a Feature *resource* in the
///  GkeHub API. See `FeatureState` for the "running state" of the Feature in the
///  Hub and across Memberships.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureResourceState {
    ///  The current state of the Feature resource in the Hub API.
    #[prost(enumeration="feature_resource_state::State", tag="1")]
    pub state: i32,
}
/// Nested message and enum types in `FeatureResourceState`.
pub mod feature_resource_state {
    ///  State describes the lifecycle status of a Feature.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        ///  State is unknown or not set.
        Unspecified = 0,
        ///  The Feature is being enabled, and the Feature resource is being created.
        ///  Once complete, the corresponding Feature will be enabled in this Hub.
        Enabling = 1,
        ///  The Feature is enabled in this Hub, and the Feature resource is fully
        ///  available.
        Active = 2,
        ///  The Feature is being disabled in this Hub, and the Feature resource
        ///  is being deleted.
        Disabling = 3,
        ///  The Feature resource is being updated.
        Updating = 4,
        ///  The Feature resource is being updated by the Hub Service.
        ServiceUpdating = 5,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Enabling => "ENABLING",
                State::Active => "ACTIVE",
                State::Disabling => "DISABLING",
                State::Updating => "UPDATING",
                State::ServiceUpdating => "SERVICE_UPDATING",
            }
        }
    }
}
///  FeatureState describes the high-level state of a Feature. It may be used to
///  describe a Feature's state at the environ-level, or per-membershop, depending
///  on the context.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureState {
    ///  The high-level, machine-readable status of this Feature.
    #[prost(enumeration="feature_state::Code", tag="1")]
    pub code: i32,
    ///  A human-readable description of the current status.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    ///  The time this status and any related Feature-specific details were updated.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `FeatureState`.
pub mod feature_state {
    ///  Code represents a machine-readable, high-level status of the Feature.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Code {
        ///  Unknown or not set.
        Unspecified = 0,
        ///  The Feature is operating normally.
        Ok = 1,
        ///  The Feature has encountered an issue, and is operating in a degraded
        ///  state. The Feature may need intervention to return to normal operation.
        ///  See the description and any associated Feature-specific details for more
        ///  information.
        Warning = 2,
        ///  The Feature is not operating or is in a severely degraded state.
        ///  The Feature may need intervention to return to normal operation.
        ///  See the description and any associated Feature-specific details for more
        ///  information.
        Error = 3,
    }
    impl Code {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Code::Unspecified => "CODE_UNSPECIFIED",
                Code::Ok => "OK",
                Code::Warning => "WARNING",
                Code::Error => "ERROR",
            }
        }
    }
}
///  CommonFeatureSpec contains Hub-wide configuration information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonFeatureSpec {
    #[prost(oneof="common_feature_spec::FeatureSpec", tags="102, 108")]
    pub feature_spec: ::core::option::Option<common_feature_spec::FeatureSpec>,
}
/// Nested message and enum types in `CommonFeatureSpec`.
pub mod common_feature_spec {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FeatureSpec {
        ///  Multicluster Ingress-specific spec.
        #[prost(message, tag="102")]
        Multiclusteringress(super::super::multiclusteringress::v1alpha::FeatureSpec),
        ///  Cloud Audit Logging-specific spec.
        #[prost(message, tag="108")]
        Cloudauditlogging(super::super::cloudauditlogging::v1alpha::FeatureSpec),
    }
}
///  CommonFeatureState contains Hub-wide Feature status information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonFeatureState {
    ///  Output only. The "running state" of the Feature in this Hub.
    #[prost(message, optional, tag="1")]
    pub state: ::core::option::Option<FeatureState>,
    #[prost(oneof="common_feature_state::FeatureState", tags="100")]
    pub feature_state: ::core::option::Option<common_feature_state::FeatureState>,
}
/// Nested message and enum types in `CommonFeatureState`.
pub mod common_feature_state {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FeatureState {
        ///  Service Mesh-specific state.
        #[prost(message, tag="100")]
        Servicemesh(super::super::servicemesh::v1alpha::FeatureState),
    }
}
///  MembershipFeatureSpec contains configuration information for a single
///  Membership.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MembershipFeatureSpec {
    #[prost(oneof="membership_feature_spec::FeatureSpec", tags="106")]
    pub feature_spec: ::core::option::Option<membership_feature_spec::FeatureSpec>,
}
/// Nested message and enum types in `MembershipFeatureSpec`.
pub mod membership_feature_spec {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FeatureSpec {
        ///  Config Management-specific spec.
        #[prost(message, tag="106")]
        Configmanagement(super::super::configmanagement::v1alpha::MembershipSpec),
    }
}
///  MembershipFeatureState contains Feature status information for a single
///  Membership.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MembershipFeatureState {
    ///  The high-level state of this Feature for a single membership.
    #[prost(message, optional, tag="1")]
    pub state: ::core::option::Option<FeatureState>,
    #[prost(oneof="membership_feature_state::FeatureState", tags="100, 104, 106")]
    pub feature_state: ::core::option::Option<membership_feature_state::FeatureState>,
}
/// Nested message and enum types in `MembershipFeatureState`.
pub mod membership_feature_state {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FeatureState {
        ///  Service Mesh-specific state.
        #[prost(message, tag="100")]
        Servicemesh(super::super::servicemesh::v1alpha::MembershipState),
        ///  Metering-specific spec.
        #[prost(message, tag="104")]
        Metering(super::super::metering::v1alpha::MembershipState),
        ///  Config Management-specific state.
        #[prost(message, tag="106")]
        Configmanagement(super::super::configmanagement::v1alpha::MembershipState),
    }
}
///  Request message for `GkeHub.ListFeatures` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFeaturesRequest {
    ///  The parent (project and location) where the Features will be listed.
    ///  Specified in the format `projects/*/locations/*`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  When requesting a 'page' of resources, `page_size` specifies number of
    ///  resources to return. If unspecified or set to 0, all resources will
    ///  be returned.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    ///  Token returned by previous call to `ListFeatures` which
    ///  specifies the position in the list from where to continue listing the
    ///  resources.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    ///  Lists Features that match the filter expression, following the syntax
    ///  outlined in <https://google.aip.dev/160.>
    ///
    ///  Examples:
    ///
    ///    - Feature with the name "servicemesh" in project "foo-proj":
    ///
    ///        name = "projects/foo-proj/locations/global/features/servicemesh"
    ///
    ///    - Features that have a label called `foo`:
    ///
    ///        labels.foo:*
    ///
    ///    - Features that have a label called `foo` whose value is `bar`:
    ///
    ///        labels.foo = bar
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    ///  One or more fields to compare and use to sort the output.
    ///  See <https://google.aip.dev/132#ordering.>
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
///  Response message for the `GkeHub.ListFeatures` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFeaturesResponse {
    ///  The list of matching Features
    #[prost(message, repeated, tag="1")]
    pub resources: ::prost::alloc::vec::Vec<Feature>,
    ///  A token to request the next page of resources from the
    ///  `ListFeatures` method. The value of an empty string means
    ///  that there are no more resources to return.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  Request message for `GkeHub.GetFeature` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFeatureRequest {
    ///  The Feature resource name in the format
    ///  `projects/*/locations/*/features/*`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Request message for the `GkeHub.CreateFeature` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFeatureRequest {
    ///  The parent (project and location) where the Feature will be created.
    ///  Specified in the format `projects/*/locations/*`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  The ID of the feature to create.
    #[prost(string, tag="2")]
    pub feature_id: ::prost::alloc::string::String,
    ///  The Feature resource to create.
    #[prost(message, optional, tag="3")]
    pub resource: ::core::option::Option<Feature>,
    ///  Optional. A request ID to identify requests. Specify a unique request ID
    ///  so that if you must retry your request, the server will know to ignore
    ///  the request if it has already been completed. The server will guarantee
    ///  that for at least 60 minutes after the first request.
    ///
    ///  For example, consider a situation where you make an initial request and
    ///  the request times out. If you make the request again with the same request
    ///  ID, the server can check if original operation with the same request ID
    ///  was received, and if so, will ignore the second request. This prevents
    ///  clients from accidentally creating duplicate commitments.
    ///
    ///  The request ID must be a valid UUID with the exception that zero UUID is
    ///  not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag="4")]
    pub request_id: ::prost::alloc::string::String,
}
///  Request message for `GkeHub.DeleteFeature` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFeatureRequest {
    ///  The Feature resource name in the format
    ///  `projects/*/locations/*/features/*`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  If set to true, the delete will ignore any outstanding resources for
    ///  this Feature (that is, `FeatureState.has_resources` is set to true). These
    ///  resources will NOT be cleaned up or modified in any way.
    #[prost(bool, tag="2")]
    pub force: bool,
    ///  Optional. A request ID to identify requests. Specify a unique request ID
    ///  so that if you must retry your request, the server will know to ignore
    ///  the request if it has already been completed. The server will guarantee
    ///  that for at least 60 minutes after the first request.
    ///
    ///  For example, consider a situation where you make an initial request and
    ///  the request times out. If you make the request again with the same request
    ///  ID, the server can check if original operation with the same request ID
    ///  was received, and if so, will ignore the second request. This prevents
    ///  clients from accidentally creating duplicate commitments.
    ///
    ///  The request ID must be a valid UUID with the exception that zero UUID is
    ///  not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag="3")]
    pub request_id: ::prost::alloc::string::String,
}
///  Request message for `GkeHub.UpdateFeature` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFeatureRequest {
    ///  The Feature resource name in the format
    ///  `projects/*/locations/*/features/*`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Mask of fields to update.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    ///  Only fields specified in update_mask are updated.
    ///  If you specify a field in the update_mask but don't specify its value here
    ///  that field will be deleted.
    ///  If you are updating a map field, set the value of a key to null or empty
    ///  string to delete the key from the map. It's not possible to update a key's
    ///  value to the empty string.
    ///  If you specify the update_mask to be a special path "*", fully replaces all
    ///  user-modifiable fields to match `resource`.
    #[prost(message, optional, tag="3")]
    pub resource: ::core::option::Option<Feature>,
    ///  Optional. A request ID to identify requests. Specify a unique request ID
    ///  so that if you must retry your request, the server will know to ignore
    ///  the request if it has already been completed. The server will guarantee
    ///  that for at least 60 minutes after the first request.
    ///
    ///  For example, consider a situation where you make an initial request and
    ///  the request times out. If you make the request again with the same request
    ///  ID, the server can check if original operation with the same request ID
    ///  was received, and if so, will ignore the second request. This prevents
    ///  clients from accidentally creating duplicate commitments.
    ///
    ///  The request ID must be a valid UUID with the exception that zero UUID is
    ///  not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag="4")]
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
    pub status_detail: ::prost::alloc::string::String,
    ///  Output only. Identifies whether the user has requested cancellation
    ///  of the operation. Operations that have successfully been cancelled
    ///  have \[Operation.error][\] value with a \[google.rpc.Status.code][google.rpc.Status.code\] of 1,
    ///  corresponding to `Code.CANCELLED`.
    #[prost(bool, tag="6")]
    pub cancel_requested: bool,
    ///  Output only. API version used to start the operation.
    #[prost(string, tag="7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod gke_hub_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The GKE Hub service handles the registration of many Kubernetes clusters to
    /// Google Cloud, and the management of multi-cluster features over those
    /// clusters.
    ///
    /// The GKE Hub service operates on the following resources:
    ///
    /// * [Membership][google.cloud.gkehub.v1alpha.Membership]
    /// * [Feature][google.cloud.gkehub.v1alpha.Feature]
    ///
    /// GKE Hub is currently only available in the global region.
    ///
    /// **Membership management may be non-trivial:** it is recommended to use one
    /// of the Google-provided client libraries or tools where possible when working
    /// with Membership resources.
    #[derive(Debug, Clone)]
    pub struct GkeHubClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GkeHubClient<tonic::transport::Channel> {
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
    impl<T> GkeHubClient<T>
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
        ) -> GkeHubClient<InterceptedService<T, F>>
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
            GkeHubClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists Features in a given project and location.
        pub async fn list_features(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFeaturesRequest>,
        ) -> Result<tonic::Response<super::ListFeaturesResponse>, tonic::Status> {
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
                "/google.cloud.gkehub.v1alpha.GkeHub/ListFeatures",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Feature.
        pub async fn get_feature(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFeatureRequest>,
        ) -> Result<tonic::Response<super::Feature>, tonic::Status> {
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
                "/google.cloud.gkehub.v1alpha.GkeHub/GetFeature",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Adds a new Feature.
        pub async fn create_feature(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFeatureRequest>,
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
                "/google.cloud.gkehub.v1alpha.GkeHub/CreateFeature",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Removes a Feature.
        pub async fn delete_feature(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFeatureRequest>,
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
                "/google.cloud.gkehub.v1alpha.GkeHub/DeleteFeature",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an existing Feature.
        pub async fn update_feature(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFeatureRequest>,
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
                "/google.cloud.gkehub.v1alpha.GkeHub/UpdateFeature",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
