use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// HttpRouteRule : The HttpRouteRule setting specifies how to match an HTTP request and the corresponding routing action that load balancing proxies perform.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HttpRouteRule {
    /// The short description conveying the intent of this routeRule. The description can have a maximum length of 1024 characters.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "headerAction", skip_serializing_if = "Option::is_none")]
    pub header_action: Option<Box<crate::google_rest_apis::compute_v1::models::HttpHeaderAction>>,
    /// The list of criteria for matching attributes of a request to this routeRule. This list has OR semantics: the request matches this routeRule when any of the matchRules are satisfied. However predicates within a given matchRule have AND semantics. All predicates within a matchRule must match for the request to match the rule.
    #[serde(rename = "matchRules", skip_serializing_if = "Option::is_none")]
    pub match_rules: Option<Vec<crate::google_rest_apis::compute_v1::models::HttpRouteRuleMatch>>,
    /// For routeRules within a given pathMatcher, priority determines the order in which a load balancer interprets routeRules. RouteRules are evaluated in order of priority, from the lowest to highest number. The priority of a rule decreases as its number increases (1, 2, 3, N+1). The first rule that matches the request is applied. You cannot configure two or more routeRules with the same priority. Priority for each rule must be set to a number from 0 to 2147483647 inclusive. Priority numbers can have gaps, which enable you to add or remove rules in the future without affecting the rest of the rules. For example, 1, 2, 3, 4, 5, 9, 12, 16 is a valid series of priority numbers to which you could add rules numbered from 6 to 8, 10 to 11, and 13 to 15 in the future without any impact on existing rules.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "routeAction", skip_serializing_if = "Option::is_none")]
    pub route_action: Option<Box<crate::google_rest_apis::compute_v1::models::HttpRouteAction>>,
    /// The full or partial URL of the backend service resource to which traffic is directed if this rule is matched. If routeAction is also specified, advanced routing actions, such as URL rewrites, take effect before sending the request to the backend. However, if service is specified, routeAction cannot contain any weightedBackendServices. Conversely, if routeAction specifies any weightedBackendServices, service must not be specified. Only one of urlRedirect, service or routeAction.weightedBackendService must be set.
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(rename = "urlRedirect", skip_serializing_if = "Option::is_none")]
    pub url_redirect: Option<Box<crate::google_rest_apis::compute_v1::models::HttpRedirectAction>>,
}

impl HttpRouteRule {
    /// The HttpRouteRule setting specifies how to match an HTTP request and the corresponding routing action that load balancing proxies perform.
    pub fn new() -> HttpRouteRule {
        HttpRouteRule {
            description: None,
            header_action: None,
            match_rules: None,
            priority: None,
            route_action: None,
            service: None,
            url_redirect: None,
        }
    }
}
