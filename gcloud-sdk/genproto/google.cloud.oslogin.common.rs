///  The POSIX account information associated with a Google account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PosixAccount {
    ///  Only one POSIX account can be marked as primary.
    #[prost(bool, tag="1")]
    pub primary: bool,
    ///  The username of the POSIX account.
    #[prost(string, tag="2")]
    pub username: ::prost::alloc::string::String,
    ///  The user ID.
    #[prost(int64, tag="3")]
    pub uid: i64,
    ///  The default group ID.
    #[prost(int64, tag="4")]
    pub gid: i64,
    ///  The path to the home directory for this account.
    #[prost(string, tag="5")]
    pub home_directory: ::prost::alloc::string::String,
    ///  The path to the logic shell for this account.
    #[prost(string, tag="6")]
    pub shell: ::prost::alloc::string::String,
    ///  The GECOS (user information) entry for this account.
    #[prost(string, tag="7")]
    pub gecos: ::prost::alloc::string::String,
    ///  System identifier for which account the username or uid applies to.
    ///  By default, the empty value is used.
    #[prost(string, tag="8")]
    pub system_id: ::prost::alloc::string::String,
    ///  Output only. A POSIX account identifier.
    #[prost(string, tag="9")]
    pub account_id: ::prost::alloc::string::String,
    ///  The operating system type where this account applies.
    #[prost(enumeration="OperatingSystemType", tag="10")]
    pub operating_system_type: i32,
    ///  Output only. The canonical resource name.
    #[prost(string, tag="11")]
    pub name: ::prost::alloc::string::String,
}
///  The SSH public key information associated with a Google account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SshPublicKey {
    ///  Public key text in SSH format, defined by
    ///  <a href="<https://www.ietf.org/rfc/rfc4253.txt"> target="_blank">RFC4253</a>
    ///  section 6.6.
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    ///  An expiration time in microseconds since epoch.
    #[prost(int64, tag="2")]
    pub expiration_time_usec: i64,
    ///  Output only. The SHA-256 fingerprint of the SSH public key.
    #[prost(string, tag="3")]
    pub fingerprint: ::prost::alloc::string::String,
    ///  Output only. The canonical resource name.
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
}
///  The operating system options for account entries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperatingSystemType {
    ///  The operating system type associated with the user account information is
    ///  unspecified.
    Unspecified = 0,
    ///  Linux user account information.
    Linux = 1,
    ///  Windows user account information.
    Windows = 2,
}
impl OperatingSystemType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperatingSystemType::Unspecified => "OPERATING_SYSTEM_TYPE_UNSPECIFIED",
            OperatingSystemType::Linux => "LINUX",
            OperatingSystemType::Windows => "WINDOWS",
        }
    }
}
