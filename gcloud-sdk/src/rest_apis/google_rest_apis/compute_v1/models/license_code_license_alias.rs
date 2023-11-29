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
pub struct LicenseCodeLicenseAlias {
    /// [Output Only] Description of this License Code.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// [Output Only] URL of license corresponding to this License Code.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
}

impl LicenseCodeLicenseAlias {
    pub fn new() -> LicenseCodeLicenseAlias {
        LicenseCodeLicenseAlias {
            description: None,
            self_link: None,
        }
    }
}
