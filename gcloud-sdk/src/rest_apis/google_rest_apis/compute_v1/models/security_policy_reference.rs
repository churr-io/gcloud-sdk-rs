use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecurityPolicyReference {
    #[serde(rename = "securityPolicy", skip_serializing_if = "Option::is_none")]
    pub security_policy: Option<String>,
}

impl SecurityPolicyReference {
    pub fn new() -> SecurityPolicyReference {
        SecurityPolicyReference {
            security_policy: None,
        }
    }
}