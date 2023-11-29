use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// InterconnectMacsecPreSharedKey : Describes a pre-shared key used to setup MACsec in static connectivity association key (CAK) mode.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InterconnectMacsecPreSharedKey {
    /// Required. A name for this pre-shared key. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A RFC3339 timestamp on or after which the key is valid. startTime can be in the future. If the keychain has a single key, startTime can be omitted. If the keychain has multiple keys, startTime is mandatory for each key. The start times of keys must be in increasing order. The start times of two consecutive keys must be at least 6 hours apart.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

impl InterconnectMacsecPreSharedKey {
    /// Describes a pre-shared key used to setup MACsec in static connectivity association key (CAK) mode.
    pub fn new() -> InterconnectMacsecPreSharedKey {
        InterconnectMacsecPreSharedKey {
            name: None,
            start_time: None,
        }
    }
}