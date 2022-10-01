use serde::{Deserialize, Serialize}; /*
                                      * Cloud Storage JSON API
                                      *
                                      * Stores and retrieves potentially large, immutable data objects.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BucketCorsInner {
    /// The value, in seconds, to return in the  Access-Control-Max-Age header used in preflight responses.
    #[serde(rename = "maxAgeSeconds", skip_serializing_if = "Option::is_none")]
    pub max_age_seconds: Option<i32>,
    /// The list of HTTP methods on which to include CORS response headers, (GET, OPTIONS, POST, etc) Note: \"*\" is permitted in the list of methods, and means \"any method\".
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<Vec<String>>,
    /// The list of Origins eligible to receive CORS response headers. Note: \"*\" is permitted in the list of origins, and means \"any Origin\".
    #[serde(rename = "origin", skip_serializing_if = "Option::is_none")]
    pub origin: Option<Vec<String>>,
    /// The list of HTTP headers other than the simple response headers to give permission for the user-agent to share across domains.
    #[serde(rename = "responseHeader", skip_serializing_if = "Option::is_none")]
    pub response_header: Option<Vec<String>>,
}

impl BucketCorsInner {
    pub fn new() -> BucketCorsInner {
        BucketCorsInner {
            max_age_seconds: None,
            method: None,
            origin: None,
            response_header: None,
        }
    }
}