//  Manifest section specific to Calendar Add-ons.

///  Calendar add-on manifest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalendarAddOnManifest {
    ///  Defines an endpoint that will be executed contexts that don't
    ///  match a declared contextual trigger. Any cards generated by this function
    ///  will always be available to the user, but may be eclipsed by contextual
    ///  content when this add-on declares more targeted triggers.
    ///
    ///  If present, this overrides the configuration from
    ///  `addOns.common.homepageTrigger`.
    #[prost(message, optional, tag="6")]
    pub homepage_trigger: ::core::option::Option<super::HomepageExtensionPoint>,
    ///  Defines conference solutions provided by this add-on.
    #[prost(message, repeated, tag="3")]
    pub conference_solution: ::prost::alloc::vec::Vec<ConferenceSolution>,
    ///  An endpoint to execute that creates a URL to the add-on's settings page.
    #[prost(string, tag="5")]
    pub create_settings_url_function: ::prost::alloc::string::String,
    ///  An endpoint to trigger when an event is opened (viewed/edited).
    #[prost(message, optional, tag="10")]
    pub event_open_trigger: ::core::option::Option<CalendarExtensionPoint>,
    ///  An endpoint to trigger when the open event is updated.
    #[prost(message, optional, tag="11")]
    pub event_update_trigger: ::core::option::Option<CalendarExtensionPoint>,
    ///  Define the level of data access when an event addon is triggered.
    #[prost(enumeration="calendar_add_on_manifest::EventAccess", tag="12")]
    pub current_event_access: i32,
}
/// Nested message and enum types in `CalendarAddOnManifest`.
pub mod calendar_add_on_manifest {
    ///  An enum defining the level of data access event triggers require.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EventAccess {
        ///  Default value when nothing is set for EventAccess.
        Unspecified = 0,
        ///  METADATA gives event triggers the permission to access the metadata of
        ///  events such as event id and calendar id.
        Metadata = 1,
        ///  READ gives event triggers access to all provided event fields including
        ///  the metadata, attendees, and conference data.
        Read = 3,
        ///  WRITE gives event triggers access to the metadata of events and the
        ///  ability to perform all actions, including adding attendees and setting
        ///  conference data.
        Write = 4,
        ///  READ_WRITE gives event triggers access to all provided event fields
        ///  including the metadata, attendees, and conference data and the ability to
        ///  perform all actions.
        ReadWrite = 5,
    }
    impl EventAccess {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EventAccess::Unspecified => "UNSPECIFIED",
                EventAccess::Metadata => "METADATA",
                EventAccess::Read => "READ",
                EventAccess::Write => "WRITE",
                EventAccess::ReadWrite => "READ_WRITE",
            }
        }
    }
}
///  Defines conference related values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConferenceSolution {
    ///  Required. The endpoint to call when ConferenceData should be created.
    #[prost(string, tag="1")]
    pub on_create_function: ::prost::alloc::string::String,
    ///  Required. IDs should be unique across ConferenceSolutions within one
    ///  add-on, but this is not strictly enforced. It is up to the add-on developer
    ///  to assign them uniquely, otherwise the wrong ConferenceSolution may be
    ///  used when the add-on is triggered. While the developer may change the
    ///  display name of an add-on, the ID should not be changed.
    #[prost(string, tag="4")]
    pub id: ::prost::alloc::string::String,
    ///  Required. The display name of the ConferenceSolution.
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
    ///  Required. The URL for the logo image of the ConferenceSolution.
    #[prost(string, tag="6")]
    pub logo_url: ::prost::alloc::string::String,
}
///  Common format for declaring a calendar add-on's triggers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalendarExtensionPoint {
    ///  Required. The endpoint to execute when this extension point is
    ///  activated.
    #[prost(string, tag="1")]
    pub run_function: ::prost::alloc::string::String,
}
