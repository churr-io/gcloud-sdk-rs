/// A Google Edge Cloud zone.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Zone {
    /// Required. The resource name of the zone.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time when the zone was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the zone was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels as key value pairs
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The deployment layout type.
    #[prost(string, tag = "5")]
    pub layout_name: ::prost::alloc::string::String,
}
/// Message describing Network object
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Network {
    /// Required. The canonical resource name of the network.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time when the network was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the network was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels associated with this resource.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. A free-text description of the resource. Max length 1024
    /// characters.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// IP (L3) MTU value of the network.
    /// Valid values are: 1500 and 9000.
    /// Default to 1500 if not set.
    #[prost(int32, tag = "6")]
    pub mtu: i32,
}
/// Message describing Subnet object
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subnet {
    /// Required. The canonical resource name of the subnet.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time when the subnet was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the subnet was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels associated with this resource.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. A free-text description of the resource. Max length 1024
    /// characters.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Required. The network that this subnetwork belongs to.
    #[prost(string, tag = "6")]
    pub network: ::prost::alloc::string::String,
    /// The ranges of ipv4 addresses that are owned by this subnetwork.
    #[prost(string, repeated, tag = "7")]
    pub ipv4_cidr: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The ranges of ipv6 addresses that are owned by this subnetwork.
    #[prost(string, repeated, tag = "8")]
    pub ipv6_cidr: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. VLAN id provided by user. If not specified we assign one
    /// automatically.
    #[prost(int32, tag = "9")]
    pub vlan_id: i32,
    /// Output only. Current stage of the resource to the device by config push.
    #[prost(enumeration = "ResourceState", tag = "10")]
    pub state: i32,
}
/// Message describing Interconnect object
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Interconnect {
    /// Required. The canonical resource name of the interconnect.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time when the subnet was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the subnet was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels associated with this resource.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. A free-text description of the resource. Max length 1024
    /// characters.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Optional. Type of interconnect, which takes only the value 'DEDICATED' for
    /// now.
    #[prost(enumeration = "interconnect::InterconnectType", tag = "6")]
    pub interconnect_type: i32,
    /// Output only. Unique identifier for the link.
    #[prost(string, tag = "7")]
    pub uuid: ::prost::alloc::string::String,
    /// Output only. Cloud resource name of the switch device.
    #[prost(string, tag = "8")]
    pub device_cloud_resource_name: ::prost::alloc::string::String,
    /// Output only. Physical ports (e.g., TenGigE0/0/0/1) that form the
    /// interconnect.
    #[prost(string, repeated, tag = "9")]
    pub physical_ports: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Interconnect`.
pub mod interconnect {
    /// Type of interconnect.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum InterconnectType {
        /// Unspecified.
        Unspecified = 0,
        /// Dedicated Interconnect.
        Dedicated = 1,
    }
    impl InterconnectType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InterconnectType::Unspecified => "INTERCONNECT_TYPE_UNSPECIFIED",
                InterconnectType::Dedicated => "DEDICATED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INTERCONNECT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "DEDICATED" => Some(Self::Dedicated),
                _ => None,
            }
        }
    }
}
/// Message describing InterconnectAttachment object
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterconnectAttachment {
    /// Required. The canonical resource name of the interconnect attachment.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time when the interconnect attachment was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the interconnect attachment was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels associated with this resource.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. A free-text description of the resource. Max length 1024
    /// characters.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Required. The canonical name of underlying Interconnect object that this
    /// attachment's traffic will traverse through. The name is in the form of
    /// `projects/{project}/locations/{location}/zones/{zone}/interconnects/{interconnect}`.
    #[prost(string, tag = "6")]
    pub interconnect: ::prost::alloc::string::String,
    /// Optional. The canonical Network name in the form of
    /// `projects/{project}/locations/{location}/zones/{zone}/networks/{network}`.
    #[prost(string, tag = "11")]
    pub network: ::prost::alloc::string::String,
    /// Required. VLAN id provided by user. Must be site-wise unique.
    #[prost(int32, tag = "8")]
    pub vlan_id: i32,
    /// IP (L3) MTU value of the virtual edge cloud.
    /// Valid values are: 1500 and 9000.
    /// Default to 1500 if not set.
    #[prost(int32, tag = "9")]
    pub mtu: i32,
    /// Output only. Current stage of the resource to the device by config push.
    #[prost(enumeration = "ResourceState", tag = "10")]
    pub state: i32,
}
/// Message describing Router object
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Router {
    /// Required. The canonical resource name of the router.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time when the router was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the router was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels associated with this resource.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. A free-text description of the resource. Max length 1024
    /// characters.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Required. The canonical name of the network to which this router belongs.
    /// The name is in the form of
    /// `projects/{project}/locations/{location}/zones/{zone}/networks/{network}`.
    #[prost(string, tag = "6")]
    pub network: ::prost::alloc::string::String,
    /// Router interfaces.
    #[prost(message, repeated, tag = "7")]
    pub interface: ::prost::alloc::vec::Vec<router::Interface>,
    /// BGP peers.
    #[prost(message, repeated, tag = "8")]
    pub bgp_peer: ::prost::alloc::vec::Vec<router::BgpPeer>,
    /// BGP information specific to this router.
    #[prost(message, optional, tag = "9")]
    pub bgp: ::core::option::Option<router::Bgp>,
    /// Output only. Current stage of the resource to the device by config push.
    #[prost(enumeration = "ResourceState", tag = "10")]
    pub state: i32,
    /// Optional. A list of CIDRs in IP/Length format to advertise northbound as
    /// static routes from this router.
    #[prost(string, repeated, tag = "11")]
    pub route_advertisements: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Router`.
pub mod router {
    /// Router Interface defines the GDCE zone side layer-3 information for
    /// building the BGP session.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Interface {
        /// Name of this interface entry. Unique within the Zones resource.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// IP address and range of the interface.
        #[prost(string, tag = "3")]
        pub ipv4_cidr: ::prost::alloc::string::String,
        /// IPv6 address and range of the interface.
        #[prost(string, tag = "6")]
        pub ipv6_cidr: ::prost::alloc::string::String,
        /// The canonical name of the linked Interconnect attachment.
        #[prost(string, tag = "2")]
        pub linked_interconnect_attachment: ::prost::alloc::string::String,
        /// The canonical name of the subnetwork resource that this interface
        /// belongs to.
        #[prost(string, tag = "4")]
        pub subnetwork: ::prost::alloc::string::String,
        /// Create loopback interface in the router when specified.
        /// The number of IP addresses must match the number of TOR devices.
        #[prost(string, repeated, tag = "5")]
        pub loopback_ip_addresses: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
    }
    /// BGPPeer defines the peer side layer-3 information for building the BGP
    /// session.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BgpPeer {
        /// Name of this BGP peer. Unique within the Zones resource.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Name of the RouterInterface the BGP peer is associated with.
        #[prost(string, tag = "2")]
        pub interface: ::prost::alloc::string::String,
        /// IP range of the interface within Google.
        #[prost(string, tag = "3")]
        pub interface_ipv4_cidr: ::prost::alloc::string::String,
        /// IPv6 range of the interface within Google.
        #[prost(string, tag = "7")]
        pub interface_ipv6_cidr: ::prost::alloc::string::String,
        /// IP range of the BGP interface outside Google.
        #[prost(string, tag = "4")]
        pub peer_ipv4_cidr: ::prost::alloc::string::String,
        /// IPv6 range of the BGP interface outside Google.
        #[prost(string, tag = "6")]
        pub peer_ipv6_cidr: ::prost::alloc::string::String,
        /// Peer BGP Autonomous System Number (ASN). Each BGP interface may use
        /// a different value.
        #[prost(uint32, tag = "5")]
        pub peer_asn: u32,
        /// Output only. Local BGP Autonomous System Number (ASN).
        /// This field is ST_NOT_REQUIRED because it stores private ASNs, which are
        /// meaningless outside the zone in which they are being used.
        #[prost(uint32, tag = "8")]
        pub local_asn: u32,
    }
    /// BGP information specific to this router.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Bgp {
        /// Locally assigned BGP ASN.
        #[prost(uint32, tag = "1")]
        pub asn: u32,
        /// The interval in seconds between BGP keepalive messages that are
        /// sent to the peer. Default is 20 with value between 20 and 60.
        #[prost(uint32, tag = "2")]
        pub keepalive_interval_in_seconds: u32,
    }
}
/// LinkLayerAddress contains an IP address and corresponding link-layer address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkLayerAddress {
    /// The MAC address of this neighbor.
    #[prost(string, tag = "1")]
    pub mac_address: ::prost::alloc::string::String,
    /// The IP address of this neighbor.
    #[prost(string, tag = "2")]
    pub ip_address: ::prost::alloc::string::String,
}
/// SubnetStatus contains detailed and current technical information about this
/// subnet resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubnetStatus {
    /// The name of CCFE subnet resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// BVI MAC address.
    #[prost(string, tag = "2")]
    pub mac_address: ::prost::alloc::string::String,
    /// A list of LinkLayerAddress, describing the ip address and corresponding
    /// link-layer address of the neighbors for this subnet.
    #[prost(message, repeated, tag = "3")]
    pub link_layer_addresses: ::prost::alloc::vec::Vec<LinkLayerAddress>,
}
/// Diagnostics information about interconnect, contains detailed and current
/// technical information about Google's side of the connection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterconnectDiagnostics {
    /// The MAC address of the Interconnect's bundle interface.
    #[prost(string, tag = "1")]
    pub mac_address: ::prost::alloc::string::String,
    /// A list of LinkLayerAddress, describing the ip address and corresponding
    /// link-layer address of the neighbors for this interconnect.
    #[prost(message, repeated, tag = "2")]
    pub link_layer_addresses: ::prost::alloc::vec::Vec<LinkLayerAddress>,
    /// A list of LinkStatus objects, used to describe the status for each link on
    /// the Interconnect.
    #[prost(message, repeated, tag = "3")]
    pub links: ::prost::alloc::vec::Vec<interconnect_diagnostics::LinkStatus>,
}
/// Nested message and enum types in `InterconnectDiagnostics`.
pub mod interconnect_diagnostics {
    /// Describing the status for each link on the Interconnect.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LinkStatus {
        /// The unique ID for this link assigned during turn up by Google.
        #[prost(string, tag = "1")]
        pub circuit_id: ::prost::alloc::string::String,
        /// Describing the state of a LACP link.
        #[prost(message, optional, tag = "2")]
        pub lacp_status: ::core::option::Option<LinkLacpStatus>,
        /// A list of LinkLLDPStatus objects, used to describe LLDP status of each
        /// peer for each link on the Interconnect.
        #[prost(message, repeated, tag = "3")]
        pub lldp_statuses: ::prost::alloc::vec::Vec<LinkLldpStatus>,
        /// Packet counts specific statistics for this link.
        #[prost(message, optional, tag = "4")]
        pub packet_counts: ::core::option::Option<PacketCounts>,
    }
    /// Containing a collection of interface-related statistics objects.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PacketCounts {
        /// The number of packets that are delivered.
        #[prost(int64, tag = "1")]
        pub inbound_unicast: i64,
        /// The number of inbound packets that contained errors.
        #[prost(int64, tag = "2")]
        pub inbound_errors: i64,
        /// The number of inbound packets that were chosen to be discarded even
        /// though no errors had been detected to prevent their being deliverable.
        #[prost(int64, tag = "3")]
        pub inbound_discards: i64,
        /// The total number of packets that are requested be transmitted.
        #[prost(int64, tag = "4")]
        pub outbound_unicast: i64,
        /// The number of outbound packets that could not be transmitted because of
        /// errors.
        #[prost(int64, tag = "5")]
        pub outbound_errors: i64,
        /// The number of outbound packets that were chosen to be discarded even
        /// though no errors had been detected to prevent their being transmitted.
        #[prost(int64, tag = "6")]
        pub outbound_discards: i64,
    }
    /// Describing the status of a LACP link.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LinkLacpStatus {
        /// The state of a LACP link.
        #[prost(enumeration = "link_lacp_status::State", tag = "1")]
        pub state: i32,
        /// System ID of the port on Google's side of the LACP exchange.
        #[prost(string, tag = "2")]
        pub google_system_id: ::prost::alloc::string::String,
        /// System ID of the port on the neighbor's side of the LACP exchange.
        #[prost(string, tag = "3")]
        pub neighbor_system_id: ::prost::alloc::string::String,
        /// A true value indicates that the participant will allow the link to be
        /// used as part of the aggregate.
        /// A false value indicates the link should be used as an individual link.
        #[prost(bool, tag = "4")]
        pub aggregatable: bool,
        /// If true, the participant is collecting incoming frames on the link,
        /// otherwise false
        #[prost(bool, tag = "5")]
        pub collecting: bool,
        /// When true, the participant is distributing outgoing frames; when false,
        /// distribution is disabled
        #[prost(bool, tag = "6")]
        pub distributing: bool,
    }
    /// Nested message and enum types in `LinkLACPStatus`.
    pub mod link_lacp_status {
        /// State enum for LACP link.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum State {
            /// The default state indicating state is in unknown state.
            Unknown = 0,
            /// The link is configured and active within the bundle.
            Active = 1,
            /// The link is not configured within the bundle, this means the rest of
            /// the object should be empty.
            Detached = 2,
        }
        impl State {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    State::Unknown => "UNKNOWN",
                    State::Active => "ACTIVE",
                    State::Detached => "DETACHED",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "UNKNOWN" => Some(Self::Unknown),
                    "ACTIVE" => Some(Self::Active),
                    "DETACHED" => Some(Self::Detached),
                    _ => None,
                }
            }
        }
    }
    /// Describing a LLDP link.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LinkLldpStatus {
        /// The peer system's administratively assigned name.
        #[prost(string, tag = "1")]
        pub peer_system_name: ::prost::alloc::string::String,
        /// The textual description of the network entity of LLDP peer.
        #[prost(string, tag = "2")]
        pub peer_system_description: ::prost::alloc::string::String,
        /// The peer chassis component of the endpoint identifier associated with the
        /// transmitting LLDP agent.
        #[prost(string, tag = "3")]
        pub peer_chassis_id: ::prost::alloc::string::String,
        /// The format and source of the peer chassis identifier string.
        #[prost(string, tag = "4")]
        pub peer_chassis_id_type: ::prost::alloc::string::String,
        /// The port component of the endpoint identifier associated with the
        /// transmitting LLDP agent. If the specified port is an IEEE 802.3 Repeater
        /// port, then this TLV is optional.
        #[prost(string, tag = "5")]
        pub peer_port_id: ::prost::alloc::string::String,
        /// The format and source of the peer port identifier string.
        #[prost(string, tag = "6")]
        pub peer_port_id_type: ::prost::alloc::string::String,
    }
}
/// Describing the current status of a router.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouterStatus {
    /// The canonical name of the network to which this router belongs.
    #[prost(string, tag = "1")]
    pub network: ::prost::alloc::string::String,
    /// A list of BgpPeerStatus objects, describing all BGP peers related to this
    /// router.
    #[prost(message, repeated, tag = "2")]
    pub bgp_peer_status: ::prost::alloc::vec::Vec<router_status::BgpPeerStatus>,
}
/// Nested message and enum types in `RouterStatus`.
pub mod router_status {
    /// Status of a BGP peer.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BgpPeerStatus {
        /// Name of this BGP peer. Unique within the Routers resource.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// IP address of the local BGP interface.
        #[prost(string, tag = "2")]
        pub ip_address: ::prost::alloc::string::String,
        /// IP address of the remote BGP interface.
        #[prost(string, tag = "3")]
        pub peer_ip_address: ::prost::alloc::string::String,
        /// The current status of BGP.
        #[prost(enumeration = "bgp_peer_status::BgpStatus", tag = "4")]
        pub status: i32,
        /// BGP state as specified in RFC1771.
        #[prost(string, tag = "5")]
        pub state: ::prost::alloc::string::String,
        /// Time this session has been up.
        /// Format:
        ///   14 years, 51 weeks, 6 days, 23 hours, 59 minutes, 59 seconds
        #[prost(string, tag = "6")]
        pub uptime: ::prost::alloc::string::String,
        /// Time this session has been up, in seconds.
        #[prost(int64, tag = "7")]
        pub uptime_seconds: i64,
        /// A collection of counts for prefixes.
        #[prost(message, optional, tag = "8")]
        pub prefix_counter: ::core::option::Option<PrefixCounter>,
    }
    /// Nested message and enum types in `BgpPeerStatus`.
    pub mod bgp_peer_status {
        /// Status of the BGP peer: {UP, DOWN}
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum BgpStatus {
            /// The default status indicating BGP session is in unknown state.
            Unknown = 0,
            /// The UP status indicating BGP session is established.
            Up = 1,
            /// The DOWN state indicating BGP session is not established yet.
            Down = 2,
        }
        impl BgpStatus {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    BgpStatus::Unknown => "UNKNOWN",
                    BgpStatus::Up => "UP",
                    BgpStatus::Down => "DOWN",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "UNKNOWN" => Some(Self::Unknown),
                    "UP" => Some(Self::Up),
                    "DOWN" => Some(Self::Down),
                    _ => None,
                }
            }
        }
    }
    /// PrefixCounter contains a collection of prefixes related counts.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PrefixCounter {
        /// Number of prefixes advertised.
        #[prost(int64, tag = "1")]
        pub advertised: i64,
        /// Number of prefixes denied.
        #[prost(int64, tag = "2")]
        pub denied: i64,
        /// Number of prefixes received.
        #[prost(int64, tag = "3")]
        pub received: i64,
        /// Number of prefixes sent.
        #[prost(int64, tag = "4")]
        pub sent: i64,
        /// Number of prefixes suppressed.
        #[prost(int64, tag = "5")]
        pub suppressed: i64,
        /// Number of prefixes withdrawn.
        #[prost(int64, tag = "6")]
        pub withdrawn: i64,
    }
}
/// ResourceState describes the state the resource.
/// A normal lifecycle of a new resource being created would be: PENDING ->
/// PROVISIONING -> RUNNING. A normal lifecycle of an existing resource being
/// deleted would be: RUNNING -> DELETING. Any failures during processing will
/// result the resource to be in a SUSPENDED state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResourceState {
    /// Unspecified state.
    StateUnknown = 0,
    /// The resource is being prepared to be applied to the rack.
    StatePending = 1,
    /// The resource has started being applied to the rack.
    StateProvisioning = 2,
    /// The resource has been pushed to the rack.
    StateRunning = 3,
    /// The resource failed to push to the rack.
    StateSuspended = 4,
    /// The resource is under deletion.
    StateDeleting = 5,
}
impl ResourceState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResourceState::StateUnknown => "STATE_UNKNOWN",
            ResourceState::StatePending => "STATE_PENDING",
            ResourceState::StateProvisioning => "STATE_PROVISIONING",
            ResourceState::StateRunning => "STATE_RUNNING",
            ResourceState::StateSuspended => "STATE_SUSPENDED",
            ResourceState::StateDeleting => "STATE_DELETING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATE_UNKNOWN" => Some(Self::StateUnknown),
            "STATE_PENDING" => Some(Self::StatePending),
            "STATE_PROVISIONING" => Some(Self::StateProvisioning),
            "STATE_RUNNING" => Some(Self::StateRunning),
            "STATE_SUSPENDED" => Some(Self::StateSuspended),
            "STATE_DELETING" => Some(Self::StateDeleting),
            _ => None,
        }
    }
}
/// Message for requesting list of Zones
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListZonesRequest {
    /// Required. Parent value for ListZonesRequest
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing Zones
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListZonesResponse {
    /// The list of Zone
    #[prost(message, repeated, tag = "1")]
    pub zones: ::prost::alloc::vec::Vec<Zone>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a Zone
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetZoneRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for requesting list of Networks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNetworksRequest {
    /// Required. Parent value for ListNetworksRequest
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing Networks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNetworksResponse {
    /// The list of Network
    #[prost(message, repeated, tag = "1")]
    pub networks: ::prost::alloc::vec::Vec<Network>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a Network
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNetworkRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a Network
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNetworkRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Id of the requesting object
    /// If auto-generating Id server-side, remove this field and
    /// network_id from the method_signature of Create RPC
    #[prost(string, tag = "2")]
    pub network_id: ::prost::alloc::string::String,
    /// Required. The resource being created
    #[prost(message, optional, tag = "3")]
    pub network: ::core::option::Option<Network>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for deleting a Network
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNetworkRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for requesting list of Subnets
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSubnetsRequest {
    /// Required. Parent value for ListSubnetsRequest
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing Subnets
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSubnetsResponse {
    /// The list of Subnet
    #[prost(message, repeated, tag = "1")]
    pub subnets: ::prost::alloc::vec::Vec<Subnet>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a Subnet
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSubnetRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a Subnet
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSubnetRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Id of the requesting object
    /// If auto-generating Id server-side, remove this field and
    /// subnet_id from the method_signature of Create RPC
    #[prost(string, tag = "2")]
    pub subnet_id: ::prost::alloc::string::String,
    /// Required. The resource being created
    #[prost(message, optional, tag = "3")]
    pub subnet: ::core::option::Option<Subnet>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for updating a Subnet
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSubnetRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// Subnet resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated
    #[prost(message, optional, tag = "2")]
    pub subnet: ::core::option::Option<Subnet>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for deleting a Subnet
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSubnetRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for requesting list of Interconnects
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInterconnectsRequest {
    /// Required. Parent value for ListInterconnectsRequest
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing Interconnects
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInterconnectsResponse {
    /// The list of Interconnect
    #[prost(message, repeated, tag = "1")]
    pub interconnects: ::prost::alloc::vec::Vec<Interconnect>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a Interconnect
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInterconnectRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for requesting list of InterconnectAttachments
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInterconnectAttachmentsRequest {
    /// Required. Parent value for ListInterconnectAttachmentsRequest
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing InterconnectAttachments
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInterconnectAttachmentsResponse {
    /// The list of InterconnectAttachment
    #[prost(message, repeated, tag = "1")]
    pub interconnect_attachments: ::prost::alloc::vec::Vec<InterconnectAttachment>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a InterconnectAttachment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInterconnectAttachmentRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a InterconnectAttachment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInterconnectAttachmentRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Id of the requesting object
    /// If auto-generating Id server-side, remove this field and
    /// interconnect_attachment_id from the method_signature of Create RPC
    #[prost(string, tag = "2")]
    pub interconnect_attachment_id: ::prost::alloc::string::String,
    /// Required. The resource being created
    #[prost(message, optional, tag = "3")]
    pub interconnect_attachment: ::core::option::Option<InterconnectAttachment>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for deleting a InterconnectAttachment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInterconnectAttachmentRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for requesting list of Routers
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRoutersRequest {
    /// Required. Parent value for ListRoutersRequest
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing Routers
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRoutersResponse {
    /// The list of Router
    #[prost(message, repeated, tag = "1")]
    pub routers: ::prost::alloc::vec::Vec<Router>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a Router
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRouterRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a Router
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRouterRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Id of the requesting object
    /// If auto-generating Id server-side, remove this field and
    /// router_id from the method_signature of Create RPC
    #[prost(string, tag = "2")]
    pub router_id: ::prost::alloc::string::String,
    /// Required. The resource being created
    #[prost(message, optional, tag = "3")]
    pub router: ::core::option::Option<Router>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for updating a Router
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRouterRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// Router resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated
    #[prost(message, optional, tag = "2")]
    pub router: ::core::option::Option<Router>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for deleting a Router
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRouterRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Represents the metadata of the long-running operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have been cancelled successfully
    /// have [Operation.error][] value with a
    /// [google.rpc.Status.code][google.rpc.Status.code] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Message for requesting the diagnostics of a network within a specific zone.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiagnoseNetworkRequest {
    /// Required. The name of the network resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// DiagnoseNetworkResponse contains the current status for a specific network.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiagnoseNetworkResponse {
    /// The time when the network status was last updated.
    #[prost(message, optional, tag = "1")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The network status of a specific network.
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<diagnose_network_response::NetworkStatus>,
}
/// Nested message and enum types in `DiagnoseNetworkResponse`.
pub mod diagnose_network_response {
    /// NetworkStatus has a list of status for the subnets under the current
    /// network.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NetworkStatus {
        /// A list of status for the subnets under the current network.
        #[prost(message, repeated, tag = "1")]
        pub subnet_status: ::prost::alloc::vec::Vec<super::SubnetStatus>,
    }
}
/// Message for requesting the diagnostics of an interconnect within a specific
/// zone.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiagnoseInterconnectRequest {
    /// Required. The name of the interconnect resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// DiagnoseInterconnectResponse contains the current diagnostics for a
/// specific interconnect.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiagnoseInterconnectResponse {
    /// The time when the interconnect diagnostics was last updated.
    #[prost(message, optional, tag = "1")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The network status of a specific interconnect.
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<InterconnectDiagnostics>,
}
/// Message for requesting diagnositcs of a router within a specific zone.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiagnoseRouterRequest {
    /// Required. The name of the router resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// DiagnoseRouterResponse contains the current status for a specific router.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiagnoseRouterResponse {
    /// The time when the router status was last updated.
    #[prost(message, optional, tag = "1")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The network status of a specific router.
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<RouterStatus>,
}
/// Message for initializing a specified zone
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeZoneRequest {
    /// Required. The name of the zone resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The response of initializing a zone
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeZoneResponse {}
/// Generated client implementations.
pub mod edge_network_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// EdgeNetwork API provides managed, highly available cloud dynamic network
    /// configuration service to the GEC customer to enable edge application and
    /// network function solutions. This allows the customers to easily define and
    /// configure the network setup and property to meet the workload requirement.
    #[derive(Debug, Clone)]
    pub struct EdgeNetworkClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EdgeNetworkClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> EdgeNetworkClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> EdgeNetworkClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            EdgeNetworkClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// InitializeZone will initialize resources for a zone in a project.
        pub async fn initialize_zone(
            &mut self,
            request: impl tonic::IntoRequest<super::InitializeZoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InitializeZoneResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/InitializeZone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "InitializeZone",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Zones in a given project and location.
        pub async fn list_zones(
            &mut self,
            request: impl tonic::IntoRequest<super::ListZonesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListZonesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/ListZones",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "ListZones",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single Zone.
        pub async fn get_zone(
            &mut self,
            request: impl tonic::IntoRequest<super::GetZoneRequest>,
        ) -> std::result::Result<tonic::Response<super::Zone>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/GetZone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.edgenetwork.v1.EdgeNetwork", "GetZone"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Networks in a given project and location.
        pub async fn list_networks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNetworksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNetworksResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/ListNetworks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "ListNetworks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single Network.
        pub async fn get_network(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNetworkRequest>,
        ) -> std::result::Result<tonic::Response<super::Network>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/GetNetwork",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "GetNetwork",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get the diagnostics of a single network resource.
        pub async fn diagnose_network(
            &mut self,
            request: impl tonic::IntoRequest<super::DiagnoseNetworkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DiagnoseNetworkResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/DiagnoseNetwork",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "DiagnoseNetwork",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new Network in a given project and location.
        pub async fn create_network(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNetworkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/CreateNetwork",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "CreateNetwork",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single Network.
        pub async fn delete_network(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNetworkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/DeleteNetwork",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "DeleteNetwork",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Subnets in a given project and location.
        pub async fn list_subnets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSubnetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSubnetsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/ListSubnets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "ListSubnets",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single Subnet.
        pub async fn get_subnet(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSubnetRequest>,
        ) -> std::result::Result<tonic::Response<super::Subnet>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/GetSubnet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "GetSubnet",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new Subnet in a given project and location.
        pub async fn create_subnet(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSubnetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/CreateSubnet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "CreateSubnet",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single Subnet.
        pub async fn update_subnet(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSubnetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/UpdateSubnet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "UpdateSubnet",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single Subnet.
        pub async fn delete_subnet(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSubnetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/DeleteSubnet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "DeleteSubnet",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Interconnects in a given project and location.
        pub async fn list_interconnects(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInterconnectsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListInterconnectsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/ListInterconnects",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "ListInterconnects",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single Interconnect.
        pub async fn get_interconnect(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInterconnectRequest>,
        ) -> std::result::Result<tonic::Response<super::Interconnect>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/GetInterconnect",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "GetInterconnect",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get the diagnostics of a single interconnect resource.
        pub async fn diagnose_interconnect(
            &mut self,
            request: impl tonic::IntoRequest<super::DiagnoseInterconnectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DiagnoseInterconnectResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/DiagnoseInterconnect",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "DiagnoseInterconnect",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists InterconnectAttachments in a given project and location.
        pub async fn list_interconnect_attachments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInterconnectAttachmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListInterconnectAttachmentsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/ListInterconnectAttachments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "ListInterconnectAttachments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single InterconnectAttachment.
        pub async fn get_interconnect_attachment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInterconnectAttachmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InterconnectAttachment>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/GetInterconnectAttachment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "GetInterconnectAttachment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new InterconnectAttachment in a given project and location.
        pub async fn create_interconnect_attachment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInterconnectAttachmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/CreateInterconnectAttachment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "CreateInterconnectAttachment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single InterconnectAttachment.
        pub async fn delete_interconnect_attachment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInterconnectAttachmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/DeleteInterconnectAttachment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "DeleteInterconnectAttachment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Routers in a given project and location.
        pub async fn list_routers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRoutersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRoutersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/ListRouters",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "ListRouters",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single Router.
        pub async fn get_router(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRouterRequest>,
        ) -> std::result::Result<tonic::Response<super::Router>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/GetRouter",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "GetRouter",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get the diagnostics of a single router resource.
        pub async fn diagnose_router(
            &mut self,
            request: impl tonic::IntoRequest<super::DiagnoseRouterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DiagnoseRouterResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/DiagnoseRouter",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "DiagnoseRouter",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new Router in a given project and location.
        pub async fn create_router(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRouterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/CreateRouter",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "CreateRouter",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single Router.
        pub async fn update_router(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRouterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/UpdateRouter",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "UpdateRouter",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single Router.
        pub async fn delete_router(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRouterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.edgenetwork.v1.EdgeNetwork/DeleteRouter",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.edgenetwork.v1.EdgeNetwork",
                        "DeleteRouter",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
