/// Orchestration cluster represents a GKE cluster with config controller and
/// TNA specific components installed on it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrchestrationCluster {
    /// Name of the orchestration cluster. The name of orchestration cluster cannot
    /// be more than 24 characters.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Management configuration of the underlying GKE cluster.
    #[prost(message, optional, tag = "5")]
    pub management_config: ::core::option::Option<ManagementConfig>,
    /// Output only. \[Output only\] Create time stamp.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. \[Output only\] Update time stamp.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels as key value pairs.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Provides the TNA version installed on the cluster.
    #[prost(string, tag = "6")]
    pub tna_version: ::prost::alloc::string::String,
    /// Output only. State of the Orchestration Cluster.
    #[prost(enumeration = "orchestration_cluster::State", tag = "7")]
    pub state: i32,
}
/// Nested message and enum types in `OrchestrationCluster`.
pub mod orchestration_cluster {
    /// Possible states that the Orchestration Cluster can be in.
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
    pub enum State {
        /// Unspecified state.
        Unspecified = 0,
        /// OrchestrationCluster is being created.
        Creating = 1,
        /// OrchestrationCluster has been created and is ready for use.
        Active = 2,
        /// OrchestrationCluster is being deleted.
        Deleting = 3,
        /// OrchestrationCluster encountered an error and is in an indeterministic
        /// state. User can still initiate a delete operation on this state.
        Failed = 4,
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
                State::Active => "ACTIVE",
                State::Deleting => "DELETING",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "ACTIVE" => Some(Self::Active),
                "DELETING" => Some(Self::Deleting),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
/// EdgeSlm represents an SLM instance which manages the lifecycle of edge
/// components installed on Workload clusters managed by an Orchestration
/// Cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EdgeSlm {
    /// Name of the EdgeSlm resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. Reference to the orchestration cluster on which templates for
    /// this resources will be applied. This should be of format
    /// projects/{project}/locations/{location}/orchestrationClusters/{orchestration_cluster}.
    #[prost(string, tag = "5")]
    pub orchestration_cluster: ::prost::alloc::string::String,
    /// Output only. \[Output only\] Create time stamp.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. \[Output only\] Update time stamp.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Labels as key value pairs. The key and value should contain
    /// characters which are UTF-8 compliant and less than 50 characters.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Provides the active TNA version for this resource.
    #[prost(string, tag = "6")]
    pub tna_version: ::prost::alloc::string::String,
    /// Output only. State of the EdgeSlm resource.
    #[prost(enumeration = "edge_slm::State", tag = "7")]
    pub state: i32,
    /// Optional. Type of workload cluster for which an EdgeSLM resource is
    /// created.
    #[prost(enumeration = "edge_slm::WorkloadClusterType", tag = "8")]
    pub workload_cluster_type: i32,
}
/// Nested message and enum types in `EdgeSlm`.
pub mod edge_slm {
    /// Possible states of the resource.
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
    pub enum State {
        /// Unspecified state.
        Unspecified = 0,
        /// EdgeSlm is being created.
        Creating = 1,
        /// EdgeSlm has been created and is ready for use.
        Active = 2,
        /// EdgeSlm is being deleted.
        Deleting = 3,
        /// EdgeSlm encountered an error and is in an indeterministic
        /// state. User can still initiate a delete operation on this state.
        Failed = 4,
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
                State::Active => "ACTIVE",
                State::Deleting => "DELETING",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "ACTIVE" => Some(Self::Active),
                "DELETING" => Some(Self::Deleting),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
    /// Workload clusters supported by TNA. New values will be added to the enum
    /// list as TNA adds supports for new workload clusters in future.
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
    pub enum WorkloadClusterType {
        /// Unspecified workload cluster.
        Unspecified = 0,
        /// Workload cluster is a GDCE cluster.
        Gdce = 1,
        /// Workload cluster is a GKE cluster.
        Gke = 2,
    }
    impl WorkloadClusterType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WorkloadClusterType::Unspecified => "WORKLOAD_CLUSTER_TYPE_UNSPECIFIED",
                WorkloadClusterType::Gdce => "GDCE",
                WorkloadClusterType::Gke => "GKE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "WORKLOAD_CLUSTER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "GDCE" => Some(Self::Gdce),
                "GKE" => Some(Self::Gke),
                _ => None,
            }
        }
    }
}
/// A Blueprint contains a collection of kubernetes resources in the form of
/// YAML files. The file contents of a blueprint are collectively known as
/// package. A blueprint can be
/// a) imported from TNA's public catalog
/// b) modified as per a user's need
/// c) proposed and approved.
/// On approval, a revision of blueprint is created which can be used to
/// create a deployment on Orchestration or Workload Cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Blueprint {
    /// The name of the blueprint. If unspecified, the name will be autogenerated
    /// from server side. Name of the blueprint must not contain `@` character.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Immutable. The revision ID of the blueprint.
    /// A new revision is committed whenever a blueprint is approved.
    #[prost(string, tag = "2")]
    pub revision_id: ::prost::alloc::string::String,
    /// Required. Immutable. The public blueprint ID from which this blueprint was
    /// created.
    #[prost(string, tag = "3")]
    pub source_blueprint: ::prost::alloc::string::String,
    /// Output only. The timestamp that the revision was created.
    #[prost(message, optional, tag = "5")]
    pub revision_create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Approval state of the blueprint (DRAFT, PROPOSED, APPROVED)
    #[prost(enumeration = "blueprint::ApprovalState", tag = "6")]
    pub approval_state: i32,
    /// Optional. Human readable name of a Blueprint.
    #[prost(string, tag = "7")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. Name of the repository where the blueprint files are stored.
    #[prost(string, tag = "8")]
    pub repository: ::prost::alloc::string::String,
    /// Optional. Files present in a blueprint.
    /// When invoking UpdateBlueprint API, only the modified files should be
    /// included in this. Files that are not included in the update of a blueprint
    /// will not be changed.
    #[prost(message, repeated, tag = "9")]
    pub files: ::prost::alloc::vec::Vec<File>,
    /// Optional. Labels are key-value attributes that can be set on a blueprint
    /// resource by the user.
    #[prost(map = "string, string", tag = "10")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Blueprint creation time.
    #[prost(message, optional, tag = "11")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the blueprint was updated.
    #[prost(message, optional, tag = "12")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Source provider is the author of a public blueprint, from
    /// which this blueprint is created.
    #[prost(string, tag = "13")]
    pub source_provider: ::prost::alloc::string::String,
    /// Output only. DeploymentLevel of a blueprint signifies where the blueprint
    /// will be applied. e.g. \[HYDRATION, SINGLE_DEPLOYMENT, MULTI_DEPLOYMENT\]
    #[prost(enumeration = "DeploymentLevel", tag = "14")]
    pub deployment_level: i32,
    /// Output only. Indicates if the deployment created from this blueprint can be
    /// rolled back.
    #[prost(bool, tag = "15")]
    pub rollback_support: bool,
}
/// Nested message and enum types in `Blueprint`.
pub mod blueprint {
    /// Approval state indicates the state of a Blueprint in its approval
    /// lifecycle.
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
    pub enum ApprovalState {
        /// Unspecified state.
        Unspecified = 0,
        /// A blueprint starts in DRAFT state once it is created. All edits are made
        /// to the blueprint in DRAFT state.
        Draft = 1,
        /// When the edits are ready for review, blueprint can be proposed and moves
        /// to PROPOSED state. Edits cannot be made to a blueprint in PROPOSED state.
        Proposed = 2,
        /// When a proposed blueprint is approved, it moves to APPROVED state. A new
        /// revision is committed. The latest committed revision can be used to
        /// create a deployment on Orchestration or Workload Cluster. Edits to an
        /// APPROVED blueprint changes its state back to DRAFT. The last committed
        /// revision of a blueprint represents its latest APPROVED state.
        Approved = 3,
    }
    impl ApprovalState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ApprovalState::Unspecified => "APPROVAL_STATE_UNSPECIFIED",
                ApprovalState::Draft => "DRAFT",
                ApprovalState::Proposed => "PROPOSED",
                ApprovalState::Approved => "APPROVED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "APPROVAL_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "DRAFT" => Some(Self::Draft),
                "PROPOSED" => Some(Self::Proposed),
                "APPROVED" => Some(Self::Approved),
                _ => None,
            }
        }
    }
}
/// A Blueprint contains a collection of kubernetes resources in the form of
/// YAML files. The file contents of a blueprint are collectively known as
/// package.
/// Public blueprint is a TNA provided blueprint that in present in TNA's public
/// catalog. A user can copy the public blueprint to their private catalog for
/// further modifications.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicBlueprint {
    /// Name of the public blueprint.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The display name of the public blueprint.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The description of the public blueprint.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// DeploymentLevel of a blueprint signifies where the blueprint will be
    /// applied. e.g. \[HYDRATION, SINGLE_DEPLOYMENT, MULTI_DEPLOYMENT\]
    #[prost(enumeration = "DeploymentLevel", tag = "4")]
    pub deployment_level: i32,
    /// Source provider is the author of a public blueprint. e.g. Google, vendors
    #[prost(string, tag = "5")]
    pub source_provider: ::prost::alloc::string::String,
    /// Output only. Indicates if the deployment created from this blueprint can be
    /// rolled back.
    #[prost(bool, tag = "15")]
    pub rollback_support: bool,
}
/// Deployment contains a collection of YAML files (This collection is also known
/// as package) that can to applied on an orchestration cluster (GKE cluster with
/// TNA addons) or a workload cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deployment {
    /// The name of the deployment.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Immutable. The revision ID of the deployment.
    /// A new revision is committed whenever a change in deployment is applied.
    #[prost(string, tag = "2")]
    pub revision_id: ::prost::alloc::string::String,
    /// Required. The blueprint revision from which this deployment was created.
    #[prost(string, tag = "3")]
    pub source_blueprint_revision: ::prost::alloc::string::String,
    /// Output only. The timestamp that the revision was created.
    #[prost(message, optional, tag = "4")]
    pub revision_create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. State of the deployment (DRAFT, APPLIED, DELETING).
    #[prost(enumeration = "deployment::State", tag = "5")]
    pub state: i32,
    /// Optional. Human readable name of a Deployment.
    #[prost(string, tag = "6")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. Name of the repository where the deployment package files are
    /// stored.
    #[prost(string, tag = "7")]
    pub repository: ::prost::alloc::string::String,
    /// Optional. Files present in a deployment.
    /// When invoking UpdateDeployment API, only the modified files should be
    /// included in this. Files that are not included in the update of a deployment
    /// will not be changed.
    #[prost(message, repeated, tag = "8")]
    pub files: ::prost::alloc::vec::Vec<File>,
    /// Optional. Labels are key-value attributes that can be set on a deployment
    /// resource by the user.
    #[prost(map = "string, string", tag = "9")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Deployment creation time.
    #[prost(message, optional, tag = "10")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the deployment was updated.
    #[prost(message, optional, tag = "11")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Source provider is the author of a public blueprint, from
    /// which this deployment is created.
    #[prost(string, tag = "12")]
    pub source_provider: ::prost::alloc::string::String,
    /// Optional. Immutable. The WorkloadCluster on which to create the Deployment.
    /// This field should only be passed when the deployment_level of the source
    /// blueprint specifies deployments on workload clusters e.g.
    /// WORKLOAD_CLUSTER_DEPLOYMENT.
    #[prost(string, tag = "13")]
    pub workload_cluster: ::prost::alloc::string::String,
    /// Output only. Attributes to where the deployment can inflict changes. The
    /// value can only be \[SINGLE_DEPLOYMENT, MULTI_DEPLOYMENT\].
    #[prost(enumeration = "DeploymentLevel", tag = "14")]
    pub deployment_level: i32,
    /// Output only. Indicates if the deployment can be rolled back, exported from
    /// public blueprint.
    #[prost(bool, tag = "15")]
    pub rollback_support: bool,
}
/// Nested message and enum types in `Deployment`.
pub mod deployment {
    /// State defines which state the current deployment is in.
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
    pub enum State {
        /// Unspecified state.
        Unspecified = 0,
        /// A deployment starts in DRAFT state. All edits are made in DRAFT state. A
        /// deployment opened for editing after applying will be in draft state,
        /// while its prevision revision will be its current applied version.
        Draft = 1,
        /// This state means that the contents (YAML files containing kubernetes
        /// resources) of the deployment have been applied to an Orchestration or
        /// Workload Cluster. A revision is created when a deployment is applied.
        /// This revision will represent the latest view of what is applied on the
        /// cluster until the deployment is modified and applied again, which will
        /// create a new revision.
        Applied = 2,
        /// A deployment in DELETING state has been marked for deletion. Its
        /// deletion status can be queried using `ComputeDeploymentStatus` API. No
        /// updates are allowed to a deployment in DELETING state.
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
                State::Draft => "DRAFT",
                State::Applied => "APPLIED",
                State::Deleting => "DELETING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "DRAFT" => Some(Self::Draft),
                "APPLIED" => Some(Self::Applied),
                "DELETING" => Some(Self::Deleting),
                _ => None,
            }
        }
    }
}
/// A collection of kubernetes yaml files which are deployed on a Workload
/// Cluster. Hydrated Deployments are created by TNA intent based automation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HydratedDeployment {
    /// Output only. The name of the hydrated deployment.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. State of the hydrated deployment (DRAFT, APPLIED).
    #[prost(enumeration = "hydrated_deployment::State", tag = "2")]
    pub state: i32,
    /// Optional. File contents of a hydrated deployment.
    /// When invoking UpdateHydratedBlueprint API, only the modified files should
    /// be included in this. Files that are not included in the update of a
    /// hydrated deployment will not be changed.
    #[prost(message, repeated, tag = "3")]
    pub files: ::prost::alloc::vec::Vec<File>,
    /// Output only. WorkloadCluster identifies which workload cluster will the
    /// hydrated deployment will be deployed on.
    #[prost(string, tag = "4")]
    pub workload_cluster: ::prost::alloc::string::String,
}
/// Nested message and enum types in `HydratedDeployment`.
pub mod hydrated_deployment {
    /// State defines which state the current hydrated deployment is in.
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
    pub enum State {
        /// Unspecified state.
        Unspecified = 0,
        /// A hydrated deployment starts in DRAFT state. All edits are made in DRAFT
        /// state.
        Draft = 1,
        /// When the edit is applied, the hydrated deployment moves to APPLIED
        /// state. No changes can be made once a hydrated deployment is applied.
        Applied = 2,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Draft => "DRAFT",
                State::Applied => "APPLIED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "DRAFT" => Some(Self::Draft),
                "APPLIED" => Some(Self::Applied),
                _ => None,
            }
        }
    }
}
/// Message for requesting list of OrchestrationClusters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrchestrationClustersRequest {
    /// Required. Parent value for ListOrchestrationClustersRequest
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing OrchestrationClusters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrchestrationClustersResponse {
    /// The list of OrchestrationCluster
    #[prost(message, repeated, tag = "1")]
    pub orchestration_clusters: ::prost::alloc::vec::Vec<OrchestrationCluster>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a OrchestrationCluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrchestrationClusterRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a OrchestrationCluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOrchestrationClusterRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Id of the requesting object
    /// If auto-generating Id server-side, remove this field and
    /// orchestration_cluster_id from the method_signature of Create RPC
    #[prost(string, tag = "2")]
    pub orchestration_cluster_id: ::prost::alloc::string::String,
    /// Required. The resource being created
    #[prost(message, optional, tag = "3")]
    pub orchestration_cluster: ::core::option::Option<OrchestrationCluster>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for deleting a OrchestrationCluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOrchestrationClusterRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for requesting list of EdgeSlms
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEdgeSlmsRequest {
    /// Required. Parent value for ListEdgeSlmsRequest
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing EdgeSlms.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEdgeSlmsResponse {
    /// The list of EdgeSlm
    #[prost(message, repeated, tag = "1")]
    pub edge_slms: ::prost::alloc::vec::Vec<EdgeSlm>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a EdgeSlm.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEdgeSlmRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a EdgeSlm.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEdgeSlmRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Id of the requesting object
    /// If auto-generating Id server-side, remove this field and
    /// edge_slm_id from the method_signature of Create RPC
    #[prost(string, tag = "2")]
    pub edge_slm_id: ::prost::alloc::string::String,
    /// Required. The resource being created
    #[prost(message, optional, tag = "3")]
    pub edge_slm: ::core::option::Option<EdgeSlm>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for deleting a EdgeSlm.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEdgeSlmRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request object for `CreateBlueprint`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBlueprintRequest {
    /// Required. The name of parent resource.
    /// Format should be -
    /// "projects/{project_id}/locations/{location_name}/orchestrationClusters/{orchestration_cluster}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The name of the blueprint.
    #[prost(string, tag = "2")]
    pub blueprint_id: ::prost::alloc::string::String,
    /// Required. The `Blueprint` to create.
    #[prost(message, optional, tag = "3")]
    pub blueprint: ::core::option::Option<Blueprint>,
}
/// Request object for `UpdateBlueprint`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBlueprintRequest {
    /// Required. The `blueprint` to update.
    #[prost(message, optional, tag = "1")]
    pub blueprint: ::core::option::Option<Blueprint>,
    /// Required. Update mask is used to specify the fields to be overwritten in
    /// the `blueprint` resource by the update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request object for `GetBlueprint`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlueprintRequest {
    /// Required. The name of the blueprint.
    /// Case 1: If the name provided in the request is
    /// {blueprint_id}@{revision_id}, then the revision with revision_id will be
    /// returned. Case 2: If the name provided in the request is {blueprint}, then
    /// the current state of the blueprint is returned.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Defines the type of view of the blueprint.
    /// When field is not present BLUEPRINT_VIEW_BASIC is considered as default.
    #[prost(enumeration = "BlueprintView", tag = "2")]
    pub view: i32,
}
/// Request object for `DeleteBlueprint`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBlueprintRequest {
    /// Required. The name of blueprint to delete.
    /// Blueprint name should be in the format {blueprint_id}, if
    /// {blueprint_id}@{revision_id} is passed then the API throws invalid
    /// argument.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for `ListBlueprints`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBlueprintsRequest {
    /// Required. The name of parent orchestration cluster resource.
    /// Format should be -
    /// "projects/{project_id}/locations/{location_name}/orchestrationClusters/{orchestration_cluster}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Filtering only supports equality on blueprint state.
    /// It should be in the form: "state = DRAFT". `OR` operator can be used to
    /// get response for multiple states. e.g. "state = DRAFT OR state = PROPOSED".
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. The maximum number of blueprints to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The page token, received from a previous ListBlueprints call.
    /// It can be provided to retrieve the subsequent page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response object for `ListBlueprints`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBlueprintsResponse {
    /// The list of requested blueprints.
    #[prost(message, repeated, tag = "1")]
    pub blueprints: ::prost::alloc::vec::Vec<Blueprint>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request object for `ApproveBlueprint`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveBlueprintRequest {
    /// Required. The name of the blueprint to approve. The blueprint must be in
    /// Proposed state. A new revision is committed on approval.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for `ProposeBlueprint`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposeBlueprintRequest {
    /// Required. The name of the blueprint being proposed.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for `RejectBlueprint`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejectBlueprintRequest {
    /// Required. The name of the blueprint being rejected.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for `ListBlueprintRevisions`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBlueprintRevisionsRequest {
    /// Required. The name of the blueprint to list revisions for.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The maximum number of revisions to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token, received from a previous ListBlueprintRevisions call
    /// It can be provided to retrieve the subsequent page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response object for `ListBlueprintRevisions`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBlueprintRevisionsResponse {
    /// The revisions of the blueprint.
    #[prost(message, repeated, tag = "1")]
    pub blueprints: ::prost::alloc::vec::Vec<Blueprint>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request object for `SearchBlueprintRevisions`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchBlueprintRevisionsRequest {
    /// Required. The name of parent orchestration cluster resource.
    /// Format should be -
    /// "projects/{project_id}/locations/{location_name}/orchestrationClusters/{orchestration_cluster}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Supported queries:
    /// 1. ""                       : Lists all revisions across all blueprints.
    /// 2. "latest=true"            : Lists latest revisions across all blueprints.
    /// 3. "name={name}"            : Lists all revisions of blueprint with name
    /// {name}.
    /// 4. "name={name} latest=true": Lists latest revision of blueprint with name
    /// {name}
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    /// Optional. The maximum number of blueprints revisions to return per page.
    /// max page size = 100, default page size = 20.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Optional. The page token, received from a previous search call.
    /// It can be provided to retrieve the subsequent page.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response object for `SearchBlueprintRevisions`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchBlueprintRevisionsResponse {
    /// The list of requested blueprint revisions.
    #[prost(message, repeated, tag = "1")]
    pub blueprints: ::prost::alloc::vec::Vec<Blueprint>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request object for `DiscardBlueprintChanges`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscardBlueprintChangesRequest {
    /// Required. The name of the blueprint of which changes are being discarded.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response object for `DiscardBlueprintChanges`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscardBlueprintChangesResponse {}
/// Request object for `ListPublicBlueprints`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPublicBlueprintsRequest {
    /// Required. Parent value of public blueprint.
    /// Format should be -
    /// "projects/{project_id}/locations/{location_name}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer items than
    /// requested. If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response object for `ListPublicBlueprints`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPublicBlueprintsResponse {
    /// The list of public blueprints to return.
    #[prost(message, repeated, tag = "1")]
    pub public_blueprints: ::prost::alloc::vec::Vec<PublicBlueprint>,
    /// Output only. A token identifying a page of results the server should
    /// return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request object for `GetPublicBlueprint`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPublicBlueprintRequest {
    /// Required. The name of the public blueprint.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for `CreateDeployment`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDeploymentRequest {
    /// Required. The name of parent resource.
    /// Format should be -
    /// "projects/{project_id}/locations/{location_name}/orchestrationClusters/{orchestration_cluster}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The name of the deployment.
    #[prost(string, tag = "2")]
    pub deployment_id: ::prost::alloc::string::String,
    /// Required. The `Deployment` to create.
    #[prost(message, optional, tag = "3")]
    pub deployment: ::core::option::Option<Deployment>,
}
/// Request object for `UpdateDeployment`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDeploymentRequest {
    /// Required. The `deployment` to update.
    #[prost(message, optional, tag = "1")]
    pub deployment: ::core::option::Option<Deployment>,
    /// Required. Update mask is used to specify the fields to be overwritten in
    /// the `deployment` resource by the update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request object for `GetDeployment`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeploymentRequest {
    /// Required. The name of the deployment.
    /// Case 1: If the name provided in the request is
    /// {deployment_id}@{revision_id}, then the revision with revision_id will be
    /// returned.
    /// Case 2: If the name provided in the request is {deployment}, then
    /// the current state of the deployment is returned.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Defines the type of view of the deployment.
    /// When field is not present VIEW_BASIC is considered as default.
    #[prost(enumeration = "DeploymentView", tag = "2")]
    pub view: i32,
}
/// Request object for `RemoveDeployment`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDeploymentRequest {
    /// Required. The name of deployment to initiate delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for `ListDeployments`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeploymentsRequest {
    /// Required. The name of parent orchestration cluster resource.
    /// Format should be -
    /// "projects/{project_id}/locations/{location_name}/orchestrationClusters/{orchestration_cluster}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Filtering only supports equality on deployment state.
    /// It should be in the form: "state = DRAFT". `OR` operator can be used to
    /// get response for multiple states. e.g. "state = DRAFT OR state = APPLIED".
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. The maximum number of deployments to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The page token, received from a previous ListDeployments call.
    /// It can be provided to retrieve the subsequent page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response object for `ListDeployments`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeploymentsResponse {
    /// The list of requested deployments.
    #[prost(message, repeated, tag = "1")]
    pub deployments: ::prost::alloc::vec::Vec<Deployment>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for listing all revisions of a deployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeploymentRevisionsRequest {
    /// Required. The name of the deployment to list revisions for.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The maximum number of revisions to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The page token, received from a previous ListDeploymentRevisions
    /// call Provide this to retrieve the subsequent page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// List of deployment revisions for a given deployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeploymentRevisionsResponse {
    /// The revisions of the deployment.
    #[prost(message, repeated, tag = "1")]
    pub deployments: ::prost::alloc::vec::Vec<Deployment>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request object for `SearchDeploymentRevisions`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchDeploymentRevisionsRequest {
    /// Required. The name of parent orchestration cluster resource.
    /// Format should be -
    /// "projects/{project_id}/locations/{location_name}/orchestrationClusters/{orchestration_cluster}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Supported queries:
    /// 1. ""                       : Lists all revisions across all deployments.
    /// 2. "latest=true"            : Lists latest revisions across all
    /// deployments.
    /// 3. "name={name}"            : Lists all revisions of deployment with name
    /// {name}.
    /// 4. "name={name} latest=true": Lists latest revision of deployment with name
    /// {name}
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    /// Optional. The maximum number of deployment revisions to return per page.
    /// max page size = 100, default page size = 20.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Optional. The page token, received from a previous search call.
    /// It can be provided to retrieve the subsequent page.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response object for `SearchDeploymentRevisions`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchDeploymentRevisionsResponse {
    /// The list of requested deployment revisions.
    #[prost(message, repeated, tag = "1")]
    pub deployments: ::prost::alloc::vec::Vec<Deployment>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request object for `DiscardDeploymentChanges`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscardDeploymentChangesRequest {
    /// Required. The name of the deployment of which changes are being discarded.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response object for `DiscardDeploymentChanges`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscardDeploymentChangesResponse {}
/// Request object for `ApplyDeployment`. The resources in given deployment
/// gets applied to Orchestration Cluster. A new revision is created when a
/// deployment is applied.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyDeploymentRequest {
    /// Required. The name of the deployment to apply to orchestration cluster.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for `ComputeDeploymentStatus`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeDeploymentStatusRequest {
    /// Required. The name of the deployment without revisionID.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response object for `ComputeDeploymentStatus`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeDeploymentStatusResponse {
    /// The name of the deployment.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Aggregated status of a deployment.
    #[prost(enumeration = "Status", tag = "2")]
    pub aggregated_status: i32,
    /// Output only. Resource level status details in deployments.
    #[prost(message, repeated, tag = "3")]
    pub resource_statuses: ::prost::alloc::vec::Vec<ResourceStatus>,
}
/// Request object for `RollbackDeployment`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackDeploymentRequest {
    /// Required. Name of the deployment.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The revision id of deployment to roll back to.
    #[prost(string, tag = "2")]
    pub revision_id: ::prost::alloc::string::String,
}
/// Represents the metadata of the long-running operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have been cancelled successfully
    /// have [Operation.error][] value with a
    /// [google.rpc.Status.code][google.rpc.Status.code] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Request object for `GetHydratedDeployment`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHydratedDeploymentRequest {
    /// Required. Name of the hydrated deployment.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for `ListHydratedDeployments`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHydratedDeploymentsRequest {
    /// Required. The deployment managing the hydrated deployments.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of hydrated deployments to return. The service
    /// may return fewer than this value. If unspecified, at most 50 hydrated
    /// deployments will be returned. The maximum value is 1000. Values above 1000
    /// will be set to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The page token, received from a previous ListHydratedDeployments
    /// call. Provide this to retrieve the subsequent page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response object for `ListHydratedDeployments`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHydratedDeploymentsResponse {
    /// The list of hydrated deployments.
    #[prost(message, repeated, tag = "1")]
    pub hydrated_deployments: ::prost::alloc::vec::Vec<HydratedDeployment>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request object for `UpdateHydratedDeployment`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateHydratedDeploymentRequest {
    /// Required. The hydrated deployment to update.
    #[prost(message, optional, tag = "1")]
    pub hydrated_deployment: ::core::option::Option<HydratedDeployment>,
    /// Required. The list of fields to update. Update mask supports a special
    /// value `*` which fully replaces (equivalent to PUT) the resource provided.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request for applying a hydrated deployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyHydratedDeploymentRequest {
    /// Required. The name of the hydrated deployment to apply.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Configuration of the cluster management
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagementConfig {
    /// The configuration can be one of StandardManagementConfig
    /// and FullManagementConfig
    #[prost(oneof = "management_config::OneofConfig", tags = "1, 2")]
    pub oneof_config: ::core::option::Option<management_config::OneofConfig>,
}
/// Nested message and enum types in `ManagementConfig`.
pub mod management_config {
    /// The configuration can be one of StandardManagementConfig
    /// and FullManagementConfig
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneofConfig {
        /// Configuration of the standard (GKE) cluster management
        #[prost(message, tag = "1")]
        StandardManagementConfig(super::StandardManagementConfig),
        /// Configuration of the full (Autopilot) cluster management. Full cluster
        /// management is a preview feature.
        #[prost(message, tag = "2")]
        FullManagementConfig(super::FullManagementConfig),
    }
}
/// Configuration of the standard (GKE) cluster management.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardManagementConfig {
    /// Optional. Name of the VPC Network to put the GKE cluster and nodes in. The
    /// VPC will be created if it doesn't exist.
    #[prost(string, tag = "1")]
    pub network: ::prost::alloc::string::String,
    /// Optional. Specifies the subnet that the interface will be part of. Network
    /// key must be specified and the subnet must be a subnetwork of the specified
    /// network.
    #[prost(string, tag = "2")]
    pub subnet: ::prost::alloc::string::String,
    /// Optional. The /28 network that the masters will use. It should be free
    /// within the network.
    #[prost(string, tag = "3")]
    pub master_ipv4_cidr_block: ::prost::alloc::string::String,
    /// Optional. The IP address range for the cluster pod IPs. Set to blank to
    /// have a range chosen with the default size. Set to /netmask (e.g. /14) to
    /// have a range chosen with a specific netmask. Set to a CIDR notation
    /// (e.g. 10.96.0.0/14) from the RFC-1918 private networks (e.g. 10.0.0.0/8,
    /// 172.16.0.0/12, 192.168.0.0/16) to pick a specific range to use.
    #[prost(string, tag = "4")]
    pub cluster_cidr_block: ::prost::alloc::string::String,
    /// Optional. The IP address range for the cluster service IPs. Set to blank to
    /// have a range chosen with the default size. Set to /netmask (e.g. /14) to
    /// have a range chosen with a specific netmask. Set to a CIDR notation (e.g.
    /// 10.96.0.0/14) from the RFC-1918 private networks (e.g. 10.0.0.0/8,
    /// 172.16.0.0/12, 192.168.0.0/16) to pick a specific range to use.
    #[prost(string, tag = "5")]
    pub services_cidr_block: ::prost::alloc::string::String,
    /// Optional. The name of the existing secondary range in the cluster's
    /// subnetwork to use for pod IP addresses. Alternatively, cluster_cidr_block
    /// can be used to automatically create a GKE-managed one.
    #[prost(string, tag = "6")]
    pub cluster_named_range: ::prost::alloc::string::String,
    /// Optional. The name of the existing secondary range in the cluster's
    /// subnetwork to use for service ClusterIPs. Alternatively,
    /// services_cidr_block can be used to automatically create a GKE-managed one.
    #[prost(string, tag = "7")]
    pub services_named_range: ::prost::alloc::string::String,
    /// Optional. Master Authorized Network that supports multiple CIDR blocks.
    /// Allows access to the k8s master from multiple blocks. It cannot be set at
    /// the same time with the field man_block.
    #[prost(message, optional, tag = "8")]
    pub master_authorized_networks_config: ::core::option::Option<
        MasterAuthorizedNetworksConfig,
    >,
}
/// Configuration of the full (Autopilot) cluster management
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FullManagementConfig {
    /// Optional. Name of the VPC Network to put the GKE cluster and nodes in. The
    /// VPC will be created if it doesn't exist.
    #[prost(string, tag = "1")]
    pub network: ::prost::alloc::string::String,
    /// Optional. Specifies the subnet that the interface will be part of. Network
    /// key must be specified and the subnet must be a subnetwork of the specified
    /// network.
    #[prost(string, tag = "2")]
    pub subnet: ::prost::alloc::string::String,
    /// Optional. The /28 network that the masters will use.
    #[prost(string, tag = "3")]
    pub master_ipv4_cidr_block: ::prost::alloc::string::String,
    /// Optional. The IP address range for the cluster pod IPs. Set to blank to
    /// have a range chosen with the default size. Set to /netmask (e.g. /14) to
    /// have a range chosen with a specific netmask. Set to a CIDR notation
    /// (e.g. 10.96.0.0/14) from the RFC-1918 private networks (e.g. 10.0.0.0/8,
    /// 172.16.0.0/12, 192.168.0.0/16) to pick a specific range to use.
    #[prost(string, tag = "4")]
    pub cluster_cidr_block: ::prost::alloc::string::String,
    /// Optional. The IP address range for the cluster service IPs. Set to blank to
    /// have a range chosen with the default size. Set to /netmask (e.g. /14) to
    /// have a range chosen with a specific netmask. Set to a CIDR notation (e.g.
    /// 10.96.0.0/14) from the RFC-1918 private networks (e.g. 10.0.0.0/8,
    /// 172.16.0.0/12, 192.168.0.0/16) to pick a specific range to use.
    #[prost(string, tag = "5")]
    pub services_cidr_block: ::prost::alloc::string::String,
    /// Optional. The name of the existing secondary range in the cluster's
    /// subnetwork to use for pod IP addresses. Alternatively, cluster_cidr_block
    /// can be used to automatically create a GKE-managed one.
    #[prost(string, tag = "6")]
    pub cluster_named_range: ::prost::alloc::string::String,
    /// Optional. The name of the existing secondary range in the cluster's
    /// subnetwork to use for service ClusterIPs. Alternatively,
    /// services_cidr_block can be used to automatically create a GKE-managed one.
    #[prost(string, tag = "7")]
    pub services_named_range: ::prost::alloc::string::String,
    /// Optional. Master Authorized Network that supports multiple CIDR blocks.
    /// Allows access to the k8s master from multiple blocks. It cannot be set at
    /// the same time with the field man_block.
    #[prost(message, optional, tag = "8")]
    pub master_authorized_networks_config: ::core::option::Option<
        MasterAuthorizedNetworksConfig,
    >,
}
/// Configuration of the Master Authorized Network that support multiple CIDRs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterAuthorizedNetworksConfig {
    /// Optional. cidr_blocks define up to 50 external networks that could access
    /// Kubernetes master through HTTPS.
    #[prost(message, repeated, tag = "1")]
    pub cidr_blocks: ::prost::alloc::vec::Vec<
        master_authorized_networks_config::CidrBlock,
    >,
}
/// Nested message and enum types in `MasterAuthorizedNetworksConfig`.
pub mod master_authorized_networks_config {
    /// CidrBlock contains an optional name and one CIDR block.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CidrBlock {
        /// Optional. display_name is an optional field for users to identify CIDR
        /// blocks.
        #[prost(string, tag = "1")]
        pub display_name: ::prost::alloc::string::String,
        /// Optional. cidr_block must be specified in CIDR notation when using
        /// master_authorized_networks_config. Currently, the user could still use
        /// the deprecated man_block field, so this field is currently optional, but
        /// will be required in the future.
        #[prost(string, tag = "2")]
        pub cidr_block: ::prost::alloc::string::String,
    }
}
/// File represents a yaml file present in a blueprint's package.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct File {
    /// Required. Path of the file in package.
    /// e.g. `gdce/v1/cluster.yaml`
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// Optional. The contents of a file in string format.
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
    /// Optional. Signifies whether a file is marked for deletion.
    #[prost(bool, tag = "3")]
    pub deleted: bool,
    /// Optional. Indicates whether changes are allowed to a file. If the field is
    /// not set, the file cannot be edited.
    #[prost(bool, tag = "4")]
    pub editable: bool,
}
/// Status of a deployment resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceStatus {
    /// Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Namespace of the resource.
    #[prost(string, tag = "2")]
    pub resource_namespace: ::prost::alloc::string::String,
    /// Group to which the resource belongs to.
    #[prost(string, tag = "3")]
    pub group: ::prost::alloc::string::String,
    /// Version of the resource.
    #[prost(string, tag = "4")]
    pub version: ::prost::alloc::string::String,
    /// Kind of the resource.
    #[prost(string, tag = "5")]
    pub kind: ::prost::alloc::string::String,
    /// Output only. Resource type.
    #[prost(enumeration = "ResourceType", tag = "6")]
    pub resource_type: i32,
    /// Output only. Status of the resource.
    #[prost(enumeration = "Status", tag = "7")]
    pub status: i32,
    /// Output only. Detailed status of NFDeploy.
    #[prost(message, optional, tag = "8")]
    pub nf_deploy_status: ::core::option::Option<NfDeployStatus>,
}
/// Deployment status of NFDeploy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NfDeployStatus {
    /// Output only. Total number of NFs targeted by this deployment
    #[prost(int32, tag = "1")]
    pub targeted_nfs: i32,
    /// Output only. Total number of NFs targeted by this deployment with a Ready
    /// Condition set.
    #[prost(int32, tag = "2")]
    pub ready_nfs: i32,
    /// Output only. Per-Site Status.
    #[prost(message, repeated, tag = "3")]
    pub sites: ::prost::alloc::vec::Vec<NfDeploySiteStatus>,
}
/// Per-Site Status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NfDeploySiteStatus {
    /// Output only. Site id.
    #[prost(string, tag = "1")]
    pub site: ::prost::alloc::string::String,
    /// Output only. If true, the Site Deletion is in progress.
    #[prost(bool, tag = "2")]
    pub pending_deletion: bool,
    /// Output only. Hydration status.
    #[prost(message, optional, tag = "3")]
    pub hydration: ::core::option::Option<HydrationStatus>,
    /// Output only. Workload status.
    #[prost(message, optional, tag = "4")]
    pub workload: ::core::option::Option<WorkloadStatus>,
}
/// Hydration status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HydrationStatus {
    /// Output only. SiteVersion Hydration is targeting.
    #[prost(message, optional, tag = "1")]
    pub site_version: ::core::option::Option<SiteVersion>,
    /// Output only. Status.
    #[prost(string, tag = "2")]
    pub status: ::prost::alloc::string::String,
}
/// SiteVersion Hydration is targeting.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SiteVersion {
    /// Output only. NF vendor.
    #[prost(string, tag = "1")]
    pub nf_vendor: ::prost::alloc::string::String,
    /// Output only. NF vendor type.
    #[prost(string, tag = "2")]
    pub nf_type: ::prost::alloc::string::String,
    /// Output only. NF version.
    #[prost(string, tag = "3")]
    pub nf_version: ::prost::alloc::string::String,
}
/// Workload status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkloadStatus {
    /// Output only. SiteVersion running in the workload cluster.
    #[prost(message, optional, tag = "1")]
    pub site_version: ::core::option::Option<SiteVersion>,
    /// Output only. Status.
    #[prost(string, tag = "2")]
    pub status: ::prost::alloc::string::String,
}
/// BlueprintView defines the type of view of the blueprint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BlueprintView {
    /// Unspecified enum value.
    Unspecified = 0,
    /// View which only contains metadata.
    Basic = 1,
    /// View which contains metadata and files it encapsulates.
    Full = 2,
}
impl BlueprintView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BlueprintView::Unspecified => "BLUEPRINT_VIEW_UNSPECIFIED",
            BlueprintView::Basic => "BLUEPRINT_VIEW_BASIC",
            BlueprintView::Full => "BLUEPRINT_VIEW_FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BLUEPRINT_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "BLUEPRINT_VIEW_BASIC" => Some(Self::Basic),
            "BLUEPRINT_VIEW_FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// DeploymentView defines the type of view of the deployment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeploymentView {
    /// Unspecified enum value.
    Unspecified = 0,
    /// View which only contains metadata.
    Basic = 1,
    /// View which contains metadata and files it encapsulates.
    Full = 2,
}
impl DeploymentView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DeploymentView::Unspecified => "DEPLOYMENT_VIEW_UNSPECIFIED",
            DeploymentView::Basic => "DEPLOYMENT_VIEW_BASIC",
            DeploymentView::Full => "DEPLOYMENT_VIEW_FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEPLOYMENT_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "DEPLOYMENT_VIEW_BASIC" => Some(Self::Basic),
            "DEPLOYMENT_VIEW_FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// Represent type of CR.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResourceType {
    /// Unspecified resource type.
    Unspecified = 0,
    /// User specified NF Deploy CR.
    NfDeployResource = 1,
    /// CRs that are part of a blueprint.
    DeploymentResource = 2,
}
impl ResourceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResourceType::Unspecified => "RESOURCE_TYPE_UNSPECIFIED",
            ResourceType::NfDeployResource => "NF_DEPLOY_RESOURCE",
            ResourceType::DeploymentResource => "DEPLOYMENT_RESOURCE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESOURCE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "NF_DEPLOY_RESOURCE" => Some(Self::NfDeployResource),
            "DEPLOYMENT_RESOURCE" => Some(Self::DeploymentResource),
            _ => None,
        }
    }
}
/// Status of an entity (resource, deployment).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    /// Unknown state.
    Unspecified = 0,
    /// Under progress.
    InProgress = 1,
    /// Running and ready to serve traffic.
    Active = 2,
    /// Failed or stalled.
    Failed = 3,
    /// Delete in progress.
    Deleting = 4,
    /// Deleted deployment.
    Deleted = 5,
    /// NFDeploy specific status. Peering in progress.
    Peering = 10,
    /// K8s objects such as NetworkAttachmentDefinition don't have a defined
    /// status.
    NotApplicable = 11,
}
impl Status {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Status::Unspecified => "STATUS_UNSPECIFIED",
            Status::InProgress => "STATUS_IN_PROGRESS",
            Status::Active => "STATUS_ACTIVE",
            Status::Failed => "STATUS_FAILED",
            Status::Deleting => "STATUS_DELETING",
            Status::Deleted => "STATUS_DELETED",
            Status::Peering => "STATUS_PEERING",
            Status::NotApplicable => "STATUS_NOT_APPLICABLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "STATUS_IN_PROGRESS" => Some(Self::InProgress),
            "STATUS_ACTIVE" => Some(Self::Active),
            "STATUS_FAILED" => Some(Self::Failed),
            "STATUS_DELETING" => Some(Self::Deleting),
            "STATUS_DELETED" => Some(Self::Deleted),
            "STATUS_PEERING" => Some(Self::Peering),
            "STATUS_NOT_APPLICABLE" => Some(Self::NotApplicable),
            _ => None,
        }
    }
}
/// DeploymentLevel of a blueprint signifies where the blueprint will be
/// applied.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeploymentLevel {
    /// Default unspecified deployment level.
    Unspecified = 0,
    /// Blueprints at HYDRATION level cannot be used to create a Deployment
    /// (A user cannot manually initate deployment of these blueprints on
    /// orchestration or workload cluster).
    /// These blueprints stay in a user's private catalog and are configured and
    /// deployed by TNA automation.
    Hydration = 1,
    /// Blueprints at SINGLE_DEPLOYMENT level can be
    /// a) Modified in private catalog.
    /// b) Used to create a deployment on orchestration cluster by the user, once
    /// approved.
    SingleDeployment = 2,
    /// Blueprints at MULTI_DEPLOYMENT level can be
    /// a) Modified in private catalog.
    /// b) Used to create a deployment on orchestration cluster which will create
    /// further hydrated deployments.
    MultiDeployment = 3,
    /// Blueprints at WORKLOAD_CLUSTER_DEPLOYMENT level can be
    /// a) Modified in private catalog.
    /// b) Used to create a deployment on workload cluster by the user, once
    /// approved.
    WorkloadClusterDeployment = 4,
}
impl DeploymentLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DeploymentLevel::Unspecified => "DEPLOYMENT_LEVEL_UNSPECIFIED",
            DeploymentLevel::Hydration => "HYDRATION",
            DeploymentLevel::SingleDeployment => "SINGLE_DEPLOYMENT",
            DeploymentLevel::MultiDeployment => "MULTI_DEPLOYMENT",
            DeploymentLevel::WorkloadClusterDeployment => "WORKLOAD_CLUSTER_DEPLOYMENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEPLOYMENT_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
            "HYDRATION" => Some(Self::Hydration),
            "SINGLE_DEPLOYMENT" => Some(Self::SingleDeployment),
            "MULTI_DEPLOYMENT" => Some(Self::MultiDeployment),
            "WORKLOAD_CLUSTER_DEPLOYMENT" => Some(Self::WorkloadClusterDeployment),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod telco_automation_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// TelcoAutomation Service manages the control plane cluster a.k.a.
    /// Orchestration Cluster (GKE cluster with config controller) of TNA. It also
    /// exposes blueprint APIs which manages the lifecycle of blueprints that control
    /// the infrastructure setup (e.g GDCE clusters) and deployment of network
    /// functions.
    #[derive(Debug, Clone)]
    pub struct TelcoAutomationClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TelcoAutomationClient<tonic::transport::Channel> {
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
    impl<T> TelcoAutomationClient<T>
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
        ) -> TelcoAutomationClient<InterceptedService<T, F>>
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
            TelcoAutomationClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists OrchestrationClusters in a given project and location.
        pub async fn list_orchestration_clusters(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrchestrationClustersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrchestrationClustersResponse>,
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/ListOrchestrationClusters",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "ListOrchestrationClusters",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single OrchestrationCluster.
        pub async fn get_orchestration_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrchestrationClusterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OrchestrationCluster>,
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/GetOrchestrationCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "GetOrchestrationCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new OrchestrationCluster in a given project and location.
        pub async fn create_orchestration_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateOrchestrationClusterRequest>,
        ) -> std::result::Result<
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/CreateOrchestrationCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "CreateOrchestrationCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single OrchestrationCluster.
        pub async fn delete_orchestration_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteOrchestrationClusterRequest>,
        ) -> std::result::Result<
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/DeleteOrchestrationCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "DeleteOrchestrationCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists EdgeSlms in a given project and location.
        pub async fn list_edge_slms(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEdgeSlmsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListEdgeSlmsResponse>,
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/ListEdgeSlms",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "ListEdgeSlms",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single EdgeSlm.
        pub async fn get_edge_slm(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEdgeSlmRequest>,
        ) -> std::result::Result<tonic::Response<super::EdgeSlm>, tonic::Status> {
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/GetEdgeSlm",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "GetEdgeSlm",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new EdgeSlm in a given project and location.
        pub async fn create_edge_slm(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEdgeSlmRequest>,
        ) -> std::result::Result<
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/CreateEdgeSlm",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "CreateEdgeSlm",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single EdgeSlm.
        pub async fn delete_edge_slm(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEdgeSlmRequest>,
        ) -> std::result::Result<
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/DeleteEdgeSlm",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "DeleteEdgeSlm",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a blueprint.
        pub async fn create_blueprint(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBlueprintRequest>,
        ) -> std::result::Result<tonic::Response<super::Blueprint>, tonic::Status> {
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/CreateBlueprint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "CreateBlueprint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a blueprint.
        pub async fn update_blueprint(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBlueprintRequest>,
        ) -> std::result::Result<tonic::Response<super::Blueprint>, tonic::Status> {
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/UpdateBlueprint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "UpdateBlueprint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the requested blueprint.
        pub async fn get_blueprint(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlueprintRequest>,
        ) -> std::result::Result<tonic::Response<super::Blueprint>, tonic::Status> {
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/GetBlueprint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "GetBlueprint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a blueprint and all its revisions.
        pub async fn delete_blueprint(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBlueprintRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/DeleteBlueprint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "DeleteBlueprint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List all blueprints.
        pub async fn list_blueprints(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBlueprintsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBlueprintsResponse>,
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/ListBlueprints",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "ListBlueprints",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Approves a blueprint and commits a new revision.
        pub async fn approve_blueprint(
            &mut self,
            request: impl tonic::IntoRequest<super::ApproveBlueprintRequest>,
        ) -> std::result::Result<tonic::Response<super::Blueprint>, tonic::Status> {
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/ApproveBlueprint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "ApproveBlueprint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Proposes a blueprint for approval of changes.
        pub async fn propose_blueprint(
            &mut self,
            request: impl tonic::IntoRequest<super::ProposeBlueprintRequest>,
        ) -> std::result::Result<tonic::Response<super::Blueprint>, tonic::Status> {
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/ProposeBlueprint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "ProposeBlueprint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Rejects a blueprint revision proposal and flips it back to Draft state.
        pub async fn reject_blueprint(
            &mut self,
            request: impl tonic::IntoRequest<super::RejectBlueprintRequest>,
        ) -> std::result::Result<tonic::Response<super::Blueprint>, tonic::Status> {
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/RejectBlueprint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "RejectBlueprint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List blueprint revisions of a given blueprint.
        pub async fn list_blueprint_revisions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBlueprintRevisionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBlueprintRevisionsResponse>,
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/ListBlueprintRevisions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "ListBlueprintRevisions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Searches across blueprint revisions.
        pub async fn search_blueprint_revisions(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchBlueprintRevisionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchBlueprintRevisionsResponse>,
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/SearchBlueprintRevisions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "SearchBlueprintRevisions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Searches across deployment revisions.
        pub async fn search_deployment_revisions(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchDeploymentRevisionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchDeploymentRevisionsResponse>,
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/SearchDeploymentRevisions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "SearchDeploymentRevisions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Discards the changes in a blueprint and reverts the blueprint to the last
        /// approved blueprint revision. No changes take place if a blueprint does not
        /// have revisions.
        pub async fn discard_blueprint_changes(
            &mut self,
            request: impl tonic::IntoRequest<super::DiscardBlueprintChangesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DiscardBlueprintChangesResponse>,
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/DiscardBlueprintChanges",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "DiscardBlueprintChanges",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists the blueprints in TNA's public catalog. Default page size = 20,
        /// Max Page Size = 100.
        pub async fn list_public_blueprints(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPublicBlueprintsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPublicBlueprintsResponse>,
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/ListPublicBlueprints",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "ListPublicBlueprints",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the requested public blueprint.
        pub async fn get_public_blueprint(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPublicBlueprintRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PublicBlueprint>,
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/GetPublicBlueprint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "GetPublicBlueprint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a deployment.
        pub async fn create_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDeploymentRequest>,
        ) -> std::result::Result<tonic::Response<super::Deployment>, tonic::Status> {
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/CreateDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "CreateDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a deployment.
        pub async fn update_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDeploymentRequest>,
        ) -> std::result::Result<tonic::Response<super::Deployment>, tonic::Status> {
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/UpdateDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "UpdateDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the requested deployment.
        pub async fn get_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDeploymentRequest>,
        ) -> std::result::Result<tonic::Response<super::Deployment>, tonic::Status> {
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/GetDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "GetDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Removes the deployment by marking it as DELETING. Post which deployment and
        /// it's revisions gets deleted.
        pub async fn remove_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveDeploymentRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/RemoveDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "RemoveDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List all deployments.
        pub async fn list_deployments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDeploymentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDeploymentsResponse>,
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/ListDeployments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "ListDeployments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List deployment revisions of a given deployment.
        pub async fn list_deployment_revisions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDeploymentRevisionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDeploymentRevisionsResponse>,
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/ListDeploymentRevisions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "ListDeploymentRevisions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Discards the changes in a deployment and reverts the deployment to the last
        /// approved deployment revision. No changes take place if a deployment does
        /// not have revisions.
        pub async fn discard_deployment_changes(
            &mut self,
            request: impl tonic::IntoRequest<super::DiscardDeploymentChangesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DiscardDeploymentChangesResponse>,
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/DiscardDeploymentChanges",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "DiscardDeploymentChanges",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Applies the deployment's YAML files to the parent orchestration cluster.
        pub async fn apply_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::ApplyDeploymentRequest>,
        ) -> std::result::Result<tonic::Response<super::Deployment>, tonic::Status> {
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/ApplyDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "ApplyDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the requested deployment status.
        pub async fn compute_deployment_status(
            &mut self,
            request: impl tonic::IntoRequest<super::ComputeDeploymentStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ComputeDeploymentStatusResponse>,
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/ComputeDeploymentStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "ComputeDeploymentStatus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Rollback the active deployment to the given past approved deployment
        /// revision.
        pub async fn rollback_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::RollbackDeploymentRequest>,
        ) -> std::result::Result<tonic::Response<super::Deployment>, tonic::Status> {
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/RollbackDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "RollbackDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the requested hydrated deployment.
        pub async fn get_hydrated_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetHydratedDeploymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::HydratedDeployment>,
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/GetHydratedDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "GetHydratedDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List all hydrated deployments present under a deployment.
        pub async fn list_hydrated_deployments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListHydratedDeploymentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListHydratedDeploymentsResponse>,
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/ListHydratedDeployments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "ListHydratedDeployments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a hydrated deployment.
        pub async fn update_hydrated_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateHydratedDeploymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::HydratedDeployment>,
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/UpdateHydratedDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "UpdateHydratedDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Applies a hydrated deployment to a workload cluster.
        pub async fn apply_hydrated_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::ApplyHydratedDeploymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::HydratedDeployment>,
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
                "/google.cloud.telcoautomation.v1.TelcoAutomation/ApplyHydratedDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.telcoautomation.v1.TelcoAutomation",
                        "ApplyHydratedDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
