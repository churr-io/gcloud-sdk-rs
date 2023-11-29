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
pub struct PublicDelegatedPrefixesScopedList {
    /// [Output Only] A list of PublicDelegatedPrefixes contained in this scope.
    #[serde(
        rename = "publicDelegatedPrefixes",
        skip_serializing_if = "Option::is_none"
    )]
    pub public_delegated_prefixes:
        Option<Vec<crate::google_rest_apis::compute_v1::models::PublicDelegatedPrefix>>,
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning: Option<
        Box<crate::google_rest_apis::compute_v1::models::PublicDelegatedPrefixesScopedListWarning>,
    >,
}

impl PublicDelegatedPrefixesScopedList {
    pub fn new() -> PublicDelegatedPrefixesScopedList {
        PublicDelegatedPrefixesScopedList {
            public_delegated_prefixes: None,
            warning: None,
        }
    }
}