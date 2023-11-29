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
pub struct TargetSslProxiesSetCertificateMapRequest {
    /// URL of the Certificate Map to associate with this TargetSslProxy. Accepted format is //certificatemanager.googleapis.com/projects/{project }/locations/{location}/certificateMaps/{resourceName}.
    #[serde(rename = "certificateMap", skip_serializing_if = "Option::is_none")]
    pub certificate_map: Option<String>,
}

impl TargetSslProxiesSetCertificateMapRequest {
    pub fn new() -> TargetSslProxiesSetCertificateMapRequest {
        TargetSslProxiesSetCertificateMapRequest {
            certificate_map: None,
        }
    }
}
