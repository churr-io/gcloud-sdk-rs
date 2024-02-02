/// The user profile information used for logging in to a virtual machine on
/// Google Compute Engine.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginProfile {
    /// Required. A unique user ID.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The list of POSIX accounts associated with the user.
    #[prost(message, repeated, tag = "2")]
    pub posix_accounts: ::prost::alloc::vec::Vec<super::common::PosixAccount>,
    /// A map from SSH public key fingerprint to the associated key object.
    #[prost(map = "string, message", tag = "3")]
    pub ssh_public_keys: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::common::SshPublicKey,
    >,
    /// The registered security key credentials for a user.
    #[prost(message, repeated, tag = "5")]
    pub security_keys: ::prost::alloc::vec::Vec<SecurityKey>,
}
/// A request message for creating an SSH public key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSshPublicKeyRequest {
    /// Required. The unique ID for the user in format `users/{user}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The SSH public key and expiration time.
    #[prost(message, optional, tag = "2")]
    pub ssh_public_key: ::core::option::Option<super::common::SshPublicKey>,
}
/// A request message for deleting a POSIX account entry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePosixAccountRequest {
    /// Required. A reference to the POSIX account to update. POSIX accounts are
    /// identified by the project ID they are associated with. A reference to the
    /// POSIX account is in format `users/{user}/projects/{project}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request message for deleting an SSH public key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSshPublicKeyRequest {
    /// Required. The fingerprint of the public key to update. Public keys are
    /// identified by their SHA-256 fingerprint. The fingerprint of the public key
    /// is in format `users/{user}/sshPublicKeys/{fingerprint}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request message for retrieving the login profile information for a user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLoginProfileRequest {
    /// Required. The unique ID for the user in format `users/{user}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The project ID of the Google Cloud Platform project.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// A system ID for filtering the results of the request.
    #[prost(string, tag = "3")]
    pub system_id: ::prost::alloc::string::String,
    /// The view configures whether to retrieve security keys information.
    #[prost(enumeration = "LoginProfileView", tag = "4")]
    pub view: i32,
}
/// A request message for retrieving an SSH public key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSshPublicKeyRequest {
    /// Required. The fingerprint of the public key to retrieve. Public keys are
    /// identified by their SHA-256 fingerprint. The fingerprint of the public key
    /// is in format `users/{user}/sshPublicKeys/{fingerprint}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request message for importing an SSH public key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportSshPublicKeyRequest {
    /// The unique ID for the user in format `users/{user}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The SSH public key and expiration time.
    #[prost(message, optional, tag = "2")]
    pub ssh_public_key: ::core::option::Option<super::common::SshPublicKey>,
    /// The project ID of the Google Cloud Platform project.
    #[prost(string, tag = "3")]
    pub project_id: ::prost::alloc::string::String,
    /// The view configures whether to retrieve security keys information.
    #[prost(enumeration = "LoginProfileView", tag = "4")]
    pub view: i32,
    /// Optional. The regions to which to assert that the key was written.
    /// If unspecified, defaults to all regions.
    /// Regions are listed at <https://cloud.google.com/about/locations#region.>
    #[prost(string, repeated, tag = "5")]
    pub regions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A response message for importing an SSH public key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportSshPublicKeyResponse {
    /// The login profile information for the user.
    #[prost(message, optional, tag = "1")]
    pub login_profile: ::core::option::Option<LoginProfile>,
    /// Detailed information about import results.
    #[prost(string, tag = "2")]
    pub details: ::prost::alloc::string::String,
}
/// A request message for updating an SSH public key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSshPublicKeyRequest {
    /// Required. The fingerprint of the public key to update. Public keys are
    /// identified by their SHA-256 fingerprint. The fingerprint of the public key
    /// is in format `users/{user}/sshPublicKeys/{fingerprint}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The SSH public key and expiration time.
    #[prost(message, optional, tag = "2")]
    pub ssh_public_key: ::core::option::Option<super::common::SshPublicKey>,
    /// Mask to control which fields get updated. Updates all if not present.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The credential information for a Google registered security key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityKey {
    /// Public key text in SSH format, defined by
    /// [RFC4253]("<https://www.ietf.org/rfc/rfc4253.txt">) section 6.6.
    #[prost(string, tag = "1")]
    pub public_key: ::prost::alloc::string::String,
    /// Hardware-backed private key text in SSH format.
    #[prost(string, tag = "2")]
    pub private_key: ::prost::alloc::string::String,
    /// The security key nickname explicitly set by the user.
    #[prost(string, optional, tag = "5")]
    pub device_nickname: ::core::option::Option<::prost::alloc::string::String>,
    /// The FIDO protocol type used to register this credential.
    #[prost(oneof = "security_key::ProtocolType", tags = "3, 4")]
    pub protocol_type: ::core::option::Option<security_key::ProtocolType>,
}
/// Nested message and enum types in `SecurityKey`.
pub mod security_key {
    /// The FIDO protocol type used to register this credential.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ProtocolType {
        /// The U2F protocol type.
        #[prost(message, tag = "3")]
        UniversalTwoFactor(super::UniversalTwoFactor),
        /// The Web Authentication protocol type.
        #[prost(message, tag = "4")]
        WebAuthn(super::WebAuthn),
    }
}
/// Security key information specific to the U2F protocol.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UniversalTwoFactor {
    /// Application ID for the U2F protocol.
    #[prost(string, tag = "1")]
    pub app_id: ::prost::alloc::string::String,
}
/// Security key information specific to the Web Authentication protocol.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebAuthn {
    /// Relying party ID for Web Authentication.
    #[prost(string, tag = "1")]
    pub rp_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignSshPublicKeyRequest {
    /// The SSH public key to sign.
    #[prost(string, tag = "1")]
    pub ssh_public_key: ::prost::alloc::string::String,
    /// The parent project and zone for the signing request. This is needed to
    /// properly ensure per-organization ISS processing and potentially to provide
    /// for the possibility of zone-specific certificates used in the signing
    /// process.
    #[prost(string, tag = "2")]
    pub parent: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignSshPublicKeyResponse {
    /// The signed SSH public key to use in the SSH handshake.
    #[prost(string, tag = "1")]
    pub signed_ssh_public_key: ::prost::alloc::string::String,
}
/// The login profile view limits the user content retrieved.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LoginProfileView {
    /// The default login profile view. The API defaults to the BASIC view.
    Unspecified = 0,
    /// Includes POSIX and SSH key information.
    Basic = 1,
    /// Include security key information for the user.
    SecurityKey = 2,
}
impl LoginProfileView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LoginProfileView::Unspecified => "LOGIN_PROFILE_VIEW_UNSPECIFIED",
            LoginProfileView::Basic => "BASIC",
            LoginProfileView::SecurityKey => "SECURITY_KEY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOGIN_PROFILE_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "BASIC" => Some(Self::Basic),
            "SECURITY_KEY" => Some(Self::SecurityKey),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod os_login_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Cloud OS Login API
    ///
    /// The Cloud OS Login API allows you to manage users and their associated SSH
    /// public keys for logging into virtual machines on Google Cloud Platform.
    #[derive(Debug, Clone)]
    pub struct OsLoginServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OsLoginServiceClient<tonic::transport::Channel> {
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
    impl<T> OsLoginServiceClient<T>
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
        ) -> OsLoginServiceClient<InterceptedService<T, F>>
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
            OsLoginServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Create an SSH public key
        pub async fn create_ssh_public_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSshPublicKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::common::SshPublicKey>,
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
                "/google.cloud.oslogin.v1beta.OsLoginService/CreateSshPublicKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.oslogin.v1beta.OsLoginService",
                        "CreateSshPublicKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a POSIX account.
        pub async fn delete_posix_account(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePosixAccountRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.oslogin.v1beta.OsLoginService/DeletePosixAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.oslogin.v1beta.OsLoginService",
                        "DeletePosixAccount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an SSH public key.
        pub async fn delete_ssh_public_key(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSshPublicKeyRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.oslogin.v1beta.OsLoginService/DeleteSshPublicKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.oslogin.v1beta.OsLoginService",
                        "DeleteSshPublicKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves the profile information used for logging in to a virtual machine
        /// on Google Compute Engine.
        pub async fn get_login_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLoginProfileRequest>,
        ) -> std::result::Result<tonic::Response<super::LoginProfile>, tonic::Status> {
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
                "/google.cloud.oslogin.v1beta.OsLoginService/GetLoginProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.oslogin.v1beta.OsLoginService",
                        "GetLoginProfile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves an SSH public key.
        pub async fn get_ssh_public_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSshPublicKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::common::SshPublicKey>,
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
                "/google.cloud.oslogin.v1beta.OsLoginService/GetSshPublicKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.oslogin.v1beta.OsLoginService",
                        "GetSshPublicKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Adds an SSH public key and returns the profile information. Default POSIX
        /// account information is set when no username and UID exist as part of the
        /// login profile.
        pub async fn import_ssh_public_key(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportSshPublicKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImportSshPublicKeyResponse>,
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
                "/google.cloud.oslogin.v1beta.OsLoginService/ImportSshPublicKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.oslogin.v1beta.OsLoginService",
                        "ImportSshPublicKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an SSH public key and returns the profile information. This method
        /// supports patch semantics.
        pub async fn update_ssh_public_key(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSshPublicKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::common::SshPublicKey>,
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
                "/google.cloud.oslogin.v1beta.OsLoginService/UpdateSshPublicKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.oslogin.v1beta.OsLoginService",
                        "UpdateSshPublicKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Signs an SSH public key for a user to authenticate to an instance.
        pub async fn sign_ssh_public_key(
            &mut self,
            request: impl tonic::IntoRequest<super::SignSshPublicKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SignSshPublicKeyResponse>,
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
                "/google.cloud.oslogin.v1beta.OsLoginService/SignSshPublicKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.oslogin.v1beta.OsLoginService",
                        "SignSshPublicKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
