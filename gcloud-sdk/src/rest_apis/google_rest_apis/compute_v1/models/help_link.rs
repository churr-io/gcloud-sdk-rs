use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// HelpLink : Describes a URL link.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HelpLink {
    /// Describes what the link offers.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The URL of the link.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl HelpLink {
    /// Describes a URL link.
    pub fn new() -> HelpLink {
        HelpLink {
            description: None,
            url: None,
        }
    }
}