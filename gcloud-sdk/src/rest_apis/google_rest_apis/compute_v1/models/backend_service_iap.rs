use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// BackendServiceIap : Identity-Aware Proxy

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BackendServiceIap {
    /// Whether the serving infrastructure will authenticate and authorize all incoming requests.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// OAuth2 client ID to use for the authentication flow.
    #[serde(rename = "oauth2ClientId", skip_serializing_if = "Option::is_none")]
    pub oauth2_client_id: Option<String>,
    /// OAuth2 client secret to use for the authentication flow. For security reasons, this value cannot be retrieved via the API. Instead, the SHA-256 hash of the value is returned in the oauth2ClientSecretSha256 field. @InputOnly
    #[serde(rename = "oauth2ClientSecret", skip_serializing_if = "Option::is_none")]
    pub oauth2_client_secret: Option<String>,
    /// [Output Only] SHA256 hash value for the field oauth2_client_secret above.
    #[serde(
        rename = "oauth2ClientSecretSha256",
        skip_serializing_if = "Option::is_none"
    )]
    pub oauth2_client_secret_sha256: Option<String>,
}

impl BackendServiceIap {
    /// Identity-Aware Proxy
    pub fn new() -> BackendServiceIap {
        BackendServiceIap {
            enabled: None,
            oauth2_client_id: None,
            oauth2_client_secret: None,
            oauth2_client_secret_sha256: None,
        }
    }
}
