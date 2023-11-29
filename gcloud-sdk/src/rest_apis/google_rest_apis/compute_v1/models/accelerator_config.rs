use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// AcceleratorConfig : A specification of the type and number of accelerator cards attached to the instance.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AcceleratorConfig {
    /// The number of the guest accelerator cards exposed to this instance.
    #[serde(rename = "acceleratorCount", skip_serializing_if = "Option::is_none")]
    pub accelerator_count: Option<i32>,
    /// Full or partial URL of the accelerator type resource to attach to this instance. For example: projects/my-project/zones/us-central1-c/acceleratorTypes/nvidia-tesla-p100 If you are creating an instance template, specify only the accelerator name. See GPUs on Compute Engine for a full list of accelerator types.
    #[serde(rename = "acceleratorType", skip_serializing_if = "Option::is_none")]
    pub accelerator_type: Option<String>,
}

impl AcceleratorConfig {
    /// A specification of the type and number of accelerator cards attached to the instance.
    pub fn new() -> AcceleratorConfig {
        AcceleratorConfig {
            accelerator_count: None,
            accelerator_type: None,
        }
    }
}
