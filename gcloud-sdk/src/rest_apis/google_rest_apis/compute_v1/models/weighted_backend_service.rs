use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// WeightedBackendService : In contrast to a single BackendService in HttpRouteAction to which all matching traffic is directed to, WeightedBackendService allows traffic to be split across multiple backend services. The volume of traffic for each backend service is proportional to the weight specified in each WeightedBackendService

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WeightedBackendService {
    /// The full or partial URL to the default BackendService resource. Before forwarding the request to backendService, the load balancer applies any relevant headerActions specified as part of this backendServiceWeight.
    #[serde(rename = "backendService", skip_serializing_if = "Option::is_none")]
    pub backend_service: Option<String>,
    #[serde(rename = "headerAction", skip_serializing_if = "Option::is_none")]
    pub header_action: Option<Box<crate::google_rest_apis::compute_v1::models::HttpHeaderAction>>,
    /// Specifies the fraction of traffic sent to a backend service, computed as weight / (sum of all weightedBackendService weights in routeAction) . The selection of a backend service is determined only for new traffic. Once a user's request has been directed to a backend service, subsequent requests are sent to the same backend service as determined by the backend service's session affinity policy. The value must be from 0 to 1000.
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

impl WeightedBackendService {
    /// In contrast to a single BackendService in HttpRouteAction to which all matching traffic is directed to, WeightedBackendService allows traffic to be split across multiple backend services. The volume of traffic for each backend service is proportional to the weight specified in each WeightedBackendService
    pub fn new() -> WeightedBackendService {
        WeightedBackendService {
            backend_service: None,
            header_action: None,
            weight: None,
        }
    }
}
