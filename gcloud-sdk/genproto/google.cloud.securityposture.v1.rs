/// A rule used to express this policy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyRule {
    /// A condition which determines whether this rule is used
    /// in the evaluation of the policy. When set, the `expression` field in
    /// the `Expr' must include from 1 to 10 subexpressions, joined by the "||"
    /// or "&&" operators. Each subexpression must be of the form
    /// "resource.matchTag('<ORG_ID>/tag_key_short_name,
    /// 'tag_value_short_name')" or "resource.matchTagId('tagKeys/key_id',
    /// 'tagValues/value_id')" where key_name and value_name are the resource
    /// names for Label Keys and Values. These names are available from the Tag
    /// Manager Service. An example expression is:
    /// "resource.matchTag('123456789/environment,
    /// 'prod')" or "resource.matchTagId('tagKeys/123',
    /// 'tagValues/456')".
    #[prost(message, optional, tag = "5")]
    pub condition: ::core::option::Option<super::super::super::r#type::Expr>,
    #[prost(oneof = "policy_rule::Kind", tags = "1, 2, 3, 4")]
    pub kind: ::core::option::Option<policy_rule::Kind>,
}
/// Nested message and enum types in `PolicyRule`.
pub mod policy_rule {
    /// A message that holds specific allowed and denied values.
    /// This message can define specific values and subtrees of the Resource
    /// Manager resource hierarchy (`Organizations`, `Folders`, `Projects`) that
    /// are allowed or denied. This is achieved by using the `under:` and
    /// optional `is:` prefixes.
    /// The `under:` prefix is used to denote resource subtree values.
    /// The `is:` prefix is used to denote specific values, and is required only
    /// if the value contains a ":". Values prefixed with "is:" are treated the
    /// same as values with no prefix.
    /// Ancestry subtrees must be in one of the following formats:
    ///
    /// - `projects/<project-id>` (for example, `projects/tokyo-rain-123`)
    /// - `folders/<folder-id>` (for example, `folders/1234`)
    /// - `organizations/<organization-id>` (for example, `organizations/1234`)
    ///
    /// The `supports_under` field of the associated `Constraint`  defines
    /// whether ancestry prefixes can be used.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StringValues {
        /// List of values allowed at this resource.
        #[prost(string, repeated, tag = "1")]
        pub allowed_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// List of values denied at this resource.
        #[prost(string, repeated, tag = "2")]
        pub denied_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// List of values to be used for this policy rule. This field can be set
        /// only in policies for list constraints.
        #[prost(message, tag = "1")]
        Values(StringValues),
        /// Setting this to true means that all values are allowed. This field can
        /// be set only in policies for list constraints.
        #[prost(bool, tag = "2")]
        AllowAll(bool),
        /// Setting this to true means that all values are denied. This field can
        /// be set only in policies for list constraints.
        #[prost(bool, tag = "3")]
        DenyAll(bool),
        /// If `true`, then the policy is enforced. If `false`, then any
        /// configuration is acceptable.
        /// This field can be set only in policies for boolean constraints.
        #[prost(bool, tag = "4")]
        Enforce(bool),
    }
}
/// A custom constraint defined by customers which can *only* be applied to the
/// given resource types and organization.
///
/// By creating a custom constraint, customers can apply policies of this
/// custom constraint. *Creating a custom constraint itself does NOT apply any
/// policy enforcement*.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomConstraint {
    /// Immutable. Name of the constraint. This is unique within the organization.
    /// Format of the name should be
    ///
    /// -
    /// `organizations/{organization_id}/customConstraints/{custom_constraint_id}`
    ///
    /// Example: `organizations/123/customConstraints/custom.createOnlyE2TypeVms`
    ///
    /// The max length is 70 characters and the minimum length is 1. Note that the
    /// prefix `organizations/{organization_id}/customConstraints/` is not counted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The resource instance type on which this policy applies. Format
    /// will be of the form : `<canonical service name>/<type>` Example:
    ///
    ///   - `compute.googleapis.com/Instance`.
    #[prost(string, repeated, tag = "2")]
    pub resource_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// All the operations being applied for this constraint.
    #[prost(enumeration = "custom_constraint::MethodType", repeated, tag = "3")]
    pub method_types: ::prost::alloc::vec::Vec<i32>,
    /// Org policy condition/expression. For example:
    /// `resource.instanceName.matches("\[production|test\]_.*_(\d)+")` or,
    /// `resource.management.auto_upgrade == true`
    ///
    /// The max length of the condition is 1000 characters.
    #[prost(string, tag = "4")]
    pub condition: ::prost::alloc::string::String,
    /// Allow or deny type.
    #[prost(enumeration = "custom_constraint::ActionType", tag = "5")]
    pub action_type: i32,
    /// One line display name for the UI.
    /// The max length of the display_name is 200 characters.
    #[prost(string, tag = "6")]
    pub display_name: ::prost::alloc::string::String,
    /// Detailed information about this custom policy constraint.
    /// The max length of the description is 2000 characters.
    #[prost(string, tag = "7")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The last time this custom constraint was updated. This
    /// represents the last time that the `CreateCustomConstraint` or
    /// `UpdateCustomConstraint` RPC was called
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `CustomConstraint`.
pub mod custom_constraint {
    /// The operation for which this constraint will be applied. To apply this
    /// constraint only when creating new VMs, the `method_types` should be
    /// `CREATE` only. To apply this constraint when creating or deleting
    /// VMs, the `method_types` should be `CREATE` and `DELETE`.
    ///
    /// `UPDATE` only custom constraints are not supported. Use `CREATE` or
    /// `CREATE, UPDATE`.
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
    pub enum MethodType {
        /// Unspecified. Results in an error.
        Unspecified = 0,
        /// Constraint applied when creating the resource.
        Create = 1,
        /// Constraint applied when updating the resource.
        Update = 2,
        /// Constraint applied when deleting the resource.
        /// Not supported yet.
        Delete = 3,
    }
    impl MethodType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MethodType::Unspecified => "METHOD_TYPE_UNSPECIFIED",
                MethodType::Create => "CREATE",
                MethodType::Update => "UPDATE",
                MethodType::Delete => "DELETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "METHOD_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATE" => Some(Self::Create),
                "UPDATE" => Some(Self::Update),
                "DELETE" => Some(Self::Delete),
                _ => None,
            }
        }
    }
    /// Allow or deny type.
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
    pub enum ActionType {
        /// Unspecified. Results in an error.
        Unspecified = 0,
        /// Allowed action type.
        Allow = 1,
        /// Deny action type.
        Deny = 2,
    }
    impl ActionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ActionType::Unspecified => "ACTION_TYPE_UNSPECIFIED",
                ActionType::Allow => "ALLOW",
                ActionType::Deny => "DENY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACTION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "ALLOW" => Some(Self::Allow),
                "DENY" => Some(Self::Deny),
                _ => None,
            }
        }
    }
}
/// Message for Org Policy Canned Constraint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrgPolicyConstraint {
    /// Required. Org Policy Canned Constraint id.
    #[prost(string, tag = "1")]
    pub canned_constraint_id: ::prost::alloc::string::String,
    /// Required. Org PolicySpec rules.
    #[prost(message, repeated, tag = "2")]
    pub policy_rules: ::prost::alloc::vec::Vec<PolicyRule>,
}
/// Message for Org Policy Custom Constraint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrgPolicyConstraintCustom {
    /// Required. Org Policy Custom Constraint.
    #[prost(message, optional, tag = "1")]
    pub custom_constraint: ::core::option::Option<CustomConstraint>,
    /// Required. Org Policyspec rules.
    #[prost(message, repeated, tag = "2")]
    pub policy_rules: ::prost::alloc::vec::Vec<PolicyRule>,
}
/// Defines the properties in a custom module configuration for Security
/// Health Analytics. Use the custom module configuration to create custom
/// detectors that generate custom findings for resources that you specify.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomConfig {
    /// Required. The CEL expression to evaluate to produce findings. When the
    /// expression evaluates to true against a resource, a finding is generated.
    #[prost(message, optional, tag = "1")]
    pub predicate: ::core::option::Option<super::super::super::r#type::Expr>,
    /// Optional. Custom output properties.
    #[prost(message, optional, tag = "2")]
    pub custom_output: ::core::option::Option<custom_config::CustomOutputSpec>,
    /// Required. The resource types that the custom module operates on. Each
    /// custom module can specify up to 5 resource types.
    #[prost(message, optional, tag = "3")]
    pub resource_selector: ::core::option::Option<custom_config::ResourceSelector>,
    /// Required. The severity to assign to findings generated by the module.
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
            /// Required. Name of the property for the custom output.
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
        /// Required. The resource types to run the detector on.
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
/// Message for Security Health Analytics built-in detector.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityHealthAnalyticsModule {
    /// Required. The name of the module eg: BIGQUERY_TABLE_CMEK_DISABLED.
    #[prost(string, tag = "1")]
    pub module_name: ::prost::alloc::string::String,
    /// The state of enablement for the module at its level of the resource
    /// hierarchy.
    #[prost(enumeration = "EnablementState", tag = "2")]
    pub module_enablement_state: i32,
}
/// Message for SHA Custom Module
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityHealthAnalyticsCustomModule {
    /// Output only. Immutable. The id of the custom module.
    /// The id is server-generated and is not user settable.
    /// It will be a numeric id containing 1-20 digits.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Optional. The display name of the Security Health Analytics custom module.
    /// This display name becomes the finding category for all findings that are
    /// returned by this custom module. The display name must be between 1 and
    /// 128 characters, start with a lowercase letter, and contain alphanumeric
    /// characters or underscores only.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. custom module details
    #[prost(message, optional, tag = "3")]
    pub config: ::core::option::Option<CustomConfig>,
    /// The state of enablement for the module at its level of the resource
    /// hierarchy.
    #[prost(enumeration = "EnablementState", tag = "4")]
    pub module_enablement_state: i32,
}
/// Possible enablement states of a service or module.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EnablementState {
    /// Default value. This value is unused.
    Unspecified = 0,
    /// State is enabled.
    Enabled = 1,
    /// State is disabled.
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
    /// Output only. This is a output only optional field which will be filled only
    /// in cases where PostureDeployments enter failure states like UPDATE_FAILED
    /// or CREATE_FAILED or DELETE_FAILED.
    #[prost(string, tag = "8")]
    pub error_message: ::prost::alloc::string::String,
}
/// Postures
/// Definition of a Posture.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Posture {
    /// Required. Identifier. The name of this Posture resource, in the format of
    /// organizations/{org_id}/locations/{location_id}/postures/{posture}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. State of Posture resource.
    #[prost(enumeration = "posture::State", tag = "2")]
    pub state: i32,
    /// Output only. Immutable. The revision ID of the posture.
    /// The format is an 8-character hexadecimal string.
    /// <https://google.aip.dev/162>
    #[prost(string, tag = "3")]
    pub revision_id: ::prost::alloc::string::String,
    /// Output only. The timestamp that the posture was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp that the posture was updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. User provided description of the posture.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    /// Required. List of Policy sets.
    #[prost(message, repeated, tag = "7")]
    pub policy_sets: ::prost::alloc::vec::Vec<PolicySet>,
    /// Optional. An opaque tag indicating the current version of the Posture, used
    /// for concurrency control. When the `Posture` is returned from either a
    /// `GetPosture` or a `ListPostures` request, this `etag` indicates the version
    /// of the current `Posture` to use when executing a read-modify-write loop.
    ///
    /// When the `Posture` is used in a `UpdatePosture` method, use the `etag`
    /// value that was returned from a `GetPosture` request as part of a
    /// read-modify-write loop for concurrency control. Not setting the `etag` in a
    /// `UpdatePosture` request will result in an unconditional write of the
    /// `Posture`.
    #[prost(string, tag = "8")]
    pub etag: ::prost::alloc::string::String,
    /// Optional. User annotations. These attributes can only be set and used by
    /// the user, and not by Google Security Postures.
    /// .
    #[prost(map = "string, string", tag = "9")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Whether or not this Posture is in the process of being
    /// updated.
    #[prost(bool, tag = "10")]
    pub reconciling: bool,
}
/// Nested message and enum types in `Posture`.
pub mod posture {
    /// State of a Posture.
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
        /// Unspecified operation state.
        Unspecified = 0,
        /// The Posture is marked deprecated when it is not in use by the user.
        Deprecated = 1,
        /// The Posture is created successfully but is not yet ready for usage.
        Draft = 2,
        /// The Posture state is active. Ready for use/deployments.
        Active = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Deprecated => "DEPRECATED",
                State::Draft => "DRAFT",
                State::Active => "ACTIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "DEPRECATED" => Some(Self::Deprecated),
                "DRAFT" => Some(Self::Draft),
                "ACTIVE" => Some(Self::Active),
                _ => None,
            }
        }
    }
}
/// PolicySet representation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicySet {
    /// Required. ID of the Policy set.
    #[prost(string, tag = "1")]
    pub policy_set_id: ::prost::alloc::string::String,
    /// Optional. Description of the Policy set.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Required. List of policies.
    #[prost(message, repeated, tag = "3")]
    pub policies: ::prost::alloc::vec::Vec<Policy>,
}
/// Policy representation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Policy {
    /// Required. ID of the Policy that is user generated, immutable and unique
    /// within the scope of a policy set.
    #[prost(string, tag = "1")]
    pub policy_id: ::prost::alloc::string::String,
    /// Optional. Contains list of mapping for a Policy to a standard and control.
    #[prost(message, repeated, tag = "2")]
    pub compliance_standards: ::prost::alloc::vec::Vec<policy::ComplianceStandard>,
    /// Required. Constraint details.
    #[prost(message, optional, tag = "3")]
    pub constraint: ::core::option::Option<Constraint>,
    /// Optional. Description of the Policy.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Policy`.
pub mod policy {
    /// Mapping for a Policy to standard and control.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ComplianceStandard {
        /// Optional. The compliance standard that the Policy maps to, e.g.: CIS-2.0.
        #[prost(string, tag = "1")]
        pub standard: ::prost::alloc::string::String,
        /// Optional. Control mapping provided by user for this Policy. e.g.: 1.5.
        #[prost(string, tag = "2")]
        pub control: ::prost::alloc::string::String,
    }
}
/// Representation of a Constraint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Constraint {
    #[prost(oneof = "constraint::Implementation", tags = "3, 4, 5, 6")]
    pub implementation: ::core::option::Option<constraint::Implementation>,
}
/// Nested message and enum types in `Constraint`.
pub mod constraint {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Implementation {
        /// Optional. SHA built-in detector.
        #[prost(message, tag = "3")]
        SecurityHealthAnalyticsModule(super::SecurityHealthAnalyticsModule),
        /// Optional. SHA custom detector.
        #[prost(message, tag = "4")]
        SecurityHealthAnalyticsCustomModule(super::SecurityHealthAnalyticsCustomModule),
        /// Optional. Org Policy builtin constraint.
        #[prost(message, tag = "5")]
        OrgPolicyConstraint(super::OrgPolicyConstraint),
        /// Optional. Org Policy custom constraint.
        #[prost(message, tag = "6")]
        OrgPolicyConstraintCustom(super::OrgPolicyConstraintCustom),
    }
}
/// Message for requesting list of Postures.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPosturesRequest {
    /// Required. Parent value for ListPosturesRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Message for response to listing Postures.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPosturesResponse {
    /// The list of Posture.
    #[prost(message, repeated, tag = "1")]
    pub postures: ::prost::alloc::vec::Vec<Posture>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Unreachable resources.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for requesting list of Posture revisions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPostureRevisionsRequest {
    /// Required. Name value for ListPostureRevisionsRequest.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer items than
    /// requested. If unspecified, server will pick 100 as default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Message for response to listing PostureRevisions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPostureRevisionsResponse {
    /// The list of Posture revisions.
    #[prost(message, repeated, tag = "1")]
    pub revisions: ::prost::alloc::vec::Vec<Posture>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Message for getting a Posture.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPostureRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Posture revision which needs to be retrieved.
    #[prost(string, tag = "3")]
    pub revision_id: ::prost::alloc::string::String,
}
/// Message for creating a Posture.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePostureRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User provided identifier. It should be unique in scope of an
    /// Organization and location.
    #[prost(string, tag = "2")]
    pub posture_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "3")]
    pub posture: ::core::option::Option<Posture>,
}
/// Message for updating a Posture.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePostureRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// Posture resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated.
    #[prost(message, optional, tag = "2")]
    pub posture: ::core::option::Option<Posture>,
    /// Required. Posture revision which needs to be updated.
    #[prost(string, tag = "3")]
    pub revision_id: ::prost::alloc::string::String,
}
/// Message for deleting a Posture.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePostureRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Etag value of the Posture to be deleted.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
}
/// Message for extracting existing policies on a workload as a Posture.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtractPostureRequest {
    /// Required. The parent resource name. The format of this value is as follows:
    /// `organizations/{organization}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User provided identifier. It should be unique in scope of an
    /// Organization and location.
    #[prost(string, tag = "2")]
    pub posture_id: ::prost::alloc::string::String,
    /// Required. Workload from which the policies are to be extracted, it should
    /// belong to the same organization defined in parent. The format of this value
    /// varies depending on the scope of the request:
    /// - `folder/folderNumber`
    /// - `project/projectNumber`
    /// - `organization/organizationNumber`
    #[prost(string, tag = "3")]
    pub workload: ::prost::alloc::string::String,
}
/// ========================== PostureDeployments ==========================
/// Message describing PostureDeployment resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostureDeployment {
    /// Required. The name of this PostureDeployment resource, in the format of
    /// organizations/{organization}/locations/{location_id}/postureDeployments/{postureDeployment}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Target resource where the Posture will be deployed. Currently
    /// supported resources are of types: projects/projectNumber,
    /// folders/folderNumber, organizations/organizationNumber.
    #[prost(string, tag = "13")]
    pub target_resource: ::prost::alloc::string::String,
    /// Output only. State of PostureDeployment resource.
    #[prost(enumeration = "posture_deployment::State", tag = "2")]
    pub state: i32,
    /// Required. Posture that needs to be deployed.
    /// Format:
    /// organizations/{org_id}/locations/{location_id}/postures/<posture>
    /// Example:
    /// organizations/99/locations/global/postures/les-miserables.
    #[prost(string, tag = "3")]
    pub posture_id: ::prost::alloc::string::String,
    /// Required. Revision_id of the Posture that is to be deployed.
    #[prost(string, tag = "4")]
    pub posture_revision_id: ::prost::alloc::string::String,
    /// Output only. The timestamp that the PostureDeployment was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp that the PostureDeployment was updated.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. User provided description of the PostureDeployment.
    #[prost(string, tag = "7")]
    pub description: ::prost::alloc::string::String,
    /// Optional. An opaque tag indicating the current version of the
    /// PostureDeployment, used for concurrency control. When the
    /// `PostureDeployment` is returned from either a `GetPostureDeployment` or a
    /// `ListPostureDeployments` request, this `etag` indicates the version of the
    /// current `PostureDeployment` to use when executing a read-modify-write loop.
    ///
    /// When the `PostureDeployment` is used in a `UpdatePostureDeployment` method,
    /// use the `etag` value that was returned from a `GetPostureDeployment`
    /// request as part of a read-modify-write loop for concurrency control. Not
    /// setting the `etag` in a `UpdatePostureDeployment` request will result in an
    /// unconditional write of the `PostureDeployment`.
    #[prost(string, tag = "8")]
    pub etag: ::prost::alloc::string::String,
    /// Optional. User annotations. These attributes can only be set and used by
    /// the user, and not by Google Security Postures.
    /// .
    #[prost(map = "string, string", tag = "9")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Whether or not this Posture is in the process of being
    /// updated.
    #[prost(bool, tag = "10")]
    pub reconciling: bool,
    /// Output only. This is a output only optional field which will be filled in
    /// case where PostureDeployment state is UPDATE_FAILED or CREATE_FAILED or
    /// DELETE_FAILED. It denotes the desired Posture.
    #[prost(string, tag = "11")]
    pub desired_posture_id: ::prost::alloc::string::String,
    /// Output only. Output only optional field which provides revision_id of the
    /// desired_posture_id.
    #[prost(string, tag = "12")]
    pub desired_posture_revision_id: ::prost::alloc::string::String,
    /// Output only. This is a output only optional field which will be filled in
    /// case where PostureDeployment enters a failure state like UPDATE_FAILED or
    /// CREATE_FAILED or DELETE_FAILED.
    #[prost(string, tag = "14")]
    pub failure_message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `PostureDeployment`.
pub mod posture_deployment {
    /// State of a PostureDeployment.
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
        /// Unspecified operation state.
        Unspecified = 0,
        /// The PostureDeployment is being created.
        Creating = 1,
        /// The PostureDeployment is being deleted.
        Deleting = 2,
        /// The PostureDeployment state is being updated.
        Updating = 3,
        /// The PostureDeployment state is active and in use.
        Active = 4,
        /// The PostureDeployment creation failed.
        CreateFailed = 5,
        /// The PostureDeployment update failed.
        UpdateFailed = 6,
        /// The PostureDeployment deletion failed.
        DeleteFailed = 7,
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
                State::Deleting => "DELETING",
                State::Updating => "UPDATING",
                State::Active => "ACTIVE",
                State::CreateFailed => "CREATE_FAILED",
                State::UpdateFailed => "UPDATE_FAILED",
                State::DeleteFailed => "DELETE_FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "DELETING" => Some(Self::Deleting),
                "UPDATING" => Some(Self::Updating),
                "ACTIVE" => Some(Self::Active),
                "CREATE_FAILED" => Some(Self::CreateFailed),
                "UPDATE_FAILED" => Some(Self::UpdateFailed),
                "DELETE_FAILED" => Some(Self::DeleteFailed),
                _ => None,
            }
        }
    }
}
/// Message for requesting list of PostureDeployments.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPostureDeploymentsRequest {
    /// Required. Parent value for ListPostureDeploymentsRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer items than
    /// requested. If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter to be applied on the resource, defined by EBNF grammar
    /// <https://google.aip.dev/assets/misc/ebnf-filtering.txt.>
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Message for response to listing PostureDeployments.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPostureDeploymentsResponse {
    /// The list of PostureDeployment.
    #[prost(message, repeated, tag = "1")]
    pub posture_deployments: ::prost::alloc::vec::Vec<PostureDeployment>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a PostureDeployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPostureDeploymentRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a PostureDeployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePostureDeploymentRequest {
    /// Required. Value for parent.
    /// Format: organizations/{org_id}/locations/{location}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User provided identifier. It should be unique in scope of an
    /// Organization and location.
    #[prost(string, tag = "2")]
    pub posture_deployment_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "3")]
    pub posture_deployment: ::core::option::Option<PostureDeployment>,
}
/// Message for updating a PostureDeployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePostureDeploymentRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// PostureDeployment resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated.
    #[prost(message, optional, tag = "2")]
    pub posture_deployment: ::core::option::Option<PostureDeployment>,
}
/// Message for deleting a PostureDeployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePostureDeploymentRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Etag value of the PostureDeployment to be deleted.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
}
/// PostureTemplates
/// Message describing PostureTemplate object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostureTemplate {
    /// Output only. Identifier. The name of the Posture template will be of the
    /// format
    /// organizations/{organization}/locations/{location}/postureTemplates/{postureTemplate}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The revision_id of a PostureTemplate.
    #[prost(string, tag = "2")]
    pub revision_id: ::prost::alloc::string::String,
    /// Output only. Description of the Posture template.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. State of PostureTemplate resource.
    #[prost(enumeration = "posture_template::State", tag = "4")]
    pub state: i32,
    /// Output only. Policy_sets to be used by the user.
    #[prost(message, repeated, tag = "5")]
    pub policy_sets: ::prost::alloc::vec::Vec<PolicySet>,
}
/// Nested message and enum types in `PostureTemplate`.
pub mod posture_template {
    /// State of a PostureTemplate
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
        /// Unspecified state
        Unspecified = 0,
        /// If the Posture template is adhering to the latest controls and standards.
        Active = 1,
        /// If the Posture template controls and standards are outdated and not
        /// recommended for use.
        Deprecated = 2,
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
                State::Deprecated => "DEPRECATED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "DEPRECATED" => Some(Self::Deprecated),
                _ => None,
            }
        }
    }
}
/// Message for requesting list of Posture Templates.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPostureTemplatesRequest {
    /// Required. Parent value for ListPostureTemplatesRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer items than
    /// requested. If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter to be applied on the resource, defined by EBNF grammar
    /// <https://google.aip.dev/assets/misc/ebnf-filtering.txt.>
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Message for response to listing PostureTemplates.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPostureTemplatesResponse {
    /// The list of PostureTemplate.
    #[prost(message, repeated, tag = "1")]
    pub posture_templates: ::prost::alloc::vec::Vec<PostureTemplate>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Message for getting a Posture Template.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPostureTemplateRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Specific revision_id of a Posture Template.
    /// PostureTemplate revision_id which needs to be retrieved.
    #[prost(string, tag = "2")]
    pub revision_id: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod security_posture_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service describing handlers for resources.
    #[derive(Debug, Clone)]
    pub struct SecurityPostureClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SecurityPostureClient<tonic::transport::Channel> {
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
    impl<T> SecurityPostureClient<T>
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
        ) -> SecurityPostureClient<InterceptedService<T, F>>
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
            SecurityPostureClient::new(InterceptedService::new(inner, interceptor))
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
        /// (-- This option restricts the visibility of the API to only projects that
        /// will
        /// (-- be labeled as `PREVIEW` or `GOOGLE_INTERNAL` by the service.
        /// (-- option (google.api.api_visibility).restriction =
        /// "PREVIEW,GOOGLE_INTERNAL"; Postures Lists Postures in a given organization
        /// and location. In case a posture has multiple revisions, the latest revision
        /// as per UpdateTime will be returned.
        pub async fn list_postures(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPosturesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPosturesResponse>,
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
                "/google.cloud.securityposture.v1.SecurityPosture/ListPostures",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securityposture.v1.SecurityPosture",
                        "ListPostures",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists revisions of a Posture in a given organization and location.
        pub async fn list_posture_revisions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPostureRevisionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPostureRevisionsResponse>,
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
                "/google.cloud.securityposture.v1.SecurityPosture/ListPostureRevisions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securityposture.v1.SecurityPosture",
                        "ListPostureRevisions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a posture in a given organization and location.
        /// User must provide revision_id to retrieve a specific revision of the
        /// resource.
        /// NOT_FOUND error is returned if the revision_id or the Posture name does not
        /// exist. In case revision_id is not provided then the latest Posture revision
        /// by UpdateTime is returned.
        pub async fn get_posture(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPostureRequest>,
        ) -> std::result::Result<tonic::Response<super::Posture>, tonic::Status> {
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
                "/google.cloud.securityposture.v1.SecurityPosture/GetPosture",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securityposture.v1.SecurityPosture",
                        "GetPosture",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new Posture resource.
        /// If a Posture with the specified name already exists in the specified
        /// organization and location, the long running operation returns a
        /// [ALREADY_EXISTS][google.rpc.Code.ALREADY_EXISTS] error.
        pub async fn create_posture(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePostureRequest>,
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
                "/google.cloud.securityposture.v1.SecurityPosture/CreatePosture",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securityposture.v1.SecurityPosture",
                        "CreatePosture",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an existing Posture.
        /// A new revision of the posture will be created if the revision to be
        /// updated is currently deployed on a workload.
        /// Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the
        /// Posture does not exist.
        /// Returns a `google.rpc.Status` with `google.rpc.Code.ABORTED` if the etag
        /// supplied in the request does not match the persisted etag of the Posture.
        /// Updatable fields are state, description and policy_sets.
        /// State update operation cannot be clubbed with update of description and
        /// policy_sets.
        /// An ACTIVE posture can be updated to both DRAFT or DEPRECATED states.
        /// Postures in DRAFT or DEPRECATED states can only be updated to ACTIVE state.
        pub async fn update_posture(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePostureRequest>,
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
                "/google.cloud.securityposture.v1.SecurityPosture/UpdatePosture",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securityposture.v1.SecurityPosture",
                        "UpdatePosture",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes all the revisions of a resource.
        /// A posture can only be deleted when none of the revisions are deployed to
        /// any workload.
        pub async fn delete_posture(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePostureRequest>,
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
                "/google.cloud.securityposture.v1.SecurityPosture/DeletePosture",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securityposture.v1.SecurityPosture",
                        "DeletePosture",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Extracts existing policies on a workload as a posture.
        /// If a Posture on the given workload already exists, the long running
        /// operation returns a [ALREADY_EXISTS][google.rpc.Code.ALREADY_EXISTS] error.
        pub async fn extract_posture(
            &mut self,
            request: impl tonic::IntoRequest<super::ExtractPostureRequest>,
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
                "/google.cloud.securityposture.v1.SecurityPosture/ExtractPosture",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securityposture.v1.SecurityPosture",
                        "ExtractPosture",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// PostureDeployments
        /// Lists PostureDeployments in a given project and location.
        pub async fn list_posture_deployments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPostureDeploymentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPostureDeploymentsResponse>,
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
                "/google.cloud.securityposture.v1.SecurityPosture/ListPostureDeployments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securityposture.v1.SecurityPosture",
                        "ListPostureDeployments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single PostureDeployment.
        pub async fn get_posture_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPostureDeploymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostureDeployment>,
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
                "/google.cloud.securityposture.v1.SecurityPosture/GetPostureDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securityposture.v1.SecurityPosture",
                        "GetPostureDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new PostureDeployment in a given project and location.
        pub async fn create_posture_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePostureDeploymentRequest>,
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
                "/google.cloud.securityposture.v1.SecurityPosture/CreatePostureDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securityposture.v1.SecurityPosture",
                        "CreatePostureDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single PostureDeployment.
        pub async fn update_posture_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePostureDeploymentRequest>,
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
                "/google.cloud.securityposture.v1.SecurityPosture/UpdatePostureDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securityposture.v1.SecurityPosture",
                        "UpdatePostureDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single PostureDeployment.
        pub async fn delete_posture_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePostureDeploymentRequest>,
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
                "/google.cloud.securityposture.v1.SecurityPosture/DeletePostureDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securityposture.v1.SecurityPosture",
                        "DeletePostureDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// PostureTemplates
        /// Lists all the PostureTemplates available to the user.
        pub async fn list_posture_templates(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPostureTemplatesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPostureTemplatesResponse>,
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
                "/google.cloud.securityposture.v1.SecurityPosture/ListPostureTemplates",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securityposture.v1.SecurityPosture",
                        "ListPostureTemplates",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a PostureTemplate.
        /// User must provide revision_id to retrieve a specific revision of the
        /// resource.
        /// NOT_FOUND error is returned if the revision_id or the PostureTemplate name
        /// does not exist. In case revision_id is not provided then the
        /// PostureTemplate with latest revision_id is returned.
        pub async fn get_posture_template(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPostureTemplateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostureTemplate>,
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
                "/google.cloud.securityposture.v1.SecurityPosture/GetPostureTemplate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securityposture.v1.SecurityPosture",
                        "GetPostureTemplate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
