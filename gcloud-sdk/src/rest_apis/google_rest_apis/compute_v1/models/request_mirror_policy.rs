use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// RequestMirrorPolicy : A policy that specifies how requests intended for the route's backends are shadowed to a separate mirrored backend service. The load balancer doesn't wait for responses from the shadow service. Before sending traffic to the shadow service, the host or authority header is suffixed with -shadow.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RequestMirrorPolicy {
    /// The full or partial URL to the BackendService resource being mirrored to. The backend service configured for a mirroring policy must reference backends that are of the same type as the original backend service matched in the URL map. Serverless NEG backends are not currently supported as a mirrored backend service.
    #[serde(rename = "backendService", skip_serializing_if = "Option::is_none")]
    pub backend_service: Option<String>,
}

impl RequestMirrorPolicy {
    /// A policy that specifies how requests intended for the route's backends are shadowed to a separate mirrored backend service. The load balancer doesn't wait for responses from the shadow service. Before sending traffic to the shadow service, the host or authority header is suffixed with -shadow.
    pub fn new() -> RequestMirrorPolicy {
        RequestMirrorPolicy {
            backend_service: None,
        }
    }
}
