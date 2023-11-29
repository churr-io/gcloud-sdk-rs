use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// InterconnectLocationRegionInfo : Information about any potential InterconnectAttachments between an Interconnect at a specific InterconnectLocation, and a specific Cloud Region.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InterconnectLocationRegionInfo {
    /// Expected round-trip time in milliseconds, from this InterconnectLocation to a VM in this region.
    #[serde(rename = "expectedRttMs", skip_serializing_if = "Option::is_none")]
    pub expected_rtt_ms: Option<String>,
    /// Identifies the network presence of this location.
    #[serde(rename = "locationPresence", skip_serializing_if = "Option::is_none")]
    pub location_presence: Option<LocationPresence>,
    /// URL for the region of this location.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl InterconnectLocationRegionInfo {
    /// Information about any potential InterconnectAttachments between an Interconnect at a specific InterconnectLocation, and a specific Cloud Region.
    pub fn new() -> InterconnectLocationRegionInfo {
        InterconnectLocationRegionInfo {
            expected_rtt_ms: None,
            location_presence: None,
            region: None,
        }
    }
}

/// Identifies the network presence of this location.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LocationPresence {
    #[serde(rename = "GLOBAL")]
    Global,
    #[serde(rename = "LOCAL_REGION")]
    LocalRegion,
    #[serde(rename = "LP_GLOBAL")]
    LpGlobal,
    #[serde(rename = "LP_LOCAL_REGION")]
    LpLocalRegion,
}

impl Default for LocationPresence {
    fn default() -> LocationPresence {
        Self::Global
    }
}
