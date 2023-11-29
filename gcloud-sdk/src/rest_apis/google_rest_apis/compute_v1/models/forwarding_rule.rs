use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// ForwardingRule : Represents a Forwarding Rule resource. Forwarding rule resources in Google Cloud can be either regional or global in scope: * [Global](https://cloud.google.com/compute/docs/reference/rest/v1/globalForwardingRules) * [Regional](https://cloud.google.com/compute/docs/reference/rest/v1/forwardingRules) A forwarding rule and its corresponding IP address represent the frontend configuration of a Google Cloud Platform load balancer. Forwarding rules can also reference target instances and Cloud VPN Classic gateways (targetVpnGateway). For more information, read Forwarding rule concepts and Using protocol forwarding.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ForwardingRule {
    /// IP address for which this forwarding rule accepts traffic. When a client sends traffic to this IP address, the forwarding rule directs the traffic to the referenced target or backendService. While creating a forwarding rule, specifying an IPAddress is required under the following circumstances: - When the target is set to targetGrpcProxy and validateForProxyless is set to true, the IPAddress should be set to 0.0.0.0. - When the target is a Private Service Connect Google APIs bundle, you must specify an IPAddress. Otherwise, you can optionally specify an IP address that references an existing static (reserved) IP address resource. When omitted, Google Cloud assigns an ephemeral IP address. Use one of the following formats to specify an IP address while creating a forwarding rule: * IP address number, as in `100.1.2.3` * IPv6 address range, as in `2600:1234::/96` * Full resource URL, as in https://www.googleapis.com/compute/v1/projects/ project_id/regions/region/addresses/address-name * Partial URL or by name, as in: - projects/project_id/regions/region/addresses/address-name - regions/region/addresses/address-name - global/addresses/address-name - address-name The forwarding rule's target or backendService, and in most cases, also the loadBalancingScheme, determine the type of IP address that you can use. For detailed information, see [IP address specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#ip_address_specifications). When reading an IPAddress, the API always returns the IP address number.
    #[serde(rename = "IPAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// The IP protocol to which this rule applies. For protocol forwarding, valid options are TCP, UDP, ESP, AH, SCTP, ICMP and L3_DEFAULT. The valid IP protocols are different for different load balancing products as described in [Load balancing features](https://cloud.google.com/load-balancing/docs/features#protocols_from_the_load_balancer_to_the_backends).
    #[serde(rename = "IPProtocol", skip_serializing_if = "Option::is_none")]
    pub ip_protocol: Option<IpProtocol>,
    /// The ports, portRange, and allPorts fields are mutually exclusive. Only packets addressed to ports in the specified range will be forwarded to the backends configured with this forwarding rule. The allPorts field has the following limitations: - It requires that the forwarding rule IPProtocol be TCP, UDP, SCTP, or L3_DEFAULT. - It's applicable only to the following products: internal passthrough Network Load Balancers, backend service-based external passthrough Network Load Balancers, and internal and external protocol forwarding. - Set this field to true to allow packets addressed to any port or packets lacking destination port information (for example, UDP fragments after the first fragment) to be forwarded to the backends configured with this forwarding rule. The L3_DEFAULT protocol requires allPorts be set to true.
    #[serde(rename = "allPorts", skip_serializing_if = "Option::is_none")]
    pub all_ports: Option<bool>,
    /// This field is used along with the backend_service field for internal load balancing or with the target field for internal TargetInstance. If set to true, clients can access the Internal TCP/UDP Load Balancer, Internal HTTP(S) and TCP Proxy Load Balancer from all regions. If false, only allows access from the local region the load balancer is located at. Note that for INTERNAL_MANAGED forwarding rules, this field cannot be changed after the forwarding rule is created.
    #[serde(rename = "allowGlobalAccess", skip_serializing_if = "Option::is_none")]
    pub allow_global_access: Option<bool>,
    /// This is used in PSC consumer ForwardingRule to control whether the PSC endpoint can be accessed from another region.
    #[serde(
        rename = "allowPscGlobalAccess",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_psc_global_access: Option<bool>,
    /// Identifies the backend service to which the forwarding rule sends traffic. Required for Internal TCP/UDP Load Balancing and Network Load Balancing; must be omitted for all other load balancer types.
    #[serde(rename = "backendService", skip_serializing_if = "Option::is_none")]
    pub backend_service: Option<String>,
    /// [Output Only] The URL for the corresponding base Forwarding Rule. By base Forwarding Rule, we mean the Forwarding Rule that has the same IP address, protocol, and port settings with the current Forwarding Rule, but without sourceIPRanges specified. Always empty if the current Forwarding Rule does not have sourceIPRanges specified.
    #[serde(rename = "baseForwardingRule", skip_serializing_if = "Option::is_none")]
    pub base_forwarding_rule: Option<String>,
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// An optional description of this resource. Provide this property when you create the resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Fingerprint of this resource. A hash of the contents stored in this object. This field is used in optimistic locking. This field will be ignored when inserting a ForwardingRule. Include the fingerprint in patch request to ensure that you do not overwrite changes that were applied from another concurrent request. To see the latest fingerprint, make a get() request to retrieve a ForwardingRule.
    #[serde(rename = "fingerprint", skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// [Output Only] The unique identifier for the resource. This identifier is defined by the server.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The IP Version that will be used by this forwarding rule. Valid options are IPV4 or IPV6.
    #[serde(rename = "ipVersion", skip_serializing_if = "Option::is_none")]
    pub ip_version: Option<IpVersion>,
    /// Indicates whether or not this load balancer can be used as a collector for packet mirroring. To prevent mirroring loops, instances behind this load balancer will not have their traffic mirrored even if a PacketMirroring rule applies to them. This can only be set to true for load balancers that have their loadBalancingScheme set to INTERNAL.
    #[serde(
        rename = "isMirroringCollector",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_mirroring_collector: Option<bool>,
    /// [Output Only] Type of the resource. Always compute#forwardingRule for Forwarding Rule resources.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// A fingerprint for the labels being applied to this resource, which is essentially a hash of the labels set used for optimistic locking. The fingerprint is initially generated by Compute Engine and changes after every request to modify or update labels. You must always provide an up-to-date fingerprint hash in order to update or change labels, otherwise the request will fail with error 412 conditionNotMet. To see the latest fingerprint, make a get() request to retrieve a ForwardingRule.
    #[serde(rename = "labelFingerprint", skip_serializing_if = "Option::is_none")]
    pub label_fingerprint: Option<String>,
    /// Labels for this resource. These can only be added or modified by the setLabels method. Each label key/value pair must comply with RFC1035. Label values may be empty.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// Specifies the forwarding rule type. For more information about forwarding rules, refer to Forwarding rule concepts.
    #[serde(
        rename = "loadBalancingScheme",
        skip_serializing_if = "Option::is_none"
    )]
    pub load_balancing_scheme: Option<LoadBalancingScheme>,
    /// Opaque filter criteria used by load balancer to restrict routing configuration to a limited set of xDS compliant clients. In their xDS requests to load balancer, xDS clients present node metadata. When there is a match, the relevant configuration is made available to those proxies. Otherwise, all the resources (e.g. TargetHttpProxy, UrlMap) referenced by the ForwardingRule are not visible to those proxies. For each metadataFilter in this list, if its filterMatchCriteria is set to MATCH_ANY, at least one of the filterLabels must match the corresponding label provided in the metadata. If its filterMatchCriteria is set to MATCH_ALL, then all of its filterLabels must match with corresponding labels provided in the metadata. If multiple metadataFilters are specified, all of them need to be satisfied in order to be considered a match. metadataFilters specified here will be applifed before those specified in the UrlMap that this ForwardingRule references. metadataFilters only applies to Loadbalancers that have their loadBalancingScheme set to INTERNAL_SELF_MANAGED.
    #[serde(rename = "metadataFilters", skip_serializing_if = "Option::is_none")]
    pub metadata_filters: Option<Vec<crate::google_rest_apis::compute_v1::models::MetadataFilter>>,
    /// Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash. For Private Service Connect forwarding rules that forward traffic to Google APIs, the forwarding rule name must be a 1-20 characters string with lowercase letters and numbers and must start with a letter.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// This field is not used for global external load balancing. For Internal TCP/UDP Load Balancing, this field identifies the network that the load balanced IP should belong to for this Forwarding Rule. If the subnetwork is specified, the network of the subnetwork will be used. If neither subnetwork nor this field is specified, the default network will be used. For Private Service Connect forwarding rules that forward traffic to Google APIs, a network must be provided.
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// This signifies the networking tier used for configuring this load balancer and can only take the following values: PREMIUM, STANDARD. For regional ForwardingRule, the valid values are PREMIUM and STANDARD. For GlobalForwardingRule, the valid value is PREMIUM. If this field is not specified, it is assumed to be PREMIUM. If IPAddress is specified, this value must be equal to the networkTier of the Address.
    #[serde(rename = "networkTier", skip_serializing_if = "Option::is_none")]
    pub network_tier: Option<NetworkTier>,
    /// This is used in PSC consumer ForwardingRule to control whether it should try to auto-generate a DNS zone or not. Non-PSC forwarding rules do not use this field. Once set, this field is not mutable.
    #[serde(rename = "noAutomateDnsZone", skip_serializing_if = "Option::is_none")]
    pub no_automate_dns_zone: Option<bool>,
    /// The ports, portRange, and allPorts fields are mutually exclusive. Only packets addressed to ports in the specified range will be forwarded to the backends configured with this forwarding rule. The portRange field has the following limitations: - It requires that the forwarding rule IPProtocol be TCP, UDP, or SCTP, and - It's applicable only to the following products: external passthrough Network Load Balancers, internal and external proxy Network Load Balancers, internal and external Application Load Balancers, external protocol forwarding, and Classic VPN. - Some products have restrictions on what ports can be used. See port specifications for details. For external forwarding rules, two or more forwarding rules cannot use the same [IPAddress, IPProtocol] pair, and cannot have overlapping portRanges. For internal forwarding rules within the same VPC network, two or more forwarding rules cannot use the same [IPAddress, IPProtocol] pair, and cannot have overlapping portRanges. @pattern: \\\\d+(?:-\\\\d+)?
    #[serde(rename = "portRange", skip_serializing_if = "Option::is_none")]
    pub port_range: Option<String>,
    /// The ports, portRange, and allPorts fields are mutually exclusive. Only packets addressed to ports in the specified range will be forwarded to the backends configured with this forwarding rule. The ports field has the following limitations: - It requires that the forwarding rule IPProtocol be TCP, UDP, or SCTP, and - It's applicable only to the following products: internal passthrough Network Load Balancers, backend service-based external passthrough Network Load Balancers, and internal protocol forwarding. - You can specify a list of up to five ports by number, separated by commas. The ports can be contiguous or discontiguous. For external forwarding rules, two or more forwarding rules cannot use the same [IPAddress, IPProtocol] pair if they share at least one port number. For internal forwarding rules within the same VPC network, two or more forwarding rules cannot use the same [IPAddress, IPProtocol] pair if they share at least one port number. @pattern: \\\\d+(?:-\\\\d+)?
    #[serde(rename = "ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<String>>,
    /// [Output Only] The PSC connection id of the PSC Forwarding Rule.
    #[serde(rename = "pscConnectionId", skip_serializing_if = "Option::is_none")]
    pub psc_connection_id: Option<String>,
    #[serde(
        rename = "pscConnectionStatus",
        skip_serializing_if = "Option::is_none"
    )]
    pub psc_connection_status: Option<PscConnectionStatus>,
    /// [Output Only] URL of the region where the regional forwarding rule resides. This field is not applicable to global forwarding rules. You must specify this field as part of the HTTP request URL. It is not settable as a field in the request body.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// Service Directory resources to register this forwarding rule with. Currently, only supports a single Service Directory resource.
    #[serde(
        rename = "serviceDirectoryRegistrations",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_directory_registrations: Option<
        Vec<
            crate::google_rest_apis::compute_v1::models::ForwardingRuleServiceDirectoryRegistration,
        >,
    >,
    /// An optional prefix to the service name for this Forwarding Rule. If specified, the prefix is the first label of the fully qualified service name. The label must be 1-63 characters long, and comply with RFC1035. Specifically, the label must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash. This field is only used for internal load balancing.
    #[serde(rename = "serviceLabel", skip_serializing_if = "Option::is_none")]
    pub service_label: Option<String>,
    /// [Output Only] The internal fully qualified service name for this Forwarding Rule. This field is only used for internal load balancing.
    #[serde(rename = "serviceName", skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// If not empty, this Forwarding Rule will only forward the traffic when the source IP address matches one of the IP addresses or CIDR ranges set here. Note that a Forwarding Rule can only have up to 64 source IP ranges, and this field can only be used with a regional Forwarding Rule whose scheme is EXTERNAL. Each source_ip_range entry should be either an IP address (for example, 1.2.3.4) or a CIDR range (for example, 1.2.3.0/24).
    #[serde(rename = "sourceIpRanges", skip_serializing_if = "Option::is_none")]
    pub source_ip_ranges: Option<Vec<String>>,
    /// This field identifies the subnetwork that the load balanced IP should belong to for this Forwarding Rule, used in internal load balancing and network load balancing with IPv6. If the network specified is in auto subnet mode, this field is optional. However, a subnetwork must be specified if the network is in custom subnet mode or when creating external forwarding rule with IPv6.
    #[serde(rename = "subnetwork", skip_serializing_if = "Option::is_none")]
    pub subnetwork: Option<String>,
    /// The URL of the target resource to receive the matched traffic. For regional forwarding rules, this target must be in the same region as the forwarding rule. For global forwarding rules, this target must be a global load balancing resource. The forwarded traffic must be of a type appropriate to the target object. - For load balancers, see the \"Target\" column in [Port specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#ip_address_specifications). - For Private Service Connect forwarding rules that forward traffic to Google APIs, provide the name of a supported Google API bundle: - vpc-sc - APIs that support VPC Service Controls. - all-apis - All supported Google APIs. - For Private Service Connect forwarding rules that forward traffic to managed services, the target must be a service attachment. The target is not mutable once set as a service attachment.
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

impl ForwardingRule {
    /// Represents a Forwarding Rule resource. Forwarding rule resources in Google Cloud can be either regional or global in scope: * [Global](https://cloud.google.com/compute/docs/reference/rest/v1/globalForwardingRules) * [Regional](https://cloud.google.com/compute/docs/reference/rest/v1/forwardingRules) A forwarding rule and its corresponding IP address represent the frontend configuration of a Google Cloud Platform load balancer. Forwarding rules can also reference target instances and Cloud VPN Classic gateways (targetVpnGateway). For more information, read Forwarding rule concepts and Using protocol forwarding.
    pub fn new() -> ForwardingRule {
        ForwardingRule {
            ip_address: None,
            ip_protocol: None,
            all_ports: None,
            allow_global_access: None,
            allow_psc_global_access: None,
            backend_service: None,
            base_forwarding_rule: None,
            creation_timestamp: None,
            description: None,
            fingerprint: None,
            id: None,
            ip_version: None,
            is_mirroring_collector: None,
            kind: None,
            label_fingerprint: None,
            labels: None,
            load_balancing_scheme: None,
            metadata_filters: None,
            name: None,
            network: None,
            network_tier: None,
            no_automate_dns_zone: None,
            port_range: None,
            ports: None,
            psc_connection_id: None,
            psc_connection_status: None,
            region: None,
            self_link: None,
            service_directory_registrations: None,
            service_label: None,
            service_name: None,
            source_ip_ranges: None,
            subnetwork: None,
            target: None,
        }
    }
}

/// The IP protocol to which this rule applies. For protocol forwarding, valid options are TCP, UDP, ESP, AH, SCTP, ICMP and L3_DEFAULT. The valid IP protocols are different for different load balancing products as described in [Load balancing features](https://cloud.google.com/load-balancing/docs/features#protocols_from_the_load_balancer_to_the_backends).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IpProtocol {
    #[serde(rename = "AH")]
    Ah,
    #[serde(rename = "ESP")]
    Esp,
    #[serde(rename = "ICMP")]
    Icmp,
    #[serde(rename = "L3_DEFAULT")]
    L3Default,
    #[serde(rename = "SCTP")]
    Sctp,
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
}

impl Default for IpProtocol {
    fn default() -> IpProtocol {
        Self::Ah
    }
}
/// The IP Version that will be used by this forwarding rule. Valid options are IPV4 or IPV6.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IpVersion {
    #[serde(rename = "IPV4")]
    Ipv4,
    #[serde(rename = "IPV6")]
    Ipv6,
    #[serde(rename = "UNSPECIFIED_VERSION")]
    UnspecifiedVersion,
}

impl Default for IpVersion {
    fn default() -> IpVersion {
        Self::Ipv4
    }
}
/// Specifies the forwarding rule type. For more information about forwarding rules, refer to Forwarding rule concepts.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LoadBalancingScheme {
    #[serde(rename = "EXTERNAL")]
    External,
    #[serde(rename = "EXTERNAL_MANAGED")]
    ExternalManaged,
    #[serde(rename = "INTERNAL")]
    Internal,
    #[serde(rename = "INTERNAL_MANAGED")]
    InternalManaged,
    #[serde(rename = "INTERNAL_SELF_MANAGED")]
    InternalSelfManaged,
    #[serde(rename = "INVALID")]
    Invalid,
}

impl Default for LoadBalancingScheme {
    fn default() -> LoadBalancingScheme {
        Self::External
    }
}
/// This signifies the networking tier used for configuring this load balancer and can only take the following values: PREMIUM, STANDARD. For regional ForwardingRule, the valid values are PREMIUM and STANDARD. For GlobalForwardingRule, the valid value is PREMIUM. If this field is not specified, it is assumed to be PREMIUM. If IPAddress is specified, this value must be equal to the networkTier of the Address.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NetworkTier {
    #[serde(rename = "FIXED_STANDARD")]
    FixedStandard,
    #[serde(rename = "PREMIUM")]
    Premium,
    #[serde(rename = "STANDARD")]
    Standard,
    #[serde(rename = "STANDARD_OVERRIDES_FIXED_STANDARD")]
    StandardOverridesFixedStandard,
}

impl Default for NetworkTier {
    fn default() -> NetworkTier {
        Self::FixedStandard
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PscConnectionStatus {
    #[serde(rename = "ACCEPTED")]
    Accepted,
    #[serde(rename = "CLOSED")]
    Closed,
    #[serde(rename = "NEEDS_ATTENTION")]
    NeedsAttention,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "STATUS_UNSPECIFIED")]
    StatusUnspecified,
}

impl Default for PscConnectionStatus {
    fn default() -> PscConnectionStatus {
        Self::Accepted
    }
}
