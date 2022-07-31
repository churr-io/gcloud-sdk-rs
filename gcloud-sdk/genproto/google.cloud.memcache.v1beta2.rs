///  A Memorystore for Memcached instance
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    ///  Required. Unique name of the resource in this scope including project and
    ///  location using the form:
    ///      `projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    ///
    ///  Note: Memcached instances are managed and addressed at the regional level
    ///  so `location_id` here refers to a Google Cloud region; however, users may
    ///  choose which zones Memcached nodes should be provisioned in within an
    ///  instance. Refer to \[zones][google.cloud.memcache.v1beta2.Instance.zones\] field for more details.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  User provided name for the instance, which is only used for display
    ///  purposes. Cannot be more than 80 characters.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    ///  Resource labels to represent user-provided metadata.
    ///  Refer to cloud documentation on labels for more details.
    ///  <https://cloud.google.com/compute/docs/labeling-resources>
    #[prost(map="string, string", tag="3")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  The full name of the Google Compute Engine
    ///  \[network\](<https://cloud.google.com/vpc/docs/vpc>) to which the
    ///  instance is connected. If left unspecified, the `default` network
    ///  will be used.
    #[prost(string, tag="4")]
    pub authorized_network: ::prost::alloc::string::String,
    ///  Zones in which Memcached nodes should be provisioned.
    ///  Memcached nodes will be equally distributed across these zones. If not
    ///  provided, the service will by default create nodes in all zones in the
    ///  region for the instance.
    #[prost(string, repeated, tag="5")]
    pub zones: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  Required. Number of nodes in the Memcached instance.
    #[prost(int32, tag="6")]
    pub node_count: i32,
    ///  Required. Configuration for Memcached nodes.
    #[prost(message, optional, tag="7")]
    pub node_config: ::core::option::Option<instance::NodeConfig>,
    ///  The major version of Memcached software.
    ///  If not provided, latest supported version will be used. Currently the
    ///  latest supported major version is `MEMCACHE_1_5`.
    ///  The minor version will be automatically determined by our system based on
    ///  the latest supported minor version.
    #[prost(enumeration="MemcacheVersion", tag="9")]
    pub memcache_version: i32,
    ///  Optional: User defined parameters to apply to the memcached process
    ///  on each node.
    #[prost(message, optional, tag="11")]
    pub parameters: ::core::option::Option<MemcacheParameters>,
    ///  Output only. List of Memcached nodes.
    ///  Refer to \[Node][google.cloud.memcache.v1beta2.Instance.Node\] message for more details.
    #[prost(message, repeated, tag="12")]
    pub memcache_nodes: ::prost::alloc::vec::Vec<instance::Node>,
    ///  Output only. The time the instance was created.
    #[prost(message, optional, tag="13")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The time the instance was updated.
    #[prost(message, optional, tag="14")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The state of this Memcached instance.
    #[prost(enumeration="instance::State", tag="15")]
    pub state: i32,
    ///  Output only. The full version of memcached server running on this instance.
    ///  System automatically determines the full memcached version for an instance
    ///  based on the input MemcacheVersion.
    ///  The full version format will be "memcached-1.5.16".
    #[prost(string, tag="18")]
    pub memcache_full_version: ::prost::alloc::string::String,
    ///  List of messages that describe the current state of the Memcached instance.
    #[prost(message, repeated, tag="19")]
    pub instance_messages: ::prost::alloc::vec::Vec<instance::InstanceMessage>,
    ///  Output only. Endpoint for the Discovery API.
    #[prost(string, tag="20")]
    pub discovery_endpoint: ::prost::alloc::string::String,
    ///  Output only. Returns true if there is an update waiting to be applied
    #[prost(bool, tag="21")]
    pub update_available: bool,
}
/// Nested message and enum types in `Instance`.
pub mod instance {
    ///  Configuration for a Memcached Node.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NodeConfig {
        ///  Required. Number of cpus per Memcached node.
        #[prost(int32, tag="1")]
        pub cpu_count: i32,
        ///  Required. Memory size in MiB for each Memcached node.
        #[prost(int32, tag="2")]
        pub memory_size_mb: i32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Node {
        ///  Output only. Identifier of the Memcached node. The node id does not
        ///  include project or location like the Memcached instance name.
        #[prost(string, tag="1")]
        pub node_id: ::prost::alloc::string::String,
        ///  Output only. Location (GCP Zone) for the Memcached node.
        #[prost(string, tag="2")]
        pub zone: ::prost::alloc::string::String,
        ///  Output only. Current state of the Memcached node.
        #[prost(enumeration="node::State", tag="3")]
        pub state: i32,
        ///  Output only. Hostname or IP address of the Memcached node used by the
        ///  clients to connect to the Memcached server on this node.
        #[prost(string, tag="4")]
        pub host: ::prost::alloc::string::String,
        ///  Output only. The port number of the Memcached server on this node.
        #[prost(int32, tag="5")]
        pub port: i32,
        ///  User defined parameters currently applied to the node.
        #[prost(message, optional, tag="6")]
        pub parameters: ::core::option::Option<super::MemcacheParameters>,
        ///  Output only. Returns true if there is an update waiting to be applied
        #[prost(bool, tag="7")]
        pub update_available: bool,
    }
    /// Nested message and enum types in `Node`.
    pub mod node {
        ///  Different states of a Memcached node.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum State {
            ///  Node state is not set.
            Unspecified = 0,
            ///  Node is being created.
            Creating = 1,
            ///  Node has been created and ready to be used.
            Ready = 2,
            ///  Node is being deleted.
            Deleting = 3,
            ///  Node is being updated.
            Updating = 4,
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
                    State::Updating => "UPDATING",
                }
            }
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InstanceMessage {
        ///  A code that correspond to one type of user-facing message.
        #[prost(enumeration="instance_message::Code", tag="1")]
        pub code: i32,
        ///  Message on memcached instance which will be exposed to users.
        #[prost(string, tag="2")]
        pub message: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `InstanceMessage`.
    pub mod instance_message {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Code {
            ///  Message Code not set.
            Unspecified = 0,
            ///  Memcached nodes are distributed unevenly.
            ZoneDistributionUnbalanced = 1,
        }
        impl Code {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Code::Unspecified => "CODE_UNSPECIFIED",
                    Code::ZoneDistributionUnbalanced => "ZONE_DISTRIBUTION_UNBALANCED",
                }
            }
        }
    }
    ///  Different states of a Memcached instance.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        ///  State not set.
        Unspecified = 0,
        ///  Memcached instance is being created.
        Creating = 1,
        ///  Memcached instance has been created and ready to be used.
        Ready = 2,
        ///  Memcached instance is being deleted.
        Deleting = 4,
        ///  Memcached instance is going through maintenance, e.g. data plane rollout.
        PerformingMaintenance = 5,
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
                State::PerformingMaintenance => "PERFORMING_MAINTENANCE",
            }
        }
    }
}
///  Request for \[ListInstances][google.cloud.memcache.v1beta2.CloudMemcache.ListInstances\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    ///  Required. The resource name of the instance location using the form:
    ///      `projects/{project_id}/locations/{location_id}`
    ///  where `location_id` refers to a GCP region
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  The maximum number of items to return.
    ///
    ///  If not specified, a default value of 1000 will be used by the service.
    ///  Regardless of the `page_size` value, the response may include a partial
    ///  list and a caller should only rely on response's
    ///  \[`next_page_token`][google.cloud.memcache.v1beta2.ListInstancesResponse.next_page_token\]
    ///  to determine if there are more instances left to be queried.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    ///  The `next_page_token` value returned from a previous List request, if any.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    ///  List filter. For example, exclude all Memcached instances with name as
    ///  my-instance by specifying `"name != my-instance"`.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    ///  Sort results. Supported values are "name", "name desc" or "" (unsorted).
    #[prost(string, tag="5")]
    pub order_by: ::prost::alloc::string::String,
}
///  Response for \[ListInstances][google.cloud.memcache.v1beta2.CloudMemcache.ListInstances\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    ///  A list of Memcached instances in the project in the specified location,
    ///  or across all locations.
    ///
    ///  If the `location_id` in the parent field of the request is "-", all regions
    ///  available to the project are queried, and the results aggregated.
    #[prost(message, repeated, tag="1")]
    pub resources: ::prost::alloc::vec::Vec<Instance>,
    ///  Token to retrieve the next page of results, or empty if there are no more
    ///  results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    ///  Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///  Request for \[GetInstance][google.cloud.memcache.v1beta2.CloudMemcache.GetInstance\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    ///  Required. Memcached instance resource name in the format:
    ///      `projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    ///  where `location_id` refers to a GCP region
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Request for \[CreateInstance][google.cloud.memcache.v1beta2.CloudMemcache.CreateInstance\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceRequest {
    ///  Required. The resource name of the instance location using the form:
    ///      `projects/{project_id}/locations/{location_id}`
    ///  where `location_id` refers to a GCP region
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Required. The logical name of the Memcached instance in the user
    ///  project with the following restrictions:
    ///
    ///  * Must contain only lowercase letters, numbers, and hyphens.
    ///  * Must start with a letter.
    ///  * Must be between 1-40 characters.
    ///  * Must end with a number or a letter.
    ///  * Must be unique within the user project / location.
    ///
    ///  If any of the above are not met, the API raises an invalid argument error.
    #[prost(string, tag="2")]
    pub instance_id: ::prost::alloc::string::String,
    ///  Required. A Memcached \[Instance\] resource
    #[prost(message, optional, tag="3")]
    pub resource: ::core::option::Option<Instance>,
}
///  Request for \[UpdateInstance][google.cloud.memcache.v1beta2.CloudMemcache.UpdateInstance\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceRequest {
    ///  Required. Mask of fields to update.
    ///   *  `displayName`
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    ///  Required. A Memcached \[Instance\] resource.
    ///  Only fields specified in update_mask are updated.
    #[prost(message, optional, tag="2")]
    pub resource: ::core::option::Option<Instance>,
}
///  Request for \[DeleteInstance][google.cloud.memcache.v1beta2.CloudMemcache.DeleteInstance\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    ///  Required. Memcached instance resource name in the format:
    ///      `projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    ///  where `location_id` refers to a GCP region
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Request for \[ApplyParameters][google.cloud.memcache.v1beta2.CloudMemcache.ApplyParameters\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyParametersRequest {
    ///  Required. Resource name of the Memcached instance for which parameter group updates
    ///  should be applied.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Nodes to which the instance-level parameter group is applied.
    #[prost(string, repeated, tag="2")]
    pub node_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  Whether to apply instance-level parameter group to all nodes. If set to
    ///  true, users are restricted from specifying individual nodes, and
    ///  `ApplyParameters` updates all nodes within the instance.
    #[prost(bool, tag="3")]
    pub apply_all: bool,
}
///  Request for \[UpdateParameters][google.cloud.memcache.v1beta2.CloudMemcache.UpdateParameters\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateParametersRequest {
    ///  Required. Resource name of the Memcached instance for which the parameters should be
    ///  updated.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Required. Mask of fields to update.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    ///  The parameters to apply to the instance.
    #[prost(message, optional, tag="3")]
    pub parameters: ::core::option::Option<MemcacheParameters>,
}
///  Request for \[ApplySoftwareUpdate][google.cloud.memcache.v1beta2.CloudMemcache.ApplySoftwareUpdate\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplySoftwareUpdateRequest {
    ///  Required. Resource name of the Memcached instance for which software update should be
    ///  applied.
    #[prost(string, tag="1")]
    pub instance: ::prost::alloc::string::String,
    ///  Nodes to which we should apply the update to. Note all the selected nodes
    ///  are updated in parallel.
    #[prost(string, repeated, tag="2")]
    pub node_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  Whether to apply the update to all nodes. If set to
    ///  true, will explicitly restrict users from specifying any nodes, and apply
    ///  software update to all nodes (where applicable) within the instance.
    #[prost(bool, tag="3")]
    pub apply_all: bool,
}
///  The unique ID associated with this set of parameters. Users
///  can use this id to determine if the parameters associated with the instance
///  differ from the parameters associated with the nodes. A discrepancy between
///  parameter ids can inform users that they may need to take action to apply
///  parameters on nodes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemcacheParameters {
    ///  Output only.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    ///  User defined set of parameters to use in the memcached process.
    #[prost(map="string, string", tag="3")]
    pub params: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
///  Represents the metadata of a long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    ///  Output only. Time when the operation was created.
    #[prost(message, optional, tag="1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. Time when the operation finished running.
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
///  Metadata for the given \[google.cloud.location.Location][google.cloud.location.Location\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationMetadata {
    ///  Output only. The set of available zones in the location. The map is keyed
    ///  by the lowercase ID of each zone, as defined by GCE. These keys can be
    ///  specified in the `zones` field when creating a Memcached instance.
    #[prost(map="string, message", tag="1")]
    pub available_zones: ::std::collections::HashMap<::prost::alloc::string::String, ZoneMetadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZoneMetadata {
}
///  Memcached versions supported by our service.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MemcacheVersion {
    Unspecified = 0,
    ///  Memcached 1.5 version.
    Memcache15 = 1,
}
impl MemcacheVersion {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MemcacheVersion::Unspecified => "MEMCACHE_VERSION_UNSPECIFIED",
            MemcacheVersion::Memcache15 => "MEMCACHE_1_5",
        }
    }
}
/// Generated client implementations.
pub mod cloud_memcache_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Configures and manages Cloud Memorystore for Memcached instances.
    ///
    ///
    /// The `memcache.googleapis.com` service implements the Google Cloud Memorystore
    /// for Memcached API and defines the following resource model for managing
    /// Memorystore Memcached (also called Memcached below) instances:
    /// * The service works with a collection of cloud projects, named: `/projects/*`
    /// * Each project has a collection of available locations, named: `/locations/*`
    /// * Each location has a collection of Memcached instances, named:
    /// `/instances/*`
    /// * As such, Memcached instances are resources of the form:
    ///   `/projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    ///
    /// Note that location_id must be a GCP `region`; for example:
    /// * `projects/my-memcached-project/locations/us-central1/instances/my-memcached`
    #[derive(Debug, Clone)]
    pub struct CloudMemcacheClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CloudMemcacheClient<tonic::transport::Channel> {
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
    impl<T> CloudMemcacheClient<T>
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
        ) -> CloudMemcacheClient<InterceptedService<T, F>>
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
            CloudMemcacheClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists Instances in a given location.
        pub async fn list_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstancesRequest>,
        ) -> Result<tonic::Response<super::ListInstancesResponse>, tonic::Status> {
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
                "/google.cloud.memcache.v1beta2.CloudMemcache/ListInstances",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Instance.
        pub async fn get_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceRequest>,
        ) -> Result<tonic::Response<super::Instance>, tonic::Status> {
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
                "/google.cloud.memcache.v1beta2.CloudMemcache/GetInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Instance in a given location.
        pub async fn create_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInstanceRequest>,
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
                "/google.cloud.memcache.v1beta2.CloudMemcache/CreateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an existing Instance in a given project and location.
        pub async fn update_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateInstanceRequest>,
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
                "/google.cloud.memcache.v1beta2.CloudMemcache/UpdateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the defined Memcached parameters for an existing instance.
        /// This method only stages the parameters, it must be followed by
        /// `ApplyParameters` to apply the parameters to nodes of the Memcached
        /// instance.
        pub async fn update_parameters(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateParametersRequest>,
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
                "/google.cloud.memcache.v1beta2.CloudMemcache/UpdateParameters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Instance.
        pub async fn delete_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstanceRequest>,
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
                "/google.cloud.memcache.v1beta2.CloudMemcache/DeleteInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// `ApplyParameters` restarts the set of specified nodes in order to update
        /// them to the current set of parameters for the Memcached Instance.
        pub async fn apply_parameters(
            &mut self,
            request: impl tonic::IntoRequest<super::ApplyParametersRequest>,
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
                "/google.cloud.memcache.v1beta2.CloudMemcache/ApplyParameters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates software on the selected nodes of the Instance.
        pub async fn apply_software_update(
            &mut self,
            request: impl tonic::IntoRequest<super::ApplySoftwareUpdateRequest>,
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
                "/google.cloud.memcache.v1beta2.CloudMemcache/ApplySoftwareUpdate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
