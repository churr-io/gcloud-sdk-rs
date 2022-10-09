/// Request for creating a workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkloadRequest {
    /// Required. The resource name of the new Workload's parent.
    /// Must be of the form `organizations/{org_id}/locations/{location_id}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Assured Workload to create
    #[prost(message, optional, tag="2")]
    pub workload: ::core::option::Option<Workload>,
    /// Optional. A identifier associated with the workload and underlying projects which
    /// allows for the break down of billing costs for a workload. The value
    /// provided for the identifier will add a label to the workload and contained
    /// projects with the identifier as the value.
    #[prost(string, tag="3")]
    pub external_id: ::prost::alloc::string::String,
}
/// Request for Updating a workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorkloadRequest {
    /// Required. The workload to update.
    /// The workload's `name` field is used to identify the workload to be updated.
    /// Format:
    /// organizations/{org_id}/locations/{location_id}/workloads/{workload_id}
    #[prost(message, optional, tag="1")]
    pub workload: ::core::option::Option<Workload>,
    /// Required. The list of fields to be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request for deleting a Workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkloadRequest {
    /// Required. The `name` field is used to identify the workload.
    /// Format:
    /// organizations/{org_id}/locations/{location_id}/workloads/{workload_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The etag of the workload.
    /// If this is provided, it must match the server's etag.
    #[prost(string, tag="2")]
    pub etag: ::prost::alloc::string::String,
}
/// Request for fetching a workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkloadRequest {
    /// Required. The resource name of the Workload to fetch. This is the workloads's
    /// relative path in the API, formatted as
    /// "organizations/{organization_id}/locations/{location_id}/workloads/{workload_id}".
    /// For example,
    /// "organizations/123/locations/us-east1/workloads/assured-workload-1".
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for fetching workloads in an organization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkloadsRequest {
    /// Required. Parent Resource to list workloads from.
    /// Must be of the form `organizations/{org_id}/locations/{location}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Page size.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Page token returned from previous request. Page token contains context from
    /// previous request. Page token needs to be passed in the second and following
    /// requests.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// A custom filter for filtering by properties of a workload. At this time,
    /// only filtering by labels is supported.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response of ListWorkloads endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkloadsResponse {
    /// List of Workloads under a given parent.
    #[prost(message, repeated, tag="1")]
    pub workloads: ::prost::alloc::vec::Vec<Workload>,
    /// The next page token. Return empty if reached the last page.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// An Workload object for managing highly regulated workloads of cloud
/// customers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Workload {
    /// Optional. The resource name of the workload.
    /// Format:
    /// organizations/{organization}/locations/{location}/workloads/{workload}
    ///
    /// Read-only.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The user-assigned display name of the Workload.
    /// When present it must be between 4 to 30 characters.
    /// Allowed characters are: lowercase and uppercase letters, numbers,
    /// hyphen, and spaces.
    ///
    /// Example: My Workload
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The resources associated with this workload.
    /// These resources will be created when creating the workload.
    /// If any of the projects already exist, the workload creation will fail.
    /// Always read only.
    #[prost(message, repeated, tag="3")]
    pub resources: ::prost::alloc::vec::Vec<workload::ResourceInfo>,
    /// Required. Immutable. Compliance Regime associated with this workload.
    #[prost(enumeration="workload::ComplianceRegime", tag="4")]
    pub compliance_regime: i32,
    /// Output only. Immutable. The Workload creation timestamp.
    #[prost(message, optional, tag="5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The billing account used for the resources which are
    /// direct children of workload. This billing account is initially associated
    /// with the resources created as part of Workload creation.
    /// After the initial creation of these resources, the customer can change
    /// the assigned billing account.
    /// The resource name has the form
    /// `billingAccounts/{billing_account_id}`. For example,
    /// `billingAccounts/012345-567890-ABCDEF`.
    #[prost(string, tag="6")]
    pub billing_account: ::prost::alloc::string::String,
    /// Optional. ETag of the workload, it is calculated on the basis
    /// of the Workload contents. It will be used in Update & Delete operations.
    #[prost(string, tag="9")]
    pub etag: ::prost::alloc::string::String,
    /// Optional. Labels applied to the workload.
    #[prost(map="string, string", tag="10")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Input only. The parent resource for the resources managed by this Assured Workload. May
    /// be either empty or a folder resource which is a child of the
    /// Workload parent. If not specified all resources are created under the
    /// parent organization.
    /// Format:
    /// folders/{folder_id}
    #[prost(string, tag="13")]
    pub provisioned_resources_parent: ::prost::alloc::string::String,
    /// Input only. Settings used to create a CMEK crypto key. When set, a project with a KMS
    /// CMEK key is provisioned.
    /// This field is deprecated as of Feb 28, 2022.
    /// In order to create a Keyring, callers should specify,
    /// ENCRYPTION_KEYS_PROJECT or KEYRING in ResourceSettings.resource_type field.
    #[deprecated]
    #[prost(message, optional, tag="14")]
    pub kms_settings: ::core::option::Option<workload::KmsSettings>,
    /// Input only. Resource properties that are used to customize workload resources.
    /// These properties (such as custom project id) will be used to create
    /// workload resources if possible. This field is optional.
    #[prost(message, repeated, tag="15")]
    pub resource_settings: ::prost::alloc::vec::Vec<workload::ResourceSettings>,
    /// Output only. Represents the KAJ enrollment state of the given workload.
    #[prost(enumeration="workload::KajEnrollmentState", tag="17")]
    pub kaj_enrollment_state: i32,
    /// Optional. Indicates the sovereignty status of the given workload.
    /// Currently meant to be used by Europe/Canada customers.
    #[prost(bool, tag="18")]
    pub enable_sovereign_controls: bool,
    /// Output only. Represents the SAA enrollment response of the given workload.
    /// SAA enrollment response is queried during GetWorkload call.
    /// In failure cases, user friendly error message is shown in SAA details page.
    #[prost(message, optional, tag="20")]
    pub saa_enrollment_response: ::core::option::Option<workload::SaaEnrollmentResponse>,
    /// Output only. Urls for services which are compliant for this Assured Workload, but which
    /// are currently disallowed by the ResourceUsageRestriction org policy.
    /// Invoke RestrictAllowedResources endpoint to allow your project developers
    /// to use these services in their environment."
    #[prost(string, repeated, tag="24")]
    pub compliant_but_disallowed_services: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Compliance Regime associated with this workload.
    #[prost(enumeration="workload::Partner", tag="25")]
    pub partner: i32,
}
/// Nested message and enum types in `Workload`.
pub mod workload {
    /// Represent the resources that are children of this Workload.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResourceInfo {
        /// Resource identifier.
        /// For a project this represents project_number.
        #[prost(int64, tag="1")]
        pub resource_id: i64,
        /// Indicates the type of resource.
        #[prost(enumeration="resource_info::ResourceType", tag="2")]
        pub resource_type: i32,
    }
    /// Nested message and enum types in `ResourceInfo`.
    pub mod resource_info {
        /// The type of resource.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum ResourceType {
            /// Unknown resource type.
            Unspecified = 0,
            /// Consumer project.
            /// AssuredWorkloads Projects are no longer supported. This field will be
            /// ignored only in CreateWorkload requests. ListWorkloads and GetWorkload
            /// will continue to provide projects information.
            /// Use CONSUMER_FOLDER instead.
            ConsumerProject = 1,
            /// Consumer Folder.
            ConsumerFolder = 4,
            /// Consumer project containing encryption keys.
            EncryptionKeysProject = 2,
            /// Keyring resource that hosts encryption keys.
            Keyring = 3,
        }
        impl ResourceType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ResourceType::Unspecified => "RESOURCE_TYPE_UNSPECIFIED",
                    ResourceType::ConsumerProject => "CONSUMER_PROJECT",
                    ResourceType::ConsumerFolder => "CONSUMER_FOLDER",
                    ResourceType::EncryptionKeysProject => "ENCRYPTION_KEYS_PROJECT",
                    ResourceType::Keyring => "KEYRING",
                }
            }
        }
    }
    /// Settings specific to the Key Management Service.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KmsSettings {
        /// Required. Input only. Immutable. The time at which the Key Management Service will automatically create a
        /// new version of the crypto key and mark it as the primary.
        #[prost(message, optional, tag="1")]
        pub next_rotation_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Required. Input only. Immutable. \[next_rotation_time\] will be advanced by this period when the Key
        /// Management Service automatically rotates a key. Must be at least 24 hours
        /// and at most 876,000 hours.
        #[prost(message, optional, tag="2")]
        pub rotation_period: ::core::option::Option<::prost_types::Duration>,
    }
    /// Represent the custom settings for the resources to be created.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResourceSettings {
        /// Resource identifier.
        /// For a project this represents project_id. If the project is already
        /// taken, the workload creation will fail.
        /// For KeyRing, this represents the keyring_id.
        /// For a folder, don't set this value as folder_id is assigned by Google.
        #[prost(string, tag="1")]
        pub resource_id: ::prost::alloc::string::String,
        /// Indicates the type of resource. This field should be specified to
        /// correspond the id to the right project type (CONSUMER_PROJECT or
        /// ENCRYPTION_KEYS_PROJECT)
        #[prost(enumeration="resource_info::ResourceType", tag="2")]
        pub resource_type: i32,
        /// User-assigned resource display name.
        /// If not empty it will be used to create a resource with the specified
        /// name.
        #[prost(string, tag="3")]
        pub display_name: ::prost::alloc::string::String,
    }
    /// Signed Access Approvals (SAA) enrollment response.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SaaEnrollmentResponse {
        /// Indicates SAA enrollment status of a given workload.
        #[prost(enumeration="saa_enrollment_response::SetupState", optional, tag="1")]
        pub setup_status: ::core::option::Option<i32>,
        /// Indicates SAA enrollment setup error if any.
        #[prost(enumeration="saa_enrollment_response::SetupError", repeated, tag="2")]
        pub setup_errors: ::prost::alloc::vec::Vec<i32>,
    }
    /// Nested message and enum types in `SaaEnrollmentResponse`.
    pub mod saa_enrollment_response {
        /// Setup state of SAA enrollment.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum SetupState {
            /// Unspecified.
            Unspecified = 0,
            /// SAA enrollment pending.
            StatusPending = 1,
            /// SAA enrollment comopleted.
            StatusComplete = 2,
        }
        impl SetupState {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    SetupState::Unspecified => "SETUP_STATE_UNSPECIFIED",
                    SetupState::StatusPending => "STATUS_PENDING",
                    SetupState::StatusComplete => "STATUS_COMPLETE",
                }
            }
        }
        /// Setup error of SAA enrollment.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum SetupError {
            /// Unspecified.
            Unspecified = 0,
            /// Invalid states for all customers, to be redirected to AA UI for
            /// additional details.
            ErrorInvalidBaseSetup = 1,
            /// Returned when there is not an EKM key configured.
            ErrorMissingExternalSigningKey = 2,
            /// Returned when there are no enrolled services or the customer is
            /// enrolled in CAA only for a subset of services.
            ErrorNotAllServicesEnrolled = 3,
            /// Returned when exception was encountered during evaluation of other
            /// criteria.
            ErrorSetupCheckFailed = 4,
        }
        impl SetupError {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    SetupError::Unspecified => "SETUP_ERROR_UNSPECIFIED",
                    SetupError::ErrorInvalidBaseSetup => "ERROR_INVALID_BASE_SETUP",
                    SetupError::ErrorMissingExternalSigningKey => "ERROR_MISSING_EXTERNAL_SIGNING_KEY",
                    SetupError::ErrorNotAllServicesEnrolled => "ERROR_NOT_ALL_SERVICES_ENROLLED",
                    SetupError::ErrorSetupCheckFailed => "ERROR_SETUP_CHECK_FAILED",
                }
            }
        }
    }
    /// Supported Compliance Regimes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ComplianceRegime {
        /// Unknown compliance regime.
        Unspecified = 0,
        /// Information protection as per DoD IL4 requirements.
        Il4 = 1,
        /// Criminal Justice Information Services (CJIS) Security policies.
        Cjis = 2,
        /// FedRAMP High data protection controls
        FedrampHigh = 3,
        /// FedRAMP Moderate data protection controls
        FedrampModerate = 4,
        /// Assured Workloads For US Regions data protection controls
        UsRegionalAccess = 5,
        /// Health Insurance Portability and Accountability Act controls
        Hipaa = 6,
        /// Health Information Trust Alliance controls
        Hitrust = 7,
        /// Assured Workloads For EU Regions and Support controls
        EuRegionsAndSupport = 8,
        /// Assured Workloads For Canada Regions and Support controls
        CaRegionsAndSupport = 9,
        /// International Traffic in Arms Regulations
        Itar = 10,
        /// Assured Workloads for Partners;
        AssuredWorkloadsForPartners = 12,
    }
    impl ComplianceRegime {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ComplianceRegime::Unspecified => "COMPLIANCE_REGIME_UNSPECIFIED",
                ComplianceRegime::Il4 => "IL4",
                ComplianceRegime::Cjis => "CJIS",
                ComplianceRegime::FedrampHigh => "FEDRAMP_HIGH",
                ComplianceRegime::FedrampModerate => "FEDRAMP_MODERATE",
                ComplianceRegime::UsRegionalAccess => "US_REGIONAL_ACCESS",
                ComplianceRegime::Hipaa => "HIPAA",
                ComplianceRegime::Hitrust => "HITRUST",
                ComplianceRegime::EuRegionsAndSupport => "EU_REGIONS_AND_SUPPORT",
                ComplianceRegime::CaRegionsAndSupport => "CA_REGIONS_AND_SUPPORT",
                ComplianceRegime::Itar => "ITAR",
                ComplianceRegime::AssuredWorkloadsForPartners => "ASSURED_WORKLOADS_FOR_PARTNERS",
            }
        }
    }
    /// Key Access Justifications(KAJ) Enrollment State.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum KajEnrollmentState {
        /// Default State for KAJ Enrollment.
        Unspecified = 0,
        /// Pending State for KAJ Enrollment.
        Pending = 1,
        /// Complete State for KAJ Enrollment.
        Complete = 2,
    }
    impl KajEnrollmentState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                KajEnrollmentState::Unspecified => "KAJ_ENROLLMENT_STATE_UNSPECIFIED",
                KajEnrollmentState::Pending => "KAJ_ENROLLMENT_STATE_PENDING",
                KajEnrollmentState::Complete => "KAJ_ENROLLMENT_STATE_COMPLETE",
            }
        }
    }
    /// Supported Assured Workloads Partners.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Partner {
        /// Unknown compliance regime.
        Unspecified = 0,
        /// S3NS regime
        LocalControlsByS3ns = 1,
    }
    impl Partner {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Partner::Unspecified => "PARTNER_UNSPECIFIED",
                Partner::LocalControlsByS3ns => "LOCAL_CONTROLS_BY_S3NS",
            }
        }
    }
}
/// Operation metadata to give request details of CreateWorkload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkloadOperationMetadata {
    /// Optional. Time when the operation was created.
    #[prost(message, optional, tag="1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The display name of the workload.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. The parent of the workload.
    #[prost(string, tag="3")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Compliance controls that should be applied to the resources managed by
    /// the workload.
    #[prost(enumeration="workload::ComplianceRegime", tag="4")]
    pub compliance_regime: i32,
}
/// Request for restricting list of available resources in Workload environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestrictAllowedResourcesRequest {
    /// Required. The resource name of the Workload. This is the workloads's
    /// relative path in the API, formatted as
    /// "organizations/{organization_id}/locations/{location_id}/workloads/{workload_id}".
    /// For example,
    /// "organizations/123/locations/us-east1/workloads/assured-workload-1".
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The type of restriction for using gcp products in the Workload environment.
    #[prost(enumeration="restrict_allowed_resources_request::RestrictionType", tag="2")]
    pub restriction_type: i32,
}
/// Nested message and enum types in `RestrictAllowedResourcesRequest`.
pub mod restrict_allowed_resources_request {
    /// The type of restriction.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RestrictionType {
        /// Unknown restriction type.
        Unspecified = 0,
        /// Allow the use all of all gcp products, irrespective of the compliance
        /// posture. This effectively removes gcp.restrictServiceUsage OrgPolicy
        /// on the AssuredWorkloads Folder.
        AllowAllGcpResources = 1,
        /// Based on Workload's compliance regime, allowed list changes.
        /// See - <https://cloud.google.com/assured-workloads/docs/supported-products>
        /// for the list of supported resources.
        AllowCompliantResources = 2,
    }
    impl RestrictionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RestrictionType::Unspecified => "RESTRICTION_TYPE_UNSPECIFIED",
                RestrictionType::AllowAllGcpResources => "ALLOW_ALL_GCP_RESOURCES",
                RestrictionType::AllowCompliantResources => "ALLOW_COMPLIANT_RESOURCES",
            }
        }
    }
}
/// Response for restricting the list of allowed resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestrictAllowedResourcesResponse {
}
/// Request for acknowledging the violation
/// Next Id: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcknowledgeViolationRequest {
    /// Required. The resource name of the Violation to acknowledge.
    /// Format:
    /// organizations/{organization}/locations/{location}/workloads/{workload}/violations/{violation}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Business justification explaining the need for violation acknowledgement
    #[prost(string, tag="2")]
    pub comment: ::prost::alloc::string::String,
    /// Optional. Name of the OrgPolicy which was modified with non-compliant change and
    /// resulted in this violation.
    /// Format:
    /// projects/{project_number}/policies/{constraint_name}
    /// folders/{folder_id}/policies/{constraint_name}
    /// organizations/{organization_id}/policies/{constraint_name}
    #[prost(string, tag="3")]
    pub non_compliant_org_policy: ::prost::alloc::string::String,
}
/// Response for violation acknowledgement
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcknowledgeViolationResponse {
}
/// Interval defining a time window.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeWindow {
    /// The start of the time window.
    #[prost(message, optional, tag="1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end of the time window.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request for fetching violations in an organization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListViolationsRequest {
    /// Required. The Workload name.
    /// Format `organizations/{org_id}/locations/{location}/workloads/{workload}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Specifies the time window for retrieving active Violations.
    /// When specified, retrieves Violations that were active between start_time
    /// and end_time.
    #[prost(message, optional, tag="2")]
    pub interval: ::core::option::Option<TimeWindow>,
    /// Optional. Page size.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Optional. Page token returned from previous request.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. A custom filter for filtering by the Violations properties.
    #[prost(string, tag="5")]
    pub filter: ::prost::alloc::string::String,
}
/// Response of ListViolations endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListViolationsResponse {
    /// List of Violations under a Workload.
    #[prost(message, repeated, tag="1")]
    pub violations: ::prost::alloc::vec::Vec<Violation>,
    /// The next page token. Returns empty if reached the last page.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for fetching a Workload Violation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetViolationRequest {
    /// Required. The resource name of the Violation to fetch (ie. Violation.name).
    /// Format:
    /// organizations/{organization}/locations/{location}/workloads/{workload}/violations/{violation}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Workload monitoring Violation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Violation {
    /// Output only. Immutable. Name of the Violation.
    /// Format:
    /// organizations/{organization}/locations/{location}/workloads/{workload_id}/violations/{violations_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Description for the Violation.
    /// e.g. OrgPolicy gcp.resourceLocations has non compliant value.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Time of the event which triggered the Violation.
    #[prost(message, optional, tag="3")]
    pub begin_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last time when the Violation record was updated.
    #[prost(message, optional, tag="4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time of the event which fixed the Violation.
    /// If the violation is ACTIVE this will be empty.
    #[prost(message, optional, tag="5")]
    pub resolve_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Category under which this violation is mapped.
    /// e.g. Location, Service Usage, Access, Encryption, etc.
    #[prost(string, tag="6")]
    pub category: ::prost::alloc::string::String,
    /// Output only. State of the violation
    #[prost(enumeration="violation::State", tag="7")]
    pub state: i32,
    /// Output only. Immutable. The org-policy-constraint that was incorrectly changed, which resulted in
    /// this violation.
    #[prost(string, tag="8")]
    pub org_policy_constraint: ::prost::alloc::string::String,
    /// Output only. Immutable. Audit Log Link for violated resource
    /// Format:
    /// <https://console.cloud.google.com/logs/query;query={logName}{protoPayload.resourceName}{timeRange}{folder}>
    #[prost(string, tag="11")]
    pub audit_log_link: ::prost::alloc::string::String,
    /// Output only. Immutable. Name of the OrgPolicy which was modified with non-compliant change and
    /// resulted this violation.
    ///   Format:
    ///   projects/{project_number}/policies/{constraint_name}
    ///   folders/{folder_id}/policies/{constraint_name}
    ///   organizations/{organization_id}/policies/{constraint_name}
    #[prost(string, tag="12")]
    pub non_compliant_org_policy: ::prost::alloc::string::String,
    /// Output only. Compliance violation remediation
    #[prost(message, optional, tag="13")]
    pub remediation: ::core::option::Option<violation::Remediation>,
    /// Output only. A boolean that indicates if the violation is acknowledged
    #[prost(bool, tag="14")]
    pub acknowledged: bool,
    /// Optional. Timestamp when this violation was acknowledged last.
    /// This will be absent when acknowledged field is marked as false.
    #[prost(message, optional, tag="15")]
    pub acknowledgement_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `Violation`.
pub mod violation {
    /// Represents remediation guidance to resolve compliance violation for
    /// AssuredWorkload
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Remediation {
        /// Required. Remediation instructions to resolve violations
        #[prost(message, optional, tag="1")]
        pub instructions: ::core::option::Option<remediation::Instructions>,
        /// Values that can resolve the violation
        /// For example: for list org policy violations, this will either be the list
        /// of allowed or denied values
        #[prost(string, repeated, tag="2")]
        pub compliant_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Output only. Reemediation type based on the type of org policy values violated
        #[prost(enumeration="remediation::RemediationType", tag="3")]
        pub remediation_type: i32,
    }
    /// Nested message and enum types in `Remediation`.
    pub mod remediation {
        /// Instructions to remediate violation
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Instructions {
            /// Remediation instructions to resolve violation via gcloud cli
            #[prost(message, optional, tag="1")]
            pub gcloud_instructions: ::core::option::Option<instructions::Gcloud>,
            /// Remediation instructions to resolve violation via cloud console
            #[prost(message, optional, tag="2")]
            pub console_instructions: ::core::option::Option<instructions::Console>,
        }
        /// Nested message and enum types in `Instructions`.
        pub mod instructions {
            /// Remediation instructions to resolve violation via gcloud cli
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Gcloud {
                /// Gcloud command to resolve violation
                #[prost(string, repeated, tag="1")]
                pub gcloud_commands: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                /// Steps to resolve violation via gcloud cli
                #[prost(string, repeated, tag="2")]
                pub steps: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                /// Additional urls for more information about steps
                #[prost(string, repeated, tag="3")]
                pub additional_links: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            }
            /// Remediation instructions to resolve violation via cloud console
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Console {
                /// Link to console page where violations can be resolved
                #[prost(string, repeated, tag="1")]
                pub console_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                /// Steps to resolve violation via cloud console
                #[prost(string, repeated, tag="2")]
                pub steps: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                /// Additional urls for more information about steps
                #[prost(string, repeated, tag="3")]
                pub additional_links: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            }
        }
        /// Classifying remediation into various types based on the kind of
        /// violation. For example, violations caused due to changes in boolean org
        /// policy requires different remediation instructions compared to violation
        /// caused due to changes in allowed values of list org policy.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum RemediationType {
            /// Unspecified remediation type
            Unspecified = 0,
            /// Remediation type for boolean org policy
            RemediationBooleanOrgPolicyViolation = 1,
            /// Remediation type for list org policy which have allowed values in the
            /// monitoring rule
            RemediationListAllowedValuesOrgPolicyViolation = 2,
            /// Remediation type for list org policy which have denied values in the
            /// monitoring rule
            RemediationListDeniedValuesOrgPolicyViolation = 3,
            /// Remediation type for gcp.restrictCmekCryptoKeyProjects
            RemediationRestrictCmekCryptoKeyProjectsOrgPolicyViolation = 4,
        }
        impl RemediationType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    RemediationType::Unspecified => "REMEDIATION_TYPE_UNSPECIFIED",
                    RemediationType::RemediationBooleanOrgPolicyViolation => "REMEDIATION_BOOLEAN_ORG_POLICY_VIOLATION",
                    RemediationType::RemediationListAllowedValuesOrgPolicyViolation => "REMEDIATION_LIST_ALLOWED_VALUES_ORG_POLICY_VIOLATION",
                    RemediationType::RemediationListDeniedValuesOrgPolicyViolation => "REMEDIATION_LIST_DENIED_VALUES_ORG_POLICY_VIOLATION",
                    RemediationType::RemediationRestrictCmekCryptoKeyProjectsOrgPolicyViolation => "REMEDIATION_RESTRICT_CMEK_CRYPTO_KEY_PROJECTS_ORG_POLICY_VIOLATION",
                }
            }
        }
    }
    /// Violation State Values
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified state.
        Unspecified = 0,
        /// Violation is resolved.
        Resolved = 2,
        /// Violation is Unresolved
        Unresolved = 3,
        /// Violation is Exception
        Exception = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Resolved => "RESOLVED",
                State::Unresolved => "UNRESOLVED",
                State::Exception => "EXCEPTION",
            }
        }
    }
}
/// Generated client implementations.
pub mod assured_workloads_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to manage AssuredWorkloads.
    #[derive(Debug, Clone)]
    pub struct AssuredWorkloadsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AssuredWorkloadsServiceClient<tonic::transport::Channel> {
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
    impl<T> AssuredWorkloadsServiceClient<T>
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
        ) -> AssuredWorkloadsServiceClient<InterceptedService<T, F>>
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
            AssuredWorkloadsServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// Creates Assured Workload.
        pub async fn create_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkloadRequest>,
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
                "/google.cloud.assuredworkloads.v1.AssuredWorkloadsService/CreateWorkload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an existing workload.
        /// Currently allows updating of workload display_name and labels.
        /// For force updates don't set etag field in the Workload.
        /// Only one update operation per workload can be in progress.
        pub async fn update_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWorkloadRequest>,
        ) -> Result<tonic::Response<super::Workload>, tonic::Status> {
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
                "/google.cloud.assuredworkloads.v1.AssuredWorkloadsService/UpdateWorkload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Restrict the list of resources allowed in the Workload environment.
        /// The current list of allowed products can be found at
        /// https://cloud.google.com/assured-workloads/docs/supported-products
        /// In addition to assuredworkloads.workload.update permission, the user should
        /// also have orgpolicy.policy.set permission on the folder resource
        /// to use this functionality.
        pub async fn restrict_allowed_resources(
            &mut self,
            request: impl tonic::IntoRequest<super::RestrictAllowedResourcesRequest>,
        ) -> Result<
            tonic::Response<super::RestrictAllowedResourcesResponse>,
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
                "/google.cloud.assuredworkloads.v1.AssuredWorkloadsService/RestrictAllowedResources",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the workload. Make sure that workload's direct children are already
        /// in a deleted state, otherwise the request will fail with a
        /// FAILED_PRECONDITION error.
        pub async fn delete_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWorkloadRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.assuredworkloads.v1.AssuredWorkloadsService/DeleteWorkload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets Assured Workload associated with a CRM Node
        pub async fn get_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkloadRequest>,
        ) -> Result<tonic::Response<super::Workload>, tonic::Status> {
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
                "/google.cloud.assuredworkloads.v1.AssuredWorkloadsService/GetWorkload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists Assured Workloads under a CRM Node.
        pub async fn list_workloads(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWorkloadsRequest>,
        ) -> Result<tonic::Response<super::ListWorkloadsResponse>, tonic::Status> {
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
                "/google.cloud.assuredworkloads.v1.AssuredWorkloadsService/ListWorkloads",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists the Violations in the AssuredWorkload Environment.
        /// Callers may also choose to read across multiple Workloads as per
        /// [AIP-159](https://google.aip.dev/159) by using '-' (the hyphen or dash
        /// character) as a wildcard character instead of workload-id in the parent.
        /// Format `organizations/{org_id}/locations/{location}/workloads/-`
        pub async fn list_violations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListViolationsRequest>,
        ) -> Result<tonic::Response<super::ListViolationsResponse>, tonic::Status> {
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
                "/google.cloud.assuredworkloads.v1.AssuredWorkloadsService/ListViolations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves Assured Workload Violation based on ID.
        pub async fn get_violation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetViolationRequest>,
        ) -> Result<tonic::Response<super::Violation>, tonic::Status> {
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
                "/google.cloud.assuredworkloads.v1.AssuredWorkloadsService/GetViolation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Acknowledges an existing violation. By acknowledging a violation, users
        /// acknowledge the existence of a compliance violation in their workload and
        /// decide to ignore it due to a valid business justification. Acknowledgement
        /// is a permanent operation and it cannot be reverted.
        pub async fn acknowledge_violation(
            &mut self,
            request: impl tonic::IntoRequest<super::AcknowledgeViolationRequest>,
        ) -> Result<
            tonic::Response<super::AcknowledgeViolationResponse>,
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
                "/google.cloud.assuredworkloads.v1.AssuredWorkloadsService/AcknowledgeViolation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
