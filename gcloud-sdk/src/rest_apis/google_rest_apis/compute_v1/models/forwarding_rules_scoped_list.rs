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
pub struct ForwardingRulesScopedList {
    /// A list of forwarding rules contained in this scope.
    #[serde(rename = "forwardingRules", skip_serializing_if = "Option::is_none")]
    pub forwarding_rules: Option<Vec<crate::google_rest_apis::compute_v1::models::ForwardingRule>>,
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning:
        Option<Box<crate::google_rest_apis::compute_v1::models::ForwardingRulesScopedListWarning>>,
}

impl ForwardingRulesScopedList {
    pub fn new() -> ForwardingRulesScopedList {
        ForwardingRulesScopedList {
            forwarding_rules: None,
            warning: None,
        }
    }
}
