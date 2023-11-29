use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// BackendServiceConnectionTrackingPolicy : Connection Tracking configuration for this BackendService.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BackendServiceConnectionTrackingPolicy {
    /// Specifies connection persistence when backends are unhealthy. The default value is DEFAULT_FOR_PROTOCOL. If set to DEFAULT_FOR_PROTOCOL, the existing connections persist on unhealthy backends only for connection-oriented protocols (TCP and SCTP) and only if the Tracking Mode is PER_CONNECTION (default tracking mode) or the Session Affinity is configured for 5-tuple. They do not persist for UDP. If set to NEVER_PERSIST, after a backend becomes unhealthy, the existing connections on the unhealthy backend are never persisted on the unhealthy backend. They are always diverted to newly selected healthy backends (unless all backends are unhealthy). If set to ALWAYS_PERSIST, existing connections always persist on unhealthy backends regardless of protocol and session affinity. It is generally not recommended to use this mode overriding the default. For more details, see [Connection Persistence for Network Load Balancing](https://cloud.google.com/load-balancing/docs/network/networklb-backend-service#connection-persistence) and [Connection Persistence for Internal TCP/UDP Load Balancing](https://cloud.google.com/load-balancing/docs/internal#connection-persistence).
    #[serde(
        rename = "connectionPersistenceOnUnhealthyBackends",
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_persistence_on_unhealthy_backends:
        Option<ConnectionPersistenceOnUnhealthyBackends>,
    /// Enable Strong Session Affinity for Network Load Balancing. This option is not available publicly.
    #[serde(
        rename = "enableStrongAffinity",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_strong_affinity: Option<bool>,
    /// Specifies how long to keep a Connection Tracking entry while there is no matching traffic (in seconds). For Internal TCP/UDP Load Balancing: - The minimum (default) is 10 minutes and the maximum is 16 hours. - It can be set only if Connection Tracking is less than 5-tuple (i.e. Session Affinity is CLIENT_IP_NO_DESTINATION, CLIENT_IP or CLIENT_IP_PROTO, and Tracking Mode is PER_SESSION). For Network Load Balancer the default is 60 seconds. This option is not available publicly.
    #[serde(rename = "idleTimeoutSec", skip_serializing_if = "Option::is_none")]
    pub idle_timeout_sec: Option<i32>,
    /// Specifies the key used for connection tracking. There are two options: - PER_CONNECTION: This is the default mode. The Connection Tracking is performed as per the Connection Key (default Hash Method) for the specific protocol. - PER_SESSION: The Connection Tracking is performed as per the configured Session Affinity. It matches the configured Session Affinity. For more details, see [Tracking Mode for Network Load Balancing](https://cloud.google.com/load-balancing/docs/network/networklb-backend-service#tracking-mode) and [Tracking Mode for Internal TCP/UDP Load Balancing](https://cloud.google.com/load-balancing/docs/internal#tracking-mode).
    #[serde(rename = "trackingMode", skip_serializing_if = "Option::is_none")]
    pub tracking_mode: Option<TrackingMode>,
}

impl BackendServiceConnectionTrackingPolicy {
    /// Connection Tracking configuration for this BackendService.
    pub fn new() -> BackendServiceConnectionTrackingPolicy {
        BackendServiceConnectionTrackingPolicy {
            connection_persistence_on_unhealthy_backends: None,
            enable_strong_affinity: None,
            idle_timeout_sec: None,
            tracking_mode: None,
        }
    }
}

/// Specifies connection persistence when backends are unhealthy. The default value is DEFAULT_FOR_PROTOCOL. If set to DEFAULT_FOR_PROTOCOL, the existing connections persist on unhealthy backends only for connection-oriented protocols (TCP and SCTP) and only if the Tracking Mode is PER_CONNECTION (default tracking mode) or the Session Affinity is configured for 5-tuple. They do not persist for UDP. If set to NEVER_PERSIST, after a backend becomes unhealthy, the existing connections on the unhealthy backend are never persisted on the unhealthy backend. They are always diverted to newly selected healthy backends (unless all backends are unhealthy). If set to ALWAYS_PERSIST, existing connections always persist on unhealthy backends regardless of protocol and session affinity. It is generally not recommended to use this mode overriding the default. For more details, see [Connection Persistence for Network Load Balancing](https://cloud.google.com/load-balancing/docs/network/networklb-backend-service#connection-persistence) and [Connection Persistence for Internal TCP/UDP Load Balancing](https://cloud.google.com/load-balancing/docs/internal#connection-persistence).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConnectionPersistenceOnUnhealthyBackends {
    #[serde(rename = "ALWAYS_PERSIST")]
    AlwaysPersist,
    #[serde(rename = "DEFAULT_FOR_PROTOCOL")]
    DefaultForProtocol,
    #[serde(rename = "NEVER_PERSIST")]
    NeverPersist,
}

impl Default for ConnectionPersistenceOnUnhealthyBackends {
    fn default() -> ConnectionPersistenceOnUnhealthyBackends {
        Self::AlwaysPersist
    }
}
/// Specifies the key used for connection tracking. There are two options: - PER_CONNECTION: This is the default mode. The Connection Tracking is performed as per the Connection Key (default Hash Method) for the specific protocol. - PER_SESSION: The Connection Tracking is performed as per the configured Session Affinity. It matches the configured Session Affinity. For more details, see [Tracking Mode for Network Load Balancing](https://cloud.google.com/load-balancing/docs/network/networklb-backend-service#tracking-mode) and [Tracking Mode for Internal TCP/UDP Load Balancing](https://cloud.google.com/load-balancing/docs/internal#tracking-mode).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TrackingMode {
    #[serde(rename = "INVALID_TRACKING_MODE")]
    InvalidTrackingMode,
    #[serde(rename = "PER_CONNECTION")]
    PerConnection,
    #[serde(rename = "PER_SESSION")]
    PerSession,
}

impl Default for TrackingMode {
    fn default() -> TrackingMode {
        Self::InvalidTrackingMode
    }
}
