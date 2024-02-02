/// Cloud Logging details for execution info
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudLoggingDetails {
    /// Optional. Severity selected by the customer for the logs to be sent to
    /// Cloud Logging, for the integration version getting executed.
    #[prost(
        enumeration = "cloud_logging_details::CloudLoggingSeverity",
        optional,
        tag = "1"
    )]
    pub cloud_logging_severity: ::core::option::Option<i32>,
    /// Optional. Status of whether Cloud Logging is enabled or not for the
    /// integration version getting executed.
    #[prost(bool, optional, tag = "2")]
    pub enable_cloud_logging: ::core::option::Option<bool>,
}
/// Nested message and enum types in `CloudLoggingDetails`.
pub mod cloud_logging_details {
    /// The severity will be mapped to the Integration Execution State.
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
    pub enum CloudLoggingSeverity {
        /// Unspecified
        Unspecified = 0,
        /// If Severity selected is `INFO`, then all the Integration Execution States
        /// (`IN_PROCESS`, `ON_HOLD`, `SUCCEEDED`, `SUSPENDED`, `ERROR`, `CANCELLED`)
        /// will be sent to Cloud Logging.
        Info = 2,
        /// If Severity selected is `ERROR`, then only the following Integration
        /// Execution States (`ERROR`, `CANCELLED`) will be sent to Cloud Logging.
        Error = 3,
        /// If Severity selected is `WARNING`, then only the following Integration
        /// Execution States (`ERROR`, `CANCELLED`) will be sent to Cloud Logging.
        Warning = 4,
    }
    impl CloudLoggingSeverity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CloudLoggingSeverity::Unspecified => "CLOUD_LOGGING_SEVERITY_UNSPECIFIED",
                CloudLoggingSeverity::Info => "INFO",
                CloudLoggingSeverity::Error => "ERROR",
                CloudLoggingSeverity::Warning => "WARNING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CLOUD_LOGGING_SEVERITY_UNSPECIFIED" => Some(Self::Unspecified),
                "INFO" => Some(Self::Info),
                "ERROR" => Some(Self::Error),
                "WARNING" => Some(Self::Warning),
                _ => None,
            }
        }
    }
}
/// Configuration detail of coordinate, it used for UI
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coordinate {
    /// Required. X axis of the coordinate
    #[prost(int32, tag = "1")]
    pub x: i32,
    /// Required. Y axis of the coordinate
    #[prost(int32, tag = "2")]
    pub y: i32,
}
/// The type of the parameter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueType {
    #[prost(oneof = "value_type::Value", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9")]
    pub value: ::core::option::Option<value_type::Value>,
}
/// Nested message and enum types in `ValueType`.
pub mod value_type {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// String.
        #[prost(string, tag = "1")]
        StringValue(::prost::alloc::string::String),
        /// Integer.
        #[prost(int64, tag = "2")]
        IntValue(i64),
        /// Double Number.
        #[prost(double, tag = "3")]
        DoubleValue(f64),
        /// Boolean.
        #[prost(bool, tag = "4")]
        BooleanValue(bool),
        /// String Array.
        #[prost(message, tag = "5")]
        StringArray(super::StringParameterArray),
        /// Integer Array.
        #[prost(message, tag = "6")]
        IntArray(super::IntParameterArray),
        /// Double Number Array.
        #[prost(message, tag = "7")]
        DoubleArray(super::DoubleParameterArray),
        /// Boolean Array.
        #[prost(message, tag = "8")]
        BooleanArray(super::BooleanParameterArray),
        /// Json.
        #[prost(string, tag = "9")]
        JsonValue(::prost::alloc::string::String),
    }
}
/// This message only contains a field of string array.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringParameterArray {
    /// String array.
    #[prost(string, repeated, tag = "1")]
    pub string_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// This message only contains a field of integer array.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntParameterArray {
    /// Integer array.
    #[prost(int64, repeated, tag = "1")]
    pub int_values: ::prost::alloc::vec::Vec<i64>,
}
/// This message only contains a field of double number array.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleParameterArray {
    /// Double number array.
    #[prost(double, repeated, tag = "1")]
    pub double_values: ::prost::alloc::vec::Vec<f64>,
}
/// This message only contains a field of boolean array.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BooleanParameterArray {
    /// Boolean array.
    #[prost(bool, repeated, tag = "1")]
    pub boolean_values: ::prost::alloc::vec::Vec<bool>,
}
/// This message is used for processing and persisting (when applicable) key
/// value pair parameters for each event in the event bus.
/// Next available id: 4
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventParameter {
    /// Key is used to retrieve the corresponding parameter value. This should be
    /// unique for a given fired event. These parameters must be predefined in the
    /// integration definition.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// Values for the defined keys. Each value can either be string, int, double
    /// or any proto message.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<ValueType>,
    /// True if this parameter should be masked in the logs
    #[prost(bool, tag = "3")]
    pub masked: bool,
}
/// Indicates the status of the integration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IntegrationState {
    /// Default.
    Unspecified = 0,
    /// Draft.
    Draft = 1,
    /// Active.
    Active = 2,
    /// Archived.
    Archived = 3,
    /// Snapshot.
    Snapshot = 4,
}
impl IntegrationState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IntegrationState::Unspecified => "INTEGRATION_STATE_UNSPECIFIED",
            IntegrationState::Draft => "DRAFT",
            IntegrationState::Active => "ACTIVE",
            IntegrationState::Archived => "ARCHIVED",
            IntegrationState::Snapshot => "SNAPSHOT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INTEGRATION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "DRAFT" => Some(Self::Draft),
            "ACTIVE" => Some(Self::Active),
            "ARCHIVED" => Some(Self::Archived),
            "SNAPSHOT" => Some(Self::Snapshot),
            _ => None,
        }
    }
}
/// Options for how to validate json schemas.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JsonValidationOption {
    /// As per the default behavior, no validation will be run. Will not override
    /// any option set in a Task.
    Unspecified = 0,
    /// Do not run any validation against JSON schemas.
    Skip = 1,
    /// Validate all potential input JSON parameters against schemas specified in
    /// IntegrationParameter.
    PreExecution = 2,
    /// Validate all potential output JSON parameters against schemas specified in
    /// IntegrationParameter.
    PostExecution = 3,
    /// Perform both PRE_EXECUTION and POST_EXECUTION validations.
    PrePostExecution = 4,
}
impl JsonValidationOption {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            JsonValidationOption::Unspecified => "JSON_VALIDATION_OPTION_UNSPECIFIED",
            JsonValidationOption::Skip => "SKIP",
            JsonValidationOption::PreExecution => "PRE_EXECUTION",
            JsonValidationOption::PostExecution => "POST_EXECUTION",
            JsonValidationOption::PrePostExecution => "PRE_POST_EXECUTION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "JSON_VALIDATION_OPTION_UNSPECIFIED" => Some(Self::Unspecified),
            "SKIP" => Some(Self::Skip),
            "PRE_EXECUTION" => Some(Self::PreExecution),
            "POST_EXECUTION" => Some(Self::PostExecution),
            "PRE_POST_EXECUTION" => Some(Self::PrePostExecution),
            _ => None,
        }
    }
}
/// The task configuration details. This is not the implementation of Task.
/// There might be multiple TaskConfigs for the same Task.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskConfig {
    /// Optional. The name for the task.
    #[prost(string, tag = "1")]
    pub task: ::prost::alloc::string::String,
    /// Required. The identifier of this task within its parent event config,
    /// specified by the client. This should be unique among all the tasks belong
    /// to the same event config. We use this field as the identifier to
    /// find next tasks (via field `next_tasks.task_id`).
    #[prost(string, tag = "2")]
    pub task_id: ::prost::alloc::string::String,
    /// Optional. The customized parameters the user can pass to this task.
    #[prost(map = "string, message", tag = "3")]
    pub parameters: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        EventParameter,
    >,
    /// Optional. Determines the number of times the
    /// task will be retried on failure and with what retry strategy.
    /// This is applicable for asynchronous calls to Eventbus alone (Post To
    /// Queue, Schedule etc.).
    #[prost(message, optional, tag = "4")]
    pub failure_policy: ::core::option::Option<FailurePolicy>,
    /// Optional. Determines the number of times the
    /// task will be retried on failure and with what retry strategy.
    /// This is applicable for synchronous calls to Eventbus alone (Post).
    #[prost(message, optional, tag = "5")]
    pub synchronous_call_failure_policy: ::core::option::Option<FailurePolicy>,
    /// Optional. The set of tasks that are next in line to be executed as per the
    /// execution graph defined for the parent event, specified by
    /// `event_config_id`. Each of these next tasks are executed
    /// only if the condition associated with them evaluates to true.
    #[prost(message, repeated, tag = "6")]
    pub next_tasks: ::prost::alloc::vec::Vec<NextTask>,
    /// Optional. The policy dictating the execution of the next set of tasks for
    /// the current task.
    #[prost(enumeration = "task_config::NextTasksExecutionPolicy", tag = "7")]
    pub next_tasks_execution_policy: i32,
    /// Optional. The policy dictating the execution strategy of this task.
    #[prost(enumeration = "task_config::TaskExecutionStrategy", tag = "8")]
    pub task_execution_strategy: i32,
    /// Optional. User-provided label that is attached to this TaskConfig in the
    /// UI.
    #[prost(string, tag = "9")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. Determines what action to take upon successful task completion.
    #[prost(message, optional, tag = "10")]
    pub success_policy: ::core::option::Option<SuccessPolicy>,
    /// Optional. If set, overrides the option configured in the Task
    /// implementation class.
    #[prost(enumeration = "JsonValidationOption", tag = "11")]
    pub json_validation_option: i32,
    /// Optional. User-provided description intended to give additional business
    /// context about the task.
    #[prost(string, tag = "12")]
    pub description: ::prost::alloc::string::String,
    /// Optional. Used to define task-template name if task is of type
    /// task-template
    #[prost(string, tag = "13")]
    pub task_template: ::prost::alloc::string::String,
    /// Optional. Optional
    /// Error catcher id of the error catch flow which will be executed when
    /// execution error happens in the task
    #[prost(string, tag = "17")]
    pub error_catcher_id: ::prost::alloc::string::String,
    /// Optional. External task type of the task
    #[prost(enumeration = "task_config::ExternalTaskType", tag = "15")]
    pub external_task_type: i32,
    /// Optional. Informs the front-end application where to draw this error
    /// catcher config on the UI.
    #[prost(message, optional, tag = "16")]
    pub position: ::core::option::Option<Coordinate>,
}
/// Nested message and enum types in `TaskConfig`.
pub mod task_config {
    /// Various policies for executing the next set of tasks.
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
    pub enum NextTasksExecutionPolicy {
        /// Default.
        Unspecified = 0,
        /// Execute all the tasks that satisfy their associated condition.
        RunAllMatch = 1,
        /// Execute the first task that satisfies the associated condition.
        RunFirstMatch = 2,
    }
    impl NextTasksExecutionPolicy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                NextTasksExecutionPolicy::Unspecified => {
                    "NEXT_TASKS_EXECUTION_POLICY_UNSPECIFIED"
                }
                NextTasksExecutionPolicy::RunAllMatch => "RUN_ALL_MATCH",
                NextTasksExecutionPolicy::RunFirstMatch => "RUN_FIRST_MATCH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NEXT_TASKS_EXECUTION_POLICY_UNSPECIFIED" => Some(Self::Unspecified),
                "RUN_ALL_MATCH" => Some(Self::RunAllMatch),
                "RUN_FIRST_MATCH" => Some(Self::RunFirstMatch),
                _ => None,
            }
        }
    }
    /// Various policies to trigger the execution of this task.
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
    pub enum TaskExecutionStrategy {
        /// Default. If the strategy is not set explicitly, it will default to
        /// `WHEN_ALL_SUCCEED`.
        Unspecified = 0,
        /// Wait until all of its previous tasks finished execution, then verify at
        /// least one of the edge conditions is met, and execute if possible. This
        /// should be considered as WHEN_ALL_TASKS_SUCCEED.
        WhenAllSucceed = 1,
        /// Start execution as long as any of its previous tasks finished execution
        /// and the corresponding edge condition is met (since we will execute if
        /// only that succeeding edge condition is met).
        WhenAnySucceed = 2,
        /// Wait until all of its previous tasks finished execution, then verify
        /// the all edge conditions are met and execute if possible.
        WhenAllTasksAndConditionsSucceed = 3,
    }
    impl TaskExecutionStrategy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TaskExecutionStrategy::Unspecified => {
                    "TASK_EXECUTION_STRATEGY_UNSPECIFIED"
                }
                TaskExecutionStrategy::WhenAllSucceed => "WHEN_ALL_SUCCEED",
                TaskExecutionStrategy::WhenAnySucceed => "WHEN_ANY_SUCCEED",
                TaskExecutionStrategy::WhenAllTasksAndConditionsSucceed => {
                    "WHEN_ALL_TASKS_AND_CONDITIONS_SUCCEED"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TASK_EXECUTION_STRATEGY_UNSPECIFIED" => Some(Self::Unspecified),
                "WHEN_ALL_SUCCEED" => Some(Self::WhenAllSucceed),
                "WHEN_ANY_SUCCEED" => Some(Self::WhenAnySucceed),
                "WHEN_ALL_TASKS_AND_CONDITIONS_SUCCEED" => {
                    Some(Self::WhenAllTasksAndConditionsSucceed)
                }
                _ => None,
            }
        }
    }
    /// Defines the type of the task for external customer
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
    pub enum ExternalTaskType {
        /// Default value. External task type is not specified
        Unspecified = 0,
        /// Tasks belongs to the normal task flows
        NormalTask = 1,
        /// Task belongs to the error catch task flows
        ErrorTask = 2,
    }
    impl ExternalTaskType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ExternalTaskType::Unspecified => "EXTERNAL_TASK_TYPE_UNSPECIFIED",
                ExternalTaskType::NormalTask => "NORMAL_TASK",
                ExternalTaskType::ErrorTask => "ERROR_TASK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EXTERNAL_TASK_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "NORMAL_TASK" => Some(Self::NormalTask),
                "ERROR_TASK" => Some(Self::ErrorTask),
                _ => None,
            }
        }
    }
}
/// Policy that dictates the behavior for the task after it completes
/// successfully.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuccessPolicy {
    /// State to which the execution snapshot status will be set if the task
    /// succeeds.
    #[prost(enumeration = "success_policy::FinalState", tag = "1")]
    pub final_state: i32,
}
/// Nested message and enum types in `SuccessPolicy`.
pub mod success_policy {
    /// The state of execution.
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
    pub enum FinalState {
        /// UNSPECIFIED.
        Unspecified = 0,
        /// The default behavior, where successful tasks will be marked as SUCCEEDED.
        Succeeded = 1,
        /// Sets the state to SUSPENDED after executing.  This is required for
        /// SuspensionTask; event execution will continue once the user calls
        /// ResolveSuspensions with the event_execution_info_id and the task number.
        Suspended = 2,
    }
    impl FinalState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FinalState::Unspecified => "FINAL_STATE_UNSPECIFIED",
                FinalState::Succeeded => "SUCCEEDED",
                FinalState::Suspended => "SUSPENDED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FINAL_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "SUCCEEDED" => Some(Self::Succeeded),
                "SUSPENDED" => Some(Self::Suspended),
                _ => None,
            }
        }
    }
}
/// Policy that defines the task retry logic and failure type. If no
/// FailurePolicy is defined for a task, all its dependent tasks will not be
/// executed (i.e, a `retry_strategy` of NONE will be applied).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailurePolicy {
    /// Defines what happens to the task upon failure.
    #[prost(enumeration = "failure_policy::RetryStrategy", tag = "1")]
    pub retry_strategy: i32,
    /// Required if retry_strategy is FIXED_INTERVAL or
    /// LINEAR/EXPONENTIAL_BACKOFF/RESTART_INTEGRATION_WITH_BACKOFF. Defines the
    /// number of times the task will be retried if failed.
    #[prost(int32, tag = "2")]
    pub max_retries: i32,
    /// Required if retry_strategy is FIXED_INTERVAL or
    /// LINEAR/EXPONENTIAL_BACKOFF/RESTART_INTEGRATION_WITH_BACKOFF. Defines the
    /// initial interval in seconds for backoff.
    #[prost(message, optional, tag = "3")]
    pub interval_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `FailurePolicy`.
pub mod failure_policy {
    /// The behavior when the taks failed.
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
    pub enum RetryStrategy {
        /// UNSPECIFIED.
        Unspecified = 0,
        /// Ignores the failure of this task. The rest of the integration will be
        /// executed Assuming this task succeeded.
        Ignore = 1,
        /// Causes a permanent failure of the task. However, if the last task(s)
        /// of event was successfully completed despite the failure of this task,
        /// it has no impact on the integration.
        None = 2,
        /// Causes a permanent failure of the event. It is different from NONE
        /// because this will mark the event as FAILED by shutting down the
        /// event execution.
        Fatal = 3,
        /// The task will be retried from the failed task onwards after a fixed
        /// delay. A max-retry count is required to be specified with this
        /// strategy. A jitter is added to each exponential interval so that
        /// concurrently failing tasks of the same type do not end up retrying
        /// after the exact same exponential interval. max_retries and
        /// interval_in_seconds must be specified.
        FixedInterval = 4,
        /// The task will be retried from the failed task onwards after a fixed
        /// delay that linearly increases with each retry attempt. A jitter is
        /// added to each exponential interval so that concurrently failing tasks
        /// of the same type do not end up retrying after the exact same
        /// exponential interval. A max-retry count is required to be specified
        /// with this strategy. max_retries and interval_in_seconds must be
        /// specified.
        LinearBackoff = 5,
        /// The task will be retried after an exponentially increasing period of
        /// time with each failure. A jitter is added to each exponential interval
        /// so that concurrently failing tasks of the same type do not end up
        /// retrying after the exact same exponential interval. A max-retry count
        /// is required to be specified with this strategy. `max_retries` and
        /// `interval_in_seconds` must be specified.
        ExponentialBackoff = 6,
        /// The entire integration will be restarted with the initial parameters that
        /// were set when the event was fired. A max-retry count is required to be
        /// specified with this strategy. `max_retries` and `interval_in_seconds`
        /// must be specified.
        RestartIntegrationWithBackoff = 7,
    }
    impl RetryStrategy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RetryStrategy::Unspecified => "RETRY_STRATEGY_UNSPECIFIED",
                RetryStrategy::Ignore => "IGNORE",
                RetryStrategy::None => "NONE",
                RetryStrategy::Fatal => "FATAL",
                RetryStrategy::FixedInterval => "FIXED_INTERVAL",
                RetryStrategy::LinearBackoff => "LINEAR_BACKOFF",
                RetryStrategy::ExponentialBackoff => "EXPONENTIAL_BACKOFF",
                RetryStrategy::RestartIntegrationWithBackoff => {
                    "RESTART_INTEGRATION_WITH_BACKOFF"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RETRY_STRATEGY_UNSPECIFIED" => Some(Self::Unspecified),
                "IGNORE" => Some(Self::Ignore),
                "NONE" => Some(Self::None),
                "FATAL" => Some(Self::Fatal),
                "FIXED_INTERVAL" => Some(Self::FixedInterval),
                "LINEAR_BACKOFF" => Some(Self::LinearBackoff),
                "EXPONENTIAL_BACKOFF" => Some(Self::ExponentialBackoff),
                "RESTART_INTEGRATION_WITH_BACKOFF" => {
                    Some(Self::RestartIntegrationWithBackoff)
                }
                _ => None,
            }
        }
    }
}
/// The task that is next in line to be executed, if the
/// condition specified evaluated to true.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NextTask {
    /// ID of the next task.
    #[prost(string, tag = "1")]
    pub task_config_id: ::prost::alloc::string::String,
    /// Task number of the next task.
    #[prost(string, tag = "2")]
    pub task_id: ::prost::alloc::string::String,
    /// Standard filter expression for this task to become an eligible next task.
    #[prost(string, tag = "3")]
    pub condition: ::prost::alloc::string::String,
    /// User-provided label that is attached to this edge in the UI.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
    /// User-provided description intended to give additional business context
    /// about the task.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
}
/// Log entry to log execution info for the monitored resource
/// `integrations.googleapis.com/IntegrationVersion`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionInfo {
    /// Name of the integration.
    #[prost(string, tag = "2")]
    pub integration: ::prost::alloc::string::String,
    /// The customer's project number.
    #[prost(string, tag = "4")]
    pub project_id: ::prost::alloc::string::String,
    /// The trigger id of the integration trigger config. If both trigger_id
    /// and client_id is present, the integration is executed from the start tasks
    /// provided by the matching trigger config otherwise it is executed from the
    /// default start tasks.
    #[prost(string, tag = "5")]
    pub trigger_id: ::prost::alloc::string::String,
    /// Execution parameters come in as part of the request.
    #[prost(map = "string, message", tag = "6")]
    pub request_params: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        EventParameter,
    >,
    /// Execution parameters come out as part of the response.
    #[prost(map = "string, message", tag = "7")]
    pub response_params: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        EventParameter,
    >,
    /// Errors, warnings, and informationals associated with the workflow/task.
    /// The order in which the errors were added by the workflow/task is
    /// maintained.
    #[prost(message, repeated, tag = "10")]
    pub errors: ::prost::alloc::vec::Vec<ErrorDetail>,
    /// The configuration details for a task.
    #[prost(message, repeated, tag = "13")]
    pub task_configs: ::prost::alloc::vec::Vec<TaskConfig>,
    /// Pointer to the active version it is executing.
    #[prost(string, tag = "14")]
    pub integration_version_number: ::prost::alloc::string::String,
    /// Auto-generated primary key.
    #[prost(string, tag = "15")]
    pub execution_id: ::prost::alloc::string::String,
    /// Output only. State of the integration version
    #[prost(enumeration = "IntegrationState", tag = "16")]
    pub integration_version_state: i32,
    /// Database persistence policy for execution info
    #[prost(bool, tag = "17")]
    pub enable_database_persistence: bool,
    /// Cloud Logging details for execution info
    #[prost(message, optional, tag = "18")]
    pub cloud_logging_details: ::core::option::Option<CloudLoggingDetails>,
    /// The details about this integration execution.
    #[prost(message, optional, tag = "19")]
    pub integration_execution_details: ::core::option::Option<
        IntegrationExecutionDetails,
    >,
    /// Specifies whether this execution info corresponds to actual integration or
    /// test case.
    #[prost(enumeration = "ExecutionType", tag = "20")]
    pub execution_type: i32,
    /// The ways user posts this event.
    #[prost(enumeration = "execution_info::ExecutionMethod", tag = "21")]
    pub execution_method: i32,
    /// An increasing sequence that is set when a new snapshot (Integration
    /// Version) is created.
    #[prost(int64, tag = "22")]
    pub integration_snapshot_number: i64,
}
/// Nested message and enum types in `ExecutionInfo`.
pub mod execution_info {
    /// ExecutionMethod Enum
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
    pub enum ExecutionMethod {
        /// Default value.
        Unspecified = 0,
        /// Sync post.
        Post = 1,
        /// Async post with schedule time.
        Schedule = 2,
        /// Async post.
        PostToQueue = 3,
    }
    impl ExecutionMethod {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ExecutionMethod::Unspecified => "EXECUTION_METHOD_UNSPECIFIED",
                ExecutionMethod::Post => "POST",
                ExecutionMethod::Schedule => "SCHEDULE",
                ExecutionMethod::PostToQueue => "POST_TO_QUEUE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EXECUTION_METHOD_UNSPECIFIED" => Some(Self::Unspecified),
                "POST" => Some(Self::Post),
                "SCHEDULE" => Some(Self::Schedule),
                "POST_TO_QUEUE" => Some(Self::PostToQueue),
                _ => None,
            }
        }
    }
}
/// Contains the details of the execution info: this includes the tasks execution
/// details plus the integration execution statistics.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntegrationExecutionDetails {
    /// Output only. The execution state of this Integration.
    #[prost(
        enumeration = "integration_execution_details::IntegrationExecutionState",
        tag = "1"
    )]
    pub integration_execution_state: i32,
    /// Execution snapshot.
    #[prost(message, repeated, tag = "2")]
    pub integration_execution_snapshot: ::prost::alloc::vec::Vec<
        IntegrationExecutionSnapshot,
    >,
    /// Status for the current execution attempt.
    #[prost(message, repeated, tag = "3")]
    pub execution_attempt_stats: ::prost::alloc::vec::Vec<AttemptStats>,
    /// Next scheduled execution time in case the execution status was
    /// RETRY_ON_HOLD.
    #[prost(message, optional, tag = "4")]
    pub next_execution_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Indicates the number of times the execution has restarted from the
    /// beginning.
    #[prost(int32, tag = "5")]
    pub execution_retries_count: i32,
}
/// Nested message and enum types in `IntegrationExecutionDetails`.
pub mod integration_execution_details {
    /// Enum ExecutionState.
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
    pub enum IntegrationExecutionState {
        /// Default value.
        Unspecified = 0,
        /// Integration is received and waiting for the execution. This happens when
        /// firing the Integration via "postToQueue" or "schedule".
        OnHold = 1,
        /// Integration is under processing.
        InProcess = 2,
        /// Integration execution successfully finished. There's no more change after
        /// this state.
        Succeeded = 3,
        /// Integration execution failed. There's no more change after this state.
        Failed = 4,
        /// Integration execution canceled by user. There's no more change after this
        /// state.
        Cancelled = 5,
        /// Integration execution failed and waiting for retry.
        RetryOnHold = 6,
        /// Integration execution suspended and waiting for manual intervention.
        Suspended = 7,
    }
    impl IntegrationExecutionState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IntegrationExecutionState::Unspecified => {
                    "INTEGRATION_EXECUTION_STATE_UNSPECIFIED"
                }
                IntegrationExecutionState::OnHold => "ON_HOLD",
                IntegrationExecutionState::InProcess => "IN_PROCESS",
                IntegrationExecutionState::Succeeded => "SUCCEEDED",
                IntegrationExecutionState::Failed => "FAILED",
                IntegrationExecutionState::Cancelled => "CANCELLED",
                IntegrationExecutionState::RetryOnHold => "RETRY_ON_HOLD",
                IntegrationExecutionState::Suspended => "SUSPENDED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INTEGRATION_EXECUTION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ON_HOLD" => Some(Self::OnHold),
                "IN_PROCESS" => Some(Self::InProcess),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                "CANCELLED" => Some(Self::Cancelled),
                "RETRY_ON_HOLD" => Some(Self::RetryOnHold),
                "SUSPENDED" => Some(Self::Suspended),
                _ => None,
            }
        }
    }
}
/// Contains the snapshot of the integration execution for a given checkpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntegrationExecutionSnapshot {
    /// Indicates "right after which checkpoint task's execution" this snapshot
    /// is taken.
    #[prost(string, tag = "1")]
    pub checkpoint_task_number: ::prost::alloc::string::String,
    /// Indicates when this snapshot is taken.
    #[prost(message, optional, tag = "2")]
    pub snapshot_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Snapshot metadata.
    #[prost(message, optional, tag = "3")]
    pub integration_execution_snapshot_metadata: ::core::option::Option<
        integration_execution_snapshot::IntegrationExecutionSnapshotMetadata,
    >,
    /// All of the task execution details at the given point of time.
    #[prost(message, repeated, tag = "4")]
    pub task_execution_details: ::prost::alloc::vec::Vec<TaskExecutionDetails>,
    /// All of the computed conditions that been calculated.
    #[prost(message, repeated, tag = "5")]
    pub condition_results: ::prost::alloc::vec::Vec<ConditionResult>,
    /// The parameters in Event object.
    #[prost(map = "string, message", tag = "6")]
    pub execution_params: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        EventParameter,
    >,
}
/// Nested message and enum types in `IntegrationExecutionSnapshot`.
pub mod integration_execution_snapshot {
    /// Metadata for the integration/task retry.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IntegrationExecutionSnapshotMetadata {
        /// The task number associated with this snapshot. Could be empty.
        #[prost(string, tag = "1")]
        pub task_number: ::prost::alloc::string::String,
        /// the task name associated with this snapshot. Could be empty.
        #[prost(string, tag = "2")]
        pub task: ::prost::alloc::string::String,
        /// the integration execution attempt number this snapshot belongs to.
        #[prost(int32, tag = "3")]
        pub integration_execution_attempt_num: i32,
        /// the task attempt number this snapshot belongs to. Could be empty.
        #[prost(int32, tag = "4")]
        pub task_attempt_num: i32,
        /// the task label associated with this snapshot. Could be empty.
        #[prost(string, tag = "5")]
        pub task_label: ::prost::alloc::string::String,
        /// Ancestor task number for the task(it will only be non-empty if the task
        /// is under 'private workflow')
        #[prost(string, repeated, tag = "6")]
        pub ancestor_task_numbers: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// Ancestor iteration number for the task(it will only be non-empty if the
        /// task is under 'private workflow')
        #[prost(string, repeated, tag = "7")]
        pub ancestor_iteration_numbers: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// The direct integration which the event execution snapshots belongs to
        #[prost(string, tag = "8")]
        pub integration: ::prost::alloc::string::String,
    }
}
/// Contains the details of the execution of this task.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskExecutionDetails {
    /// Pointer to the task config it used for execution.
    #[prost(string, tag = "1")]
    pub task_number: ::prost::alloc::string::String,
    /// The execution state of this task.
    #[prost(enumeration = "task_execution_details::TaskExecutionState", tag = "2")]
    pub task_execution_state: i32,
    /// Status for the current task execution attempt.
    #[prost(message, repeated, tag = "3")]
    pub task_attempt_stats: ::prost::alloc::vec::Vec<AttemptStats>,
}
/// Nested message and enum types in `TaskExecutionDetails`.
pub mod task_execution_details {
    /// Enum TaskExecutionState.
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
    pub enum TaskExecutionState {
        /// Default value.
        Unspecified = 0,
        /// Task is waiting for its precondition tasks to finish to start the
        /// execution.
        PendingExecution = 1,
        /// Task is under processing.
        InProcess = 2,
        /// Task execution successfully finished. There's no more change after
        /// this state.
        Succeed = 3,
        /// Task execution failed. There's no more change after this state.
        Failed = 4,
        /// Task execution failed and cause the whole integration execution to fail
        /// immediately. There's no more change after this state.
        Fatal = 5,
        /// Task execution failed and waiting for retry.
        RetryOnHold = 6,
        /// Task execution skipped. This happens when its precondition wasn't met,
        /// or the integration execution been canceled before reach to the task.
        /// There's no more changes after this state.
        Skipped = 7,
        /// Task execution canceled when in progress. This happens when integration
        /// execution been canceled or any other task fall in fatal state.
        Cancelled = 8,
        /// Task is waiting for its dependency tasks' rollback to finish to start
        /// its rollback.
        PendingRollback = 9,
        /// Task is rolling back.
        RollbackInProcess = 10,
        /// Task is rolled back. This is the state we will set regardless of
        /// rollback succeeding or failing.
        Rolledback = 11,
        /// Task is a SuspensionTask which has executed once, creating a pending
        /// suspension.
        Suspended = 12,
    }
    impl TaskExecutionState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TaskExecutionState::Unspecified => "TASK_EXECUTION_STATE_UNSPECIFIED",
                TaskExecutionState::PendingExecution => "PENDING_EXECUTION",
                TaskExecutionState::InProcess => "IN_PROCESS",
                TaskExecutionState::Succeed => "SUCCEED",
                TaskExecutionState::Failed => "FAILED",
                TaskExecutionState::Fatal => "FATAL",
                TaskExecutionState::RetryOnHold => "RETRY_ON_HOLD",
                TaskExecutionState::Skipped => "SKIPPED",
                TaskExecutionState::Cancelled => "CANCELLED",
                TaskExecutionState::PendingRollback => "PENDING_ROLLBACK",
                TaskExecutionState::RollbackInProcess => "ROLLBACK_IN_PROCESS",
                TaskExecutionState::Rolledback => "ROLLEDBACK",
                TaskExecutionState::Suspended => "SUSPENDED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TASK_EXECUTION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING_EXECUTION" => Some(Self::PendingExecution),
                "IN_PROCESS" => Some(Self::InProcess),
                "SUCCEED" => Some(Self::Succeed),
                "FAILED" => Some(Self::Failed),
                "FATAL" => Some(Self::Fatal),
                "RETRY_ON_HOLD" => Some(Self::RetryOnHold),
                "SKIPPED" => Some(Self::Skipped),
                "CANCELLED" => Some(Self::Cancelled),
                "PENDING_ROLLBACK" => Some(Self::PendingRollback),
                "ROLLBACK_IN_PROCESS" => Some(Self::RollbackInProcess),
                "ROLLEDBACK" => Some(Self::Rolledback),
                "SUSPENDED" => Some(Self::Suspended),
                _ => None,
            }
        }
    }
}
/// Status for the execution attempt.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttemptStats {
    /// The start time of the integration execution for current attempt. This could
    /// be in the future if it's been scheduled.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end time of the integration execution for current attempt.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// An error, warning, or information message associated with an integration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorDetail {
    /// The full text of the error message, including any parameters that were
    /// thrown along with the exception.
    #[prost(string, tag = "1")]
    pub error_message: ::prost::alloc::string::String,
    /// The task try-number, in which, the error occurred.  If zero, the error
    /// happened at the integration level.
    #[prost(int32, tag = "2")]
    pub task_number: i32,
}
/// Contains the combined condition calculation results.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConditionResult {
    /// the current task number.
    #[prost(string, tag = "1")]
    pub current_task_number: ::prost::alloc::string::String,
    /// the next task number.
    #[prost(string, tag = "2")]
    pub next_task_number: ::prost::alloc::string::String,
    /// the result comes out after evaluate the combined condition. True if there's
    /// no combined condition specified.
    #[prost(bool, tag = "3")]
    pub result: bool,
}
/// Specifies whether this execution info corresponds to actual integration or
/// test case.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionType {
    /// Unspecified value.
    Unspecified = 0,
    /// Execution corresponds to run of an integration version.
    IntegrationVersion = 1,
    /// Execution corresponds to run of a functional test case.
    TestCase = 2,
}
impl ExecutionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecutionType::Unspecified => "EXECUTION_TYPE_UNSPECIFIED",
            ExecutionType::IntegrationVersion => "INTEGRATION_VERSION",
            ExecutionType::TestCase => "TEST_CASE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXECUTION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "INTEGRATION_VERSION" => Some(Self::IntegrationVersion),
            "TEST_CASE" => Some(Self::TestCase),
            _ => None,
        }
    }
}
/// Enum Product.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Product {
    /// Default value.
    Unspecified = 0,
    /// Integration Platform.
    Ip = 1,
    /// Apigee.
    Apigee = 2,
    /// Security Command Center.
    Security = 3,
}
impl Product {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Product::Unspecified => "PRODUCT_UNSPECIFIED",
            Product::Ip => "IP",
            Product::Apigee => "APIGEE",
            Product::Security => "SECURITY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PRODUCT_UNSPECIFIED" => Some(Self::Unspecified),
            "IP" => Some(Self::Ip),
            "APIGEE" => Some(Self::Apigee),
            "SECURITY" => Some(Self::Security),
            _ => None,
        }
    }
}
