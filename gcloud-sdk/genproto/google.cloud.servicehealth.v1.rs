/// Represents service health events that may affect Google Cloud products.
/// Event resource is a read-only view and does not allow any modifications. All
/// fields are output only.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// Output only. Identifier. Name of the event. Unique name of the event in
    /// this scope including project and location using the form
    /// `projects/{project_id}/locations/{location}/events/{event_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Brief description for the event.
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// Output only. Free-form, human-readable description.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The category of the event.
    #[prost(enumeration = "event::EventCategory", tag = "4")]
    pub category: i32,
    /// Output only. The detailed category of the event.
    #[prost(enumeration = "event::DetailedCategory", tag = "21")]
    pub detailed_category: i32,
    /// Output only. The current state of the event.
    #[prost(enumeration = "event::State", tag = "5")]
    pub state: i32,
    /// Output only. The current detailed state of the incident.
    #[prost(enumeration = "event::DetailedState", tag = "19")]
    pub detailed_state: i32,
    /// Google Cloud products and locations impacted by the event.
    #[prost(message, repeated, tag = "20")]
    pub event_impacts: ::prost::alloc::vec::Vec<EventImpact>,
    /// Output only. Communicates why a given event is deemed relevant in the
    /// context of a given project.
    #[prost(enumeration = "event::Relevance", tag = "8")]
    pub relevance: i32,
    /// Output only. Event updates are correspondence from Google.
    #[prost(message, repeated, tag = "9")]
    pub updates: ::prost::alloc::vec::Vec<EventUpdate>,
    /// Output only. When `detailed_state`=`MERGED`, `parent_event` contains the
    /// name of the parent event. All further updates will be published to the
    /// parent event.
    #[prost(string, tag = "10")]
    pub parent_event: ::prost::alloc::string::String,
    /// Output only. The time when the event was last modified.
    #[prost(message, optional, tag = "12")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The start time of the event, if applicable.
    #[prost(message, optional, tag = "13")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The end time of the event, if applicable.
    #[prost(message, optional, tag = "14")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the next update can be expected.
    #[prost(message, optional, tag = "15")]
    pub next_update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `Event`.
pub mod event {
    /// The category of the event. This enum lists all possible categories of
    /// event.
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
    pub enum EventCategory {
        /// Unspecified category.
        Unspecified = 0,
        /// Event category for service outage or degradation.
        Incident = 2,
    }
    impl EventCategory {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EventCategory::Unspecified => "EVENT_CATEGORY_UNSPECIFIED",
                EventCategory::Incident => "INCIDENT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EVENT_CATEGORY_UNSPECIFIED" => Some(Self::Unspecified),
                "INCIDENT" => Some(Self::Incident),
                _ => None,
            }
        }
    }
    /// The detailed category of an event. Contains all possible states for all
    /// event categories.
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
    pub enum DetailedCategory {
        /// Unspecified detailed category.
        Unspecified = 0,
        /// Indicates an event with category INCIDENT has a confirmed impact to at
        /// least one Google Cloud product.
        ConfirmedIncident = 1,
        /// Indicates an event with category INCIDENT is under investigation to
        /// determine if it has a confirmed impact on any Google Cloud products.
        EmergingIncident = 2,
    }
    impl DetailedCategory {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DetailedCategory::Unspecified => "DETAILED_CATEGORY_UNSPECIFIED",
                DetailedCategory::ConfirmedIncident => "CONFIRMED_INCIDENT",
                DetailedCategory::EmergingIncident => "EMERGING_INCIDENT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DETAILED_CATEGORY_UNSPECIFIED" => Some(Self::Unspecified),
                "CONFIRMED_INCIDENT" => Some(Self::ConfirmedIncident),
                "EMERGING_INCIDENT" => Some(Self::EmergingIncident),
                _ => None,
            }
        }
    }
    /// The state of the event. This enum lists all possible states of event.
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
        /// Event is actively affecting a Google Cloud product and will continue to
        /// receive updates.
        Active = 1,
        /// Event is no longer affecting the Google Cloud product or has been merged
        /// with another event.
        Closed = 2,
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
                State::Closed => "CLOSED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "CLOSED" => Some(Self::Closed),
                _ => None,
            }
        }
    }
    /// The detailed state of the incident. This enum lists all possible detailed
    /// states of an incident.
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
    pub enum DetailedState {
        /// Unspecified detail state.
        Unspecified = 0,
        /// Google engineers are actively investigating the event to determine the
        /// impact.
        Emerging = 1,
        /// The incident is confirmed and impacting at least one Google Cloud
        /// product. Ongoing status updates will be provided until it is resolved.
        Confirmed = 2,
        /// The incident is no longer affecting any Google Cloud product, and there
        /// will be no further updates.
        Resolved = 3,
        /// The incident was merged into a parent incident. All further updates will
        /// be published to the parent only. The `parent_event` field contains the
        /// name of the parent.
        Merged = 4,
        /// The incident was automatically closed because the issues couldn’t be
        /// confirmed or is no longer impacting Google Cloud Products and/or
        /// Locations.
        AutoClosed = 9,
        /// The incident was verified as non-impactful. No further action required.
        FalsePositive = 10,
    }
    impl DetailedState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DetailedState::Unspecified => "DETAILED_STATE_UNSPECIFIED",
                DetailedState::Emerging => "EMERGING",
                DetailedState::Confirmed => "CONFIRMED",
                DetailedState::Resolved => "RESOLVED",
                DetailedState::Merged => "MERGED",
                DetailedState::AutoClosed => "AUTO_CLOSED",
                DetailedState::FalsePositive => "FALSE_POSITIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DETAILED_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "EMERGING" => Some(Self::Emerging),
                "CONFIRMED" => Some(Self::Confirmed),
                "RESOLVED" => Some(Self::Resolved),
                "MERGED" => Some(Self::Merged),
                "AUTO_CLOSED" => Some(Self::AutoClosed),
                "FALSE_POSITIVE" => Some(Self::FalsePositive),
                _ => None,
            }
        }
    }
    /// Communicates why a given incident is deemed relevant in the context of a
    /// given project. This enum lists all possible detailed states of relevance.
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
    pub enum Relevance {
        /// Unspecified relevance.
        Unspecified = 0,
        /// The relevance of the incident to the project is unknown.
        Unknown = 2,
        /// The incident does not impact the project.
        NotImpacted = 6,
        /// The incident is associated with a Google Cloud product your project uses,
        /// but the incident may not be impacting your project. For example, the
        /// incident may be impacting a Google Cloud product that your project uses,
        /// but in a location that your project does not use.
        PartiallyRelated = 7,
        /// The incident has a direct connection with your project and impacts a
        /// Google Cloud product in a location your project uses.
        Related = 8,
        /// The incident is verified to be impacting your project.
        Impacted = 9,
    }
    impl Relevance {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Relevance::Unspecified => "RELEVANCE_UNSPECIFIED",
                Relevance::Unknown => "UNKNOWN",
                Relevance::NotImpacted => "NOT_IMPACTED",
                Relevance::PartiallyRelated => "PARTIALLY_RELATED",
                Relevance::Related => "RELATED",
                Relevance::Impacted => "IMPACTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RELEVANCE_UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN" => Some(Self::Unknown),
                "NOT_IMPACTED" => Some(Self::NotImpacted),
                "PARTIALLY_RELATED" => Some(Self::PartiallyRelated),
                "RELATED" => Some(Self::Related),
                "IMPACTED" => Some(Self::Impacted),
                _ => None,
            }
        }
    }
}
/// Represents service health events that may affect Google Cloud products used
/// across the organization. It is a read-only view and does not allow any
/// modifications.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationEvent {
    /// Output only. Identifier. Name of the event. Unique name of the event in
    /// this scope including organization ID and location using the form
    /// `organizations/{organization_id}/locations/{location}/organizationEvents/{event_id}`.
    ///
    /// `organization_id` - see [Getting your organization resource
    /// ID](<https://cloud.google.com/resource-manager/docs/creating-managing-organization#retrieving_your_organization_id>).<br>
    /// `location` - The location to get the service health events from.<br>
    /// `event_id` - Organization event ID to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Brief description for the event.
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// Output only. Free-form, human-readable description.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The category of the event.
    #[prost(enumeration = "organization_event::EventCategory", tag = "4")]
    pub category: i32,
    /// Output only. The detailed category of the event.
    #[prost(enumeration = "organization_event::DetailedCategory", tag = "17")]
    pub detailed_category: i32,
    /// Output only. The current state of the event.
    #[prost(enumeration = "organization_event::State", tag = "5")]
    pub state: i32,
    /// Output only. The current detailed state of the incident.
    #[prost(enumeration = "organization_event::DetailedState", tag = "16")]
    pub detailed_state: i32,
    /// Output only. Represents the Google Cloud products and locations impacted by
    /// the event.
    #[prost(message, repeated, tag = "15")]
    pub event_impacts: ::prost::alloc::vec::Vec<EventImpact>,
    /// Output only. Incident-only field. Event updates are correspondence from
    /// Google.
    #[prost(message, repeated, tag = "8")]
    pub updates: ::prost::alloc::vec::Vec<EventUpdate>,
    /// Output only. When `detailed_state`=`MERGED`, `parent_event` contains the
    /// name of the parent event. All further updates will be published to the
    /// parent event.
    #[prost(string, tag = "9")]
    pub parent_event: ::prost::alloc::string::String,
    /// Output only. The time the update was posted.
    #[prost(message, optional, tag = "11")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The start time of the event, if applicable.
    #[prost(message, optional, tag = "12")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The end time of the event, if applicable.
    #[prost(message, optional, tag = "13")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Incident-only field. The time when the next update can be
    /// expected.
    #[prost(message, optional, tag = "14")]
    pub next_update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `OrganizationEvent`.
pub mod organization_event {
    /// The category of the event. This enum lists all possible categories of
    /// event.
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
    pub enum EventCategory {
        /// Unspecified category.
        Unspecified = 0,
        /// Event category for service outage or degradation.
        Incident = 2,
    }
    impl EventCategory {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EventCategory::Unspecified => "EVENT_CATEGORY_UNSPECIFIED",
                EventCategory::Incident => "INCIDENT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EVENT_CATEGORY_UNSPECIFIED" => Some(Self::Unspecified),
                "INCIDENT" => Some(Self::Incident),
                _ => None,
            }
        }
    }
    /// The detailed category of an event. Contains all possible states for all
    /// event categories.
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
    pub enum DetailedCategory {
        /// Unspecified detailed category.
        Unspecified = 0,
        /// Indicates an event with category INCIDENT has a confirmed impact to at
        /// least one Google Cloud product.
        ConfirmedIncident = 1,
        /// Indicates an event with category INCIDENT is under investigation to
        /// determine if it has a confirmed impact on any Google Cloud products.
        EmergingIncident = 2,
    }
    impl DetailedCategory {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DetailedCategory::Unspecified => "DETAILED_CATEGORY_UNSPECIFIED",
                DetailedCategory::ConfirmedIncident => "CONFIRMED_INCIDENT",
                DetailedCategory::EmergingIncident => "EMERGING_INCIDENT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DETAILED_CATEGORY_UNSPECIFIED" => Some(Self::Unspecified),
                "CONFIRMED_INCIDENT" => Some(Self::ConfirmedIncident),
                "EMERGING_INCIDENT" => Some(Self::EmergingIncident),
                _ => None,
            }
        }
    }
    /// The state of the organization event. This enum lists all possible states of
    /// event.
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
        /// Event is actively affecting a Google Cloud product and will continue to
        /// receive updates.
        Active = 1,
        /// Event is no longer affecting the Google Cloud product or has been merged
        /// with another event.
        Closed = 2,
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
                State::Closed => "CLOSED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "CLOSED" => Some(Self::Closed),
                _ => None,
            }
        }
    }
    /// The detailed state of the incident. This enum lists all possible detailed
    /// states of an incident.
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
    pub enum DetailedState {
        /// Unspecified detail state.
        Unspecified = 0,
        /// Google engineers are actively investigating the incident to determine the
        /// impact.
        Emerging = 1,
        /// The incident is confirmed and impacting at least one Google Cloud
        /// product. Ongoing status updates will be provided until it is resolved.
        Confirmed = 2,
        /// The incident is no longer affecting any Google Cloud product, and there
        /// will be no further updates.
        Resolved = 3,
        /// The incident was merged into a parent event. All further updates will be
        /// published to the parent only. The `parent_event` contains the name of the
        /// parent.
        Merged = 4,
        /// The incident was automatically closed because the issues couldn’t be
        /// confirmed or is no longer impacting Google Cloud Products and/or
        /// Locations.
        AutoClosed = 9,
        /// The incident was verified as non-impactful. No further action required.
        FalsePositive = 10,
    }
    impl DetailedState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DetailedState::Unspecified => "DETAILED_STATE_UNSPECIFIED",
                DetailedState::Emerging => "EMERGING",
                DetailedState::Confirmed => "CONFIRMED",
                DetailedState::Resolved => "RESOLVED",
                DetailedState::Merged => "MERGED",
                DetailedState::AutoClosed => "AUTO_CLOSED",
                DetailedState::FalsePositive => "FALSE_POSITIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DETAILED_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "EMERGING" => Some(Self::Emerging),
                "CONFIRMED" => Some(Self::Confirmed),
                "RESOLVED" => Some(Self::Resolved),
                "MERGED" => Some(Self::Merged),
                "AUTO_CLOSED" => Some(Self::AutoClosed),
                "FALSE_POSITIVE" => Some(Self::FalsePositive),
                _ => None,
            }
        }
    }
}
/// Records an update made to the event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUpdate {
    /// Output only. The time the update was posted.
    #[prost(message, optional, tag = "1")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Brief title for the event.
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// Output only. Free-form, human-readable description.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Symptoms of the event, if available.
    #[prost(string, tag = "4")]
    pub symptom: ::prost::alloc::string::String,
    /// Output only. Workaround steps to remediate the event impact, if available.
    #[prost(string, tag = "5")]
    pub workaround: ::prost::alloc::string::String,
}
/// Represents the locations impacted by the event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    /// Location impacted by the event. Example: `"us-central1"`
    #[prost(string, tag = "1")]
    pub location_name: ::prost::alloc::string::String,
}
/// Represents the Google Cloud product impacted by the event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Product {
    /// Google Cloud product impacted by the event. Example: `"Google Cloud SQL"`
    #[prost(string, tag = "1")]
    pub product_name: ::prost::alloc::string::String,
}
/// Represents the Google Cloud products and locations impacted by the event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventImpact {
    /// Google Cloud product impacted by the event.
    #[prost(message, optional, tag = "1")]
    pub product: ::core::option::Option<Product>,
    /// Location impacted by the event.
    #[prost(message, optional, tag = "2")]
    pub location: ::core::option::Option<Location>,
}
/// Represents impact to assets at organizational level. It is a read-only view
/// and does not allow any modifications.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationImpact {
    /// Output only. Identifier. Unique name of the organization impact in this
    /// scope including organization and location using the form
    /// `organizations/{organization_id}/locations/{location}/organizationImpacts/{organization_impact_id}`.
    ///
    /// `organization_id` - ID (number) of the organization that contains the
    /// event. To get your `organization_id`, see
    /// [Getting your organization resource
    /// ID](<https://cloud.google.com/resource-manager/docs/creating-managing-organization#retrieving_your_organization_id>).<br>
    /// `organization_impact_id` - ID of the [OrganizationImpact
    /// resource](/service-health/docs/reference/rest/v1beta/organizations.locations.organizationImpacts#OrganizationImpact).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. A list of event names impacting the asset.
    #[prost(string, repeated, tag = "2")]
    pub events: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. Google Cloud asset possibly impacted by the specified events.
    #[prost(message, optional, tag = "3")]
    pub asset: ::core::option::Option<Asset>,
    /// Output only. The time when the affected project was last modified.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Represents the asset impacted by the events.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    /// Output only. Full name of the resource as defined in
    /// [Resource
    /// Names](<https://cloud.google.com/apis/design/resource_names#full_resource_name>).
    #[prost(string, tag = "1")]
    pub asset_name: ::prost::alloc::string::String,
    /// Output only. Type of the asset. Example:
    /// `"cloudresourcemanager.googleapis.com/Project"`
    #[prost(string, tag = "2")]
    pub asset_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventsRequest {
    /// Required. Parent value using the form
    /// `projects/{project_id}/locations/{location}/events`.
    ///
    /// `project_id` - ID of the project for which to list service health
    /// events.
    /// `location` - The location to get the service health events from.
    /// To retrieve service health events of category = INCIDENT, use `location` =
    /// `global`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of events that should be returned.  Acceptable
    /// values are 1 to 100, inclusive. (The default value is 10.) If more results
    /// are available, the service returns a next_page_token that you can use to
    /// get the next page of results in subsequent list requests. The service may
    /// return fewer events than the requested page_size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    /// Provide Page token returned by a previous `ListEvents` call to retrieve the
    /// next page of results. When paginating, all other parameters provided to
    /// `ListEvents` must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. A filter expression that filters resources listed in the
    /// response. The expression takes the following forms: <br>
    /// *   field=value for `category` and `state`<br>
    /// *   field &lt;, >, &lt;=, or >= value for `update_time` <br>
    /// Examples: `category=INCIDENT`, `update_time>=2000-01-01T11:30:00-04:00`
    /// <br>
    ///
    /// Multiple filter queries are separated by spaces. Example:
    /// `category=INCIDENT state=ACTIVE`.
    ///
    /// By default, each expression is an AND expression. However, you can include
    /// AND and OR expressions explicitly.
    ///
    /// Filter is supported for the following fields: `category`, `state`,
    /// `update_time`
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Event fields to include in response.
    #[prost(enumeration = "EventView", tag = "6")]
    pub view: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventsResponse {
    /// Output only. List of events.
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    /// Output only. The continuation token, used to page through large result
    /// sets. Provide this value in a subsequent request as page_token to retrieve
    /// the next page.
    ///
    /// If this field is not present, there are no subsequent results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Output only. Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting an event
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventRequest {
    /// Required. Unique name of the event in this scope including project
    /// and location using the form
    /// `projects/{project_id}/locations/{location}/events/{event_id}`.
    ///
    /// `project_id` - Project ID of the project that contains the event. <br>
    /// `location` - The location to get the service health events from. <br>
    /// `event_id` - Event ID to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrganizationEventsRequest {
    /// Required. Parent value using the form
    /// `organizations/{organization_id}/locations/{location}/organizationEvents`.
    ///
    /// `organization_id` - ID (number) of the project that contains the event. To
    /// get your `organization_id`, see
    /// [Getting your organization resource
    /// ID](<https://cloud.google.com/resource-manager/docs/creating-managing-organization#retrieving_your_organization_id>).<br>
    /// `location` - The location to get the service health events from. To
    /// retrieve service health events of category = INCIDENT, use `location` =
    /// `global`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of events that should be returned.  Acceptable
    /// values are `1` to `100`, inclusive. (The default value is `10`.) If more
    /// results are available, the service returns a `next_page_token` that you can
    /// use to get the next page of results in subsequent list requests. The
    /// service may return fewer events than the requested `page_size`.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    ///
    /// Provide Page token returned by a previous `ListOrganizationEvents` call to
    /// retrieve the next page of results.
    ///
    /// When paginating, all other parameters provided to
    /// `ListOrganizationEvents` must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. A filter expression that filters resources listed in the
    /// response. The expression takes the following forms:
    ///
    /// *   field=value for `category` and `state`
    /// *   field &lt;, >, &lt;=, or >= value for `update_time`
    ///
    /// Examples: `category=INCIDENT`, `update_time>=2000-01-01T11:30:00-04:00`
    ///
    /// Multiple filter queries are space-separated. Example:
    /// `category=INCIDENT state=ACTIVE`.
    ///
    /// By default, each expression is an AND expression. However, you can include
    /// AND and OR expressions explicitly.
    ///
    /// Filter is supported for the following fields: `category`, `state`,
    /// `update_time`
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. OrganizationEvent fields to include in response.
    #[prost(enumeration = "OrganizationEventView", tag = "6")]
    pub view: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrganizationEventsResponse {
    /// Output only. List of organization events affecting an organization.
    #[prost(message, repeated, tag = "1")]
    pub organization_events: ::prost::alloc::vec::Vec<OrganizationEvent>,
    /// Output only. The continuation token, used to page through large result
    /// sets. Provide this value in a subsequent request as `page_token` to
    /// retrieve the next page.
    ///
    /// If this field is not present, there are no subsequent results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Output only. Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrganizationEventRequest {
    /// Required. Unique name of the event in this scope including organization and
    /// event ID using the form
    /// `organizations/{organization_id}/locations/locations/global/organizationEvents/{event_id}`.
    ///
    /// `organization_id` - ID (number) of the project that contains the event. To
    /// get your `organization_id`, see
    /// [Getting your organization resource
    /// ID](<https://cloud.google.com/resource-manager/docs/creating-managing-organization#retrieving_your_organization_id>).<br>
    /// `event_id` - Organization event ID to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for requesting list of OrganizationImpacts
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrganizationImpactsRequest {
    /// Required. Parent value using the form
    /// `organizations/{organization_id}/locations/{location}/organizationImpacts`.
    ///
    /// `organization_id` - ID (number) of the project that contains the event. To
    /// get your `organization_id`, see
    /// [Getting your organization resource
    /// ID](<https://cloud.google.com/resource-manager/docs/creating-managing-organization#retrieving_your_organization_id>).
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of events that should be returned. Acceptable
    /// values are `1` to `100`, inclusive. The default value is `10`.
    ///
    ///   If more results are available, the service returns a
    /// `next_page_token` that can be used to get the next page of results in
    /// subsequent list requests. The service may return fewer
    /// [impacts](/service-health/docs/reference/rest/v1beta/organizations.locations.organizationImpacts#OrganizationImpact)
    /// than the requested `page_size`.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    ///
    /// Provide `page_token` returned by a previous `ListOrganizationImpacts` call
    /// to retrieve the next page of results.
    ///
    /// When paginating, all other parameters provided to `ListOrganizationImpacts`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. A filter expression that filters resources listed in the
    /// response. The expression is in the form of `field:value` for checking if a
    /// repeated field contains a value.
    ///
    /// Example:
    /// `events:organizations%2F{organization_id}%2Flocations%2Fglobal%2ForganizationEvents%2Fevent-id`
    ///
    /// To get your `{organization_id}`, see
    /// [Getting your organization resource
    /// ID](<https://cloud.google.com/resource-manager/docs/creating-managing-organization#retrieving_your_organization_id>).
    ///
    /// Multiple filter queries are separated by spaces.
    ///
    /// By default, each expression is an AND expression. However, you can include
    /// AND and OR expressions explicitly.
    /// Filter is supported for the following fields: `events`.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrganizationImpactsResponse {
    /// Output only. List of
    /// [impacts](/service-health/docs/reference/rest/v1beta/organizations.locations.organizationImpacts#OrganizationImpact)
    /// for an organization affected by service health events.
    #[prost(message, repeated, tag = "1")]
    pub organization_impacts: ::prost::alloc::vec::Vec<OrganizationImpact>,
    /// Output only. The continuation token, used to page through large result
    /// sets. Provide this value in a subsequent request as `page_token` to
    /// retrieve the next page.
    ///
    /// If this field is not present, there are no subsequent results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Output only. Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrganizationImpactRequest {
    /// Required. Name of the resource using the form
    /// `organizations/{organization_id}/locations/global/organizationImpacts/{organization_impact_id}`.
    ///
    /// `organization_id` - ID (number) of the organization that contains the
    /// event. To get your `organization_id`, see
    /// [Getting your organization resource
    /// ID](<https://cloud.google.com/resource-manager/docs/creating-managing-organization#retrieving_your_organization_id>).<br>
    /// `organization_impact_id` - ID of the [OrganizationImpact
    /// resource](/service-health/docs/reference/rest/v1beta/organizations.locations.organizationImpacts#OrganizationImpact).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The event fields to include in ListEvents API response. This enum lists all
/// possible event views.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EventView {
    /// Unspecified event view. Default to `EVENT_VIEW_BASIC`.
    Unspecified = 0,
    /// Includes all fields except `updates`. This view is the default for
    /// ListEvents API.
    Basic = 1,
    /// Includes all event fields.
    Full = 2,
}
impl EventView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EventView::Unspecified => "EVENT_VIEW_UNSPECIFIED",
            EventView::Basic => "EVENT_VIEW_BASIC",
            EventView::Full => "EVENT_VIEW_FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EVENT_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "EVENT_VIEW_BASIC" => Some(Self::Basic),
            "EVENT_VIEW_FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// The organization event fields to include in ListOrganizationEvents API
/// response. This enum lists all possible organization event views.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrganizationEventView {
    /// Unspecified event view. Default to `ORGANIZATION_EVENT_VIEW_BASIC`.
    Unspecified = 0,
    /// Includes all organization event fields except `updates`. This view is the
    /// default for ListOrganizationEvents API.
    Basic = 1,
    /// Includes all organization event fields.
    Full = 2,
}
impl OrganizationEventView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrganizationEventView::Unspecified => "ORGANIZATION_EVENT_VIEW_UNSPECIFIED",
            OrganizationEventView::Basic => "ORGANIZATION_EVENT_VIEW_BASIC",
            OrganizationEventView::Full => "ORGANIZATION_EVENT_VIEW_FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORGANIZATION_EVENT_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "ORGANIZATION_EVENT_VIEW_BASIC" => Some(Self::Basic),
            "ORGANIZATION_EVENT_VIEW_FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod service_health_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Request service health events relevant to your Google Cloud project.
    #[derive(Debug, Clone)]
    pub struct ServiceHealthClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ServiceHealthClient<tonic::transport::Channel> {
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
    impl<T> ServiceHealthClient<T>
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
        ) -> ServiceHealthClient<InterceptedService<T, F>>
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
            ServiceHealthClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists events under a given project and location.
        pub async fn list_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListEventsResponse>,
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
                "/google.cloud.servicehealth.v1.ServiceHealth/ListEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicehealth.v1.ServiceHealth",
                        "ListEvents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a resource containing information about an event.
        pub async fn get_event(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEventRequest>,
        ) -> std::result::Result<tonic::Response<super::Event>, tonic::Status> {
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
                "/google.cloud.servicehealth.v1.ServiceHealth/GetEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicehealth.v1.ServiceHealth",
                        "GetEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists organization events under a given organization and location.
        pub async fn list_organization_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrganizationEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrganizationEventsResponse>,
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
                "/google.cloud.servicehealth.v1.ServiceHealth/ListOrganizationEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicehealth.v1.ServiceHealth",
                        "ListOrganizationEvents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a resource containing information about an event affecting an
        /// organization .
        pub async fn get_organization_event(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrganizationEventRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OrganizationEvent>,
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
                "/google.cloud.servicehealth.v1.ServiceHealth/GetOrganizationEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicehealth.v1.ServiceHealth",
                        "GetOrganizationEvent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists assets impacted by organization events under a given organization and
        /// location.
        pub async fn list_organization_impacts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrganizationImpactsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrganizationImpactsResponse>,
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
                "/google.cloud.servicehealth.v1.ServiceHealth/ListOrganizationImpacts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicehealth.v1.ServiceHealth",
                        "ListOrganizationImpacts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves a resource containing information about impact to an asset under
        /// an organization affected by a service health event.
        pub async fn get_organization_impact(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrganizationImpactRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OrganizationImpact>,
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
                "/google.cloud.servicehealth.v1.ServiceHealth/GetOrganizationImpact",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.servicehealth.v1.ServiceHealth",
                        "GetOrganizationImpact",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
