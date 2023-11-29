use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// RouterNat : Represents a Nat resource. It enables the VMs within the specified subnetworks to access Internet without external IP addresses. It specifies a list of subnetworks (and the ranges within) that want to use NAT. Customers can also provide the external IPs that would be used for NAT. GCP would auto-allocate ephemeral IPs if no external IPs are provided.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RouterNat {
    /// The network tier to use when automatically reserving NAT IP addresses. Must be one of: PREMIUM, STANDARD. If not specified, then the current project-level default tier is used.
    #[serde(rename = "autoNetworkTier", skip_serializing_if = "Option::is_none")]
    pub auto_network_tier: Option<AutoNetworkTier>,
    /// A list of URLs of the IP resources to be drained. These IPs must be valid static external IPs that have been assigned to the NAT. These IPs should be used for updating/patching a NAT only.
    #[serde(rename = "drainNatIps", skip_serializing_if = "Option::is_none")]
    pub drain_nat_ips: Option<Vec<String>>,
    /// Enable Dynamic Port Allocation. If not specified, it is disabled by default. If set to true, - Dynamic Port Allocation will be enabled on this NAT config. - enableEndpointIndependentMapping cannot be set to true. - If minPorts is set, minPortsPerVm must be set to a power of two greater than or equal to 32. If minPortsPerVm is not set, a minimum of 32 ports will be allocated to a VM from this NAT config.
    #[serde(
        rename = "enableDynamicPortAllocation",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_dynamic_port_allocation: Option<bool>,
    #[serde(
        rename = "enableEndpointIndependentMapping",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_endpoint_independent_mapping: Option<bool>,
    /// List of NAT-ted endpoint types supported by the Nat Gateway. If the list is empty, then it will be equivalent to include ENDPOINT_TYPE_VM
    #[serde(rename = "endpointTypes", skip_serializing_if = "Option::is_none")]
    pub endpoint_types: Option<Vec<EndpointTypes>>,
    /// Timeout (in seconds) for ICMP connections. Defaults to 30s if not set.
    #[serde(rename = "icmpIdleTimeoutSec", skip_serializing_if = "Option::is_none")]
    pub icmp_idle_timeout_sec: Option<i32>,
    #[serde(rename = "logConfig", skip_serializing_if = "Option::is_none")]
    pub log_config: Option<Box<crate::google_rest_apis::compute_v1::models::RouterNatLogConfig>>,
    /// Maximum number of ports allocated to a VM from this NAT config when Dynamic Port Allocation is enabled. If Dynamic Port Allocation is not enabled, this field has no effect. If Dynamic Port Allocation is enabled, and this field is set, it must be set to a power of two greater than minPortsPerVm, or 64 if minPortsPerVm is not set. If Dynamic Port Allocation is enabled and this field is not set, a maximum of 65536 ports will be allocated to a VM from this NAT config.
    #[serde(rename = "maxPortsPerVm", skip_serializing_if = "Option::is_none")]
    pub max_ports_per_vm: Option<i32>,
    /// Minimum number of ports allocated to a VM from this NAT config. If not set, a default number of ports is allocated to a VM. This is rounded up to the nearest power of 2. For example, if the value of this field is 50, at least 64 ports are allocated to a VM.
    #[serde(rename = "minPortsPerVm", skip_serializing_if = "Option::is_none")]
    pub min_ports_per_vm: Option<i32>,
    /// Unique name of this Nat service. The name must be 1-63 characters long and comply with RFC1035.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify the NatIpAllocateOption, which can take one of the following values: - MANUAL_ONLY: Uses only Nat IP addresses provided by customers. When there are not enough specified Nat IPs, the Nat service fails for new VMs. - AUTO_ONLY: Nat IPs are allocated by Google Cloud Platform; customers can't specify any Nat IPs. When choosing AUTO_ONLY, then nat_ip should be empty.
    #[serde(
        rename = "natIpAllocateOption",
        skip_serializing_if = "Option::is_none"
    )]
    pub nat_ip_allocate_option: Option<NatIpAllocateOption>,
    /// A list of URLs of the IP resources used for this Nat service. These IP addresses must be valid static external IP addresses assigned to the project.
    #[serde(rename = "natIps", skip_serializing_if = "Option::is_none")]
    pub nat_ips: Option<Vec<String>>,
    /// A list of rules associated with this NAT.
    #[serde(rename = "rules", skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<crate::google_rest_apis::compute_v1::models::RouterNatRule>>,
    /// Specify the Nat option, which can take one of the following values: - ALL_SUBNETWORKS_ALL_IP_RANGES: All of the IP ranges in every Subnetwork are allowed to Nat. - ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES: All of the primary IP ranges in every Subnetwork are allowed to Nat. - LIST_OF_SUBNETWORKS: A list of Subnetworks are allowed to Nat (specified in the field subnetwork below) The default is SUBNETWORK_IP_RANGE_TO_NAT_OPTION_UNSPECIFIED. Note that if this field contains ALL_SUBNETWORKS_ALL_IP_RANGES then there should not be any other Router.Nat section in any Router for this network in this region.
    #[serde(
        rename = "sourceSubnetworkIpRangesToNat",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_subnetwork_ip_ranges_to_nat: Option<SourceSubnetworkIpRangesToNat>,
    /// A list of Subnetwork resources whose traffic should be translated by NAT Gateway. It is used only when LIST_OF_SUBNETWORKS is selected for the SubnetworkIpRangeToNatOption above.
    #[serde(rename = "subnetworks", skip_serializing_if = "Option::is_none")]
    pub subnetworks:
        Option<Vec<crate::google_rest_apis::compute_v1::models::RouterNatSubnetworkToNat>>,
    /// Timeout (in seconds) for TCP established connections. Defaults to 1200s if not set.
    #[serde(
        rename = "tcpEstablishedIdleTimeoutSec",
        skip_serializing_if = "Option::is_none"
    )]
    pub tcp_established_idle_timeout_sec: Option<i32>,
    /// Timeout (in seconds) for TCP connections that are in TIME_WAIT state. Defaults to 120s if not set.
    #[serde(
        rename = "tcpTimeWaitTimeoutSec",
        skip_serializing_if = "Option::is_none"
    )]
    pub tcp_time_wait_timeout_sec: Option<i32>,
    /// Timeout (in seconds) for TCP transitory connections. Defaults to 30s if not set.
    #[serde(
        rename = "tcpTransitoryIdleTimeoutSec",
        skip_serializing_if = "Option::is_none"
    )]
    pub tcp_transitory_idle_timeout_sec: Option<i32>,
    /// Indicates whether this NAT is used for public or private IP translation. If unspecified, it defaults to PUBLIC.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Timeout (in seconds) for UDP connections. Defaults to 30s if not set.
    #[serde(rename = "udpIdleTimeoutSec", skip_serializing_if = "Option::is_none")]
    pub udp_idle_timeout_sec: Option<i32>,
}

impl RouterNat {
    /// Represents a Nat resource. It enables the VMs within the specified subnetworks to access Internet without external IP addresses. It specifies a list of subnetworks (and the ranges within) that want to use NAT. Customers can also provide the external IPs that would be used for NAT. GCP would auto-allocate ephemeral IPs if no external IPs are provided.
    pub fn new() -> RouterNat {
        RouterNat {
            auto_network_tier: None,
            drain_nat_ips: None,
            enable_dynamic_port_allocation: None,
            enable_endpoint_independent_mapping: None,
            endpoint_types: None,
            icmp_idle_timeout_sec: None,
            log_config: None,
            max_ports_per_vm: None,
            min_ports_per_vm: None,
            name: None,
            nat_ip_allocate_option: None,
            nat_ips: None,
            rules: None,
            source_subnetwork_ip_ranges_to_nat: None,
            subnetworks: None,
            tcp_established_idle_timeout_sec: None,
            tcp_time_wait_timeout_sec: None,
            tcp_transitory_idle_timeout_sec: None,
            r#type: None,
            udp_idle_timeout_sec: None,
        }
    }
}

/// The network tier to use when automatically reserving NAT IP addresses. Must be one of: PREMIUM, STANDARD. If not specified, then the current project-level default tier is used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AutoNetworkTier {
    #[serde(rename = "FIXED_STANDARD")]
    FixedStandard,
    #[serde(rename = "PREMIUM")]
    Premium,
    #[serde(rename = "STANDARD")]
    Standard,
    #[serde(rename = "STANDARD_OVERRIDES_FIXED_STANDARD")]
    StandardOverridesFixedStandard,
}

impl Default for AutoNetworkTier {
    fn default() -> AutoNetworkTier {
        Self::FixedStandard
    }
}
/// List of NAT-ted endpoint types supported by the Nat Gateway. If the list is empty, then it will be equivalent to include ENDPOINT_TYPE_VM
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EndpointTypes {
    #[serde(rename = "ENDPOINT_TYPE_MANAGED_PROXY_LB")]
    ManagedProxyLb,
    #[serde(rename = "ENDPOINT_TYPE_SWG")]
    Swg,
    #[serde(rename = "ENDPOINT_TYPE_VM")]
    Vm,
}

impl Default for EndpointTypes {
    fn default() -> EndpointTypes {
        Self::ManagedProxyLb
    }
}
/// Specify the NatIpAllocateOption, which can take one of the following values: - MANUAL_ONLY: Uses only Nat IP addresses provided by customers. When there are not enough specified Nat IPs, the Nat service fails for new VMs. - AUTO_ONLY: Nat IPs are allocated by Google Cloud Platform; customers can't specify any Nat IPs. When choosing AUTO_ONLY, then nat_ip should be empty.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NatIpAllocateOption {
    #[serde(rename = "AUTO_ONLY")]
    AutoOnly,
    #[serde(rename = "MANUAL_ONLY")]
    ManualOnly,
}

impl Default for NatIpAllocateOption {
    fn default() -> NatIpAllocateOption {
        Self::AutoOnly
    }
}
/// Specify the Nat option, which can take one of the following values: - ALL_SUBNETWORKS_ALL_IP_RANGES: All of the IP ranges in every Subnetwork are allowed to Nat. - ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES: All of the primary IP ranges in every Subnetwork are allowed to Nat. - LIST_OF_SUBNETWORKS: A list of Subnetworks are allowed to Nat (specified in the field subnetwork below) The default is SUBNETWORK_IP_RANGE_TO_NAT_OPTION_UNSPECIFIED. Note that if this field contains ALL_SUBNETWORKS_ALL_IP_RANGES then there should not be any other Router.Nat section in any Router for this network in this region.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SourceSubnetworkIpRangesToNat {
    #[serde(rename = "ALL_SUBNETWORKS_ALL_IP_RANGES")]
    AllSubnetworksAllIpRanges,
    #[serde(rename = "ALL_SUBNETWORKS_ALL_PRIMARY_IP_RANGES")]
    AllSubnetworksAllPrimaryIpRanges,
    #[serde(rename = "LIST_OF_SUBNETWORKS")]
    ListOfSubnetworks,
}

impl Default for SourceSubnetworkIpRangesToNat {
    fn default() -> SourceSubnetworkIpRangesToNat {
        Self::AllSubnetworksAllIpRanges
    }
}
/// Indicates whether this NAT is used for public or private IP translation. If unspecified, it defaults to PUBLIC.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "PRIVATE")]
    Private,
    #[serde(rename = "PUBLIC")]
    Public,
}

impl Default for Type {
    fn default() -> Type {
        Self::Private
    }
}