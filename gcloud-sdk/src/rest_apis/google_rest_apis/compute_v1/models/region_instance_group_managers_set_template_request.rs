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
pub struct RegionInstanceGroupManagersSetTemplateRequest {
    /// URL of the InstanceTemplate resource from which all new instances will be created.
    #[serde(rename = "instanceTemplate", skip_serializing_if = "Option::is_none")]
    pub instance_template: Option<String>,
}

impl RegionInstanceGroupManagersSetTemplateRequest {
    pub fn new() -> RegionInstanceGroupManagersSetTemplateRequest {
        RegionInstanceGroupManagersSetTemplateRequest {
            instance_template: None,
        }
    }
}