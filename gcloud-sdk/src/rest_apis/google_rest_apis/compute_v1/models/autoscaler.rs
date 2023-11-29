use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// Autoscaler : Represents an Autoscaler resource. Google Compute Engine has two Autoscaler resources: * [Zonal](/compute/docs/reference/rest/v1/autoscalers) * [Regional](/compute/docs/reference/rest/v1/regionAutoscalers) Use autoscalers to automatically add or delete instances from a managed instance group according to your defined autoscaling policy. For more information, read Autoscaling Groups of Instances. For zonal managed instance groups resource, use the autoscaler resource. For regional managed instance groups, use the regionAutoscalers resource.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Autoscaler {
    #[serde(rename = "autoscalingPolicy", skip_serializing_if = "Option::is_none")]
    pub autoscaling_policy:
        Option<Box<crate::google_rest_apis::compute_v1::models::AutoscalingPolicy>>,
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// An optional description of this resource. Provide this property when you create the resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// [Output Only] The unique identifier for the resource. This identifier is defined by the server.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// [Output Only] Type of the resource. Always compute#autoscaler for autoscalers.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// [Output Only] Target recommended MIG size (number of instances) computed by autoscaler. Autoscaler calculates the recommended MIG size even when the autoscaling policy mode is different from ON. This field is empty when autoscaler is not connected to an existing managed instance group or autoscaler did not generate its prediction.
    #[serde(rename = "recommendedSize", skip_serializing_if = "Option::is_none")]
    pub recommended_size: Option<i32>,
    /// [Output Only] URL of the region where the instance group resides (for autoscalers living in regional scope).
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// [Output Only] Status information of existing scaling schedules.
    #[serde(
        rename = "scalingScheduleStatus",
        skip_serializing_if = "Option::is_none"
    )]
    pub scaling_schedule_status: Option<
        ::std::collections::HashMap<
            String,
            crate::google_rest_apis::compute_v1::models::ScalingScheduleStatus,
        >,
    >,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// [Output Only] The status of the autoscaler configuration. Current set of possible values: - PENDING: Autoscaler backend hasn't read new/updated configuration. - DELETING: Configuration is being deleted. - ACTIVE: Configuration is acknowledged to be effective. Some warnings might be present in the statusDetails field. - ERROR: Configuration has errors. Actionable for users. Details are present in the statusDetails field. New values might be added in the future.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// [Output Only] Human-readable details about the current state of the autoscaler. Read the documentation for Commonly returned status messages for examples of status messages you might encounter.
    #[serde(rename = "statusDetails", skip_serializing_if = "Option::is_none")]
    pub status_details:
        Option<Vec<crate::google_rest_apis::compute_v1::models::AutoscalerStatusDetails>>,
    /// URL of the managed instance group that this autoscaler will scale. This field is required when creating an autoscaler.
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// [Output Only] URL of the zone where the instance group resides (for autoscalers living in zonal scope).
    #[serde(rename = "zone", skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

impl Autoscaler {
    /// Represents an Autoscaler resource. Google Compute Engine has two Autoscaler resources: * [Zonal](/compute/docs/reference/rest/v1/autoscalers) * [Regional](/compute/docs/reference/rest/v1/regionAutoscalers) Use autoscalers to automatically add or delete instances from a managed instance group according to your defined autoscaling policy. For more information, read Autoscaling Groups of Instances. For zonal managed instance groups resource, use the autoscaler resource. For regional managed instance groups, use the regionAutoscalers resource.
    pub fn new() -> Autoscaler {
        Autoscaler {
            autoscaling_policy: None,
            creation_timestamp: None,
            description: None,
            id: None,
            kind: None,
            name: None,
            recommended_size: None,
            region: None,
            scaling_schedule_status: None,
            self_link: None,
            status: None,
            status_details: None,
            target: None,
            zone: None,
        }
    }
}

/// [Output Only] The status of the autoscaler configuration. Current set of possible values: - PENDING: Autoscaler backend hasn't read new/updated configuration. - DELETING: Configuration is being deleted. - ACTIVE: Configuration is acknowledged to be effective. Some warnings might be present in the statusDetails field. - ERROR: Configuration has errors. Actionable for users. Details are present in the statusDetails field. New values might be added in the future.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "DELETING")]
    Deleting,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "PENDING")]
    Pending,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}
