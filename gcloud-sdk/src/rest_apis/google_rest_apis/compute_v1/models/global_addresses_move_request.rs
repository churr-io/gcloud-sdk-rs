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
pub struct GlobalAddressesMoveRequest {
    /// An optional destination address description if intended to be different from the source.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The URL of the destination address to move to. This can be a full or partial URL. For example, the following are all valid URLs to a address: - https://www.googleapis.com/compute/v1/projects/project /global/addresses/address - projects/project/global/addresses/address Note that destination project must be different from the source project. So /global/addresses/address is not valid partial url.
    #[serde(rename = "destinationAddress", skip_serializing_if = "Option::is_none")]
    pub destination_address: Option<String>,
}

impl GlobalAddressesMoveRequest {
    pub fn new() -> GlobalAddressesMoveRequest {
        GlobalAddressesMoveRequest {
            description: None,
            destination_address: None,
        }
    }
}