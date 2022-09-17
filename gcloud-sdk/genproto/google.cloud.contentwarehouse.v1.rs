///  Metadata object for CreateDocument request (currently empty).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDocumentMetadata {
}
///  Metadata object for UpdateDocument request (currently empty).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDocumentMetadata {
}
///  Meta information is used to improve the performance of the service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMetadata {
    ///  Provides user unique identification and groups information.
    #[prost(message, optional, tag="1")]
    pub user_info: ::core::option::Option<UserInfo>,
}
///  Additional information returned to client, such as debugging information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMetadata {
    ///  A unique id associated with this call. This id is logged for tracking
    ///  purpose.
    #[prost(string, tag="1")]
    pub request_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfo {
    ///  A unique user identification string, as determined by the client.
    ///  The maximum number of allowed characters is 255.
    ///  Allowed characters include numbers 0 to 9, uppercase and lowercase letters,
    ///  and restricted special symbols (:, @, +, -, _, ~)
    ///  The format is "user:xxxx@example.com";
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    ///  The unique group identifications which the user is belong to.
    ///  The format is "group:yyyy@example.com";
    #[prost(string, repeated, tag="2")]
    pub group_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///  Options for Update operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOptions {
    ///  Type for update.
    #[prost(enumeration="UpdateType", tag="1")]
    pub update_type: i32,
    ///  Field mask for merging Document fields.
    ///  For the `FieldMask` definition,
    ///  see
    ///  <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    ///  Options for merging.
    #[prost(message, optional, tag="3")]
    pub merge_fields_options: ::core::option::Option<MergeFieldsOptions>,
}
///  Options for merging updated fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeFieldsOptions {
    ///  When merging message fields, the default behavior is to merge
    ///  the content of two message fields together. If you instead want to use
    ///  the field from the source message to replace the corresponding field in
    ///  the destination message, set this flag to true. When this flag is set,
    ///  specified submessage fields that are missing in source will be cleared in
    ///  destination.
    #[prost(bool, optional, tag="1")]
    pub replace_message_fields: ::core::option::Option<bool>,
    ///  When merging repeated fields, the default behavior is to append
    ///  entries from the source repeated field to the destination repeated field.
    ///  If you instead want to keep only the entries from the source repeated
    ///  field, set this flag to true.
    ///
    ///  If you want to replace a repeated field within a message field on the
    ///  destination message, you must set both replace_repeated_fields and
    ///  replace_message_fields to true, otherwise the repeated fields will be
    ///  appended.
    #[prost(bool, optional, tag="2")]
    pub replace_repeated_fields: ::core::option::Option<bool>,
}
///  Update type of the requests.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UpdateType {
    ///  Defaults to full replace behavior, ie. FULL_REPLACE.
    Unspecified = 0,
    ///  Fully replace all the fields. Any field masks will be ignored.
    Replace = 1,
    ///  Merge the fields into the existing entities.
    Merge = 2,
    ///  Inserts the properties by names.
    InsertPropertiesByNames = 3,
    ///  Replace the properties by names.
    ReplacePropertiesByNames = 4,
    ///  Delete the properties by names.
    DeletePropertiesByNames = 5,
}
impl UpdateType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UpdateType::Unspecified => "UPDATE_TYPE_UNSPECIFIED",
            UpdateType::Replace => "UPDATE_TYPE_REPLACE",
            UpdateType::Merge => "UPDATE_TYPE_MERGE",
            UpdateType::InsertPropertiesByNames => "UPDATE_TYPE_INSERT_PROPERTIES_BY_NAMES",
            UpdateType::ReplacePropertiesByNames => "UPDATE_TYPE_REPLACE_PROPERTIES_BY_NAMES",
            UpdateType::DeletePropertiesByNames => "UPDATE_TYPE_DELETE_PROPERTIES_BY_NAMES",
        }
    }
}
///  Type of database used by the customer
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DatabaseType {
    ///  This value is required by protobuf best practices
    DbUnknown = 0,
    ///  Internal Spanner
    DbInfraSpanner = 1,
    ///  Cloud Sql with a Postgres Sql instance
    DbCloudSqlPostgres = 2,
}
impl DatabaseType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DatabaseType::DbUnknown => "DB_UNKNOWN",
            DatabaseType::DbInfraSpanner => "DB_INFRA_SPANNER",
            DatabaseType::DbCloudSqlPostgres => "DB_CLOUD_SQL_POSTGRES",
        }
    }
}
///  Access Control Mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccessControlMode {
    ///  This value is required by protobuf best practices
    AclModeUnknown = 0,
    ///  Universal Access: No document level access control.
    AclModeUniversalAccess = 1,
    ///  Document level access control with customer own Identity Service.
    AclModeDocumentLevelAccessControlByoid = 2,
    ///  Document level access control using Google Cloud Identity.
    AclModeDocumentLevelAccessControlGci = 3,
}
impl AccessControlMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccessControlMode::AclModeUnknown => "ACL_MODE_UNKNOWN",
            AccessControlMode::AclModeUniversalAccess => "ACL_MODE_UNIVERSAL_ACCESS",
            AccessControlMode::AclModeDocumentLevelAccessControlByoid => "ACL_MODE_DOCUMENT_LEVEL_ACCESS_CONTROL_BYOID",
            AccessControlMode::AclModeDocumentLevelAccessControlGci => "ACL_MODE_DOCUMENT_LEVEL_ACCESS_CONTROL_GCI",
        }
    }
}
///  Defines the structure for content warehouse document proto.
///
///  Next ID: 20
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Document {
    ///  The resource name of the document.
    ///  Format:
    ///  projects/{project_number}/locations/{location}/documents/{document_id}.
    ///
    ///  The name is ignored when creating a document.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  The reference ID set by customers. Must be unique per project and location.
    #[prost(string, tag="11")]
    pub reference_id: ::prost::alloc::string::String,
    ///  Required. Display name of the document given by the user. This name will be displayed
    ///  in the UI.
    ///  Customer can populate this field with the name of the document. This
    ///  differs from the 'title' field as 'title' is optional and stores the top
    ///  heading in the document.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    ///  Title that describes the document.
    ///  This is usually present in the top section of the document, and is a
    ///  mandatory field for the question-answering feature.
    #[prost(string, tag="18")]
    pub title: ::prost::alloc::string::String,
    ///  Uri to display the document, for example, in the UI.
    #[prost(string, tag="17")]
    pub display_uri: ::prost::alloc::string::String,
    ///  The Document schema name.
    ///  Format:
    ///  projects/{project_number}/locations/{location}/documentSchemas/{document_schema_id}.
    #[prost(string, tag="3")]
    pub document_schema_name: ::prost::alloc::string::String,
    ///  A path linked to structured content file.
    #[prost(string, tag="16")]
    pub structured_content_uri: ::prost::alloc::string::String,
    ///  List of values that are user supplied metadata.
    #[prost(message, repeated, tag="7")]
    pub properties: ::prost::alloc::vec::Vec<Property>,
    ///  Output only. The time when the document is last updated.
    #[prost(message, optional, tag="8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The time when the document is created.
    #[prost(message, optional, tag="9")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  This is used when DocAI was not used to load the document and parsing/
    ///  extracting is needed for the inline_raw_document.  For example, if
    ///  inline_raw_document is the byte representation of a PDF file, then
    ///  this should be set to: RAW_DOCUMENT_FILE_TYPE_PDF.
    #[prost(enumeration="RawDocumentFileType", tag="10")]
    pub raw_document_file_type: i32,
    ///  If true, makes the document visible to asynchronous policies and rules.
    #[prost(bool, tag="12")]
    pub async_enabled: bool,
    ///  If true, text extraction will not be performed.
    #[prost(bool, tag="19")]
    pub text_extraction_disabled: bool,
    ///  The user who creates the document.
    #[prost(string, tag="13")]
    pub creator: ::prost::alloc::string::String,
    ///  The user who lastly updates the document.
    #[prost(string, tag="14")]
    pub updater: ::prost::alloc::string::String,
    #[prost(oneof="document::StructuredContent", tags="15, 4")]
    pub structured_content: ::core::option::Option<document::StructuredContent>,
    ///  Raw document file.
    #[prost(oneof="document::RawDocument", tags="5, 6")]
    pub raw_document: ::core::option::Option<document::RawDocument>,
}
/// Nested message and enum types in `Document`.
pub mod document {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StructuredContent {
        ///  Other document format, such as PPTX, XLXS
        #[prost(string, tag="15")]
        PlainText(::prost::alloc::string::String),
        ///  Document AI format to save the structured content, including OCR.
        #[prost(message, tag="4")]
        CloudAiDocument(super::super::super::documentai::v1::Document),
    }
    ///  Raw document file.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RawDocument {
        ///  Raw document file in Cloud Storage path.
        #[prost(string, tag="5")]
        RawDocumentPath(::prost::alloc::string::String),
        ///  Raw document content.
        #[prost(bytes, tag="6")]
        InlineRawDocument(::prost::alloc::vec::Vec<u8>),
    }
}
///  References to the documents.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentReference {
    ///  Required. Name of the referenced document.
    #[prost(string, tag="1")]
    pub document_name: ::prost::alloc::string::String,
    ///  display_name of the referenced document; this name does not need to be
    ///  consistent to the display_name in the Document proto, depending on the ACL
    ///  constraint.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    ///  Stores the subset of the referenced document's content.
    ///  This is useful to allow user peek the information of the referenced
    ///  document.
    #[prost(string, tag="3")]
    pub snippet: ::prost::alloc::string::String,
    ///  The document type of the document being referenced.
    #[prost(bool, tag="4")]
    pub document_is_folder: bool,
    ///  Output only. The time when the document is last updated.
    #[prost(message, optional, tag="5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The time when the document is created.
    #[prost(message, optional, tag="6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The time when the document is deleted.
    #[prost(message, optional, tag="7")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
}
///  Property of a document.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Property {
    ///  Required. Must match the name of a PropertyDefinition in the DocumentSchema.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Type of the property.
    ///  Must match the property_options type of the matching PropertyDefinition.
    ///  Value of the Property parsed into a specific data type.
    ///  Specific type value(s) obtained from Document AIs Property.mention_text
    ///  field.
    #[prost(oneof="property::Values", tags="2, 3, 4, 5, 6, 7, 8, 9, 10")]
    pub values: ::core::option::Option<property::Values>,
}
/// Nested message and enum types in `Property`.
pub mod property {
    ///  Type of the property.
    ///  Must match the property_options type of the matching PropertyDefinition.
    ///  Value of the Property parsed into a specific data type.
    ///  Specific type value(s) obtained from Document AIs Property.mention_text
    ///  field.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Values {
        ///  Integer property values.
        #[prost(message, tag="2")]
        IntegerValues(super::IntegerArray),
        ///  Float property values.
        #[prost(message, tag="3")]
        FloatValues(super::FloatArray),
        ///  String/text property values.
        #[prost(message, tag="4")]
        TextValues(super::TextArray),
        ///  Enum property values.
        #[prost(message, tag="5")]
        EnumValues(super::EnumArray),
        ///  Nested structured data property values.
        #[prost(message, tag="6")]
        PropertyValues(super::PropertyArray),
        ///  Date time property values.
        ///  It is not supported by CMEK compliant deployment.
        #[prost(message, tag="7")]
        DateTimeValues(super::DateTimeArray),
        ///  Map property values.
        #[prost(message, tag="8")]
        MapProperty(super::MapProperty),
        ///  Timestamp property values.
        ///  It is not supported by CMEK compliant deployment.
        #[prost(message, tag="9")]
        TimestampValues(super::TimestampArray),
        ///  Boolean property values.
        ///  It is not supported by CMEK compliant deployment.
        #[prost(message, tag="10")]
        BooleanValues(super::BooleanArray),
    }
}
///  Integer values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntegerArray {
    ///  List of integer values.
    #[prost(int32, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<i32>,
}
///  Float values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatArray {
    ///  List of float values.
    #[prost(float, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<f32>,
}
///  String/text values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextArray {
    ///  List of text values.
    #[prost(string, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///  Enum values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumArray {
    ///  List of enum values.
    #[prost(string, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///  DateTime values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateTimeArray {
    ///  List of datetime values.
    ///  Both OffsetDateTime and ZonedDateTime are supported.
    #[prost(message, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<super::super::super::r#type::DateTime>,
}
///  Timestamp values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampArray {
    ///  List of timestamp values.
    #[prost(message, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<TimestampValue>,
}
///  Boolean values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BooleanArray {
    ///  List of bool values.
    #[prost(bool, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<bool>,
}
///  Timestamp value type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampValue {
    #[prost(oneof="timestamp_value::Value", tags="1, 2")]
    pub value: ::core::option::Option<timestamp_value::Value>,
}
/// Nested message and enum types in `TimestampValue`.
pub mod timestamp_value {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        ///  Timestamp value
        #[prost(message, tag="1")]
        TimestampValue(::prost_types::Timestamp),
        ///  The string must represent a valid instant in UTC and is parsed using
        ///  java.time.format.DateTimeFormatter.ISO_INSTANT.
        ///  e.g. "2013-09-29T18:46:19Z"
        #[prost(string, tag="2")]
        TextValue(::prost::alloc::string::String),
    }
}
///  Property values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertyArray {
    ///  List of property values.
    #[prost(message, repeated, tag="1")]
    pub properties: ::prost::alloc::vec::Vec<Property>,
}
///  Map property value.
///  Represents a structured entries of key value pairs, consisting of field names
///  which map to dynamically typed values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapProperty {
    ///  Unordered map of dynamically typed values.
    #[prost(map="string, message", tag="1")]
    pub fields: ::std::collections::HashMap<::prost::alloc::string::String, Value>,
}
///  `Value` represents a dynamically typed value which can be either be
///  a float, a integer, a string, or a datetime value. A producer of value is
///  expected to set one of these variants. Absence of any variant indicates an
///  error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    ///  The kind of value.
    #[prost(oneof="value::Kind", tags="1, 2, 3, 4, 5, 6, 7")]
    pub kind: ::core::option::Option<value::Kind>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    ///  The kind of value.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        ///  Represents a float value.
        #[prost(float, tag="1")]
        FloatValue(f32),
        ///  Represents a integer value.
        #[prost(int32, tag="2")]
        IntValue(i32),
        ///  Represents a string value.
        #[prost(string, tag="3")]
        StringValue(::prost::alloc::string::String),
        ///  Represents an enum value.
        #[prost(message, tag="4")]
        EnumValue(super::EnumValue),
        ///  Represents a datetime value.
        #[prost(message, tag="5")]
        DatetimeValue(super::super::super::super::r#type::DateTime),
        ///  Represents a timestamp value.
        #[prost(message, tag="6")]
        TimestampValue(super::TimestampValue),
        ///  Represents a boolean value.
        #[prost(bool, tag="7")]
        BooleanValue(bool),
    }
}
///  Represents the string value of the enum field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumValue {
    ///  String value of the enum field. This must match defined set of enums
    ///  in document schema using EnumTypeOptions.
    #[prost(string, tag="1")]
    pub value: ::prost::alloc::string::String,
}
///  When a raw document is supplied, this indicates the file format
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RawDocumentFileType {
    ///  No raw document specified or it is non-parsable
    Unspecified = 0,
    ///  Adobe PDF format
    Pdf = 1,
    ///  Microsoft Word format
    Docx = 2,
    ///  Microsoft Excel format
    Xlsx = 3,
    ///  Microsoft Powerpoint format
    Pptx = 4,
    ///  UTF-8 encoded text format
    Text = 5,
}
impl RawDocumentFileType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RawDocumentFileType::Unspecified => "RAW_DOCUMENT_FILE_TYPE_UNSPECIFIED",
            RawDocumentFileType::Pdf => "RAW_DOCUMENT_FILE_TYPE_PDF",
            RawDocumentFileType::Docx => "RAW_DOCUMENT_FILE_TYPE_DOCX",
            RawDocumentFileType::Xlsx => "RAW_DOCUMENT_FILE_TYPE_XLSX",
            RawDocumentFileType::Pptx => "RAW_DOCUMENT_FILE_TYPE_PPTX",
            RawDocumentFileType::Text => "RAW_DOCUMENT_FILE_TYPE_TEXT",
        }
    }
}
///  Response message for DocumentLinkService.ListLinkedTargets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLinkedTargetsResponse {
    ///  Target document-links.
    #[prost(message, repeated, tag="1")]
    pub document_links: ::prost::alloc::vec::Vec<DocumentLink>,
    ///  A token, which can be sent as `page_token` to retrieve the next page.
    ///  If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  Request message for DocumentLinkService.ListLinkedTargets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLinkedTargetsRequest {
    ///  Required. The name of the document, for which all target links are returned.
    ///  Format:
    ///  projects/{project_number}/locations/{location}/documents/{target_document_id}.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  The meta information collected about the document creator, used to enforce
    ///  access control for the service.
    #[prost(message, optional, tag="2")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
}
///  Response message for DocumentLinkService.ListLinkedSources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLinkedSourcesResponse {
    ///  Source document-links.
    #[prost(message, repeated, tag="1")]
    pub document_links: ::prost::alloc::vec::Vec<DocumentLink>,
    ///  A token, which can be sent as `page_token` to retrieve the next page.
    ///  If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  Response message for DocumentLinkService.ListLinkedSources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLinkedSourcesRequest {
    ///  Required. The name of the document, for which all source links are returned.
    ///  Format:
    ///  projects/{project_number}/locations/{location}/documents/{source_document_id}.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  The maximum number of document-links to return. The service may return
    ///  fewer than this value.
    ///
    ///  If unspecified, at most 50 document-links will be returned.
    ///  The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    ///  A page token, received from a previous `ListLinkedSources` call.
    ///  Provide this to retrieve the subsequent page.
    ///
    ///  When paginating, all other parameters provided to `ListLinkedSources`
    ///  must match the call that provided the page token.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
    ///  The meta information collected about the document creator, used to enforce
    ///  access control for the service.
    #[prost(message, optional, tag="2")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
}
///  A document-link between source and target document.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentLink {
    ///  Name of this document-link.
    ///  It is required that the parent derived form the name to be consistent with
    ///  the source document reference. Otherwise an exception will be thrown.
    ///  Format:
    ///  projects/{project_number}/locations/{location}/documents/{source_document_id}/documentLinks/{document_link_id}.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Document references of the source document.
    #[prost(message, optional, tag="2")]
    pub source_document_reference: ::core::option::Option<DocumentReference>,
    ///  Document references of the target document.
    #[prost(message, optional, tag="3")]
    pub target_document_reference: ::core::option::Option<DocumentReference>,
    ///  Description of this document-link.
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    ///  Output only. The time when the documentLink is last updated.
    #[prost(message, optional, tag="5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The time when the documentLink is created.
    #[prost(message, optional, tag="6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  The state of the documentlink. If target node has been deleted, the
    ///  link is marked as invalid. Removing a source node will result in removal
    ///  of all associated links.
    #[prost(enumeration="document_link::State", tag="7")]
    pub state: i32,
}
/// Nested message and enum types in `DocumentLink`.
pub mod document_link {
    ///  The state of a document-link.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        ///  Unknown state of documentlink.
        Unspecified = 0,
        ///  The documentlink has both source and target documents detected.
        Active = 1,
        ///  Target document is deleted, and mark the documentlink as soft-deleted.
        SoftDeleted = 2,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Active => "ACTIVE",
                State::SoftDeleted => "SOFT_DELETED",
            }
        }
    }
}
///  Request message for DocumentLinkService.CreateDocumentLink.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDocumentLinkRequest {
    ///  Required. Parent of the document-link to be created.
    ///  parent of document-link should be a document.
    ///  Format:
    ///  projects/{project_number}/locations/{location}/documents/{source_document_id}.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Required. Document links associated with the source documents (source_document_id).
    #[prost(message, optional, tag="2")]
    pub document_link: ::core::option::Option<DocumentLink>,
    ///  The meta information collected about the document creator, used to enforce
    ///  access control for the service.
    #[prost(message, optional, tag="3")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
}
///  Request message for DocumentLinkService.DeleteDocumentLink.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDocumentLinkRequest {
    ///  Required. The name of the document-link to be deleted.
    ///  Format:
    ///  projects/{project_number}/locations/{location}/documents/{source_document_id}/documentLinks/{document_link_id}.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  The meta information collected about the document creator, used to enforce
    ///  access control for the service.
    #[prost(message, optional, tag="2")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
}
/// Generated client implementations.
pub mod document_link_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// This service lets you manage document-links.
    /// Document-Links are treated as sub-resources under source documents.
    #[derive(Debug, Clone)]
    pub struct DocumentLinkServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DocumentLinkServiceClient<tonic::transport::Channel> {
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
    impl<T> DocumentLinkServiceClient<T>
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
        ) -> DocumentLinkServiceClient<InterceptedService<T, F>>
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
            DocumentLinkServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Return all target document-links from the document.
        pub async fn list_linked_targets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLinkedTargetsRequest>,
        ) -> Result<tonic::Response<super::ListLinkedTargetsResponse>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.DocumentLinkService/ListLinkedTargets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Return all source document-links from the document.
        pub async fn list_linked_sources(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLinkedSourcesRequest>,
        ) -> Result<tonic::Response<super::ListLinkedSourcesResponse>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.DocumentLinkService/ListLinkedSources",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Create a link between a source document and a target document.
        pub async fn create_document_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDocumentLinkRequest>,
        ) -> Result<tonic::Response<super::DocumentLink>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.DocumentLinkService/CreateDocumentLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Remove the link between the source and target documents.
        pub async fn delete_document_link(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDocumentLinkRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.DocumentLinkService/DeleteDocumentLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
///  A document schema used to define document structure.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentSchema {
    ///  The resource name of the document schema.
    ///  Format:
    ///  projects/{project_number}/locations/{location}/documentSchemas/{document_schema_id}.
    ///
    ///  The name is ignored when creating a document schema.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Required. Name of the schema given by the user. Must be unique per customer.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    ///  Document details.
    #[prost(message, repeated, tag="3")]
    pub property_definitions: ::prost::alloc::vec::Vec<PropertyDefinition>,
    ///  Document Type, true refers the document is a folder, otherwise it is
    ///  a typical document.
    #[prost(bool, tag="4")]
    pub document_is_folder: bool,
    ///  Output only. The time when the document schema is last updated.
    #[prost(message, optional, tag="5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The time when the document schema is created.
    #[prost(message, optional, tag="6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Schema description.
    #[prost(string, tag="7")]
    pub description: ::prost::alloc::string::String,
}
///  Defines the metadata for a schema property.
///
///  Next ID: 18
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertyDefinition {
    ///  Required. The name of the metadata property.
    ///  Must be unique within a document schema and is case insensitive.
    ///  Names must be non-blank, start with a letter, and can contain alphanumeric
    ///  characters and: /, :, -, _, and .
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  The display-name for the property, used for front-end.
    #[prost(string, tag="12")]
    pub display_name: ::prost::alloc::string::String,
    ///  Whether the property can have multiple values.
    #[prost(bool, tag="2")]
    pub is_repeatable: bool,
    ///  Whether the property can be filtered. If this is a sub-property, all the
    ///  parent properties must be marked filterable.
    #[prost(bool, tag="3")]
    pub is_filterable: bool,
    ///  Indicates that the property should be included in a global search.
    #[prost(bool, tag="4")]
    pub is_searchable: bool,
    ///  Whether the property is user supplied metadata.
    #[prost(bool, tag="5")]
    pub is_metadata: bool,
    ///  Whether the property is mandatory.
    ///  Default is 'false', i.e. populating property value can be skipped.
    ///  If 'true' then user must populate the value for this property.
    #[prost(bool, tag="14")]
    pub is_required: bool,
    ///  Type of the property.
    #[prost(oneof="property_definition::ValueTypeOptions", tags="7, 8, 9, 10, 11, 13, 15, 16, 17")]
    pub value_type_options: ::core::option::Option<property_definition::ValueTypeOptions>,
}
/// Nested message and enum types in `PropertyDefinition`.
pub mod property_definition {
    ///  Type of the property.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ValueTypeOptions {
        ///  Integer property.
        #[prost(message, tag="7")]
        IntegerTypeOptions(super::IntegerTypeOptions),
        ///  Float property.
        #[prost(message, tag="8")]
        FloatTypeOptions(super::FloatTypeOptions),
        ///  Text/string property.
        #[prost(message, tag="9")]
        TextTypeOptions(super::TextTypeOptions),
        ///  Nested structured data property.
        #[prost(message, tag="10")]
        PropertyTypeOptions(super::PropertyTypeOptions),
        ///  Enum/categorical property.
        #[prost(message, tag="11")]
        EnumTypeOptions(super::EnumTypeOptions),
        ///  Date time property.
        ///  It is not supported by CMEK compliant deployment.
        #[prost(message, tag="13")]
        DateTimeTypeOptions(super::DateTimeTypeOptions),
        ///  Map property.
        #[prost(message, tag="15")]
        MapTypeOptions(super::MapTypeOptions),
        ///  Timestamp property.
        ///  It is not supported by CMEK compliant deployment.
        #[prost(message, tag="16")]
        TimestampTypeOptions(super::TimestampTypeOptions),
        ///  Boolean property.
        ///  It is not supported by CMEK compliant deployment.
        #[prost(message, tag="17")]
        BooleanTypeOptions(super::BooleanTypeOptions),
    }
}
///  Configurations for an integer property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntegerTypeOptions {
}
///  Configurations for a float property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatTypeOptions {
}
///  Configurations for a text property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextTypeOptions {
}
///  Configurations for a date time property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateTimeTypeOptions {
}
///  Configurations for a Map property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapTypeOptions {
}
///  Configurations for a timestamp property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampTypeOptions {
}
///  Configurations for a boolean property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BooleanTypeOptions {
}
///  Configurations for a nested structured data property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertyTypeOptions {
    ///  Required. List of property definitions.
    #[prost(message, repeated, tag="1")]
    pub property_definitions: ::prost::alloc::vec::Vec<PropertyDefinition>,
}
///  Configurations for an enum/categorical property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumTypeOptions {
    ///  Required. List of possible enum values.
    #[prost(string, repeated, tag="1")]
    pub possible_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///  Request message for DocumentSchemaService.CreateDocumentSchema.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDocumentSchemaRequest {
    ///  Required. The parent name.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Required. The document schema to create.
    #[prost(message, optional, tag="2")]
    pub document_schema: ::core::option::Option<DocumentSchema>,
}
///  Request message for DocumentSchemaService.GetDocumentSchema.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDocumentSchemaRequest {
    ///  Required. The name of the document schema to retrieve.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Request message for DocumentSchemaService.UpdateDocumentSchema.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDocumentSchemaRequest {
    ///  Required. The name of the document schema to update.
    ///  Format:
    ///  projects/{project_number}/locations/{location}/documentSchemas/{document_schema_id}.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Required. The document schema to update with.
    #[prost(message, optional, tag="2")]
    pub document_schema: ::core::option::Option<DocumentSchema>,
}
///  Request message for DocumentSchemaService.DeleteDocumentSchema.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDocumentSchemaRequest {
    ///  Required. The name of the document schema to delete.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Request message for DocumentSchemaService.ListDocumentSchemas.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDocumentSchemasRequest {
    ///  Required. The parent, which owns this collection of document schemas.
    ///  Format: projects/{project_number}/locations/{location}.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  The maximum number of document schemas to return. The service may return
    ///  fewer than this value.
    ///  If unspecified, at most 50 document schemas will be returned.
    ///  The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    ///  A page token, received from a previous `ListDocumentSchemas` call.
    ///  Provide this to retrieve the subsequent page.
    ///
    ///  When paginating, all other parameters provided to `ListDocumentSchemas`
    ///  must match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
///  Response message for DocumentSchemaService.ListDocumentSchemas.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDocumentSchemasResponse {
    ///  The document schemas from the specified parent.
    #[prost(message, repeated, tag="1")]
    pub document_schemas: ::prost::alloc::vec::Vec<DocumentSchema>,
    ///  A token, which can be sent as `page_token` to retrieve the next page.
    ///  If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod document_schema_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct DocumentSchemaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DocumentSchemaServiceClient<tonic::transport::Channel> {
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
    impl<T> DocumentSchemaServiceClient<T>
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
        ) -> DocumentSchemaServiceClient<InterceptedService<T, F>>
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
            DocumentSchemaServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a document schema.
        pub async fn create_document_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDocumentSchemaRequest>,
        ) -> Result<tonic::Response<super::DocumentSchema>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.DocumentSchemaService/CreateDocumentSchema",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a Document Schema. Returns INVALID_ARGUMENT if the name of the
        /// Document Schema is non-empty and does not equal the existing name.
        /// Supports only appending new properties and updating existing properties
        /// will result into INVALID_ARGUMENT.
        pub async fn update_document_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDocumentSchemaRequest>,
        ) -> Result<tonic::Response<super::DocumentSchema>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.DocumentSchemaService/UpdateDocumentSchema",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a document schema. Returns NOT_FOUND if the document schema does not
        /// exist.
        pub async fn get_document_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDocumentSchemaRequest>,
        ) -> Result<tonic::Response<super::DocumentSchema>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.DocumentSchemaService/GetDocumentSchema",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a document schema. Returns NOT_FOUND if the document schema does
        /// not exist.
        pub async fn delete_document_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDocumentSchemaRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.DocumentSchemaService/DeleteDocumentSchema",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists document schemas.
        pub async fn list_document_schemas(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDocumentSchemasRequest>,
        ) -> Result<tonic::Response<super::ListDocumentSchemasResponse>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.DocumentSchemaService/ListDocumentSchemas",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
///  NEXT_ID: 13
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentQuery {
    ///  The query string that matches against the full text of the document and
    ///  the searchable properties.
    ///  The maximum number of allowed characters is 255.
    #[prost(string, tag="1")]
    pub query: ::prost::alloc::string::String,
    ///  Experimental, do not use.
    ///  If the query is a natural language question. False by default. If true,
    ///  then the question-answering feature will be used instead of search, and
    ///  `result_count` in \[SearchDocumentsRequest][google.cloud.contentwarehouse.v1.SearchDocumentsRequest\] must be set. In addition, all
    ///  other input fields related to search (pagination, histograms, etc.) will be
    ///  ignored.
    #[prost(bool, tag="12")]
    pub is_nl_query: bool,
    ///  This filter specifies a structured syntax to match against the
    ///  \[PropertyDefinition].[is_filterable][\] marked as `true`. The syntax for
    ///  this expression is a subset of SQL syntax.
    ///
    ///  Supported operators are: `=`, `!=`, `<`, `<=`, `>`, and `>=` where the left
    ///  of the operator is a property name and the right of the operator is a
    ///  number or a quoted string. You must escape backslash (\\) and quote (\")
    ///  characters. Supported functions are `LOWER(\[property_name\])` to perform a
    ///  case insensitive match and `EMPTY(\[property_name\])` to filter on the
    ///  existence of a key.
    ///
    ///  Boolean expressions (AND/OR/NOT) are supported up to 3 levels of nesting
    ///  (for example, "((A AND B AND C) OR NOT D) AND E"), a maximum of 100
    ///  comparisons or functions are allowed in the expression. The expression must
    ///  be < 6000 bytes in length.
    ///
    ///  Sample Query:
    ///  `(LOWER(driving_license)="class \"a\"" OR EMPTY(driving_license)) AND
    ///  driving_years > 10`
    #[deprecated]
    #[prost(string, tag="4")]
    pub custom_property_filter: ::prost::alloc::string::String,
    ///  Documents created/updated within a range specified by this filter are
    ///  searched against.
    #[prost(message, repeated, tag="5")]
    pub time_filters: ::prost::alloc::vec::Vec<TimeFilter>,
    ///  This filter specifies the exact document schema
    ///  \[Document.document_schema_name][google.cloud.contentwarehouse.v1.Document.document_schema_name\] of the documents to search against.
    ///
    ///  If a value isn't specified, documents within the search results are
    ///  associated with any schema. If multiple values are specified, documents
    ///  within the search results may be associated with any of the specified
    ///  schemas.
    ///
    ///  At most 20 document schema names are allowed.
    #[prost(string, repeated, tag="6")]
    pub document_schema_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  This filter specifies a structured syntax to match against the
    ///  \[PropertyDefinition.is_filterable][google.cloud.contentwarehouse.v1.PropertyDefinition.is_filterable\] marked as `true`. The relationship
    ///  between the PropertyFilters is OR.
    #[prost(message, repeated, tag="7")]
    pub property_filter: ::prost::alloc::vec::Vec<PropertyFilter>,
    ///  This filter specifies the types of files to return: ALL, FOLDER, or FILE.
    ///  If FOLDER or FILE is specified, then only either folders or files will be
    ///  returned, respectively. If ALL is specified, both folders and files will be
    ///  returned.
    ///
    ///  If no value is specified, ALL files will be returned.
    #[prost(message, optional, tag="8")]
    pub file_type_filter: ::core::option::Option<FileTypeFilter>,
    ///  Search all the documents under this specified folder.
    ///  Format:
    ///  projects/{project_number}/locations/{location}/documents/{document_id}.
    #[prost(string, tag="9")]
    pub folder_name_filter: ::prost::alloc::string::String,
    ///  For custom synonyms.
    ///  Customers provide the synonyms based on context. One customer can provide
    ///  multiple set of synonyms based on different context. The search query will
    ///  be expanded based on the custom synonyms of the query context set.
    ///  By default, no custom synonyms wll be applied if no query context is
    ///  provided.
    ///  It is not supported for CMEK compliant deployment.
    #[prost(string, repeated, tag="10")]
    pub query_context: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  The exact creator(s) of the documents to search against.
    ///
    ///  If a value isn't specified, documents within the search results are
    ///  associated with any creator. If multiple values are specified, documents
    ///  within the search results may be associated with any of the specified
    ///  creators.
    #[prost(string, repeated, tag="11")]
    pub document_creator_filter: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///  Filter on create timestamp or update timestamp of documents.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeFilter {
    #[prost(message, optional, tag="1")]
    pub time_range: ::core::option::Option<super::super::super::r#type::Interval>,
    ///  Specifies which time field to filter documents on.
    ///
    ///  Defaults to \[TimeField.UPLOAD_TIME][\].
    #[prost(enumeration="time_filter::TimeField", tag="2")]
    pub time_field: i32,
}
/// Nested message and enum types in `TimeFilter`.
pub mod time_filter {
    ///  Time field used in TimeFilter.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TimeField {
        ///  Default value.
        Unspecified = 0,
        ///  Earliest document create time.
        CreateTime = 1,
        ///  Latest document update time.
        UpdateTime = 2,
    }
    impl TimeField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TimeField::Unspecified => "TIME_FIELD_UNSPECIFIED",
                TimeField::CreateTime => "CREATE_TIME",
                TimeField::UpdateTime => "UPDATE_TIME",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertyFilter {
    ///  The Document schema name \[Document.document_schema_name][google.cloud.contentwarehouse.v1.Document.document_schema_name\].
    ///  Format:
    ///  projects/{project_number}/locations/{location}/documentSchemas/{document_schema_id}.
    #[prost(string, tag="1")]
    pub document_schema_name: ::prost::alloc::string::String,
    ///  The filter condition.
    ///  The syntax for this expression is a subset of SQL syntax.
    ///
    ///  Supported operators are: `=`, `!=`, `<`, `<=`, `>`, `>=`, and `~~` where
    ///  the left of the operator is a property name and the right of the operator
    ///  is a number or a quoted string. You must escape backslash (\\) and quote
    ///  (\") characters.
    ///
    ///  `~~` is the LIKE operator. The right of the operator must be a string. The
    ///  only supported property data type for LIKE is text_values. It provides
    ///  semantic search functionality by parsing, stemming and doing synonyms
    ///  expansion against the input query. It matches if the property contains
    ///  semantic similar content to the query. It is not regex matching or wildcard
    ///  matching. For example, "property.company ~~ \"google\"" will match records
    ///  whose property `property.compnay` have values like "Google Inc.", "Google
    ///  LLC" or "Google Company".
    ///
    ///  Supported functions are `LOWER(\[property_name\])` to perform a
    ///  case insensitive match and `EMPTY(\[property_name\])` to filter on the
    ///  existence of a key.
    ///
    ///  Boolean expressions (AND/OR/NOT) are supported up to 3 levels of nesting
    ///  (for example, "((A AND B AND C) OR NOT D) AND E"), a maximum of 100
    ///  comparisons or functions are allowed in the expression. The expression must
    ///  be < 6000 bytes in length.
    ///
    ///  Only properties that are marked filterable are allowed
    ///  (\[PropertyDefinition.is_filterable][google.cloud.contentwarehouse.v1.PropertyDefinition.is_filterable\]). Property names do not need to be
    ///  prefixed by the document schema id (as is the case with histograms),
    ///  however property names will need to be prefixed by its parent hierarchy, if
    ///  any.  For example: top_property_name.sub_property_name.
    ///
    ///  Sample Query:
    ///  `(LOWER(driving_license)="class \"a\"" OR EMPTY(driving_license)) AND
    ///  driving_years > 10`
    ///
    ///
    ///  CMEK compliant deployment only supports:
    ///
    ///  * Operators: `=`, `<`, `<=`, `>`, and `>=`.
    ///  * Boolean expressions: AND and OR.
    #[prost(string, tag="2")]
    pub condition: ::prost::alloc::string::String,
}
///  Filter for the specific types of documents returned.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileTypeFilter {
    ///  The type of files to return.
    #[prost(enumeration="file_type_filter::FileType", tag="1")]
    pub file_type: i32,
}
/// Nested message and enum types in `FileTypeFilter`.
pub mod file_type_filter {
    ///  Representation of the types of files.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FileType {
        ///  Default document type. If set, disables the filter.
        Unspecified = 0,
        ///  Returns all document types, including folders.
        All = 1,
        ///  Returns only folders.
        Folder = 2,
        ///  Returns only non-folder documents.
        Document = 3,
    }
    impl FileType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FileType::Unspecified => "FILE_TYPE_UNSPECIFIED",
                FileType::All => "ALL",
                FileType::Folder => "FOLDER",
                FileType::Document => "DOCUMENT",
            }
        }
    }
}
///  The histogram request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistogramQuery {
    ///  An expression specifies a histogram request against matching documents for
    ///  searches.
    ///
    ///  See \[SearchDocumentsRequest.histogram_queries][google.cloud.contentwarehouse.v1.SearchDocumentsRequest.histogram_queries\] for details about syntax.
    #[prost(string, tag="1")]
    pub histogram_query: ::prost::alloc::string::String,
    ///  Controls if the histogram query requires the return of a precise count.
    ///  Enable this flag may adversely impact performance.
    ///
    ///  Defaults to true.
    #[prost(bool, tag="2")]
    pub require_precise_result_size: bool,
    ///  Optional. Filter the result of histogram query by the property names. It only works
    ///  with histogram query count('FilterableProperties').
    ///  It is an optional. It will perform histogram on all the property names for
    ///  all the document schemas. Setting this field will have a better
    ///  performance.
    #[prost(message, optional, tag="3")]
    pub filters: ::core::option::Option<HistogramQueryPropertyNameFilter>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistogramQueryPropertyNameFilter {
    ///  This filter specifies the exact document schema(s)
    ///  \[Document.document_schema_name][google.cloud.contentwarehouse.v1.Document.document_schema_name\] to run histogram query against.
    ///  It is optional. It will perform histogram for property names for all the
    ///  document schemas if it is not set.
    ///
    ///  At most 10 document schema names are allowed.
    ///  Format:
    ///  projects/{project_number}/locations/{location}/documentSchemas/{document_schema_id}.
    #[prost(string, repeated, tag="1")]
    pub document_schemas: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  It is optional. It will perform histogram for all the property names if it
    ///  is not set.
    ///  The properties need to be defined with the is_filterable flag set to
    ///  true and the name of the property should be in the format:
    ///  "schemaId.propertyName". The property needs to be defined in the schema.
    ///  Example: the schema id is abc. Then the name of property for property
    ///  MORTGAGE_TYPE will be "abc.MORTGAGE_TYPE".
    #[prost(string, repeated, tag="2")]
    pub property_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  By default, the y_axis is HISTOGRAM_YAXIS_DOCUMENT if this field is not
    ///  set.
    #[prost(enumeration="histogram_query_property_name_filter::HistogramYAxis", tag="3")]
    pub y_axis: i32,
}
/// Nested message and enum types in `HistogramQueryPropertyNameFilter`.
pub mod histogram_query_property_name_filter {
    ///  The result of the histogram query count('FilterableProperties') using
    ///  HISTOGRAM_YAXIS_DOCUMENT will be:
    ///  invoice_id: 2
    ///  address: 1
    ///  payment_method: 2
    ///  line_item_description: 1
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HistogramYAxis {
        ///  Count the documents per property name.
        HistogramYaxisDocument = 0,
        ///  Count the properties per property name.
        HistogramYaxisProperty = 1,
    }
    impl HistogramYAxis {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HistogramYAxis::HistogramYaxisDocument => "HISTOGRAM_YAXIS_DOCUMENT",
                HistogramYAxis::HistogramYaxisProperty => "HISTOGRAM_YAXIS_PROPERTY",
            }
        }
    }
}
///  Histogram result that matches \[HistogramQuery][google.cloud.contentwarehouse.v1.HistogramQuery\] specified in searches.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistogramQueryResult {
    ///  Requested histogram expression.
    #[prost(string, tag="1")]
    pub histogram_query: ::prost::alloc::string::String,
    ///  A map from the values of the facet associated with distinct values to the
    ///  number of matching entries with corresponding value.
    ///
    ///  The key format is:
    ///
    ///  * (for string histogram) string values stored in the field.
    #[prost(map="string, int64", tag="2")]
    pub histogram: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
}
///  Request Option for processing Cloud AI Document in CW Document.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudAiDocumentOption {
    ///  Whether to convert all the entities to properties.
    #[prost(bool, tag="1")]
    pub enable_entities_conversions: bool,
    ///  If set, only selected entities will be converted to properties.
    #[prost(map="string, string", tag="2")]
    pub customized_entities_properties_conversions: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
///  Request message for DocumentService.CreateDocument.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDocumentRequest {
    ///  Required. The parent name.
    ///  Format: projects/{project_number}/locations/{location}.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Required. The document to create.
    #[prost(message, optional, tag="2")]
    pub document: ::core::option::Option<Document>,
    ///  The meta information collected about the end user, used to enforce access
    ///  control for the service.
    #[prost(message, optional, tag="3")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
    ///  Default document policy during creation. Conditions defined in the policy
    ///  will be ignored.
    #[prost(message, optional, tag="4")]
    pub policy: ::core::option::Option<super::super::super::iam::v1::Policy>,
    ///  Request Option for processing Cloud AI Document in CW Document.
    #[prost(message, optional, tag="5")]
    pub cloud_ai_document_option: ::core::option::Option<CloudAiDocumentOption>,
    ///  Field mask for creating Document fields. If mask path is empty,
    ///  it means all fields are masked.
    ///  For the `FieldMask` definition,
    ///  see
    ///  <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    #[prost(message, optional, tag="6")]
    pub create_mask: ::core::option::Option<::prost_types::FieldMask>,
}
///  Request message for DocumentService.GetDocument.
///
///  Next ID: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDocumentRequest {
    ///  Required. The name of the document to retrieve.
    ///  Format:
    ///  projects/{project_number}/locations/{location}/documents/{document_id} or
    ///  projects/{project_number}/locations/{location}/documents/referenceId/{reference_id}.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  The meta information collected about the end user, used to enforce access
    ///  control for the service.
    #[prost(message, optional, tag="2")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
}
///  Request message for DocumentService.UpdateDocument.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDocumentRequest {
    ///  Required. The name of the document to update.
    ///  Format:
    ///  projects/{project_number}/locations/{location}/documents/{document_id}
    ///  or
    ///  projects/{project_number}/locations/{location}/documents/referenceId/{reference_id}.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Required. The document to update.
    #[prost(message, optional, tag="2")]
    pub document: ::core::option::Option<Document>,
    ///  The meta information collected about the end user, used to enforce access
    ///  control for the service.
    #[prost(message, optional, tag="3")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
    ///  Request Option for processing Cloud AI Document in CW Document.
    #[prost(message, optional, tag="5")]
    pub cloud_ai_document_option: ::core::option::Option<CloudAiDocumentOption>,
    ///  Options for the update operation.
    #[prost(message, optional, tag="6")]
    pub update_options: ::core::option::Option<UpdateOptions>,
}
///  Request message for DocumentService.DeleteDocument.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDocumentRequest {
    ///  Required. The name of the document to delete.
    ///  Format:
    ///  projects/{project_number}/locations/{location}/documents/{document_id}
    ///  or
    ///  projects/{project_number}/locations/{location}/documents/referenceId/{reference_id}.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  The meta information collected about the end user, used to enforce access
    ///  control for the service.
    #[prost(message, optional, tag="2")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchDocumentsRequest {
    ///  Required. The parent, which owns this collection of documents.
    ///  Format: projects/{project_number}/locations/{location}.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  The meta information collected about the end user, used to enforce access
    ///  control and improve the search quality of the service.
    #[prost(message, optional, tag="3")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
    ///  Query used to search against documents (keyword, filters, etc.).
    #[prost(message, optional, tag="4")]
    pub document_query: ::core::option::Option<DocumentQuery>,
    ///  An integer that specifies the current offset (that is, starting result
    ///  location, amongst the documents deemed by the API as relevant) in search
    ///  results. This field is only considered if \[page_token][google.cloud.contentwarehouse.v1.SearchDocumentsRequest.page_token\] is unset.
    ///
    ///  The maximum allowed value is 5000. Otherwise an error is thrown.
    ///
    ///  For example, 0 means to  return results starting from the first matching
    ///  document, and 10 means to return from the 11th document. This can be used
    ///  for pagination, (for example, pageSize = 10 and offset = 10 means to return
    ///  from the second page).
    #[prost(int32, tag="5")]
    pub offset: i32,
    ///  A limit on the number of documents returned in the search results.
    ///  Increasing this value above the default value of 10 can increase search
    ///  response time. The value can be between 1 and 100.
    #[prost(int32, tag="6")]
    pub page_size: i32,
    ///  The token specifying the current offset within search results.
    ///  See \[SearchDocumentsResponse.next_page_token][google.cloud.contentwarehouse.v1.SearchDocumentsResponse.next_page_token\] for an explanation of how
    ///  to obtain the next set of query results.
    #[prost(string, tag="7")]
    pub page_token: ::prost::alloc::string::String,
    ///  The criteria determining how search results are sorted. For non-empty
    ///  query, default is `"relevance desc"`. For empty query, default is
    ///  `"upload_date desc"`.
    ///
    ///  Supported options are:
    ///
    ///  * `"relevance desc"`: By relevance descending, as determined by the API
    ///    algorithms.
    ///  * `"upload_date desc"`: By upload date descending.
    ///  * `"upload_date"`: By upload date ascending.
    ///  * `"update_date desc"`: By last updated date descending.
    ///  * `"update_date"`: By last updated date ascending.
    #[prost(string, tag="8")]
    pub order_by: ::prost::alloc::string::String,
    ///  An expression specifying a histogram request against matching
    ///  documents. Expression syntax is an aggregation function call with
    ///  histogram facets and other options.
    ///
    ///  The following aggregation functions are supported:
    ///
    ///  * `count(string_histogram_facet)`: Count the number of matching entities
    ///  for each distinct attribute value.
    ///
    ///  Data types:
    ///
    ///  * Histogram facet (aka filterable properties): Facet names with format
    ///  &lt;schema id&gt;.&lt;facet&gt;. Facets will have the
    ///  format of: \[a-zA-Z][a-zA-Z0-9_:/-.\]. If the facet is a child
    ///  facet, then the parent hierarchy needs to be specified separated by
    ///  dots in the prefix after the schema id. Thus, the format for a multi-
    ///  level facet is: &lt;schema id&gt;.&lt;parent facet name&gt;.
    ///  &lt;child facet name&gt;. Example:
    ///  schema123.root_parent_facet.middle_facet.child_facet
    ///  * DocumentSchemaId: (with no schema id prefix) to get
    ///  histograms for each document type (returns the schema id path, e.g.
    ///  projects/12345/locations/us-west/documentSchemas/abc123).
    ///
    ///  Example expression:
    ///
    ///  * Document type counts:
    ///    count('DocumentSchemaId')
    ///
    ///  * For schema id, abc123, get the counts for MORTGAGE_TYPE:
    ///    count('abc123.MORTGAGE_TYPE')
    #[prost(message, repeated, tag="9")]
    pub histogram_queries: ::prost::alloc::vec::Vec<HistogramQuery>,
    ///  Optional. Controls if the search document request requires the return of a total size
    ///  of matched documents. See \[SearchDocumentsResponse.total_size][google.cloud.contentwarehouse.v1.SearchDocumentsResponse.total_size\].
    ///
    ///  Enabling this flag may adversely impact performance.
    ///
    ///  Defaults to false.
    #[prost(bool, tag="10")]
    pub require_total_size: bool,
    ///  Experimental, do not use.
    ///  The limit on the number of documents returned for the question-answering
    ///  feature. To enable the question-answering feature, set
    ///  \[DocumentQuery].[is_nl_query][\] to true.
    #[prost(int32, tag="11")]
    pub qa_size_limit: i32,
}
///  Request message for DocumentService.FetchAcl
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchAclRequest {
    ///  Required. REQUIRED: The resource for which the policy is being requested.
    ///  Format for document:
    ///  projects/{project_number}/locations/{location}/documents/{document_id}.
    ///  Format for project: projects/{project_number}.
    #[prost(string, tag="1")]
    pub resource: ::prost::alloc::string::String,
    ///  The meta information collected about the end user, used to enforce access
    ///  control for the service.
    #[prost(message, optional, tag="2")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
    ///  For Get Project ACL only. Authorization check for end user will be ignored
    ///  when project_owner=true.
    #[prost(bool, tag="3")]
    pub project_owner: bool,
}
///  Request message for DocumentService.SetAcl.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAclRequest {
    ///  Required. REQUIRED: The resource for which the policy is being requested.
    ///  Format for document:
    ///  projects/{project_number}/locations/{location}/documents/{document_id}.
    ///  Format for project: projects/{project_number}.
    #[prost(string, tag="1")]
    pub resource: ::prost::alloc::string::String,
    ///  Required. REQUIRED: The complete policy to be applied to the `resource`. The size of
    ///  the policy is limited to a few 10s of KB.
    #[prost(message, optional, tag="2")]
    pub policy: ::core::option::Option<super::super::super::iam::v1::Policy>,
    ///  The meta information collected about the end user, used to enforce access
    ///  control for the service.
    #[prost(message, optional, tag="3")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
    ///  For Set Project ACL only. Authorization check for end user will be ignored
    ///  when project_owner=true.
    #[prost(bool, tag="4")]
    pub project_owner: bool,
}
///  Represents a set of rules from a single customer.
///  Next id: 9
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleSet {
    ///  The resource name of the rule set. Managed internally.
    ///  Format:
    ///  projects/{project_number}/locations/{location}/ruleSet/{rule_set_id}.
    ///
    ///  The name is ignored when creating a rule set.
    #[prost(string, tag="6")]
    pub name: ::prost::alloc::string::String,
    ///  Short description of the rule-set.
    #[prost(string, tag="1")]
    pub description: ::prost::alloc::string::String,
    ///  Source of the rules i.e., customer name.
    #[prost(string, tag="2")]
    pub source: ::prost::alloc::string::String,
    ///  List of rules given by the customer.
    #[prost(message, repeated, tag="3")]
    pub rules: ::prost::alloc::vec::Vec<Rule>,
    ///  Records the user defined time after which the rule-set will become active.
    #[prost(message, optional, tag="4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  The schedule with which the Ruleset is applied.
    ///
    ///  The presence of this field means that the ruleset is asynchronous.
    #[prost(enumeration="Schedule", tag="8")]
    pub schedule: i32,
    #[prost(oneof="rule_set::Expiration", tags="5, 7")]
    pub expiration: ::core::option::Option<rule_set::Expiration>,
}
/// Nested message and enum types in `RuleSet`.
pub mod rule_set {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Expiration {
        ///  The timestamp after which RuleSet will not be executed by Policy Engine.
        #[prost(message, tag="5")]
        ExpireTime(::prost_types::Timestamp),
        ///  Input only. The TTL (time to live) of the RuleSet.
        ///
        ///  If it is set, \[expire_time][google.cloud.contentwarehouse.v1.RuleSet.expire_time\] is set as current timestamp plus \[ttl][google.cloud.contentwarehouse.v1.RuleSet.ttl\].
        ///  The derived \[expire_time][google.cloud.contentwarehouse.v1.RuleSet.expire_time\] is returned in the output and \[ttl][google.cloud.contentwarehouse.v1.RuleSet.ttl\] is left
        ///  blank when retrieving the \[RuleSet][google.cloud.contentwarehouse.v1.RuleSet\].
        ///
        ///  If it is set, the RuleSet is not available for execution after current
        ///  timestamp plus \[ttl][google.cloud.contentwarehouse.v1.RuleSet.ttl\]. However, the RuleSet can still be retrieved by
        ///  \[RuleSetService.GetRuleSet][google.cloud.contentwarehouse.v1.RuleSetService.GetRuleSet\] and \[RuleSetService.ListRuleSets][google.cloud.contentwarehouse.v1.RuleSetService.ListRuleSets\].
        #[prost(message, tag="7")]
        Ttl(::prost_types::Duration),
    }
}
///  Represents the rule for a content warehouse trigger.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rule {
    ///  Short description of the rule and its context.
    #[prost(string, tag="1")]
    pub description: ::prost::alloc::string::String,
    ///  ID of the rule. It has to be unique across all the examples.
    ///  This is managed internally.
    #[prost(string, tag="2")]
    pub rule_id: ::prost::alloc::string::String,
    ///  Identifies the trigger type for running the policy.
    #[prost(enumeration="rule::TriggerType", tag="3")]
    pub trigger_type: i32,
    ///  Represents the conditional expression to be evaluated.
    ///  Expression should evaluate to a boolean result.
    ///  When the condition is true actions are executed.
    ///  Example: user_role = "hsbc_role_1" AND doc.salary > 20000
    #[prost(string, tag="4")]
    pub condition: ::prost::alloc::string::String,
    ///  List of actions that are executed when the rule is satisfied.
    #[prost(message, repeated, tag="5")]
    pub actions: ::prost::alloc::vec::Vec<Action>,
    ///  Priority of the rule for execution sequence preference.
    #[prost(float, tag="6")]
    pub priority: f32,
    ///  Indicates if the policy is currently in use or not.
    #[prost(bool, tag="7")]
    pub enabled: bool,
}
/// Nested message and enum types in `Rule`.
pub mod rule {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TriggerType {
        Unknown = 0,
        ///  Trigger for create document action.
        OnCreate = 1,
        ///  Trigger for read document action.
        OnRead = 2,
        ///  Trigger for search document action.
        OnSearch = 3,
        ///  Trigger for update document action.
        OnUpdate = 4,
        ///  Trigger for delete document action.
        OnDelete = 5,
        ///  Trigger for asynchronous document actions.
        Async = 6,
    }
    impl TriggerType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TriggerType::Unknown => "UNKNOWN",
                TriggerType::OnCreate => "ON_CREATE",
                TriggerType::OnRead => "ON_READ",
                TriggerType::OnSearch => "ON_SEARCH",
                TriggerType::OnUpdate => "ON_UPDATE",
                TriggerType::OnDelete => "ON_DELETE",
                TriggerType::Async => "ASYNC",
            }
        }
    }
}
///  Represents the action triggered by Rule Engine when the rule is true.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Action {
    ///  ID of the action. Managed internally.
    #[prost(string, tag="1")]
    pub action_id: ::prost::alloc::string::String,
    #[prost(oneof="action::Action", tags="2, 3, 4, 5, 6, 7, 8, 9, 10")]
    pub action: ::core::option::Option<action::Action>,
}
/// Nested message and enum types in `Action`.
pub mod action {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        ///  Action triggering access control operations.
        #[prost(message, tag="2")]
        AccessControl(super::AccessControlAction),
        ///  Action triggering data validation operations.
        #[prost(message, tag="3")]
        DataValidation(super::DataValidationAction),
        ///  Action triggering data update operations.
        #[prost(message, tag="4")]
        DataUpdate(super::DataUpdateAction),
        ///  Action triggering create document link operation.
        #[prost(message, tag="5")]
        AddToFolder(super::AddToFolderAction),
        ///  Action publish to Pub/Sub operation.
        #[prost(message, tag="6")]
        PublishToPubSub(super::PublishAction),
        ///  Action performing context evaluation operation.
        #[prost(message, tag="7")]
        ContextEvaluationAction(super::ContextEvaluationAction),
        ///  Action process expired data.
        #[prost(message, tag="8")]
        ExpiredDataHandlingAction(super::ExpiredDataHandlingAction),
        ///  Action removing a document from a folder.
        #[prost(message, tag="9")]
        RemoveFromFolderAction(super::RemoveFromFolderAction),
        ///  Action deleting the document.
        #[prost(message, tag="10")]
        DeleteDocumentAction(super::DeleteDocumentAction),
    }
}
///  Represents the action responsible for access control list management
///  operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessControlAction {
    ///  Identifies the type of operation.
    #[prost(enumeration="access_control_action::OperationType", tag="1")]
    pub operation_type: i32,
    ///  Represents the new policy from which bindings are added, removed or
    ///  replaced based on the type of the operation. the policy is limited to a few
    ///  10s of KB.
    #[prost(message, optional, tag="2")]
    pub policy: ::core::option::Option<super::super::super::iam::v1::Policy>,
}
/// Nested message and enum types in `AccessControlAction`.
pub mod access_control_action {
    ///  Type of ACL modification operation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OperationType {
        Unknown = 0,
        ///  Adds newly given policy bindings in the existing bindings list.
        AddPolicyBinding = 1,
        ///  Removes newly given policy bindings from the existing bindings list.
        RemovePolicyBinding = 2,
        ///  Replaces existing policy bindings with the given policy binding list
        ReplacePolicyBinding = 3,
    }
    impl OperationType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OperationType::Unknown => "UNKNOWN",
                OperationType::AddPolicyBinding => "ADD_POLICY_BINDING",
                OperationType::RemovePolicyBinding => "REMOVE_POLICY_BINDING",
                OperationType::ReplacePolicyBinding => "REPLACE_POLICY_BINDING",
            }
        }
    }
}
///  Represents the action responsible for data validation operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataValidationAction {
    ///  Map of (K, V) -> (field, string condition to be evaluated on the field)
    ///  E.g., ("age", "age > 18  && age < 60") entry triggers validation of field
    ///  age with the given condition. Map entries will be ANDed during validation.
    #[prost(map="string, string", tag="1")]
    pub conditions: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
///  Represents the action responsible for properties update operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataUpdateAction {
    ///  Map of (K, V) -> (valid name of the field, new value of the field)
    ///  E.g., ("age", "60") entry triggers update of field age with a value of 60.
    ///  If the field is not present then new entry is added.
    ///  During update action execution, value strings will be casted to
    ///  appropriate types.
    #[prost(map="string, string", tag="1")]
    pub entries: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
///  Represents the folder schema and corresponding condition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FolderSchemaCondition {
    ///  Name of the folder schema consisting of the properties to be evaluated.
    ///  Format:
    ///  projects/{project_number}/locations/{location}/documentSchemas/{document_id}.
    #[prost(string, tag="1")]
    pub folder_schema: ::prost::alloc::string::String,
    ///  The filter condition.
    ///  The syntax for this expression is a subset of SQL syntax.
    ///
    ///  Supported operators are: `=`, `!=`, `<`, `<=`, `>`, and `>=` where the left
    ///  of the operator is a property name and the right of the operator is a
    ///  number or a quoted string. You must escape backslash (\\) and quote (\")
    ///  characters. Supported functions are `LOWER(\[property_name\])` to perform a
    ///  case insensitive match and `EMPTY(\[property_name\])` to filter on the
    ///  existence of a key.
    ///
    ///  Boolean expressions (AND/OR/NOT) are supported up to 3 levels of nesting
    ///  (for example, "((A AND B AND C) OR NOT D) AND E"), a maximum of 100
    ///  comparisons or functions are allowed in the expression. The expression must
    ///  be < 6000 bytes in length.
    ///  Example: "DOC.SSN = FOLDER.ssn"
    ///  In the above example, SSN property from the document under
    ///  process is evaluated against folders' ssn property.
    ///  The document under process will be added under the folder if the condition
    ///  is evaluated as true.
    ///  Note: FOLDER prefix is used to refer to target folder's properties.
    ///  DOC prefix is used to refer to properties in the under process document.
    #[prost(string, tag="2")]
    pub condition: ::prost::alloc::string::String,
}
///  Represents the action responsible for adding document under a folder.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddToFolderAction {
    ///  Names of the folder under which new document is to be added.
    ///  Format:
    ///  projects/{project_number}/locations/{location}/documents/{document_id}.
    #[prost(string, repeated, tag="1")]
    pub folders: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  If the folders info is not available then populate FolderSchemaConditions
    ///  for finding matching folders dynamically.
    ///  Filter conditions used to derive specific folders dynamically.
    #[prost(message, repeated, tag="2")]
    pub folder_schema_conditions: ::prost::alloc::vec::Vec<FolderSchemaCondition>,
}
///  Represents the action responsible for remove a document from a specific
///  folder.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFromFolderAction {
    ///  Condition of the action to be executed.
    #[prost(string, tag="1")]
    pub condition: ::prost::alloc::string::String,
    ///  Name of the folder under which new document is to be added.
    ///  Format:
    ///  projects/{project_number}/locations/{location}/documents/{document_id}.
    #[prost(string, tag="2")]
    pub folder: ::prost::alloc::string::String,
}
///  Represents the action responsible for publishing messages to a Pub/Sub topic.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishAction {
    ///  The topic id in the Pub/Sub service for which messages will be published
    ///  to.
    #[prost(string, tag="1")]
    pub topic_id: ::prost::alloc::string::String,
    ///  Messages to be published.
    #[prost(string, repeated, tag="2")]
    pub messages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///  Represents the action responsible for performing context evaluation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextEvaluationAction {
    ///  Filter condition for the documents to be included in the evaluation.
    #[prost(string, tag="1")]
    pub condition: ::prost::alloc::string::String,
    ///  Name of the variables input for the context evaluation.
    #[prost(string, repeated, tag="2")]
    pub variable_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  Optional. cloud Pub/Sub topic-id. If specified, information of the
    ///  updated documents will be published to this topic.
    #[prost(string, tag="5")]
    pub topic_id: ::prost::alloc::string::String,
    #[prost(oneof="context_evaluation_action::ContextEvaluationMethod", tags="4, 6, 7")]
    pub context_evaluation_method: ::core::option::Option<context_evaluation_action::ContextEvaluationMethod>,
}
/// Nested message and enum types in `ContextEvaluationAction`.
pub mod context_evaluation_action {
    ///  User Cloud Function for context evaluation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CloudFunctionInfo {
        ///  Url of the Cloud Function. The Cloud Function needs to live inside
        ///  consumer project.
        #[prost(string, tag="3")]
        pub cloud_function: ::prost::alloc::string::String,
    }
    ///  Settings of user's webhook.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Webhook {
        ///  URL string for receiving a POST request every time webhook is triggered.
        #[prost(string, tag="8")]
        pub uri: ::prost::alloc::string::String,
    }
    ///  Settings of service directory for webhook under VPCSC.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WebhookWithServiceDirectory {
        ///  The service directory service resource name.
        #[prost(string, tag="9")]
        pub service: ::prost::alloc::string::String,
        ///  Detailed webhook settings.
        #[prost(message, optional, tag="10")]
        pub webhook: ::core::option::Option<Webhook>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ContextEvaluationMethod {
        ///  User Cloud Function for the context evaluation.
        #[prost(message, tag="4")]
        CloudFunctionInfo(CloudFunctionInfo),
        ///  User's webhook address for the context evaluation.
        #[prost(message, tag="6")]
        Webhook(Webhook),
        ///  User's webhook address integrated with service-directory.
        #[prost(message, tag="7")]
        WebhookWithServiceDirectory(WebhookWithServiceDirectory),
    }
}
///  Represents the action responsible for handling expired data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpiredDataHandlingAction {
    ///  Filter condition for the documents to be included in the evaluation.
    #[prost(string, tag="1")]
    pub condition: ::prost::alloc::string::String,
    ///  Expired data handling will publish errors to the Pub/Sub using topic-id
    ///  provided.
    #[prost(string, tag="2")]
    pub topic_id: ::prost::alloc::string::String,
}
///  Represents the action responsible for deleting the document.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDocumentAction {
    ///  Boolean field to select between hard vs soft delete options.
    ///  Set 'true' for 'hard delete' and 'false' for 'soft delete'.
    #[prost(bool, tag="1")]
    pub enable_hard_delete: bool,
}
///  Records the output of Rule Engine including rule evaluation and actions
///  result.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleEngineOutput {
    ///  Name of the document against which the rules and actions were evaluated.
    #[prost(string, tag="3")]
    pub document_name: ::prost::alloc::string::String,
    ///  Output from Rule Evaluator containing matched, unmatched and invalid rules.
    #[prost(message, optional, tag="1")]
    pub rule_evaluator_output: ::core::option::Option<RuleEvaluatorOutput>,
    ///  Output from Action Executor containing rule and corresponding actions
    ///  execution result.
    #[prost(message, optional, tag="2")]
    pub action_executor_output: ::core::option::Option<ActionExecutorOutput>,
}
///  Represents the output of the Rule Evaluator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleEvaluatorOutput {
    ///  List of rules fetched from database for the given request trigger type.
    #[prost(message, repeated, tag="1")]
    pub triggered_rules: ::prost::alloc::vec::Vec<Rule>,
    ///  A subset of triggered rules that are evaluated true for a given request.
    #[prost(message, repeated, tag="2")]
    pub matched_rules: ::prost::alloc::vec::Vec<Rule>,
    ///  A subset of triggered rules that failed the validation check(s) after
    ///  parsing.
    #[prost(message, repeated, tag="3")]
    pub invalid_rules: ::prost::alloc::vec::Vec<InvalidRule>,
}
///  A triggered rule that failed the validation check(s) after parsing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvalidRule {
    ///  Triggered rule.
    #[prost(message, optional, tag="1")]
    pub rule: ::core::option::Option<Rule>,
    ///  Validation error on a parsed expression.
    #[prost(string, tag="2")]
    pub error: ::prost::alloc::string::String,
}
///  Represents the output of the Action Executor.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionExecutorOutput {
    ///  List of rule and corresponding actions result.
    #[prost(message, repeated, tag="1")]
    pub rule_actions_pairs: ::prost::alloc::vec::Vec<RuleActionsPair>,
}
///  Represents a rule and outputs of associated actions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleActionsPair {
    ///  Represents the rule.
    #[prost(message, optional, tag="1")]
    pub rule: ::core::option::Option<Rule>,
    ///  Outputs of executing the actions associated with the above rule.
    #[prost(message, repeated, tag="2")]
    pub action_outputs: ::prost::alloc::vec::Vec<ActionOutput>,
}
///  Represents the result of executing an action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionOutput {
    ///  ID of the action.
    #[prost(string, tag="1")]
    pub action_id: ::prost::alloc::string::String,
    ///  State of an action.
    #[prost(enumeration="action_output::State", tag="2")]
    pub action_state: i32,
    ///  Action execution output message.
    #[prost(string, tag="3")]
    pub output_message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ActionOutput`.
pub mod action_output {
    ///  Represents execution state of the action.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        Unknown = 0,
        ///  State indicating action executed successfully.
        ActionSucceeded = 1,
        ///  State indicating action failed.
        ActionFailed = 2,
        ///  State indicating action timed out.
        ActionTimedOut = 3,
        ///  State indicating action is pending.
        ActionPending = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unknown => "UNKNOWN",
                State::ActionSucceeded => "ACTION_SUCCEEDED",
                State::ActionFailed => "ACTION_FAILED",
                State::ActionTimedOut => "ACTION_TIMED_OUT",
                State::ActionPending => "ACTION_PENDING",
            }
        }
    }
}
///  Represents the schedule with which asynchronous RuleSets are applied.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Schedule {
    ///  Unspecified schedule.
    Unspecified = 0,
    ///  Policy should be applied every calendar day at 00:00 PST.
    Daily = 1,
    ///  Policy should be applied every Sunday at 00:00 PST.
    Weekly = 2,
    ///  Policy should be applied the 1st of every calendar month at 00:00 PST.
    Monthly = 3,
}
impl Schedule {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Schedule::Unspecified => "SCHEDULE_UNSPECIFIED",
            Schedule::Daily => "SCHEDULE_DAILY",
            Schedule::Weekly => "SCHEDULE_WEEKLY",
            Schedule::Monthly => "SCHEDULE_MONTHLY",
        }
    }
}
///  Response message for DocumentService.CreateDocument.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDocumentResponse {
    ///  Document created after executing create request.
    #[prost(message, optional, tag="1")]
    pub document: ::core::option::Option<Document>,
    ///  Output from Rule Engine recording the rule evaluator and action executor's
    ///  output.
    ///
    ///  Refer format in: google/cloud/contentwarehouse/v1/rule_engine.proto
    #[prost(message, optional, tag="2")]
    pub rule_engine_output: ::core::option::Option<RuleEngineOutput>,
    ///  Additional information for the API invocation, such as the request tracking
    ///  id.
    #[prost(message, optional, tag="3")]
    pub metadata: ::core::option::Option<ResponseMetadata>,
}
///  Response message for DocumentService.UpdateDocument.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDocumentResponse {
    ///  Updated document after executing update request.
    #[prost(message, optional, tag="1")]
    pub document: ::core::option::Option<Document>,
    ///  Output from Rule Engine recording the rule evaluator and action executor's
    ///  output.
    ///
    ///  Refer format in: google/cloud/contentwarehouse/v1/rule_engine.proto
    #[prost(message, optional, tag="2")]
    pub rule_engine_output: ::core::option::Option<RuleEngineOutput>,
    ///  Additional information for the API invocation, such as the request tracking
    ///  id.
    #[prost(message, optional, tag="3")]
    pub metadata: ::core::option::Option<ResponseMetadata>,
}
///  Additional result info for the question-answering feature.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QaResult {
    ///  Highlighted sections in the snippet.
    #[prost(message, repeated, tag="1")]
    pub highlights: ::prost::alloc::vec::Vec<qa_result::Highlight>,
    ///  The calibrated confidence score for this document, in the range
    ///  [0., 1.]. This represents the confidence level for whether the returned
    ///  document and snippet answers the user's query.
    #[prost(float, tag="2")]
    pub confidence_score: f32,
}
/// Nested message and enum types in `QAResult`.
pub mod qa_result {
    ///  A text span in the search text snippet that represents a highlighted
    ///  section (answer context, highly relevant sentence, etc.).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Highlight {
        ///  Start index of the highlight.
        #[prost(int32, tag="1")]
        pub start_index: i32,
        ///  End index of the highlight, exclusive.
        #[prost(int32, tag="2")]
        pub end_index: i32,
    }
}
///  Response message for DocumentService.SearchDocuments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchDocumentsResponse {
    ///  The document entities that match the specified \[SearchDocumentsRequest][google.cloud.contentwarehouse.v1.SearchDocumentsRequest\].
    #[prost(message, repeated, tag="1")]
    pub matching_documents: ::prost::alloc::vec::Vec<search_documents_response::MatchingDocument>,
    ///  The token that specifies the starting position of the next page of results.
    ///  This field is empty if there are no more results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    ///  The total number of matched documents which is available only if the client
    ///  set \[SearchDocumentsRequest.require_total_size][google.cloud.contentwarehouse.v1.SearchDocumentsRequest.require_total_size\] to `true`. Otherwise, the
    ///  value will be `-1`.
    #[prost(int32, tag="3")]
    pub total_size: i32,
    ///  Additional information for the API invocation, such as the request tracking
    ///  id.
    #[prost(message, optional, tag="4")]
    pub metadata: ::core::option::Option<ResponseMetadata>,
    ///  The histogram results that match with the specified
    ///  \[SearchDocumentsRequest.histogram_queries][google.cloud.contentwarehouse.v1.SearchDocumentsRequest.histogram_queries\].
    #[prost(message, repeated, tag="6")]
    pub histogram_query_results: ::prost::alloc::vec::Vec<HistogramQueryResult>,
}
/// Nested message and enum types in `SearchDocumentsResponse`.
pub mod search_documents_response {
    ///  Document entry with metadata inside \[SearchDocumentsResponse][google.cloud.contentwarehouse.v1.SearchDocumentsResponse\]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatchingDocument {
        ///  Document that matches the specified \[SearchDocumentsRequest][google.cloud.contentwarehouse.v1.SearchDocumentsRequest\].
        ///  This document only contains indexed metadata information.
        #[prost(message, optional, tag="1")]
        pub document: ::core::option::Option<super::Document>,
        ///  Contains snippets of text from the document full raw text that most
        ///  closely match a search query's keywords, if available. All HTML tags in
        ///  the original fields are stripped when returned in this field, and
        ///  matching query keywords are enclosed in HTML bold tags.
        ///
        ///  If the question-answering feature is enabled, this field will instead
        ///  contain a snippet that answers the user's natural-language query. No HTML
        ///  bold tags will be present, and highlights in the answer snippet can be
        ///  found in \[QAResult.highlights][google.cloud.contentwarehouse.v1.QAResult.highlights\].
        #[prost(string, tag="2")]
        pub search_text_snippet: ::prost::alloc::string::String,
        ///  Experimental.
        ///  Additional result info if the question-answering feature is enabled.
        #[prost(message, optional, tag="3")]
        pub qa_result: ::core::option::Option<super::QaResult>,
    }
}
///  Response message for DocumentService.FetchAcl.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchAclResponse {
    ///  The IAM policy.
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::super::iam::v1::Policy>,
    ///  Additional information for the API invocation, such as the request tracking
    ///  id.
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<ResponseMetadata>,
}
///  Response message for DocumentService.SetAcl.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAclResponse {
    ///  The policy will be attached to a resource (e.g. projecct, document).
    #[prost(message, optional, tag="1")]
    pub policy: ::core::option::Option<super::super::super::iam::v1::Policy>,
    ///  Additional information for the API invocation, such as the request tracking
    ///  id.
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<ResponseMetadata>,
}
/// Generated client implementations.
pub mod document_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct DocumentServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DocumentServiceClient<tonic::transport::Channel> {
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
    impl<T> DocumentServiceClient<T>
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
        ) -> DocumentServiceClient<InterceptedService<T, F>>
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
            DocumentServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a document.
        pub async fn create_document(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDocumentRequest>,
        ) -> Result<tonic::Response<super::CreateDocumentResponse>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.DocumentService/CreateDocument",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a document. Returns NOT_FOUND if the document does not exist.
        pub async fn get_document(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDocumentRequest>,
        ) -> Result<tonic::Response<super::Document>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.DocumentService/GetDocument",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a document. Returns INVALID_ARGUMENT if the name of the document
        /// is non-empty and does not equal the existing name.
        pub async fn update_document(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDocumentRequest>,
        ) -> Result<tonic::Response<super::UpdateDocumentResponse>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.DocumentService/UpdateDocument",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a document. Returns NOT_FOUND if the document does not exist.
        pub async fn delete_document(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDocumentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.DocumentService/DeleteDocument",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Searches for documents using provided [SearchDocumentsRequest][google.cloud.contentwarehouse.v1.SearchDocumentsRequest].
        /// This call only returns documents that the caller has permission to search
        /// against.
        pub async fn search_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchDocumentsRequest>,
        ) -> Result<tonic::Response<super::SearchDocumentsResponse>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.DocumentService/SearchDocuments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the access control policy for a resource. Returns NOT_FOUND error if
        /// the resource does not exist. Returns an empty policy if the resource exists
        /// but does not have a policy set.
        pub async fn fetch_acl(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchAclRequest>,
        ) -> Result<tonic::Response<super::FetchAclResponse>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.DocumentService/FetchAcl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the access control policy for a resource. Replaces any existing
        /// policy.
        pub async fn set_acl(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAclRequest>,
        ) -> Result<tonic::Response<super::SetAclResponse>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.DocumentService/SetAcl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
///  Request message for RuleSetService.CreateRuleSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRuleSetRequest {
    ///  Required. The parent name.
    ///  Format: projects/{project_number}/locations/{location}.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Required. The rule set to create.
    #[prost(message, optional, tag="2")]
    pub rule_set: ::core::option::Option<RuleSet>,
}
///  Request message for RuleSetService.GetRuleSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRuleSetRequest {
    ///  Required. The name of the rule set to retrieve.
    ///  Format:
    ///  projects/{project_number}/locations/{location}/ruleSets/{rule_set_id}.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Request message for RuleSetService.UpdateRuleSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRuleSetRequest {
    ///  Required. The name of the rule set to update.
    ///  Format:
    ///  projects/{project_number}/locations/{location}/ruleSets/{rule_set_id}.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Required. The rule set to update.
    #[prost(message, optional, tag="2")]
    pub rule_set: ::core::option::Option<RuleSet>,
}
///  Request message for RuleSetService.DeleteRuleSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRuleSetRequest {
    ///  Required. The name of the rule set to delete.
    ///  Format:
    ///  projects/{project_number}/locations/{location}/ruleSets/{rule_set_id}.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Request message for RuleSetService.ListRuleSets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuleSetsRequest {
    ///  Required. The parent, which owns this collection of document.
    ///  Format: projects/{project_number}/locations/{location}.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  The maximum number of rule sets to return. The service may return
    ///  fewer than this value.
    ///  If unspecified, at most 50 rule sets will be returned.
    ///  The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    ///  A page token, received from a previous `ListRuleSets` call.
    ///  Provide this to retrieve the subsequent page.
    ///
    ///  When paginating, all other parameters provided to `ListRuleSets`
    ///  must match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
///  Response message for RuleSetService.ListRuleSets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuleSetsResponse {
    ///  The rule sets from the specified parent.
    #[prost(message, repeated, tag="1")]
    pub rule_sets: ::prost::alloc::vec::Vec<RuleSet>,
    ///  A token, which can be sent as `page_token` to retrieve the next page.
    ///  If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  Request message for RuleSetService.ProcessAsyncRuleSets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessAsyncRuleSetsRequest {
    ///  Required. The Location under which all async rules are stored.
    #[prost(string, tag="1")]
    pub location: ::prost::alloc::string::String,
    ///  Required. The frequency category of async rules to process.
    #[prost(enumeration="Schedule", tag="2")]
    pub schedule: i32,
    ///  Optional. Timestamp to override inferred execution time.
    #[prost(message, optional, tag="3")]
    pub execution_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Optional. Document name filter to process only a subset of Documents.
    #[prost(string, repeated, tag="4")]
    pub document_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///  Response message for RuleSetService.ProcessAsyncRuleSets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessAsyncRuleSetsResponse {
    ///  True if the processing of asynchronous policies was entirely
    ///  successful.
    #[prost(bool, tag="1")]
    pub success: bool,
    ///  Contains the errors encountered during the processing of asynchronous
    ///  policies.
    #[prost(message, repeated, tag="2")]
    pub errors: ::prost::alloc::vec::Vec<RuleEngineOutput>,
}
/// Generated client implementations.
pub mod rule_set_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct RuleSetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RuleSetServiceClient<tonic::transport::Channel> {
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
    impl<T> RuleSetServiceClient<T>
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
        ) -> RuleSetServiceClient<InterceptedService<T, F>>
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
            RuleSetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a ruleset.
        pub async fn create_rule_set(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRuleSetRequest>,
        ) -> Result<tonic::Response<super::RuleSet>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.RuleSetService/CreateRuleSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a ruleset. Returns NOT_FOUND if the ruleset does not exist.
        pub async fn get_rule_set(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRuleSetRequest>,
        ) -> Result<tonic::Response<super::RuleSet>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.RuleSetService/GetRuleSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a ruleset. Returns INVALID_ARGUMENT if the name of the ruleset
        /// is non-empty and does not equal the existing name.
        pub async fn update_rule_set(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRuleSetRequest>,
        ) -> Result<tonic::Response<super::RuleSet>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.RuleSetService/UpdateRuleSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a ruleset. Returns NOT_FOUND if the document does not exist.
        pub async fn delete_rule_set(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRuleSetRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.RuleSetService/DeleteRuleSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists rulesets.
        pub async fn list_rule_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRuleSetsRequest>,
        ) -> Result<tonic::Response<super::ListRuleSetsResponse>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.RuleSetService/ListRuleSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Runs asynchronous RuleSets.
        pub async fn process_async_rule_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::ProcessAsyncRuleSetsRequest>,
        ) -> Result<
            tonic::Response<super::ProcessAsyncRuleSetsResponse>,
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
                "/google.cloud.contentwarehouse.v1.RuleSetService/ProcessAsyncRuleSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
///  Represents a list of synonyms for a given context.
///  For example a context "sales" could contain:
///  Synonym 1: sale, invoice, bill, order
///  Synonym 2: money, credit, finance, payment
///  Synonym 3: shipping, freight, transport
///  Each SynonymSets should be disjoint
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynonymSet {
    ///  The resource name of the SynonymSet
    ///  This is mandatory for google.api.resource.
    ///  Format:
    ///  projects/{project_number}/locations/{location}/synonymSets/{context}.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  This is a freeform field. Example contexts can be "sales," "engineering,"
    ///  "real estate," "accounting," etc.
    ///  The context can be supplied during search requests.
    #[prost(string, tag="2")]
    pub context: ::prost::alloc::string::String,
    ///  List of Synonyms for the context.
    #[prost(message, repeated, tag="3")]
    pub synonyms: ::prost::alloc::vec::Vec<synonym_set::Synonym>,
}
/// Nested message and enum types in `SynonymSet`.
pub mod synonym_set {
    ///  Represents a list of words given by the customer
    ///  All these words are synonyms of each other.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Synonym {
        ///  For example: sale, invoice, bill, order
        #[prost(string, repeated, tag="1")]
        pub words: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
///  Request message for SynonymSetService.CreateSynonymSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSynonymSetRequest {
    ///  Required. The parent name.
    ///  Format: projects/{project_number}/locations/{location}.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Required. The synonymSet to be created for a context
    #[prost(message, optional, tag="2")]
    pub synonym_set: ::core::option::Option<SynonymSet>,
}
///  Request message for SynonymSetService.GetSynonymSet.
///  Will return synonymSet for a certain context.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSynonymSetRequest {
    ///  Required. The name of the synonymSet to retrieve
    ///  Format:
    ///  projects/{project_number}/locations/{location}/synonymSets/{context}.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Request message for SynonymSetService.ListSynonymSets.
///  Will return all synonymSets belonging to the customer project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSynonymSetsRequest {
    ///  Required. The parent name.
    ///  Format: projects/{project_number}/locations/{location}.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  The maximum number of synonymSets to return. The service may return
    ///  fewer than this value.
    ///  If unspecified, at most 50 rule sets will be returned.
    ///  The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    ///  A page token, received from a previous `ListSynonymSets` call.
    ///  Provide this to retrieve the subsequent page.
    ///
    ///  When paginating, all other parameters provided to `ListSynonymSets`
    ///  must match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
///  Response message for SynonymSetService.ListSynonymSets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSynonymSetsResponse {
    ///  The synonymSets from the specified parent.
    #[prost(message, repeated, tag="1")]
    pub synonym_sets: ::prost::alloc::vec::Vec<SynonymSet>,
    ///  A page token, received from a previous `ListSynonymSets` call.
    ///  Provide this to retrieve the subsequent page.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  Request message for SynonymSetService.UpdateSynonymSet.
///  Removes the SynonymSet for the specified context and replaces
///  it with the SynonymSet in this request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSynonymSetRequest {
    ///  Required. The name of the synonymSet to update
    ///  Format:
    ///  projects/{project_number}/locations/{location}/synonymSets/{context}.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Required. The synonymSet to be updated for the customer
    #[prost(message, optional, tag="2")]
    pub synonym_set: ::core::option::Option<SynonymSet>,
}
///  Request message for SynonymSetService.DeleteSynonymSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSynonymSetRequest {
    ///  Required. The name of the synonymSet to delete
    ///  Format:
    ///  projects/{project_number}/locations/{location}/synonymSets/{context}.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod synonym_set_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A Service that manage/custom customer specified SynonymSets.
    #[derive(Debug, Clone)]
    pub struct SynonymSetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SynonymSetServiceClient<tonic::transport::Channel> {
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
    impl<T> SynonymSetServiceClient<T>
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
        ) -> SynonymSetServiceClient<InterceptedService<T, F>>
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
            SynonymSetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a SynonymSet for a single context.
        /// Throws an ALREADY_EXISTS exception if a synonymset already exists
        /// for the context.
        pub async fn create_synonym_set(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSynonymSetRequest>,
        ) -> Result<tonic::Response<super::SynonymSet>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.SynonymSetService/CreateSynonymSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a SynonymSet for a particular context.
        /// Throws a NOT_FOUND exception if the Synonymset
        /// does not exist
        pub async fn get_synonym_set(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSynonymSetRequest>,
        ) -> Result<tonic::Response<super::SynonymSet>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.SynonymSetService/GetSynonymSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Remove the existing SynonymSet for the context and replaces it
        /// with a new one.
        /// Throws a NOT_FOUND exception if the SynonymSet is not found.
        pub async fn update_synonym_set(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSynonymSetRequest>,
        ) -> Result<tonic::Response<super::SynonymSet>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.SynonymSetService/UpdateSynonymSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a SynonymSet for a given context.
        /// Throws a NOT_FOUND exception if the SynonymSet is not found.
        pub async fn delete_synonym_set(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSynonymSetRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.SynonymSetService/DeleteSynonymSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns all SynonymSets (for all contexts) for the specified location.
        pub async fn list_synonym_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSynonymSetsRequest>,
        ) -> Result<tonic::Response<super::ListSynonymSetsResponse>, tonic::Status> {
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
                "/google.cloud.contentwarehouse.v1.SynonymSetService/ListSynonymSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}