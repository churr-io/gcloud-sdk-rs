///  Describes a running service that sends errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceContext {
    ///  An identifier of the service.
    ///  For example, "retail.googleapis.com".
    #[prost(string, tag="1")]
    pub service: ::prost::alloc::string::String,
}
///  HTTP request data that is related to a reported error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRequestContext {
    ///  The HTTP response status code for the request.
    #[prost(int32, tag="1")]
    pub response_status_code: i32,
}
///  Indicates a location in the source code of the service for which
///  errors are reported.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceLocation {
    ///  Human-readable name of a function or method.
    ///  For example, "google.cloud.retail.v2.UserEventService.ImportUserEvents".
    #[prost(string, tag="1")]
    pub function_name: ::prost::alloc::string::String,
}
///  A description of the context in which an error occurred.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorContext {
    ///  The HTTP request which was processed when the error was triggered.
    #[prost(message, optional, tag="1")]
    pub http_request: ::core::option::Option<HttpRequestContext>,
    ///  The location in the source code where the decision was made to
    ///  report the error, usually the place where it was logged.
    #[prost(message, optional, tag="2")]
    pub report_location: ::core::option::Option<SourceLocation>,
}
///  The error payload that is populated on LRO import APIs. Including:
///    "google.cloud.retail.v2.ProductService.ImportProducts"
///    "google.cloud.retail.v2.EventService.ImportUserEvents"
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportErrorContext {
    ///  The operation resource name of the LRO.
    #[prost(string, tag="1")]
    pub operation_name: ::prost::alloc::string::String,
    ///  Cloud Storage file path of the import source.
    ///  Can be set for batch operation error.
    #[prost(string, tag="2")]
    pub gcs_path: ::prost::alloc::string::String,
    ///  Line number of the content in file.
    ///  Should be empty for permission or batch operation error.
    #[prost(string, tag="3")]
    pub line_number: ::prost::alloc::string::String,
    ///  Detailed content which caused the error.
    ///  Should be empty for permission or batch operation error.
    #[prost(oneof="import_error_context::LineContent", tags="4, 5, 6")]
    pub line_content: ::core::option::Option<import_error_context::LineContent>,
}
/// Nested message and enum types in `ImportErrorContext`.
pub mod import_error_context {
    ///  Detailed content which caused the error.
    ///  Should be empty for permission or batch operation error.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LineContent {
        ///  The detailed content which caused the error on importing a catalog item.
        #[prost(string, tag="4")]
        CatalogItem(::prost::alloc::string::String),
        ///  The detailed content which caused the error on importing a product.
        #[prost(string, tag="5")]
        Product(::prost::alloc::string::String),
        ///  The detailed content which caused the error on importing a user event.
        #[prost(string, tag="6")]
        UserEvent(::prost::alloc::string::String),
    }
}
///  An error log which is reported to the Error Reporting system.
///  This proto a superset of
///  google.devtools.clouderrorreporting.v1beta1.ReportedErrorEvent.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorLog {
    ///  The service context in which this error has occurred.
    #[prost(message, optional, tag="1")]
    pub service_context: ::core::option::Option<ServiceContext>,
    ///  A description of the context in which the error occurred.
    #[prost(message, optional, tag="2")]
    pub context: ::core::option::Option<ErrorContext>,
    ///  A message describing the error.
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
    ///  The RPC status associated with the error log.
    #[prost(message, optional, tag="4")]
    pub status: ::core::option::Option<super::super::super::rpc::Status>,
    ///  The API request payload, represented as a protocol buffer.
    ///
    ///  Most API request types are supported. For example:
    ///
    ///    "type.googleapis.com/google.cloud.retail.v2.ProductService.CreateProductRequest"
    ///    "type.googleapis.com/google.cloud.retail.v2.UserEventService.WriteUserEventRequest"
    #[prost(message, optional, tag="5")]
    pub request_payload: ::core::option::Option<::prost_types::Struct>,
    ///  The API response payload, represented as a protocol buffer.
    ///
    ///  This is used to log some "soft errors", where the response is valid but we
    ///  consider there are some quality issues like unjoined events.
    ///
    ///  The following API responses are supported and no PII is included:
    ///    "google.cloud.retail.v2.PredictionService.Predict"
    ///    "google.cloud.retail.v2.UserEventService.WriteUserEvent"
    ///    "google.cloud.retail.v2.UserEventService.CollectUserEvent"
    #[prost(message, optional, tag="6")]
    pub response_payload: ::core::option::Option<::prost_types::Struct>,
    ///  The error payload that is populated on LRO import APIs.
    #[prost(message, optional, tag="7")]
    pub import_payload: ::core::option::Option<ImportErrorContext>,
}
