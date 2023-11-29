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
pub struct UrlMapsValidateRequest {
    /// Specifies the load balancer type(s) this validation request is for. Use EXTERNAL_MANAGED for global external Application Load Balancers and regional external Application Load Balancers. Use EXTERNAL for classic Application Load Balancers. Use INTERNAL_MANAGED for internal Application Load Balancers. For more information, refer to Choosing a load balancer. If unspecified, the load balancing scheme will be inferred from the backend service resources this URL map references. If that can not be inferred (for example, this URL map only references backend buckets, or this Url map is for rewrites and redirects only and doesn't reference any backends), EXTERNAL will be used as the default type. If specified, the scheme(s) must not conflict with the load balancing scheme of the backend service resources this Url map references.
    #[serde(
        rename = "loadBalancingSchemes",
        skip_serializing_if = "Option::is_none"
    )]
    pub load_balancing_schemes: Option<Vec<LoadBalancingSchemes>>,
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<Box<crate::google_rest_apis::compute_v1::models::UrlMap>>,
}

impl UrlMapsValidateRequest {
    pub fn new() -> UrlMapsValidateRequest {
        UrlMapsValidateRequest {
            load_balancing_schemes: None,
            resource: None,
        }
    }
}

/// Specifies the load balancer type(s) this validation request is for. Use EXTERNAL_MANAGED for global external Application Load Balancers and regional external Application Load Balancers. Use EXTERNAL for classic Application Load Balancers. Use INTERNAL_MANAGED for internal Application Load Balancers. For more information, refer to Choosing a load balancer. If unspecified, the load balancing scheme will be inferred from the backend service resources this URL map references. If that can not be inferred (for example, this URL map only references backend buckets, or this Url map is for rewrites and redirects only and doesn't reference any backends), EXTERNAL will be used as the default type. If specified, the scheme(s) must not conflict with the load balancing scheme of the backend service resources this Url map references.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LoadBalancingSchemes {
    #[serde(rename = "EXTERNAL")]
    External,
    #[serde(rename = "EXTERNAL_MANAGED")]
    ExternalManaged,
    #[serde(rename = "LOAD_BALANCING_SCHEME_UNSPECIFIED")]
    LoadBalancingSchemeUnspecified,
}

impl Default for LoadBalancingSchemes {
    fn default() -> LoadBalancingSchemes {
        Self::External
    }
}
