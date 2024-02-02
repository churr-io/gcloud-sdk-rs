/// An EffectiveSecurityHealthAnalyticsCustomModule is the representation of
/// a Security Health Analytics custom module at a specified level of the
/// resource hierarchy: organization, folder, or project. If a custom module is
/// inherited from a parent organization or folder, the value of the
/// `enablementState` property in EffectiveSecurityHealthAnalyticsCustomModule is
/// set to the value that is effective in the parent, instead of  `INHERITED`.
/// For example, if the module is enabled in a parent organization or folder, the
/// effective enablement_state for the module in all child folders or projects is
/// also `enabled`. EffectiveSecurityHealthAnalyticsCustomModule is read-only.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EffectiveSecurityHealthAnalyticsCustomModule {
    /// Output only. The resource name of the custom module.
    /// Its format is
    /// "organizations/{organization}/locations/{location}/effectiveSecurityHealthAnalyticsCustomModules/{effective_security_health_analytics_custom_module}",
    /// or
    /// "folders/{folder}/locations/{location}/effectiveSecurityHealthAnalyticsCustomModules/{effective_security_health_analytics_custom_module}",
    /// or
    /// "projects/{project}/locations/{location}/effectiveSecurityHealthAnalyticsCustomModules/{effective_security_health_analytics_custom_module}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The user-specified configuration for the module.
    #[prost(message, optional, tag = "2")]
    pub custom_config: ::core::option::Option<CustomConfig>,
    /// Output only. The effective state of enablement for the module at the given
    /// level of the hierarchy.
    #[prost(
        enumeration = "effective_security_health_analytics_custom_module::EnablementState",
        tag = "3"
    )]
    pub enablement_state: i32,
    /// Output only. The display name for the custom module. The name must be
    /// between 1 and 128 characters, start with a lowercase letter, and contain
    /// alphanumeric characters or underscores only.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `EffectiveSecurityHealthAnalyticsCustomModule`.
pub mod effective_security_health_analytics_custom_module {
    /// The enablement state of the module.
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
    pub enum EnablementState {
        /// Unspecified enablement state.
        Unspecified = 0,
        /// The module is enabled at the given level.
        Enabled = 1,
        /// The module is disabled at the given level.
        Disabled = 2,
    }
    impl EnablementState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EnablementState::Unspecified => "ENABLEMENT_STATE_UNSPECIFIED",
                EnablementState::Enabled => "ENABLED",
                EnablementState::Disabled => "DISABLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENABLEMENT_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ENABLED" => Some(Self::Enabled),
                "DISABLED" => Some(Self::Disabled),
                _ => None,
            }
        }
    }
}
/// Request message for listing effective Security Health Analytics custom
/// modules.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEffectiveSecurityHealthAnalyticsCustomModulesRequest {
    /// Required. Name of parent to list effective custom modules. Its format is
    /// "organizations/{organization}/locations/{location}",
    /// "folders/{folder}/locations/{location}",
    /// or
    /// "projects/{project}/locations/{location}"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of results to return in a single response.
    /// Default is 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The value returned by the last call indicating a continuation
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for listing effective Security Health Analytics custom
/// modules.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEffectiveSecurityHealthAnalyticsCustomModulesResponse {
    /// The list of EffectiveSecurityHealthAnalyticsCustomModule
    #[prost(message, repeated, tag = "1")]
    pub effective_security_health_analytics_custom_modules: ::prost::alloc::vec::Vec<
        EffectiveSecurityHealthAnalyticsCustomModule,
    >,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Message for getting a EffectiveSecurityHealthAnalyticsCustomModule
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEffectiveSecurityHealthAnalyticsCustomModuleRequest {
    /// Required. The resource name of the SHA custom module.
    ///
    /// Its format is:
    ///
    ///    * "organizations/{organization}/locations/{location}/effectiveSecurityHealthAnalyticsCustomModules/{module_id}".
    ///    * "folders/{folder}/locations/{location}/effectiveSecurityHealthAnalyticsCustomModules/{module_id}".
    ///    * "projects/{project}/locations/{location}/effectiveSecurityHealthAnalyticsCustomModules/{module_id}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents an instance of a Security Health Analytics custom module,
/// including its full module name, display name, enablement state, and last
/// updated time. You can create a custom module at the organization, folder, or
/// project level. Custom modules that you create at the organization or folder
/// level are inherited by the child folders and projects.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityHealthAnalyticsCustomModule {
    /// Immutable. The resource name of the custom module.
    /// Its format is
    /// "organizations/{organization}/locations/{location}/securityHealthAnalyticsCustomModules/{security_health_analytics_custom_module}",
    /// or
    /// "folders/{folder}/locations/{location}/securityHealthAnalyticsCustomModules/{security_health_analytics_custom_module}",
    /// or
    /// "projects/{project}/locations/{location}/securityHealthAnalyticsCustomModules/{security_health_analytics_custom_module}"
    ///
    /// The id {customModule} is server-generated and is not user settable.
    /// It will be a numeric id containing 1-20 digits.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The display name of the Security Health Analytics custom module.
    /// This display name becomes the finding category for all findings that are
    /// returned by this custom module. The display name must be between 1 and
    /// 128 characters, start with a lowercase letter, and contain alphanumeric
    /// characters or underscores only.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. The enablement state of the custom module.
    #[prost(
        enumeration = "security_health_analytics_custom_module::EnablementState",
        tag = "3"
    )]
    pub enablement_state: i32,
    /// Output only. The time at which the custom module was last updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The editor that last updated the custom module.
    #[prost(string, tag = "5")]
    pub last_editor: ::prost::alloc::string::String,
    /// Output only. Specifies the organization or folder from which the custom
    /// module is inherited. If empty, indicates that the custom module was created
    /// in the organization, folder, or project in which you are viewing the custom
    /// module.
    #[prost(string, tag = "6")]
    pub ancestor_module: ::prost::alloc::string::String,
    /// Optional. The user specified custom configuration for the module.
    #[prost(message, optional, tag = "7")]
    pub custom_config: ::core::option::Option<CustomConfig>,
}
/// Nested message and enum types in `SecurityHealthAnalyticsCustomModule`.
pub mod security_health_analytics_custom_module {
    /// Possible enablement states of a custom module.
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
    pub enum EnablementState {
        /// Unspecified enablement state.
        Unspecified = 0,
        /// The module is enabled at the given CRM resource.
        Enabled = 1,
        /// The module is disabled at the given CRM resource.
        Disabled = 2,
        /// State is inherited from an ancestor module. The module will either
        /// be effectively ENABLED or DISABLED based on its closest non-inherited
        /// ancestor module in the CRM hierarchy. Attempting to set a top level
        /// module (module with no parent) to the INHERITED state will result in an
        /// INVALID_ARGUMENT error.
        Inherited = 3,
    }
    impl EnablementState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EnablementState::Unspecified => "ENABLEMENT_STATE_UNSPECIFIED",
                EnablementState::Enabled => "ENABLED",
                EnablementState::Disabled => "DISABLED",
                EnablementState::Inherited => "INHERITED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENABLEMENT_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ENABLED" => Some(Self::Enabled),
                "DISABLED" => Some(Self::Disabled),
                "INHERITED" => Some(Self::Inherited),
                _ => None,
            }
        }
    }
}
/// Defines the properties in a custom module configuration for Security
/// Health Analytics. Use the custom module configuration to create custom
/// detectors that generate custom findings for resources that you specify.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomConfig {
    /// Optional. The CEL expression to evaluate to produce findings. When the
    /// expression evaluates to true against a resource, a finding is generated.
    #[prost(message, optional, tag = "1")]
    pub predicate: ::core::option::Option<super::super::super::r#type::Expr>,
    /// Optional. Custom output properties.
    #[prost(message, optional, tag = "2")]
    pub custom_output: ::core::option::Option<custom_config::CustomOutputSpec>,
    /// Optional. The CAI resource types that the custom module operates on (see
    /// go/gcp-cai-doc/supported-asset-types). Each custom module can specify up to
    /// 5 resource types.
    #[prost(message, optional, tag = "3")]
    pub resource_selector: ::core::option::Option<custom_config::ResourceSelector>,
    /// Optional. The severity to assign to findings generated by the module.
    #[prost(enumeration = "custom_config::Severity", tag = "4")]
    pub severity: i32,
    /// Optional. Text that describes the vulnerability or misconfiguration that
    /// the custom module detects. This explanation is returned with each finding
    /// instance to help investigators understand the detected issue. The text must
    /// be enclosed in quotation marks.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Optional. An explanation of the recommended steps that security teams can
    /// take to resolve the detected issue. This explanation is returned with each
    /// finding generated by this module in the `nextSteps` property of the finding
    /// JSON.
    #[prost(string, tag = "6")]
    pub recommendation: ::prost::alloc::string::String,
}
/// Nested message and enum types in `CustomConfig`.
pub mod custom_config {
    /// A set of optional name-value pairs that define custom source properties to
    /// return with each finding that is generated by the custom module. The custom
    /// source properties that are defined here are included in the finding JSON
    /// under `sourceProperties`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomOutputSpec {
        /// Optional. A list of custom output properties to add to the finding.
        #[prost(message, repeated, tag = "1")]
        pub properties: ::prost::alloc::vec::Vec<custom_output_spec::Property>,
    }
    /// Nested message and enum types in `CustomOutputSpec`.
    pub mod custom_output_spec {
        /// An individual name-value pair that defines a custom source property.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Property {
            /// Optional. Name of the property for the custom output.
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
            /// Optional. The CEL expression for the custom output. A resource property
            /// can be specified to return the value of the property or a text string
            /// enclosed in quotation marks.
            #[prost(message, optional, tag = "2")]
            pub value_expression: ::core::option::Option<
                super::super::super::super::super::r#type::Expr,
            >,
        }
    }
    /// Resource for selecting resource type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResourceSelector {
        /// Optional. The resource types to run the detector on.
        #[prost(string, repeated, tag = "1")]
        pub resource_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Defines the valid value options for the severity of a finding.
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
    pub enum Severity {
        /// Unspecified severity.
        Unspecified = 0,
        /// Critical severity.
        Critical = 1,
        /// High severity.
        High = 2,
        /// Medium severity.
        Medium = 3,
        /// Low severity.
        Low = 4,
    }
    impl Severity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Severity::Unspecified => "SEVERITY_UNSPECIFIED",
                Severity::Critical => "CRITICAL",
                Severity::High => "HIGH",
                Severity::Medium => "MEDIUM",
                Severity::Low => "LOW",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SEVERITY_UNSPECIFIED" => Some(Self::Unspecified),
                "CRITICAL" => Some(Self::Critical),
                "HIGH" => Some(Self::High),
                "MEDIUM" => Some(Self::Medium),
                "LOW" => Some(Self::Low),
                _ => None,
            }
        }
    }
}
/// Request message for listing Security Health Analytics custom modules.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSecurityHealthAnalyticsCustomModulesRequest {
    /// Required. Name of parent to list custom modules. Its format is
    /// "organizations/{organization}/locations/{location}",
    /// "folders/{folder}/locations/{location}",
    /// or
    /// "projects/{project}/locations/{location}"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of results to return in a single response.
    /// Default is 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for listing Security Health Analytics custom modules.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSecurityHealthAnalyticsCustomModulesResponse {
    /// The list of SecurityHealthAnalyticsCustomModules
    #[prost(message, repeated, tag = "1")]
    pub security_health_analytics_custom_modules: ::prost::alloc::vec::Vec<
        SecurityHealthAnalyticsCustomModule,
    >,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for listing descendant Security Health Analytics custom
/// modules.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDescendantSecurityHealthAnalyticsCustomModulesRequest {
    /// Required. Name of parent to list custom modules. Its format is
    /// "organizations/{organization}/locations/{location}",
    /// "folders/{folder}/locations/{location}",
    /// or
    /// "projects/{project}/locations/{location}"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of results to return in a single response.
    /// Default is 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for listing descendant Security Health Analytics custom
/// modules.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDescendantSecurityHealthAnalyticsCustomModulesResponse {
    /// The list of SecurityHealthAnalyticsCustomModules
    #[prost(message, repeated, tag = "1")]
    pub security_health_analytics_custom_modules: ::prost::alloc::vec::Vec<
        SecurityHealthAnalyticsCustomModule,
    >,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Message for getting a SecurityHealthAnalyticsCustomModule
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSecurityHealthAnalyticsCustomModuleRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a SecurityHealthAnalyticsCustomModule
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSecurityHealthAnalyticsCustomModuleRequest {
    /// Required. Name of the parent for the module. Its format is
    /// "organizations/{organization}/locations/{location}",
    /// "folders/{folder}/locations/{location}",
    /// or
    /// "projects/{project}/locations/{location}"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The resource being created
    #[prost(message, optional, tag = "2")]
    pub security_health_analytics_custom_module: ::core::option::Option<
        SecurityHealthAnalyticsCustomModule,
    >,
    /// Optional. When set to true, only validations (including IAM checks) will
    /// done for the request (no module will be created). An OK response indicates
    /// the request is valid while an error response indicates the request is
    /// invalid. Note that a subsequent request to actually create the module could
    /// still fail because:
    ///   1. the state could have changed (e.g. IAM permission lost) or
    ///   2. A failure occurred during creation of the module.
    /// Defaults to false.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// Message for updating a SecurityHealthAnalyticsCustomModule
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSecurityHealthAnalyticsCustomModuleRequest {
    /// Required. The list of fields to be updated. The only fields that can be
    /// updated are `enablement_state` and `custom_config`. If empty or set to the
    /// wildcard value `*`, both `enablement_state` and `custom_config` are
    /// updated.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated
    #[prost(message, optional, tag = "2")]
    pub security_health_analytics_custom_module: ::core::option::Option<
        SecurityHealthAnalyticsCustomModule,
    >,
    /// Optional. When set to true, only validations (including IAM checks) will
    /// done for the request (module will not be updated). An OK response indicates
    /// the request is valid while an error response indicates the request is
    /// invalid. Note that a subsequent request to actually update the module could
    /// still fail because 1. the state could have changed (e.g. IAM permission
    /// lost) or
    /// 2. A failure occurred while trying to update the module.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// Message for deleting a SecurityHealthAnalyticsCustomModule
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSecurityHealthAnalyticsCustomModuleRequest {
    /// Required. The resource name of the SHA custom module.
    ///
    /// Its format is:
    ///
    ///    * "organizations/{organization}/locations/{location}/securityHealthAnalyticsCustomModules/{security_health_analytics_custom_module}".
    ///    * "folders/{folder}/locations/{location}/securityHealthAnalyticsCustomModules/{security_health_analytics_custom_module}".
    ///    * "projects/{project}/locations/{location}/securityHealthAnalyticsCustomModules/{security_health_analytics_custom_module}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. When set to true, only validations (including IAM checks) will
    /// done for the request (module will not be deleted). An OK response indicates
    /// the request is valid while an error response indicates the request is
    /// invalid. Note that a subsequent request to actually delete the module could
    /// still fail because 1. the state could have changed (e.g. IAM permission
    /// lost) or
    /// 2. A failure occurred while trying to delete the module.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
}
/// Request message to simulate a CustomConfig against a given test resource.
/// Maximum size of the request is 4 MB by default.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimulateSecurityHealthAnalyticsCustomModuleRequest {
    /// Required. The relative resource name of the organization, project, or
    /// folder. For more information about relative resource names, see [Relative
    /// Resource
    /// Name](<https://cloud.google.com/apis/design/resource_names#relative_resource_name>)
    /// Example: `organizations/{organization_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The custom configuration that you need to test.
    #[prost(message, optional, tag = "2")]
    pub custom_config: ::core::option::Option<CustomConfig>,
    /// Required. Resource data to simulate custom module against.
    #[prost(message, optional, tag = "3")]
    pub resource: ::core::option::Option<
        simulate_security_health_analytics_custom_module_request::SimulatedResource,
    >,
}
/// Nested message and enum types in `SimulateSecurityHealthAnalyticsCustomModuleRequest`.
pub mod simulate_security_health_analytics_custom_module_request {
    /// Manually constructed resource name. If the custom module evaluates against
    /// only the resource data, you can omit the `iam_policy_data` field. If it
    /// evaluates only the `iam_policy_data` field, you can omit the resource data.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SimulatedResource {
        /// Required. The type of the resource, for example,
        /// `compute.googleapis.com/Disk`.
        #[prost(string, tag = "1")]
        pub resource_type: ::prost::alloc::string::String,
        /// Optional. A representation of the Google Cloud resource. Should match the
        /// Google Cloud resource JSON format.
        #[prost(message, optional, tag = "2")]
        pub resource_data: ::core::option::Option<::prost_types::Struct>,
        /// Optional. A representation of the IAM policy.
        #[prost(message, optional, tag = "3")]
        pub iam_policy_data: ::core::option::Option<
            super::super::super::super::iam::v1::Policy,
        >,
    }
}
/// A subset of the fields of the Security Center Finding proto. The minimum set
/// of fields needed to represent a simulated finding from a SHA custom module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimulatedFinding {
    /// Identifier. The [relative resource
    /// name](<https://cloud.google.com/apis/design/resource_names#relative_resource_name>)
    /// of the finding. Example:
    /// "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}",
    /// "folders/{folder_id}/sources/{source_id}/findings/{finding_id}",
    /// "projects/{project_id}/sources/{source_id}/findings/{finding_id}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The relative resource name of the source the finding belongs to. See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name>
    /// This field is immutable after creation time.
    /// For example:
    /// "organizations/{organization_id}/sources/{source_id}"
    #[prost(string, tag = "2")]
    pub parent: ::prost::alloc::string::String,
    /// For findings on Google Cloud resources, the full resource
    /// name of the Google Cloud resource this finding is for. See:
    /// <https://cloud.google.com/apis/design/resource_names#full_resource_name>
    /// When the finding is for a non-Google Cloud resource, the resourceName can
    /// be a customer or partner defined string. This field is immutable after
    /// creation time.
    #[prost(string, tag = "3")]
    pub resource_name: ::prost::alloc::string::String,
    /// The additional taxonomy group within findings from a given source.
    /// This field is immutable after creation time.
    /// Example: "XSS_FLASH_INJECTION"
    #[prost(string, tag = "4")]
    pub category: ::prost::alloc::string::String,
    /// Output only. The state of the finding.
    #[prost(enumeration = "simulated_finding::State", tag = "5")]
    pub state: i32,
    /// Source specific properties. These properties are managed by the source
    /// that writes the finding. The key names in the source_properties map must be
    /// between 1 and 255 characters, and must start with a letter and contain
    /// alphanumeric characters or underscores only.
    #[prost(map = "string, message", tag = "6")]
    pub source_properties: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost_types::Value,
    >,
    /// The time the finding was first detected. If an existing finding is updated,
    /// then this is the time the update occurred.
    /// For example, if the finding represents an open firewall, this property
    /// captures the time the detector believes the firewall became open. The
    /// accuracy is determined by the detector. If the finding is later resolved,
    /// then this time reflects when the finding was resolved. This must not
    /// be set to a value greater than the current timestamp.
    #[prost(message, optional, tag = "7")]
    pub event_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The severity of the finding. This field is managed by the source that
    /// writes the finding.
    #[prost(enumeration = "simulated_finding::Severity", tag = "8")]
    pub severity: i32,
    /// The class of the finding.
    #[prost(enumeration = "simulated_finding::FindingClass", tag = "9")]
    pub finding_class: i32,
}
/// Nested message and enum types in `SimulatedFinding`.
pub mod simulated_finding {
    /// The state of the finding.
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
        /// The finding requires attention and has not been addressed yet.
        Active = 1,
        /// The finding has been fixed, triaged as a non-issue or otherwise addressed
        /// and is no longer active.
        Inactive = 2,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Active => "ACTIVE",
                State::Inactive => "INACTIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "INACTIVE" => Some(Self::Inactive),
                _ => None,
            }
        }
    }
    /// The severity of the finding.
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
    pub enum Severity {
        /// This value is used for findings when a source doesn't write a severity
        /// value.
        Unspecified = 0,
        /// Vulnerability:
        /// A critical vulnerability is easily discoverable by an external actor,
        /// exploitable, and results in the direct ability to execute arbitrary code,
        /// exfiltrate data, and otherwise gain additional access and privileges to
        /// cloud resources and workloads. Examples include publicly accessible
        /// unprotected user data and public SSH access with weak or no
        /// passwords.
        ///
        /// Threat:
        /// Indicates a threat that is able to access, modify, or delete data or
        /// execute unauthorized code within existing resources.
        Critical = 1,
        /// Vulnerability:
        /// A high risk vulnerability can be easily discovered and exploited in
        /// combination with other vulnerabilities in order to gain direct access and
        /// the ability to execute arbitrary code, exfiltrate data, and otherwise
        /// gain additional access and privileges to cloud resources and workloads.
        /// An example is a database with weak or no passwords that is only
        /// accessible internally. This database could easily be compromised by an
        /// actor that had access to the internal network.
        ///
        /// Threat:
        /// Indicates a threat that is able to create new computational resources in
        /// an environment but not able to access data or execute code in existing
        /// resources.
        High = 2,
        /// Vulnerability:
        /// A medium risk vulnerability could be used by an actor to gain access to
        /// resources or privileges that enable them to eventually (through multiple
        /// steps or a complex exploit) gain access and the ability to execute
        /// arbitrary code or exfiltrate data. An example is a service account with
        /// access to more projects than it should have. If an actor gains access to
        /// the service account, they could potentially use that access to manipulate
        /// a project the service account was not intended to.
        ///
        /// Threat:
        /// Indicates a threat that is able to cause operational impact but may not
        /// access data or execute unauthorized code.
        Medium = 3,
        /// Vulnerability:
        /// A low risk vulnerability hampers a security organization's ability to
        /// detect vulnerabilities or active threats in their deployment, or prevents
        /// the root cause investigation of security issues. An example is monitoring
        /// and logs being disabled for resource configurations and access.
        ///
        /// Threat:
        /// Indicates a threat that has obtained minimal access to an environment but
        /// is not able to access data, execute code, or create resources.
        Low = 4,
    }
    impl Severity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Severity::Unspecified => "SEVERITY_UNSPECIFIED",
                Severity::Critical => "CRITICAL",
                Severity::High => "HIGH",
                Severity::Medium => "MEDIUM",
                Severity::Low => "LOW",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SEVERITY_UNSPECIFIED" => Some(Self::Unspecified),
                "CRITICAL" => Some(Self::Critical),
                "HIGH" => Some(Self::High),
                "MEDIUM" => Some(Self::Medium),
                "LOW" => Some(Self::Low),
                _ => None,
            }
        }
    }
    /// Represents what kind of Finding it is.
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
    pub enum FindingClass {
        /// Unspecified finding class.
        Unspecified = 0,
        /// Describes unwanted or malicious activity.
        Threat = 1,
        /// Describes a potential weakness in software that increases risk to
        /// Confidentiality & Integrity & Availability.
        Vulnerability = 2,
        /// Describes a potential weakness in cloud resource/asset configuration that
        /// increases risk.
        Misconfiguration = 3,
        /// Describes a security observation that is for informational purposes.
        Observation = 4,
        /// Describes an error that prevents some SCC functionality.
        SccError = 5,
        /// Describes a potential security risk due to a change in the security
        /// posture.
        PostureViolation = 6,
    }
    impl FindingClass {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FindingClass::Unspecified => "FINDING_CLASS_UNSPECIFIED",
                FindingClass::Threat => "THREAT",
                FindingClass::Vulnerability => "VULNERABILITY",
                FindingClass::Misconfiguration => "MISCONFIGURATION",
                FindingClass::Observation => "OBSERVATION",
                FindingClass::SccError => "SCC_ERROR",
                FindingClass::PostureViolation => "POSTURE_VIOLATION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FINDING_CLASS_UNSPECIFIED" => Some(Self::Unspecified),
                "THREAT" => Some(Self::Threat),
                "VULNERABILITY" => Some(Self::Vulnerability),
                "MISCONFIGURATION" => Some(Self::Misconfiguration),
                "OBSERVATION" => Some(Self::Observation),
                "SCC_ERROR" => Some(Self::SccError),
                "POSTURE_VIOLATION" => Some(Self::PostureViolation),
                _ => None,
            }
        }
    }
}
/// Response message for simulating a `SecurityHealthAnalyticsCustomModule`
/// against a given resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimulateSecurityHealthAnalyticsCustomModuleResponse {
    /// Result for test case in the corresponding request.
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<
        simulate_security_health_analytics_custom_module_response::SimulatedResult,
    >,
}
/// Nested message and enum types in `SimulateSecurityHealthAnalyticsCustomModuleResponse`.
pub mod simulate_security_health_analytics_custom_module_response {
    /// Possible test result.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SimulatedResult {
        #[prost(oneof = "simulated_result::Result", tags = "1, 2, 3")]
        pub result: ::core::option::Option<simulated_result::Result>,
    }
    /// Nested message and enum types in `SimulatedResult`.
    pub mod simulated_result {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Result {
            /// Finding that would be published for the test case,
            /// if a violation is detected.
            #[prost(message, tag = "1")]
            Finding(super::super::SimulatedFinding),
            /// Indicates that the test case does not trigger any violation.
            #[prost(message, tag = "2")]
            NoViolation(()),
            /// Error encountered during the test.
            #[prost(message, tag = "3")]
            Error(super::super::super::super::super::rpc::Status),
        }
    }
}
/// An EffectiveEventThreatDetectionCustomModule is the representation of
/// EventThreatDetectionCustomModule at a given level taking hierarchy into
/// account and resolving various fields accordingly. e.g. if the module is
/// enabled at the ancestor level, effective modules at all descendant levels
/// will have enablement_state set to ENABLED. Similarly, if module.inherited is
/// set, then effective module's config will contain the ancestor's config
/// details. EffectiveEventThreatDetectionCustomModule is read-only.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EffectiveEventThreatDetectionCustomModule {
    /// Immutable. The resource name of the ETD custom module.
    ///
    /// Its format is:
    ///
    ///    * "organizations/{organization}/locations/{location}/effectiveEventThreatDetectionCustomModules/{effective_event_threat_detection_custom_module}".
    ///    * "folders/{folder}/locations/{location}/effectiveEventThreatDetectionCustomModules/{effective_event_threat_detection_custom_module}".
    ///    * "projects/{project}/locations/{location}/effectiveEventThreatDetectionCustomModules/{effective_event_threat_detection_custom_module}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Config for the effective module.
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<::prost_types::Struct>,
    /// Output only. The effective state of enablement for the module at the given
    /// level of the hierarchy.
    #[prost(
        enumeration = "effective_event_threat_detection_custom_module::EnablementState",
        tag = "3"
    )]
    pub enablement_state: i32,
    /// Output only. Type for the module. e.g. CONFIGURABLE_BAD_IP.
    #[prost(string, tag = "4")]
    pub r#type: ::prost::alloc::string::String,
    /// Output only. The human readable name to be displayed for the module.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The description for the module.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
}
/// Nested message and enum types in `EffectiveEventThreatDetectionCustomModule`.
pub mod effective_event_threat_detection_custom_module {
    /// The enablement state of the module.
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
    pub enum EnablementState {
        /// Unspecified enablement state.
        Unspecified = 0,
        /// The module is enabled at the given level.
        Enabled = 1,
        /// The module is disabled at the given level.
        Disabled = 2,
    }
    impl EnablementState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EnablementState::Unspecified => "ENABLEMENT_STATE_UNSPECIFIED",
                EnablementState::Enabled => "ENABLED",
                EnablementState::Disabled => "DISABLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENABLEMENT_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ENABLED" => Some(Self::Enabled),
                "DISABLED" => Some(Self::Disabled),
                _ => None,
            }
        }
    }
}
/// Request message for listing effective Event Threat Detection custom
/// modules.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEffectiveEventThreatDetectionCustomModulesRequest {
    /// Required. Name of parent to list effective custom modules. Its format is
    /// "organizations/{organization}/locations/{location}",
    /// "folders/{folder}/locations/{location}",
    /// or
    /// "projects/{project}/locations/{location}"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of results to return in a single response.
    /// Default is 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The value returned by the last call indicating a continuation
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for listing effective Event Threat Detection custom
/// modules.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEffectiveEventThreatDetectionCustomModulesResponse {
    /// The list of EffectiveEventThreatDetectionCustomModules
    #[prost(message, repeated, tag = "1")]
    pub effective_event_threat_detection_custom_modules: ::prost::alloc::vec::Vec<
        EffectiveEventThreatDetectionCustomModule,
    >,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Message for getting a EffectiveEventThreatDetectionCustomModule
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEffectiveEventThreatDetectionCustomModuleRequest {
    /// Required. The resource name of the ETD custom module.
    ///
    /// Its format is:
    ///
    ///    * "organizations/{organization}/locations/{location}/effectiveEventThreatDetectionCustomModules/{effective_event_threat_detection_custom_module}".
    ///    * "folders/{folder}/locations/{location}/effectiveEventThreatDetectionCustomModules/{effective_event_threat_detection_custom_module}".
    ///    * "projects/{project}/locations/{location}/effectiveEventThreatDetectionCustomModules/{effective_event_threat_detection_custom_module}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// An event threat detection custom module is a Cloud SCC resource that contains
/// the configuration and enablement state of a custom module, which enables ETD
/// to write certain findings to Cloud SCC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventThreatDetectionCustomModule {
    /// Immutable. The resource name of the ETD custom module.
    ///
    /// Its format is:
    ///
    ///    * "organizations/{organization}/locations/{location}/eventThreatDetectionCustomModules/{event_threat_detection_custom_module}".
    ///    * "folders/{folder}/locations/{location}/eventThreatDetectionCustomModules/{event_threat_detection_custom_module}".
    ///    * "projects/{project}/locations/{location}/eventThreatDetectionCustomModules/{event_threat_detection_custom_module}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Config for the module. For the resident module, its config value
    /// is defined at this level. For the inherited module, its config value is
    /// inherited from the ancestor module.
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<::prost_types::Struct>,
    /// Output only. The closest ancestor module that this module inherits the
    /// enablement state from. If empty, indicates that the custom module was
    /// created in the requesting parent organization, folder, or project. The
    /// format is the same as the EventThreatDetectionCustomModule resource name.
    #[prost(string, tag = "3")]
    pub ancestor_module: ::prost::alloc::string::String,
    /// Optional. The state of enablement for the module at the given level of the
    /// hierarchy.
    #[prost(
        enumeration = "event_threat_detection_custom_module::EnablementState",
        tag = "4"
    )]
    pub enablement_state: i32,
    /// Optional. Type for the module. e.g. CONFIGURABLE_BAD_IP.
    #[prost(string, tag = "5")]
    pub r#type: ::prost::alloc::string::String,
    /// Optional. The human readable name to be displayed for the module.
    #[prost(string, tag = "6")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. The description for the module.
    #[prost(string, tag = "7")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The time the module was last updated.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The editor the module was last updated by.
    #[prost(string, tag = "9")]
    pub last_editor: ::prost::alloc::string::String,
}
/// Nested message and enum types in `EventThreatDetectionCustomModule`.
pub mod event_threat_detection_custom_module {
    /// The enablement state of the module.
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
    pub enum EnablementState {
        /// Unspecified enablement state.
        Unspecified = 0,
        /// The module is enabled at the given level.
        Enabled = 1,
        /// The module is disabled at the given level.
        Disabled = 2,
        /// State is inherited from an ancestor module. The module will either
        /// be effectively ENABLED or DISABLED based on its closest non-inherited
        /// ancestor module in the CRM hierarchy. Attempting to set a top level
        /// module (module with no parent) to the INHERITED state will result in an
        /// error.
        Inherited = 3,
    }
    impl EnablementState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EnablementState::Unspecified => "ENABLEMENT_STATE_UNSPECIFIED",
                EnablementState::Enabled => "ENABLED",
                EnablementState::Disabled => "DISABLED",
                EnablementState::Inherited => "INHERITED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENABLEMENT_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ENABLED" => Some(Self::Enabled),
                "DISABLED" => Some(Self::Disabled),
                "INHERITED" => Some(Self::Inherited),
                _ => None,
            }
        }
    }
}
/// Request message for listing Event Threat Detection custom modules.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventThreatDetectionCustomModulesRequest {
    /// Required. Name of parent to list custom modules. Its format is
    /// "organizations/{organization}/locations/{location}",
    /// "folders/{folder}/locations/{location}",
    /// or
    /// "projects/{project}/locations/{location}"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of modules to return. The service may return
    /// fewer than this value. If unspecified, at most 10 configs will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous
    /// `ListEventThreatDetectionCustomModules` call. Provide this to retrieve the
    /// subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListEventThreatDetectionCustomModules` must match the call that provided
    /// the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for listing Event Threat Detection custom modules.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventThreatDetectionCustomModulesResponse {
    /// The list of EventThreatDetectionCustomModules
    #[prost(message, repeated, tag = "1")]
    pub event_threat_detection_custom_modules: ::prost::alloc::vec::Vec<
        EventThreatDetectionCustomModule,
    >,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for listing descendant Event Threat Detection custom
/// modules.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDescendantEventThreatDetectionCustomModulesRequest {
    /// Required. Name of parent to list custom modules. Its format is
    /// "organizations/{organization}/locations/{location}",
    /// "folders/{folder}/locations/{location}",
    /// or
    /// "projects/{project}/locations/{location}"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of modules to return. The service may return
    /// fewer than this value. If unspecified, at most 10 configs will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for listing descendant Event Threat Detection custom
/// modules.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDescendantEventThreatDetectionCustomModulesResponse {
    /// The list of EventThreatDetectionCustomModules
    #[prost(message, repeated, tag = "1")]
    pub event_threat_detection_custom_modules: ::prost::alloc::vec::Vec<
        EventThreatDetectionCustomModule,
    >,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Message for getting a EventThreatDetectionCustomModule
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventThreatDetectionCustomModuleRequest {
    /// Required. The resource name of the ETD custom module.
    ///
    /// Its format is:
    ///
    ///    * "organizations/{organization}/locations/{location}/eventThreatDetectionCustomModules/{event_threat_detection_custom_module}".
    ///    * "folders/{folder}/locations/{location}/eventThreatDetectionCustomModules/{event_threat_detection_custom_module}".
    ///    * "projects/{project}/locations/{location}/eventThreatDetectionCustomModules/{event_threat_detection_custom_module}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a EventThreatDetectionCustomModule
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEventThreatDetectionCustomModuleRequest {
    /// Required. Name of parent for the module. Its format is
    /// "organizations/{organization}/locations/{location}",
    /// "folders/{folder}/locations/{location}",
    /// or
    /// "projects/{project}/locations/{location}"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The module to create. The
    /// event_threat_detection_custom_module.name will be ignored and server
    /// generated.
    #[prost(message, optional, tag = "3")]
    pub event_threat_detection_custom_module: ::core::option::Option<
        EventThreatDetectionCustomModule,
    >,
    /// Optional. When set to true, only validations (including IAM checks) will
    /// done for the request (no module will be created). An OK response indicates
    /// the request is valid while an error response indicates the request is
    /// invalid. Note that a subsequent request to actually create the module could
    /// still fail because 1. the state could have changed (e.g. IAM permission
    /// lost) or
    /// 2. A failure occurred during creation of the module.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Message for updating a EventThreatDetectionCustomModule
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEventThreatDetectionCustomModuleRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// EventThreatDetectionCustomModule resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The module being updated
    #[prost(message, optional, tag = "2")]
    pub event_threat_detection_custom_module: ::core::option::Option<
        EventThreatDetectionCustomModule,
    >,
    /// Optional. When set to true, only validations (including IAM checks) will
    /// done for the request (module will not be updated). An OK response indicates
    /// the request is valid while an error response indicates the request is
    /// invalid. Note that a subsequent request to actually update the module could
    /// still fail because 1. the state could have changed (e.g. IAM permission
    /// lost) or
    /// 2. A failure occurred while trying to update the module.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// Message for deleting a EventThreatDetectionCustomModule
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEventThreatDetectionCustomModuleRequest {
    /// Required. The resource name of the ETD custom module.
    ///
    /// Its format is:
    ///
    ///    * "organizations/{organization}/locations/{location}/eventThreatDetectionCustomModules/{event_threat_detection_custom_module}".
    ///    * "folders/{folder}/locations/{location}/eventThreatDetectionCustomModules/{event_threat_detection_custom_module}".
    ///    * "projects/{project}/locations/{location}/eventThreatDetectionCustomModules/{event_threat_detection_custom_module}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. When set to true, only validations (including IAM checks) will
    /// done for the request (module will not be deleted). An OK response indicates
    /// the request is valid while an error response indicates the request is
    /// invalid. Note that a subsequent request to actually delete the module could
    /// still fail because 1. the state could have changed (e.g. IAM permission
    /// lost) or
    /// 2. A failure occurred while trying to delete the module.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
}
/// Request to validate an Event Threat Detection custom module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateEventThreatDetectionCustomModuleRequest {
    /// Required. Resource name of the parent to validate the Custom Module under.
    ///
    /// Its format is:
    ///
    ///    * "organizations/{organization}/locations/{location}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The raw text of the module's contents. Used to generate error
    /// messages.
    #[prost(string, tag = "2")]
    pub raw_text: ::prost::alloc::string::String,
    /// Required. The type of the module (e.g. CONFIGURABLE_BAD_IP).
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
}
/// Response to validating an Event Threat Detection custom module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateEventThreatDetectionCustomModuleResponse {
    /// A list of errors returned by the validator. If the list is empty, there
    /// were no errors.
    #[prost(message, repeated, tag = "2")]
    pub errors: ::prost::alloc::vec::Vec<
        validate_event_threat_detection_custom_module_response::CustomModuleValidationError,
    >,
}
/// Nested message and enum types in `ValidateEventThreatDetectionCustomModuleResponse`.
pub mod validate_event_threat_detection_custom_module_response {
    /// An error encountered while validating the uploaded configuration of an
    /// Event Threat Detection Custom Module.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomModuleValidationError {
        /// A description of the error, suitable for human consumption. Required.
        #[prost(string, tag = "1")]
        pub description: ::prost::alloc::string::String,
        /// The path, in RFC 8901 JSON Pointer format, to the field that failed
        /// validation. This may be left empty if no specific field is affected.
        #[prost(string, tag = "2")]
        pub field_path: ::prost::alloc::string::String,
        /// The initial position of the error in the uploaded text version of the
        /// module. This field may be omitted if no specific position applies, or if
        /// one could not be computed.
        #[prost(message, optional, tag = "3")]
        pub start: ::core::option::Option<Position>,
        /// The end position of the error in the uploaded text version of the
        /// module. This field may be omitted if no specific position applies, or if
        /// one could not be computed..
        #[prost(message, optional, tag = "4")]
        pub end: ::core::option::Option<Position>,
    }
    /// A position in the uploaded text version of a module.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Position {
        /// The line position in the text
        #[prost(int32, tag = "1")]
        pub line_number: i32,
        /// The column position in the line
        #[prost(int32, tag = "2")]
        pub column_number: i32,
    }
}
/// Generated client implementations.
pub mod security_center_management_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service describing handlers for resources
    #[derive(Debug, Clone)]
    pub struct SecurityCenterManagementClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SecurityCenterManagementClient<tonic::transport::Channel> {
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
    impl<T> SecurityCenterManagementClient<T>
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
        ) -> SecurityCenterManagementClient<InterceptedService<T, F>>
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
            SecurityCenterManagementClient::new(
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
        /// Returns a list of all EffectiveSecurityHealthAnalyticsCustomModules for the
        /// given parent. This includes resident modules defined at the scope of the
        /// parent, and inherited modules, inherited from CRM ancestors (no
        /// descendants).
        pub async fn list_effective_security_health_analytics_custom_modules(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListEffectiveSecurityHealthAnalyticsCustomModulesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::ListEffectiveSecurityHealthAnalyticsCustomModulesResponse,
            >,
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
                "/google.cloud.securitycentermanagement.v1.SecurityCenterManagement/ListEffectiveSecurityHealthAnalyticsCustomModules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycentermanagement.v1.SecurityCenterManagement",
                        "ListEffectiveSecurityHealthAnalyticsCustomModules",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single EffectiveSecurityHealthAnalyticsCustomModule.
        pub async fn get_effective_security_health_analytics_custom_module(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetEffectiveSecurityHealthAnalyticsCustomModuleRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::EffectiveSecurityHealthAnalyticsCustomModule>,
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
                "/google.cloud.securitycentermanagement.v1.SecurityCenterManagement/GetEffectiveSecurityHealthAnalyticsCustomModule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycentermanagement.v1.SecurityCenterManagement",
                        "GetEffectiveSecurityHealthAnalyticsCustomModule",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns a list of all SecurityHealthAnalyticsCustomModules for the given
        /// parent. This includes resident modules defined at the scope of the parent,
        /// and inherited modules, inherited from CRM ancestors (no descendants).
        pub async fn list_security_health_analytics_custom_modules(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListSecurityHealthAnalyticsCustomModulesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListSecurityHealthAnalyticsCustomModulesResponse>,
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
                "/google.cloud.securitycentermanagement.v1.SecurityCenterManagement/ListSecurityHealthAnalyticsCustomModules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycentermanagement.v1.SecurityCenterManagement",
                        "ListSecurityHealthAnalyticsCustomModules",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns a list of all resident SecurityHealthAnalyticsCustomModules under
        /// the given CRM parent and all of the parent’s CRM descendants.
        pub async fn list_descendant_security_health_analytics_custom_modules(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListDescendantSecurityHealthAnalyticsCustomModulesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::ListDescendantSecurityHealthAnalyticsCustomModulesResponse,
            >,
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
                "/google.cloud.securitycentermanagement.v1.SecurityCenterManagement/ListDescendantSecurityHealthAnalyticsCustomModules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycentermanagement.v1.SecurityCenterManagement",
                        "ListDescendantSecurityHealthAnalyticsCustomModules",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a SecurityHealthAnalyticsCustomModule.
        pub async fn get_security_health_analytics_custom_module(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetSecurityHealthAnalyticsCustomModuleRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SecurityHealthAnalyticsCustomModule>,
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
                "/google.cloud.securitycentermanagement.v1.SecurityCenterManagement/GetSecurityHealthAnalyticsCustomModule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycentermanagement.v1.SecurityCenterManagement",
                        "GetSecurityHealthAnalyticsCustomModule",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a resident SecurityHealthAnalyticsCustomModule at the scope of the
        /// given CRM parent, and also creates inherited
        /// SecurityHealthAnalyticsCustomModules for all CRM descendants of the given
        /// parent. These modules are enabled by default.
        pub async fn create_security_health_analytics_custom_module(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CreateSecurityHealthAnalyticsCustomModuleRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SecurityHealthAnalyticsCustomModule>,
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
                "/google.cloud.securitycentermanagement.v1.SecurityCenterManagement/CreateSecurityHealthAnalyticsCustomModule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycentermanagement.v1.SecurityCenterManagement",
                        "CreateSecurityHealthAnalyticsCustomModule",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the SecurityHealthAnalyticsCustomModule under the given name based
        /// on the given update mask. Updating the enablement state is supported on
        /// both resident and inherited modules (though resident modules cannot have an
        /// enablement state of "inherited"). Updating the display name and custom
        /// config of a module is supported on resident modules only.
        pub async fn update_security_health_analytics_custom_module(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UpdateSecurityHealthAnalyticsCustomModuleRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SecurityHealthAnalyticsCustomModule>,
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
                "/google.cloud.securitycentermanagement.v1.SecurityCenterManagement/UpdateSecurityHealthAnalyticsCustomModule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycentermanagement.v1.SecurityCenterManagement",
                        "UpdateSecurityHealthAnalyticsCustomModule",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified SecurityHealthAnalyticsCustomModule and all of its
        /// descendants in the CRM hierarchy. This method is only supported for
        /// resident custom modules.
        pub async fn delete_security_health_analytics_custom_module(
            &mut self,
            request: impl tonic::IntoRequest<
                super::DeleteSecurityHealthAnalyticsCustomModuleRequest,
            >,
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
                "/google.cloud.securitycentermanagement.v1.SecurityCenterManagement/DeleteSecurityHealthAnalyticsCustomModule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycentermanagement.v1.SecurityCenterManagement",
                        "DeleteSecurityHealthAnalyticsCustomModule",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Simulates a given SecurityHealthAnalyticsCustomModule and Resource.
        pub async fn simulate_security_health_analytics_custom_module(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SimulateSecurityHealthAnalyticsCustomModuleRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SimulateSecurityHealthAnalyticsCustomModuleResponse>,
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
                "/google.cloud.securitycentermanagement.v1.SecurityCenterManagement/SimulateSecurityHealthAnalyticsCustomModule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycentermanagement.v1.SecurityCenterManagement",
                        "SimulateSecurityHealthAnalyticsCustomModule",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all effective Event Threat Detection custom modules for the
        /// given parent. This includes resident modules defined at the scope of the
        /// parent along with modules inherited from its ancestors.
        pub async fn list_effective_event_threat_detection_custom_modules(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListEffectiveEventThreatDetectionCustomModulesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::ListEffectiveEventThreatDetectionCustomModulesResponse,
            >,
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
                "/google.cloud.securitycentermanagement.v1.SecurityCenterManagement/ListEffectiveEventThreatDetectionCustomModules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycentermanagement.v1.SecurityCenterManagement",
                        "ListEffectiveEventThreatDetectionCustomModules",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets an effective ETD custom module. Retrieves the effective module at the
        /// given level. The difference between an EffectiveCustomModule and a
        /// CustomModule is that the fields for an EffectiveCustomModule are computed
        /// from ancestors if needed. For example, the enablement_state for a
        /// CustomModule can be either ENABLED, DISABLED, or INHERITED. Where as the
        /// enablement_state for an EffectiveCustomModule is always computed to ENABLED
        /// or DISABLED (the effective enablement_state).
        pub async fn get_effective_event_threat_detection_custom_module(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetEffectiveEventThreatDetectionCustomModuleRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::EffectiveEventThreatDetectionCustomModule>,
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
                "/google.cloud.securitycentermanagement.v1.SecurityCenterManagement/GetEffectiveEventThreatDetectionCustomModule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycentermanagement.v1.SecurityCenterManagement",
                        "GetEffectiveEventThreatDetectionCustomModule",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all Event Threat Detection custom modules for the given
        /// Resource Manager parent. This includes resident modules defined at the
        /// scope of the parent along with modules inherited from ancestors.
        pub async fn list_event_threat_detection_custom_modules(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListEventThreatDetectionCustomModulesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListEventThreatDetectionCustomModulesResponse>,
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
                "/google.cloud.securitycentermanagement.v1.SecurityCenterManagement/ListEventThreatDetectionCustomModules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycentermanagement.v1.SecurityCenterManagement",
                        "ListEventThreatDetectionCustomModules",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all resident Event Threat Detection custom modules under the
        /// given Resource Manager parent and its descendants.
        pub async fn list_descendant_event_threat_detection_custom_modules(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListDescendantEventThreatDetectionCustomModulesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::ListDescendantEventThreatDetectionCustomModulesResponse,
            >,
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
                "/google.cloud.securitycentermanagement.v1.SecurityCenterManagement/ListDescendantEventThreatDetectionCustomModules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycentermanagement.v1.SecurityCenterManagement",
                        "ListDescendantEventThreatDetectionCustomModules",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets an Event Threat Detection custom module.
        pub async fn get_event_threat_detection_custom_module(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetEventThreatDetectionCustomModuleRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::EventThreatDetectionCustomModule>,
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
                "/google.cloud.securitycentermanagement.v1.SecurityCenterManagement/GetEventThreatDetectionCustomModule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycentermanagement.v1.SecurityCenterManagement",
                        "GetEventThreatDetectionCustomModule",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a resident Event Threat Detection custom module at the scope of the
        /// given Resource Manager parent, and also creates inherited custom modules
        /// for all descendants of the given parent. These modules are enabled by
        /// default.
        pub async fn create_event_threat_detection_custom_module(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CreateEventThreatDetectionCustomModuleRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::EventThreatDetectionCustomModule>,
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
                "/google.cloud.securitycentermanagement.v1.SecurityCenterManagement/CreateEventThreatDetectionCustomModule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycentermanagement.v1.SecurityCenterManagement",
                        "CreateEventThreatDetectionCustomModule",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the Event Threat Detection custom module with the given name based
        /// on the given update mask. Updating the enablement state is supported for
        /// both resident and inherited modules (though resident modules cannot have an
        /// enablement state of "inherited"). Updating the display name or
        /// configuration of a module is supported for resident modules only. The type
        /// of a module cannot be changed.
        pub async fn update_event_threat_detection_custom_module(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UpdateEventThreatDetectionCustomModuleRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::EventThreatDetectionCustomModule>,
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
                "/google.cloud.securitycentermanagement.v1.SecurityCenterManagement/UpdateEventThreatDetectionCustomModule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycentermanagement.v1.SecurityCenterManagement",
                        "UpdateEventThreatDetectionCustomModule",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified Event Threat Detection custom module and all of its
        /// descendants in the Resource Manager hierarchy. This method is only
        /// supported for resident custom modules.
        pub async fn delete_event_threat_detection_custom_module(
            &mut self,
            request: impl tonic::IntoRequest<
                super::DeleteEventThreatDetectionCustomModuleRequest,
            >,
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
                "/google.cloud.securitycentermanagement.v1.SecurityCenterManagement/DeleteEventThreatDetectionCustomModule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycentermanagement.v1.SecurityCenterManagement",
                        "DeleteEventThreatDetectionCustomModule",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Validates the given Event Threat Detection custom module.
        pub async fn validate_event_threat_detection_custom_module(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ValidateEventThreatDetectionCustomModuleRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ValidateEventThreatDetectionCustomModuleResponse>,
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
                "/google.cloud.securitycentermanagement.v1.SecurityCenterManagement/ValidateEventThreatDetectionCustomModule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securitycentermanagement.v1.SecurityCenterManagement",
                        "ValidateEventThreatDetectionCustomModule",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
