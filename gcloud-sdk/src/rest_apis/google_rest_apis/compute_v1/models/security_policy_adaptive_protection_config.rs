use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// SecurityPolicyAdaptiveProtectionConfig : Configuration options for Cloud Armor Adaptive Protection (CAAP).

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecurityPolicyAdaptiveProtectionConfig {
    #[serde(rename = "layer7DdosDefenseConfig", skip_serializing_if = "Option::is_none")]
    pub layer7_ddos_defense_config: Option<Box<crate::google_rest_apis::compute_v1::models::SecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfig>>,
}

impl SecurityPolicyAdaptiveProtectionConfig {
    /// Configuration options for Cloud Armor Adaptive Protection (CAAP).
    pub fn new() -> SecurityPolicyAdaptiveProtectionConfig {
        SecurityPolicyAdaptiveProtectionConfig {
            layer7_ddos_defense_config: None,
        }
    }
}