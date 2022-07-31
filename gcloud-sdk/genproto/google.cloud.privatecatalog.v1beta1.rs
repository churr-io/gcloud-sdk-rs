///  Request message for \[PrivateCatalog.SearchCatalogs][google.cloud.privatecatalog.v1beta1.PrivateCatalog.SearchCatalogs\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCatalogsRequest {
    ///  Required. The name of the resource context. It can be in following formats:
    ///
    ///  * `projects/{project}`
    ///  * `folders/{folder}`
    ///  * `organizations/{organization}`
    #[prost(string, tag="1")]
    pub resource: ::prost::alloc::string::String,
    ///  The query to filter the catalogs. The supported queries are:
    ///
    ///  * Get a single catalog: `name=catalogs/{catalog}`
    #[prost(string, tag="2")]
    pub query: ::prost::alloc::string::String,
    ///  The maximum number of entries that are requested.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    ///  A pagination token returned from a previous call to SearchCatalogs that
    ///  indicates where this listing should continue from.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
}
///  Response message for \[PrivateCatalog.SearchCatalogs][google.cloud.privatecatalog.v1beta1.PrivateCatalog.SearchCatalogs\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCatalogsResponse {
    ///  The `Catalog`s computed from the resource context.
    #[prost(message, repeated, tag="1")]
    pub catalogs: ::prost::alloc::vec::Vec<Catalog>,
    ///  A pagination token returned from a previous call to SearchCatalogs that
    ///  indicates from where listing should continue.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  Request message for \[PrivateCatalog.SearchProducts][google.cloud.privatecatalog.v1beta1.PrivateCatalog.SearchProducts\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchProductsRequest {
    ///  Required. The name of the resource context. See \[SearchCatalogsRequest.resource][google.cloud.privatecatalog.v1beta1.SearchCatalogsRequest.resource\]
    ///  for details.
    #[prost(string, tag="1")]
    pub resource: ::prost::alloc::string::String,
    ///  The query to filter the products.
    ///
    ///  The supported queries are:
    ///  * List products of all catalogs: empty
    ///  * List products under a catalog: `parent=catalogs/{catalog}`
    ///  * Get a product by name:
    ///  `name=catalogs/{catalog}/products/{product}`
    #[prost(string, tag="2")]
    pub query: ::prost::alloc::string::String,
    ///  The maximum number of entries that are requested.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    ///  A pagination token returned from a previous call to SearchProducts that
    ///  indicates where this listing should continue from.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
}
///  Response message for \[PrivateCatalog.SearchProducts][google.cloud.privatecatalog.v1beta1.PrivateCatalog.SearchProducts\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchProductsResponse {
    ///  The `Product` resources computed from the resource context.
    #[prost(message, repeated, tag="1")]
    pub products: ::prost::alloc::vec::Vec<Product>,
    ///  A pagination token returned from a previous call to SearchProducts that
    ///  indicates from where listing should continue.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  Request message for \[PrivateCatalog.SearchVersions][google.cloud.privatecatalog.v1beta1.PrivateCatalog.SearchVersions\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchVersionsRequest {
    ///  Required. The name of the resource context. See \[SearchCatalogsRequest.resource][google.cloud.privatecatalog.v1beta1.SearchCatalogsRequest.resource\]
    ///  for details.
    #[prost(string, tag="1")]
    pub resource: ::prost::alloc::string::String,
    ///  Required. The query to filter the versions.
    ///
    ///  The supported queries are:
    ///  * List versions under a product:
    ///  `parent=catalogs/{catalog}/products/{product}`
    ///  * Get a version by name:
    ///  `name=catalogs/{catalog}/products/{product}/versions/{version}`
    #[prost(string, tag="2")]
    pub query: ::prost::alloc::string::String,
    ///  The maximum number of entries that are requested.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    ///  A pagination token returned from a previous call to SearchVersions
    ///  that indicates where this listing should continue from.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
}
///  Response message for \[PrivateCatalog.SearchVersions][google.cloud.privatecatalog.v1beta1.PrivateCatalog.SearchVersions\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchVersionsResponse {
    ///  The `Version` resources computed from the resource context.
    #[prost(message, repeated, tag="1")]
    pub versions: ::prost::alloc::vec::Vec<Version>,
    ///  A pagination token returned from a previous call to SearchVersions that
    ///  indicates from where the listing should continue.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  The readonly representation of a catalog computed with a given resource
///  context.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Catalog {
    ///  Output only. The resource name of the target catalog, in the format of
    ///  `catalogs/{catalog}'.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Output only. The descriptive name of the catalog as it appears in UIs.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    ///  Output only. The description of the catalog.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    ///  Output only. The time when the catalog was created.
    #[prost(message, optional, tag="4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The time when the catalog was last updated.
    #[prost(message, optional, tag="5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
///  The readonly representation of a product computed with a given resource
///  context.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Product {
    ///  Output only. The resource name of the target product, in the format of
    ///  `products/\[a-z][-a-z0-9]*[a-z0-9\]'.
    ///
    ///  A unique identifier for the product under a catalog.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Output only. The type of the product asset. It can be one of the following values:
    ///
    ///  * `google.deploymentmanager.Template`
    ///  * `google.cloudprivatecatalog.ListingOnly`
    ///  * `google.cloudprivatecatalog.Terraform`
    #[prost(string, tag="2")]
    pub asset_type: ::prost::alloc::string::String,
    ///  Required. Output only. The display metadata to describe the product. The JSON schema of the
    ///  metadata differs by \[Product.asset_type][google.cloud.privatecatalog.v1beta1.Product.asset_type\].
    ///  When the type is `google.deploymentmanager.Template`, the schema is as
    ///  follows:
    ///
    ///  ```
    ///  "$schema": <http://json-schema.org/draft-04/schema#>
    ///  type: object
    ///  properties:
    ///    name:
    ///      type: string
    ///      minLength: 1
    ///      maxLength: 64
    ///    description:
    ///      type: string
    ///      minLength: 1
    ///      maxLength: 2048
    ///    tagline:
    ///      type: string
    ///      minLength: 1
    ///      maxLength: 100
    ///    support_info:
    ///      type: string
    ///      minLength: 1
    ///      maxLength: 2048
    ///    creator:
    ///      type: string
    ///      minLength: 1
    ///      maxLength: 100
    ///    documentations:
    ///      type: array
    ///      items:
    ///        type: object
    ///        properties:
    ///          url:
    ///            type: string
    ///            pattern:
    ///            "^(https?)://\[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|\]"
    ///          title:
    ///            type: string
    ///            minLength: 1
    ///            maxLength: 64
    ///          description:
    ///            type: string
    ///            minLength: 1
    ///            maxLength: 2048
    ///  required:
    ///  - name
    ///  - description
    ///  additionalProperties: false
    ///
    ///  ```
    ///
    ///  When the asset type is `google.cloudprivatecatalog.ListingOnly`, the schema
    ///  is as follows:
    ///
    ///  ```
    ///  "$schema": <http://json-schema.org/draft-04/schema#>
    ///  type: object
    ///  properties:
    ///    name:
    ///      type: string
    ///      minLength: 1
    ///      maxLength: 64
    ///    description:
    ///      type: string
    ///      minLength: 1
    ///      maxLength: 2048
    ///    tagline:
    ///      type: string
    ///      minLength: 1
    ///      maxLength: 100
    ///    support_info:
    ///      type: string
    ///      minLength: 1
    ///      maxLength: 2048
    ///    creator:
    ///      type: string
    ///      minLength: 1
    ///      maxLength: 100
    ///    documentations:
    ///      type: array
    ///      items:
    ///        type: object
    ///        properties:
    ///          url:
    ///            type: string
    ///            pattern:
    ///            "^(https?)://\[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|\]"
    ///          title:
    ///            type: string
    ///            minLength: 1
    ///            maxLength: 64
    ///          description:
    ///            type: string
    ///            minLength: 1
    ///            maxLength: 2048
    ///    signup_url:
    ///      type: string
    ///      pattern:
    ///      "^(https?)://\[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|\]"
    ///  required:
    ///  - name
    ///  - description
    ///  - signup_url
    ///  additionalProperties: false
    ///
    ///  ```
    ///
    ///  When the asset type is `google.cloudprivatecatalog.Terraform`, the schema
    ///  is as follows:
    ///
    ///  ```
    ///  "$schema": <http://json-schema.org/draft-04/schema#>
    ///  type: object
    ///  properties:
    ///    name:
    ///      type: string
    ///      minLength: 1
    ///      maxLength: 64
    ///    description:
    ///      type: string
    ///      minLength: 1
    ///      maxLength: 2048
    ///    tagline:
    ///      type: string
    ///      minLength: 1
    ///      maxLength: 100
    ///    support_info:
    ///      type: string
    ///      minLength: 1
    ///      maxLength: 2048
    ///    creator:
    ///      type: string
    ///      minLength: 1
    ///      maxLength: 100
    ///    documentations:
    ///      type: array
    ///      items:
    ///        type: object
    ///        properties:
    ///          url:
    ///            type: string
    ///            pattern:
    ///            "^(https?)://\[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|\]"
    ///          title:
    ///            type: string
    ///            minLength: 1
    ///            maxLength: 64
    ///          description:
    ///            type: string
    ///            minLength: 1
    ///            maxLength: 2048
    ///  required:
    ///  - name
    ///  - description
    ///  additionalProperties: true
    #[prost(message, optional, tag="3")]
    pub display_metadata: ::core::option::Option<::prost_types::Struct>,
    ///  Output only. The icon URI of the product.
    #[prost(string, tag="4")]
    pub icon_uri: ::prost::alloc::string::String,
    ///  Output only. A collection of assets referred by a product.
    ///  This field is set for Terraform Products only.
    #[prost(message, repeated, tag="10")]
    pub asset_references: ::prost::alloc::vec::Vec<AssetReference>,
    ///  Output only. The time when the product was created.
    #[prost(message, optional, tag="5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The time when the product was last updated.
    #[prost(message, optional, tag="6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
///  Defines the reference of an asset belonging to a product.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetReference {
    ///  Output only. A unique identifier among asset references in a product.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    ///  Output only. The human-readable description of the referenced asset. Maximum 256
    ///  characters in length.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    ///  Output only. The definition of input parameters to hydrate the asset template.
    #[prost(message, optional, tag="6")]
    pub inputs: ::core::option::Option<Inputs>,
    ///  Output only. The current state of the asset reference.
    #[prost(enumeration="asset_reference::AssetValidationState", tag="7")]
    pub validation_status: i32,
    ///  Output only. The validation process metadata.
    #[prost(message, optional, tag="8")]
    pub validation_operation: ::core::option::Option<super::super::super::longrunning::Operation>,
    ///  Output only. The cloud storage source.
    #[prost(message, optional, tag="16")]
    pub gcs_source: ::core::option::Option<GcsSource>,
    ///  Output only. The creation timestamp of the asset reference.
    #[prost(message, optional, tag="12")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The last update timestamp of the asset reference.
    #[prost(message, optional, tag="13")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  The version of the source used for this asset reference.
    #[deprecated]
    #[prost(string, tag="14")]
    pub version: ::prost::alloc::string::String,
    ///  The destination of the asset.
    #[prost(oneof="asset_reference::Source", tags="10, 11, 15")]
    pub source: ::core::option::Option<asset_reference::Source>,
}
/// Nested message and enum types in `AssetReference`.
pub mod asset_reference {
    ///  Possible validation steates of an asset reference.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AssetValidationState {
        ///  Unknown state.
        Unspecified = 0,
        ///  The validation is still in process.
        Pending = 1,
        ///  The validation is done and the asset reference is valid.
        Valid = 2,
        ///  The validation is done and the asset reference is invalid.
        Invalid = 3,
    }
    impl AssetValidationState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AssetValidationState::Unspecified => "ASSET_VALIDATION_STATE_UNSPECIFIED",
                AssetValidationState::Pending => "PENDING",
                AssetValidationState::Valid => "VALID",
                AssetValidationState::Invalid => "INVALID",
            }
        }
    }
    ///  The destination of the asset.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        ///  Output only. The asset resource name if an asset is hosted by Private Catalog.
        #[prost(string, tag="10")]
        Asset(::prost::alloc::string::String),
        ///  Output only. The cloud storage object path.
        #[prost(string, tag="11")]
        GcsPath(::prost::alloc::string::String),
        ///  Output only. The git source.
        #[prost(message, tag="15")]
        GitSource(super::GitSource),
    }
}
///  Defines definition of input parameters of asset templates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Inputs {
    ///  Output only. The JSON schema defining the inputs and their formats.
    #[prost(message, optional, tag="1")]
    pub parameters: ::core::option::Option<::prost_types::Struct>,
}
///  Defines how to access Cloud Storage source.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsSource {
    ///  Output only. the cloud storage object path.
    #[prost(string, tag="1")]
    pub gcs_path: ::prost::alloc::string::String,
    ///  Output only. Generation of the object, which is set when the content of an object starts
    ///  being written.
    #[prost(int64, tag="2")]
    pub generation: i64,
    ///  Output only. The time when the object metadata was last changed.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
///  Defines how to access a Git Source.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitSource {
    ///  Location of the Git repo to build.
    #[prost(string, tag="1")]
    pub repo: ::prost::alloc::string::String,
    ///  Directory, relative to the source root, in which to run the build.
    ///
    ///  This must be a relative path. If a step's `dir` is specified and is an
    ///  absolute path, this value is ignored for that step's execution.
    #[prost(string, tag="2")]
    pub dir: ::prost::alloc::string::String,
    ///  The revision to fetch from the Git repository such as a branch, a tag, a
    ///  commit SHA, or any Git ref.
    #[prost(oneof="git_source::Ref", tags="3, 4, 5")]
    pub r#ref: ::core::option::Option<git_source::Ref>,
}
/// Nested message and enum types in `GitSource`.
pub mod git_source {
    ///  The revision to fetch from the Git repository such as a branch, a tag, a
    ///  commit SHA, or any Git ref.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Ref {
        ///  The revision commit to use.
        #[prost(string, tag="3")]
        Commit(::prost::alloc::string::String),
        ///  The revision branch to use.
        #[prost(string, tag="4")]
        Branch(::prost::alloc::string::String),
        ///  The revision tag to use.
        #[prost(string, tag="5")]
        Tag(::prost::alloc::string::String),
    }
}
///  The consumer representation of a version which is a child resource under a
///  `Product` with asset data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    ///  Output only. The resource name of the version, in the format
    ///  `catalogs/{catalog}/products/{product}/versions/\[a-z][-a-z0-9]*[a-z0-9\]'.
    ///
    ///  A unique identifier for the version under a product.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Output only. The user-supplied description of the version. Maximum of 256
    ///  characters.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    ///  Output only. The asset which has been validated and is ready to be
    ///  provisioned. See
    ///  \[google.cloud.privatecatalogproducer.v1beta.Version.asset][\] for details.
    #[prost(message, optional, tag="3")]
    pub asset: ::core::option::Option<::prost_types::Struct>,
    ///  Output only. The time when the version was created.
    #[prost(message, optional, tag="5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The time when the version was last updated.
    #[prost(message, optional, tag="6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Generated client implementations.
pub mod private_catalog_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// `PrivateCatalog` allows catalog consumers to retrieve `Catalog`, `Product`
    /// and `Version` resources under a target resource context.
    ///
    /// `Catalog` is computed based on the [Association][]s linked to the target
    /// resource and its ancestors. Each association's
    /// [google.cloud.privatecatalogproducer.v1beta.Catalog][] is transformed into a
    /// `Catalog`. If multiple associations have the same parent
    /// [google.cloud.privatecatalogproducer.v1beta.Catalog][], they are
    /// de-duplicated into one `Catalog`. Users must have
    /// `cloudprivatecatalog.catalogTargets.get` IAM permission on the resource
    /// context in order to access catalogs. `Catalog` contains the resource name and
    /// a subset of data of the original
    /// [google.cloud.privatecatalogproducer.v1beta.Catalog][].
    ///
    /// `Product` is child resource of the catalog. A `Product` contains the resource
    /// name and a subset of the data of the original
    /// [google.cloud.privatecatalogproducer.v1beta.Product][].
    ///
    /// `Version` is child resource of the product. A `Version` contains the resource
    /// name and a subset of the data of the original
    /// [google.cloud.privatecatalogproducer.v1beta.Version][].
    #[derive(Debug, Clone)]
    pub struct PrivateCatalogClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PrivateCatalogClient<tonic::transport::Channel> {
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
    impl<T> PrivateCatalogClient<T>
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
        ) -> PrivateCatalogClient<InterceptedService<T, F>>
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
            PrivateCatalogClient::new(InterceptedService::new(inner, interceptor))
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
        /// Search [Catalog][google.cloud.privatecatalog.v1beta1.Catalog] resources that consumers have access to, within the
        /// scope of the consumer cloud resource hierarchy context.
        pub async fn search_catalogs(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchCatalogsRequest>,
        ) -> Result<tonic::Response<super::SearchCatalogsResponse>, tonic::Status> {
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
                "/google.cloud.privatecatalog.v1beta1.PrivateCatalog/SearchCatalogs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Search [Product][google.cloud.privatecatalog.v1beta1.Product] resources that consumers have access to, within the
        /// scope of the consumer cloud resource hierarchy context.
        pub async fn search_products(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchProductsRequest>,
        ) -> Result<tonic::Response<super::SearchProductsResponse>, tonic::Status> {
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
                "/google.cloud.privatecatalog.v1beta1.PrivateCatalog/SearchProducts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Search [Version][google.cloud.privatecatalog.v1beta1.Version] resources that consumers have access to, within the
        /// scope of the consumer cloud resource hierarchy context.
        pub async fn search_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchVersionsRequest>,
        ) -> Result<tonic::Response<super::SearchVersionsResponse>, tonic::Status> {
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
                "/google.cloud.privatecatalog.v1beta1.PrivateCatalog/SearchVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
