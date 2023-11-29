use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// DisplayDevice : A set of Display Device options

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DisplayDevice {
    /// Defines whether the instance has Display enabled.
    #[serde(rename = "enableDisplay", skip_serializing_if = "Option::is_none")]
    pub enable_display: Option<bool>,
}

impl DisplayDevice {
    /// A set of Display Device options
    pub fn new() -> DisplayDevice {
        DisplayDevice {
            enable_display: None,
        }
    }
}
