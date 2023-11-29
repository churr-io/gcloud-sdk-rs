use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// Reference : Represents a reference to a resource.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Reference {
    /// [Output Only] Type of the resource. Always compute#reference for references.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// A description of the reference type with no implied semantics. Possible values include: 1. MEMBER_OF
    #[serde(rename = "referenceType", skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<String>,
    /// URL of the resource which refers to the target.
    #[serde(rename = "referrer", skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,
    /// URL of the resource to which this reference points.
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

impl Reference {
    /// Represents a reference to a resource.
    pub fn new() -> Reference {
        Reference {
            kind: None,
            reference_type: None,
            referrer: None,
            target: None,
        }
    }
}
