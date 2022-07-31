///  Payload proto for Notification logs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationStage {
    ///  The type of the Notification Service event.
    #[prost(enumeration="notification_stage::Stage", tag="1")]
    pub stage: i32,
    ///  Time of the NotificationServiceEvent.
    #[prost(message, optional, tag="2")]
    pub event_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  The id of the notification.
    #[prost(string, tag="3")]
    pub notification_id: ::prost::alloc::string::String,
    ///  The event that triggered the notification.
    #[prost(enumeration="notification_stage::Event", tag="4")]
    pub event: i32,
    ///  Message to denote the error related to the event if applicable.
    #[prost(string, tag="5")]
    pub message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `NotificationStage`.
pub mod notification_stage {
    ///  Types of Notification Status.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Stage {
        ///  Default.
        Unspecified = 0,
        ///  Notification was sent.
        Sent = 1,
        ///  Notification failed to send.
        SendFailure = 2,
        ///  Notification was dropped.
        Dropped = 3,
    }
    impl Stage {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Stage::Unspecified => "STAGE_UNSPECIFIED",
                Stage::Sent => "SENT",
                Stage::SendFailure => "SEND_FAILURE",
                Stage::Dropped => "DROPPED",
            }
        }
    }
    ///  Event that triggered the notification.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Event {
        ///  Default value.
        Unspecified = 0,
        ///  When a health status has been changed.
        HealthStatusChange = 1,
    }
    impl Event {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Event::Unspecified => "EVENT_UNSPECIFIED",
                Event::HealthStatusChange => "HEALTH_STATUS_CHANGE",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceEvent {
    ///  The type of the event, e.g. Create, Update, etc.
    #[prost(string, tag="1")]
    pub verb: ::prost::alloc::string::String,
    ///  The state of the instance, e.g. "RETRYING_CREATE_INSTANCE".
    #[prost(string, tag="2")]
    pub stage: ::prost::alloc::string::String,
    ///  A human-readable log message, e.g. "error in stage: CREATING, err: location
    ///  not available".
    #[prost(string, tag="3")]
    pub msg: ::prost::alloc::string::String,
    ///  The ID to uniquely locate all logs associated with a given request.
    #[prost(string, tag="4")]
    pub trace_id: ::prost::alloc::string::String,
    ///  The instance node which is the subject of the operation, if known.
    ///  Currently unused as tf actuation does not manage nodes.
    #[prost(string, tag="5")]
    pub node_id: ::prost::alloc::string::String,
}
