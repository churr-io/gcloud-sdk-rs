use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// Instance : Represents an Instance resource. An instance is a virtual machine that is hosted on Google Cloud Platform. For more information, read Virtual Machine Instances.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Instance {
    #[serde(
        rename = "advancedMachineFeatures",
        skip_serializing_if = "Option::is_none"
    )]
    pub advanced_machine_features:
        Option<Box<crate::google_rest_apis::compute_v1::models::AdvancedMachineFeatures>>,
    /// Allows this instance to send and receive packets with non-matching destination or source IPs. This is required if you plan to use this instance to forward routes. For more information, see Enabling IP Forwarding .
    #[serde(rename = "canIpForward", skip_serializing_if = "Option::is_none")]
    pub can_ip_forward: Option<bool>,
    #[serde(
        rename = "confidentialInstanceConfig",
        skip_serializing_if = "Option::is_none"
    )]
    pub confidential_instance_config:
        Option<Box<crate::google_rest_apis::compute_v1::models::ConfidentialInstanceConfig>>,
    /// [Output Only] The CPU platform used by this instance.
    #[serde(rename = "cpuPlatform", skip_serializing_if = "Option::is_none")]
    pub cpu_platform: Option<String>,
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// Whether the resource should be protected against deletion.
    #[serde(rename = "deletionProtection", skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    /// An optional description of this resource. Provide this property when you create the resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Array of disks associated with this instance. Persistent disks must be created before you can assign them.
    #[serde(rename = "disks", skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<crate::google_rest_apis::compute_v1::models::AttachedDisk>>,
    #[serde(rename = "displayDevice", skip_serializing_if = "Option::is_none")]
    pub display_device: Option<Box<crate::google_rest_apis::compute_v1::models::DisplayDevice>>,
    /// Specifies a fingerprint for this resource, which is essentially a hash of the instance's contents and used for optimistic locking. The fingerprint is initially generated by Compute Engine and changes after every request to modify or update the instance. You must always provide an up-to-date fingerprint hash in order to update the instance. To see the latest fingerprint, make get() request to the instance.
    #[serde(rename = "fingerprint", skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// A list of the type and count of accelerator cards attached to the instance.
    #[serde(rename = "guestAccelerators", skip_serializing_if = "Option::is_none")]
    pub guest_accelerators:
        Option<Vec<crate::google_rest_apis::compute_v1::models::AcceleratorConfig>>,
    /// Specifies the hostname of the instance. The specified hostname must be RFC1035 compliant. If hostname is not specified, the default hostname is [INSTANCE_NAME].c.[PROJECT_ID].internal when using the global DNS, and [INSTANCE_NAME].[ZONE].c.[PROJECT_ID].internal when using zonal DNS.
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// [Output Only] The unique identifier for the resource. This identifier is defined by the server.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "instanceEncryptionKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_encryption_key:
        Option<Box<crate::google_rest_apis::compute_v1::models::CustomerEncryptionKey>>,
    /// KeyRevocationActionType of the instance. Supported options are \"STOP\" and \"NONE\". The default value is \"NONE\" if it is not specified.
    #[serde(
        rename = "keyRevocationActionType",
        skip_serializing_if = "Option::is_none"
    )]
    pub key_revocation_action_type: Option<KeyRevocationActionType>,
    /// [Output Only] Type of the resource. Always compute#instance for instances.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// A fingerprint for this request, which is essentially a hash of the label's contents and used for optimistic locking. The fingerprint is initially generated by Compute Engine and changes after every request to modify or update labels. You must always provide an up-to-date fingerprint hash in order to update or change labels. To see the latest fingerprint, make get() request to the instance.
    #[serde(rename = "labelFingerprint", skip_serializing_if = "Option::is_none")]
    pub label_fingerprint: Option<String>,
    /// Labels to apply to this instance. These can be later modified by the setLabels method.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// [Output Only] Last start timestamp in RFC3339 text format.
    #[serde(rename = "lastStartTimestamp", skip_serializing_if = "Option::is_none")]
    pub last_start_timestamp: Option<String>,
    /// [Output Only] Last stop timestamp in RFC3339 text format.
    #[serde(rename = "lastStopTimestamp", skip_serializing_if = "Option::is_none")]
    pub last_stop_timestamp: Option<String>,
    /// [Output Only] Last suspended timestamp in RFC3339 text format.
    #[serde(
        rename = "lastSuspendedTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_suspended_timestamp: Option<String>,
    /// Full or partial URL of the machine type resource to use for this instance, in the format: zones/zone/machineTypes/machine-type. This is provided by the client when the instance is created. For example, the following is a valid partial url to a predefined machine type: zones/us-central1-f/machineTypes/n1-standard-1 To create a custom machine type, provide a URL to a machine type in the following format, where CPUS is 1 or an even number up to 32 (2, 4, 6, ... 24, etc), and MEMORY is the total memory for this instance. Memory must be a multiple of 256 MB and must be supplied in MB (e.g. 5 GB of memory is 5120 MB): zones/zone/machineTypes/custom-CPUS-MEMORY For example: zones/us-central1-f/machineTypes/custom-4-5120 For a full list of restrictions, read the Specifications for custom machine types.
    #[serde(rename = "machineType", skip_serializing_if = "Option::is_none")]
    pub machine_type: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::google_rest_apis::compute_v1::models::Metadata>>,
    /// Specifies a minimum CPU platform for the VM instance. Applicable values are the friendly names of CPU platforms, such as minCpuPlatform: \"Intel Haswell\" or minCpuPlatform: \"Intel Sandy Bridge\".
    #[serde(rename = "minCpuPlatform", skip_serializing_if = "Option::is_none")]
    pub min_cpu_platform: Option<String>,
    /// The name of the resource, provided by the client when initially creating the resource. The resource name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// An array of network configurations for this instance. These specify how interfaces are configured to interact with other network services, such as connecting to the internet. Multiple interfaces are supported per instance.
    #[serde(rename = "networkInterfaces", skip_serializing_if = "Option::is_none")]
    pub network_interfaces:
        Option<Vec<crate::google_rest_apis::compute_v1::models::NetworkInterface>>,
    #[serde(
        rename = "networkPerformanceConfig",
        skip_serializing_if = "Option::is_none"
    )]
    pub network_performance_config:
        Option<Box<crate::google_rest_apis::compute_v1::models::NetworkPerformanceConfig>>,
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<Box<crate::google_rest_apis::compute_v1::models::InstanceParams>>,
    /// The private IPv6 google access type for the VM. If not specified, use INHERIT_FROM_SUBNETWORK as default.
    #[serde(
        rename = "privateIpv6GoogleAccess",
        skip_serializing_if = "Option::is_none"
    )]
    pub private_ipv6_google_access: Option<PrivateIpv6GoogleAccess>,
    #[serde(
        rename = "reservationAffinity",
        skip_serializing_if = "Option::is_none"
    )]
    pub reservation_affinity:
        Option<Box<crate::google_rest_apis::compute_v1::models::ReservationAffinity>>,
    /// Resource policies applied to this instance.
    #[serde(rename = "resourcePolicies", skip_serializing_if = "Option::is_none")]
    pub resource_policies: Option<Vec<String>>,
    #[serde(rename = "resourceStatus", skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<Box<crate::google_rest_apis::compute_v1::models::ResourceStatus>>,
    /// [Output Only] Reserved for future use.
    #[serde(rename = "satisfiesPzi", skip_serializing_if = "Option::is_none")]
    pub satisfies_pzi: Option<bool>,
    /// [Output Only] Reserved for future use.
    #[serde(rename = "satisfiesPzs", skip_serializing_if = "Option::is_none")]
    pub satisfies_pzs: Option<bool>,
    #[serde(rename = "scheduling", skip_serializing_if = "Option::is_none")]
    pub scheduling: Option<Box<crate::google_rest_apis::compute_v1::models::Scheduling>>,
    /// [Output Only] Server-defined URL for this resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// A list of service accounts, with their specified scopes, authorized for this instance. Only one service account per VM instance is supported. Service accounts generate access tokens that can be accessed through the metadata server and used to authenticate applications on the instance. See Service Accounts for more information.
    #[serde(rename = "serviceAccounts", skip_serializing_if = "Option::is_none")]
    pub service_accounts: Option<Vec<crate::google_rest_apis::compute_v1::models::ServiceAccount>>,
    #[serde(
        rename = "shieldedInstanceConfig",
        skip_serializing_if = "Option::is_none"
    )]
    pub shielded_instance_config:
        Option<Box<crate::google_rest_apis::compute_v1::models::ShieldedInstanceConfig>>,
    #[serde(
        rename = "shieldedInstanceIntegrityPolicy",
        skip_serializing_if = "Option::is_none"
    )]
    pub shielded_instance_integrity_policy:
        Option<Box<crate::google_rest_apis::compute_v1::models::ShieldedInstanceIntegrityPolicy>>,
    /// Source machine image
    #[serde(rename = "sourceMachineImage", skip_serializing_if = "Option::is_none")]
    pub source_machine_image: Option<String>,
    #[serde(
        rename = "sourceMachineImageEncryptionKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_machine_image_encryption_key:
        Option<Box<crate::google_rest_apis::compute_v1::models::CustomerEncryptionKey>>,
    /// [Output Only] Whether a VM has been restricted for start because Compute Engine has detected suspicious activity.
    #[serde(rename = "startRestricted", skip_serializing_if = "Option::is_none")]
    pub start_restricted: Option<bool>,
    /// [Output Only] The status of the instance. One of the following values: PROVISIONING, STAGING, RUNNING, STOPPING, SUSPENDING, SUSPENDED, REPAIRING, and TERMINATED. For more information about the status of the instance, see Instance life cycle.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// [Output Only] An optional, human-readable explanation of the status.
    #[serde(rename = "statusMessage", skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Box<crate::google_rest_apis::compute_v1::models::Tags>>,
    /// [Output Only] URL of the zone where the instance resides. You must specify this field as part of the HTTP request URL. It is not settable as a field in the request body.
    #[serde(rename = "zone", skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

impl Instance {
    /// Represents an Instance resource. An instance is a virtual machine that is hosted on Google Cloud Platform. For more information, read Virtual Machine Instances.
    pub fn new() -> Instance {
        Instance {
            advanced_machine_features: None,
            can_ip_forward: None,
            confidential_instance_config: None,
            cpu_platform: None,
            creation_timestamp: None,
            deletion_protection: None,
            description: None,
            disks: None,
            display_device: None,
            fingerprint: None,
            guest_accelerators: None,
            hostname: None,
            id: None,
            instance_encryption_key: None,
            key_revocation_action_type: None,
            kind: None,
            label_fingerprint: None,
            labels: None,
            last_start_timestamp: None,
            last_stop_timestamp: None,
            last_suspended_timestamp: None,
            machine_type: None,
            metadata: None,
            min_cpu_platform: None,
            name: None,
            network_interfaces: None,
            network_performance_config: None,
            params: None,
            private_ipv6_google_access: None,
            reservation_affinity: None,
            resource_policies: None,
            resource_status: None,
            satisfies_pzi: None,
            satisfies_pzs: None,
            scheduling: None,
            self_link: None,
            service_accounts: None,
            shielded_instance_config: None,
            shielded_instance_integrity_policy: None,
            source_machine_image: None,
            source_machine_image_encryption_key: None,
            start_restricted: None,
            status: None,
            status_message: None,
            tags: None,
            zone: None,
        }
    }
}

/// KeyRevocationActionType of the instance. Supported options are \"STOP\" and \"NONE\". The default value is \"NONE\" if it is not specified.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum KeyRevocationActionType {
    #[serde(rename = "KEY_REVOCATION_ACTION_TYPE_UNSPECIFIED")]
    KeyRevocationActionTypeUnspecified,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "STOP")]
    Stop,
}

impl Default for KeyRevocationActionType {
    fn default() -> KeyRevocationActionType {
        Self::KeyRevocationActionTypeUnspecified
    }
}
/// The private IPv6 google access type for the VM. If not specified, use INHERIT_FROM_SUBNETWORK as default.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PrivateIpv6GoogleAccess {
    #[serde(rename = "ENABLE_BIDIRECTIONAL_ACCESS_TO_GOOGLE")]
    EnableBidirectionalAccessToGoogle,
    #[serde(rename = "ENABLE_OUTBOUND_VM_ACCESS_TO_GOOGLE")]
    EnableOutboundVmAccessToGoogle,
    #[serde(rename = "INHERIT_FROM_SUBNETWORK")]
    InheritFromSubnetwork,
}

impl Default for PrivateIpv6GoogleAccess {
    fn default() -> PrivateIpv6GoogleAccess {
        Self::EnableBidirectionalAccessToGoogle
    }
}
/// [Output Only] The status of the instance. One of the following values: PROVISIONING, STAGING, RUNNING, STOPPING, SUSPENDING, SUSPENDED, REPAIRING, and TERMINATED. For more information about the status of the instance, see Instance life cycle.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "DEPROVISIONING")]
    Deprovisioning,
    #[serde(rename = "PROVISIONING")]
    Provisioning,
    #[serde(rename = "REPAIRING")]
    Repairing,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "STAGING")]
    Staging,
    #[serde(rename = "STOPPED")]
    Stopped,
    #[serde(rename = "STOPPING")]
    Stopping,
    #[serde(rename = "SUSPENDED")]
    Suspended,
    #[serde(rename = "SUSPENDING")]
    Suspending,
    #[serde(rename = "TERMINATED")]
    Terminated,
}

impl Default for Status {
    fn default() -> Status {
        Self::Deprovisioning
    }
}
