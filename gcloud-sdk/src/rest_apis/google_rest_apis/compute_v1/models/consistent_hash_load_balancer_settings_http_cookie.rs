use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// ConsistentHashLoadBalancerSettingsHttpCookie : The information about the HTTP Cookie on which the hash function is based for load balancing policies that use a consistent hash.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConsistentHashLoadBalancerSettingsHttpCookie {
    /// Name of the cookie.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Path to set for the cookie.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<Box<crate::google_rest_apis::compute_v1::models::Duration>>,
}

impl ConsistentHashLoadBalancerSettingsHttpCookie {
    /// The information about the HTTP Cookie on which the hash function is based for load balancing policies that use a consistent hash.
    pub fn new() -> ConsistentHashLoadBalancerSettingsHttpCookie {
        ConsistentHashLoadBalancerSettingsHttpCookie {
            name: None,
            path: None,
            ttl: None,
        }
    }
}
