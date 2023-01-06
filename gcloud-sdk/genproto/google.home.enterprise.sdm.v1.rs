/// Device resource represents an instance of enterprise managed device in the
/// property.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Device {
    /// Required. The resource name of the device. For example:
    /// "enterprises/XYZ/devices/123".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Type of the device for general display purposes.
    /// For example: "THERMOSTAT". The device type should not be used to deduce or
    /// infer functionality of the actual device it is assigned to. Instead, use
    /// the returned traits for the device.
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// Output only. Device traits.
    #[prost(message, optional, tag = "4")]
    pub traits: ::core::option::Option<::prost_types::Struct>,
    /// Assignee details of the device.
    #[prost(message, repeated, tag = "5")]
    pub parent_relations: ::prost::alloc::vec::Vec<ParentRelation>,
}
/// Represents device relationships, for instance, structure/room to which the
/// device is assigned to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParentRelation {
    /// Output only. The name of the relation -- e.g., structure/room where the
    /// device is assigned to. For example: "enterprises/XYZ/structures/ABC" or
    /// "enterprises/XYZ/structures/ABC/rooms/123"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Output only. The custom name of the relation -- e.g., structure/room where
    /// the device is assigned to.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
}
/// Structure resource represents an instance of enterprise managed home or hotel
/// room.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Structure {
    /// Output only. The resource name of the structure. For example:
    /// "enterprises/XYZ/structures/ABC".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Structure traits.
    #[prost(message, optional, tag = "2")]
    pub traits: ::core::option::Option<::prost_types::Struct>,
}
/// Room resource represents an instance of sub-space within a structure such as
/// rooms in a hotel suite or rental apartment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Room {
    /// Output only. The resource name of the room. For example:
    /// "enterprises/XYZ/structures/ABC/rooms/123".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Room traits.
    #[prost(message, optional, tag = "2")]
    pub traits: ::core::option::Option<::prost_types::Struct>,
}
/// Request message for SmartDeviceManagementService.GetDevice
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeviceRequest {
    /// The name of the device requested. For example:
    /// "enterprises/XYZ/devices/123"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for SmartDeviceManagementService.ListDevices
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDevicesRequest {
    /// The parent enterprise to list devices under. E.g. "enterprises/XYZ".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional requested page size. Server may return fewer devices than
    /// requested. If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional token of the page to retrieve.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional filter to list devices.
    ///
    /// Filters can be done on:
    /// Device custom name (substring match):
    /// 'customName=wing'
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for SmartDeviceManagementService.ListDevices
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDevicesResponse {
    /// The list of devices.
    #[prost(message, repeated, tag = "1")]
    pub devices: ::prost::alloc::vec::Vec<Device>,
    /// The pagination token to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for SmartDeviceManagementService.ExecuteDeviceCommand
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteDeviceCommandRequest {
    /// The name of the device requested. For example:
    /// "enterprises/XYZ/devices/123"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The command name to execute, represented by the fully qualified protobuf
    /// message name.
    #[prost(string, tag = "2")]
    pub command: ::prost::alloc::string::String,
    /// The command message to execute, represented as a Struct.
    #[prost(message, optional, tag = "3")]
    pub params: ::core::option::Option<::prost_types::Struct>,
}
/// Response message for SmartDeviceManagementService.ExecuteDeviceCommand
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteDeviceCommandResponse {
    /// The results of executing the command.
    #[prost(message, optional, tag = "1")]
    pub results: ::core::option::Option<::prost_types::Struct>,
}
/// Request message for SmartDeviceManagementService.GetStructure
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStructureRequest {
    /// The name of the structure requested. For example:
    /// "enterprises/XYZ/structures/ABC".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for SmartDeviceManagementService.ListStructures
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStructuresRequest {
    /// The parent enterprise to list structures under. E.g. "enterprises/XYZ".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer structures than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The token of the page to retrieve.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional filter to list structures.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for SmartDeviceManagementService.ListStructures
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStructuresResponse {
    /// The list of structures.
    #[prost(message, repeated, tag = "1")]
    pub structures: ::prost::alloc::vec::Vec<Structure>,
    /// The pagination token to retrieve the next page of results.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for SmartDeviceManagementService.GetRoom
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoomRequest {
    /// The name of the room requested. For example:
    /// "enterprises/XYZ/structures/ABC/rooms/123".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for SmartDeviceManagementService.ListRooms
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRoomsRequest {
    /// The parent resource name of the rooms requested. For example:
    /// "enterprises/XYZ/structures/ABC".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer rooms than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The token of the page to retrieve.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for SmartDeviceManagementService.ListRooms
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRoomsResponse {
    /// The list of rooms.
    #[prost(message, repeated, tag = "1")]
    pub rooms: ::prost::alloc::vec::Vec<Room>,
    /// The pagination token to retrieve the next page of results.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod smart_device_management_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A service that allows API consumers to provision and manage Google
    /// Home structures and devices for enterprise use cases.
    #[derive(Debug, Clone)]
    pub struct SmartDeviceManagementServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SmartDeviceManagementServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SmartDeviceManagementServiceClient<T>
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
        ) -> SmartDeviceManagementServiceClient<InterceptedService<T, F>>
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
            SmartDeviceManagementServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// Gets a device managed by the enterprise.
        pub async fn get_device(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDeviceRequest>,
        ) -> Result<tonic::Response<super::Device>, tonic::Status> {
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
                "/google.home.enterprise.sdm.v1.SmartDeviceManagementService/GetDevice",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists devices managed by the enterprise.
        pub async fn list_devices(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDevicesRequest>,
        ) -> Result<tonic::Response<super::ListDevicesResponse>, tonic::Status> {
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
                "/google.home.enterprise.sdm.v1.SmartDeviceManagementService/ListDevices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Executes a command to device managed by the enterprise.
        pub async fn execute_device_command(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteDeviceCommandRequest>,
        ) -> Result<
            tonic::Response<super::ExecuteDeviceCommandResponse>,
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
                "/google.home.enterprise.sdm.v1.SmartDeviceManagementService/ExecuteDeviceCommand",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a structure managed by the enterprise.
        pub async fn get_structure(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStructureRequest>,
        ) -> Result<tonic::Response<super::Structure>, tonic::Status> {
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
                "/google.home.enterprise.sdm.v1.SmartDeviceManagementService/GetStructure",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists structures managed by the enterprise.
        pub async fn list_structures(
            &mut self,
            request: impl tonic::IntoRequest<super::ListStructuresRequest>,
        ) -> Result<tonic::Response<super::ListStructuresResponse>, tonic::Status> {
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
                "/google.home.enterprise.sdm.v1.SmartDeviceManagementService/ListStructures",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a room managed by the enterprise.
        pub async fn get_room(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRoomRequest>,
        ) -> Result<tonic::Response<super::Room>, tonic::Status> {
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
                "/google.home.enterprise.sdm.v1.SmartDeviceManagementService/GetRoom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists rooms managed by the enterprise.
        pub async fn list_rooms(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRoomsRequest>,
        ) -> Result<tonic::Response<super::ListRoomsResponse>, tonic::Status> {
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
                "/google.home.enterprise.sdm.v1.SmartDeviceManagementService/ListRooms",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
