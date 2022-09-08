///  The conversation resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Conversation {
    ///  Immutable. The resource name of the conversation.
    ///  Format:
    ///  projects/{project}/locations/{location}/conversations/{conversation}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  The source of the audio and transcription for the conversation.
    #[prost(message, optional, tag="2")]
    pub data_source: ::core::option::Option<ConversationDataSource>,
    ///  Output only. The time at which the conversation was created.
    #[prost(message, optional, tag="3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The most recent time at which the conversation was updated.
    #[prost(message, optional, tag="4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  The time at which the conversation started.
    #[prost(message, optional, tag="17")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  A user-specified language code for the conversation.
    #[prost(string, tag="14")]
    pub language_code: ::prost::alloc::string::String,
    ///  An opaque, user-specified string representing the human agent who handled
    ///  the conversation.
    #[prost(string, tag="5")]
    pub agent_id: ::prost::alloc::string::String,
    ///  A map for the user to specify any custom fields. A maximum of 20 labels per
    ///  conversation is allowed, with a maximum of 256 characters per entry.
    #[prost(map="string, string", tag="6")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  Output only. The conversation transcript.
    #[prost(message, optional, tag="8")]
    pub transcript: ::core::option::Option<conversation::Transcript>,
    ///  Immutable. The conversation medium, if unspecified will default to PHONE_CALL.
    #[prost(enumeration="conversation::Medium", tag="9")]
    pub medium: i32,
    ///  Output only. The duration of the conversation.
    #[prost(message, optional, tag="10")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    ///  Output only. The number of turns in the conversation.
    #[prost(int32, tag="11")]
    pub turn_count: i32,
    ///  Output only. The conversation's latest analysis, if one exists.
    #[prost(message, optional, tag="12")]
    pub latest_analysis: ::core::option::Option<Analysis>,
    ///  Output only. The annotations that were generated during the customer and agent
    ///  interaction.
    #[prost(message, repeated, tag="13")]
    pub runtime_annotations: ::prost::alloc::vec::Vec<RuntimeAnnotation>,
    ///  Output only. All the matched Dialogflow intents in the call. The key corresponds to a
    ///  Dialogflow intent, format:
    ///  projects/{project}/agent/{agent}/intents/{intent}
    #[prost(map="string, message", tag="18")]
    pub dialogflow_intents: ::std::collections::HashMap<::prost::alloc::string::String, DialogflowIntent>,
    ///  Obfuscated user ID which the customer sent to us.
    #[prost(string, tag="21")]
    pub obfuscated_user_id: ::prost::alloc::string::String,
    ///  Metadata that applies to the conversation.
    #[prost(oneof="conversation::Metadata", tags="7")]
    pub metadata: ::core::option::Option<conversation::Metadata>,
    ///  A time to live expiration setting, can be either a specified timestamp or a
    ///  duration from the time that the conversation creation request was received.
    ///  Conversations with an expiration set will be removed up to 24 hours after
    ///  the specified time.
    #[prost(oneof="conversation::Expiration", tags="15, 16")]
    pub expiration: ::core::option::Option<conversation::Expiration>,
}
/// Nested message and enum types in `Conversation`.
pub mod conversation {
    ///  Call-specific metadata.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CallMetadata {
        ///  The audio channel that contains the customer.
        #[prost(int32, tag="1")]
        pub customer_channel: i32,
        ///  The audio channel that contains the agent.
        #[prost(int32, tag="2")]
        pub agent_channel: i32,
    }
    ///  A message representing the transcript of a conversation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Transcript {
        ///  A list of sequential transcript segments that comprise the conversation.
        #[prost(message, repeated, tag="1")]
        pub transcript_segments: ::prost::alloc::vec::Vec<transcript::TranscriptSegment>,
    }
    /// Nested message and enum types in `Transcript`.
    pub mod transcript {
        ///  A segment of a full transcript.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TranscriptSegment {
            ///  The time that the message occurred, if provided.
            #[prost(message, optional, tag="6")]
            pub message_time: ::core::option::Option<::prost_types::Timestamp>,
            ///  The text of this segment.
            #[prost(string, tag="1")]
            pub text: ::prost::alloc::string::String,
            ///  A confidence estimate between 0.0 and 1.0 of the fidelity of this
            ///  segment. A default value of 0.0 indicates that the value is unset.
            #[prost(float, tag="2")]
            pub confidence: f32,
            ///  A list of the word-specific information for each word in the segment.
            #[prost(message, repeated, tag="3")]
            pub words: ::prost::alloc::vec::Vec<transcript_segment::WordInfo>,
            ///  The language code of this segment as a
            ///  \[BCP-47\](<https://www.rfc-editor.org/rfc/bcp/bcp47.txt>) language tag.
            ///  Example: "en-US".
            #[prost(string, tag="4")]
            pub language_code: ::prost::alloc::string::String,
            ///  For conversations derived from multi-channel audio, this is the channel
            ///  number corresponding to the audio from that channel. For
            ///  audioChannelCount = N, its output values can range from '1' to 'N'. A
            ///  channel tag of 0 indicates that the audio is mono.
            #[prost(int32, tag="5")]
            pub channel_tag: i32,
            ///  The participant of this segment.
            #[prost(message, optional, tag="9")]
            pub segment_participant: ::core::option::Option<super::super::ConversationParticipant>,
            ///  CCAI metadata relating to the current transcript segment.
            #[prost(message, optional, tag="10")]
            pub dialogflow_segment_metadata: ::core::option::Option<transcript_segment::DialogflowSegmentMetadata>,
            ///  The sentiment for this transcript segment.
            #[prost(message, optional, tag="11")]
            pub sentiment: ::core::option::Option<super::super::SentimentData>,
        }
        /// Nested message and enum types in `TranscriptSegment`.
        pub mod transcript_segment {
            ///  Word-level info for words in a transcript.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct WordInfo {
                ///  Time offset of the start of this word relative to the beginning of
                ///  the total conversation.
                #[prost(message, optional, tag="1")]
                pub start_offset: ::core::option::Option<::prost_types::Duration>,
                ///  Time offset of the end of this word relative to the beginning of the
                ///  total conversation.
                #[prost(message, optional, tag="2")]
                pub end_offset: ::core::option::Option<::prost_types::Duration>,
                ///  The word itself. Includes punctuation marks that surround the word.
                #[prost(string, tag="3")]
                pub word: ::prost::alloc::string::String,
                ///  A confidence estimate between 0.0 and 1.0 of the fidelity of this
                ///  word. A default value of 0.0 indicates that the value is unset.
                #[prost(float, tag="4")]
                pub confidence: f32,
            }
            ///  Metadata from Dialogflow relating to the current transcript segment.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct DialogflowSegmentMetadata {
                ///  Whether the transcript segment was covered under the configured smart
                ///  reply allowlist in Agent Assist.
                #[prost(bool, tag="1")]
                pub smart_reply_allowlist_covered: bool,
            }
        }
    }
    ///  Possible media for the conversation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Medium {
        ///  Default value, if unspecified will default to PHONE_CALL.
        Unspecified = 0,
        ///  The format for conversations that took place over the phone.
        PhoneCall = 1,
        ///  The format for conversations that took place over chat.
        Chat = 2,
    }
    impl Medium {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Medium::Unspecified => "MEDIUM_UNSPECIFIED",
                Medium::PhoneCall => "PHONE_CALL",
                Medium::Chat => "CHAT",
            }
        }
    }
    ///  Metadata that applies to the conversation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metadata {
        ///  Call-specific metadata.
        #[prost(message, tag="7")]
        CallMetadata(CallMetadata),
    }
    ///  A time to live expiration setting, can be either a specified timestamp or a
    ///  duration from the time that the conversation creation request was received.
    ///  Conversations with an expiration set will be removed up to 24 hours after
    ///  the specified time.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Expiration {
        ///  The time at which this conversation should expire. After this time, the
        ///  conversation data and any associated analyses will be deleted.
        #[prost(message, tag="15")]
        ExpireTime(::prost_types::Timestamp),
        ///  Input only. The TTL for this resource. If specified, then this TTL will
        ///  be used to calculate the expire time.
        #[prost(message, tag="16")]
        Ttl(::prost_types::Duration),
    }
}
///  The analysis resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Analysis {
    ///  Immutable. The resource name of the analysis.
    ///  Format:
    ///  projects/{project}/locations/{location}/conversations/{conversation}/analyses/{analysis}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Output only. The time at which the analysis was requested.
    #[prost(message, optional, tag="2")]
    pub request_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The time at which the analysis was created, which occurs when the
    ///  long-running operation completes.
    #[prost(message, optional, tag="3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The result of the analysis, which is populated when the analysis
    ///  finishes.
    #[prost(message, optional, tag="7")]
    pub analysis_result: ::core::option::Option<AnalysisResult>,
}
///  The conversation source, which is a combination of transcript and audio.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversationDataSource {
    ///  The source of the conversation.
    #[prost(oneof="conversation_data_source::Source", tags="1, 3")]
    pub source: ::core::option::Option<conversation_data_source::Source>,
}
/// Nested message and enum types in `ConversationDataSource`.
pub mod conversation_data_source {
    ///  The source of the conversation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        ///  A Cloud Storage location specification for the audio and transcript.
        #[prost(message, tag="1")]
        GcsSource(super::GcsSource),
        ///  The source when the conversation comes from Dialogflow.
        #[prost(message, tag="3")]
        DialogflowSource(super::DialogflowSource),
    }
}
///  A Cloud Storage source of conversation data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsSource {
    ///  Cloud Storage URI that points to a file that contains the conversation
    ///  audio.
    #[prost(string, tag="1")]
    pub audio_uri: ::prost::alloc::string::String,
    ///  Immutable. Cloud Storage URI that points to a file that contains the conversation
    ///  transcript.
    #[prost(string, tag="2")]
    pub transcript_uri: ::prost::alloc::string::String,
}
///  A Dialogflow source of conversation data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DialogflowSource {
    ///  Output only. The name of the Dialogflow conversation that this conversation
    ///  resource is derived from. Format:
    ///  projects/{project}/locations/{location}/conversations/{conversation}
    #[prost(string, tag="1")]
    pub dialogflow_conversation: ::prost::alloc::string::String,
    ///  Cloud Storage URI that points to a file that contains the conversation
    ///  audio.
    #[prost(string, tag="3")]
    pub audio_uri: ::prost::alloc::string::String,
}
///  The result of an analysis.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalysisResult {
    ///  The time at which the analysis ended.
    #[prost(message, optional, tag="1")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Metadata discovered during analysis.
    #[prost(oneof="analysis_result::Metadata", tags="2")]
    pub metadata: ::core::option::Option<analysis_result::Metadata>,
}
/// Nested message and enum types in `AnalysisResult`.
pub mod analysis_result {
    ///  Call-specific metadata created during analysis.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CallAnalysisMetadata {
        ///  A list of call annotations that apply to this call.
        #[prost(message, repeated, tag="2")]
        pub annotations: ::prost::alloc::vec::Vec<super::CallAnnotation>,
        ///  All the entities in the call.
        #[prost(map="string, message", tag="3")]
        pub entities: ::std::collections::HashMap<::prost::alloc::string::String, super::Entity>,
        ///  Overall conversation-level sentiment for each channel of the call.
        #[prost(message, repeated, tag="4")]
        pub sentiments: ::prost::alloc::vec::Vec<super::ConversationLevelSentiment>,
        ///  All the matched intents in the call.
        #[prost(map="string, message", tag="6")]
        pub intents: ::std::collections::HashMap<::prost::alloc::string::String, super::Intent>,
        ///  All the matched phrase matchers in the call.
        #[prost(map="string, message", tag="7")]
        pub phrase_matchers: ::std::collections::HashMap<::prost::alloc::string::String, super::PhraseMatchData>,
        ///  Overall conversation-level issue modeling result.
        #[prost(message, optional, tag="8")]
        pub issue_model_result: ::core::option::Option<super::IssueModelResult>,
    }
    ///  Metadata discovered during analysis.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metadata {
        ///  Call-specific metadata created by the analysis.
        #[prost(message, tag="2")]
        CallAnalysisMetadata(CallAnalysisMetadata),
    }
}
///  Issue Modeling result on a conversation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IssueModelResult {
    ///  Issue model that generates the result.
    ///  Format: projects/{project}/locations/{location}/issueModels/{issue_model}
    #[prost(string, tag="1")]
    pub issue_model: ::prost::alloc::string::String,
    ///  All the matched issues.
    #[prost(message, repeated, tag="2")]
    pub issues: ::prost::alloc::vec::Vec<IssueAssignment>,
}
///  One channel of conversation-level sentiment data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversationLevelSentiment {
    ///  The channel of the audio that the data applies to.
    #[prost(int32, tag="1")]
    pub channel_tag: i32,
    ///  Data specifying sentiment.
    #[prost(message, optional, tag="2")]
    pub sentiment_data: ::core::option::Option<SentimentData>,
}
///  Information about the issue.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IssueAssignment {
    ///  Resource name of the assigned issue.
    #[prost(string, tag="1")]
    pub issue: ::prost::alloc::string::String,
    ///  Score indicating the likelihood of the issue assignment.
    ///  currently bounded on \[0,1\].
    #[prost(double, tag="2")]
    pub score: f64,
    ///  Immutable. Display name of the assigned issue. This field is set at time of analyis
    ///  and immutable since then.
    #[prost(string, tag="3")]
    pub display_name: ::prost::alloc::string::String,
}
///  A piece of metadata that applies to a window of a call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallAnnotation {
    ///  The channel of the audio where the annotation occurs. For single-channel
    ///  audio, this field is not populated.
    #[prost(int32, tag="1")]
    pub channel_tag: i32,
    ///  The boundary in the conversation where the annotation starts, inclusive.
    #[prost(message, optional, tag="4")]
    pub annotation_start_boundary: ::core::option::Option<AnnotationBoundary>,
    ///  The boundary in the conversation where the annotation ends, inclusive.
    #[prost(message, optional, tag="5")]
    pub annotation_end_boundary: ::core::option::Option<AnnotationBoundary>,
    ///  The data in the annotation.
    #[prost(oneof="call_annotation::Data", tags="10, 11, 12, 13, 15, 16, 17")]
    pub data: ::core::option::Option<call_annotation::Data>,
}
/// Nested message and enum types in `CallAnnotation`.
pub mod call_annotation {
    ///  The data in the annotation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        ///  Data specifying an interruption.
        #[prost(message, tag="10")]
        InterruptionData(super::InterruptionData),
        ///  Data specifying sentiment.
        #[prost(message, tag="11")]
        SentimentData(super::SentimentData),
        ///  Data specifying silence.
        #[prost(message, tag="12")]
        SilenceData(super::SilenceData),
        ///  Data specifying a hold.
        #[prost(message, tag="13")]
        HoldData(super::HoldData),
        ///  Data specifying an entity mention.
        #[prost(message, tag="15")]
        EntityMentionData(super::EntityMentionData),
        ///  Data specifying an intent match.
        #[prost(message, tag="16")]
        IntentMatchData(super::IntentMatchData),
        ///  Data specifying a phrase match.
        #[prost(message, tag="17")]
        PhraseMatchData(super::PhraseMatchData),
    }
}
///  A point in a conversation that marks the start or the end of an annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationBoundary {
    ///  The index in the sequence of transcribed pieces of the conversation where
    ///  the boundary is located. This index starts at zero.
    #[prost(int32, tag="1")]
    pub transcript_index: i32,
    ///  A detailed boundary, which describes a more specific point.
    #[prost(oneof="annotation_boundary::DetailedBoundary", tags="3")]
    pub detailed_boundary: ::core::option::Option<annotation_boundary::DetailedBoundary>,
}
/// Nested message and enum types in `AnnotationBoundary`.
pub mod annotation_boundary {
    ///  A detailed boundary, which describes a more specific point.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DetailedBoundary {
        ///  The word index of this boundary with respect to the first word in the
        ///  transcript piece. This index starts at zero.
        #[prost(int32, tag="3")]
        WordIndex(i32),
    }
}
///  The data for an entity annotation.
///  Represents a phrase in the conversation that is a known entity, such
///  as a person, an organization, or location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    ///  The representative name for the entity.
    #[prost(string, tag="1")]
    pub display_name: ::prost::alloc::string::String,
    ///  The entity type.
    #[prost(enumeration="entity::Type", tag="2")]
    pub r#type: i32,
    ///  Metadata associated with the entity.
    ///
    ///  For most entity types, the metadata is a Wikipedia URL (`wikipedia_url`)
    ///  and Knowledge Graph MID (`mid`), if they are available. For the metadata
    ///  associated with other entity types, see the Type table below.
    #[prost(map="string, string", tag="3")]
    pub metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  The salience score associated with the entity in the [0, 1.0] range.
    ///
    ///  The salience score for an entity provides information about the
    ///  importance or centrality of that entity to the entire document text.
    ///  Scores closer to 0 are less salient, while scores closer to 1.0 are highly
    ///  salient.
    #[prost(float, tag="4")]
    pub salience: f32,
    ///  The aggregate sentiment expressed for this entity in the conversation.
    #[prost(message, optional, tag="5")]
    pub sentiment: ::core::option::Option<SentimentData>,
}
/// Nested message and enum types in `Entity`.
pub mod entity {
    ///  The type of the entity. For most entity types, the associated metadata is a
    ///  Wikipedia URL (`wikipedia_url`) and Knowledge Graph MID (`mid`). The table
    ///  below lists the associated fields for entities that have different
    ///  metadata.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        ///  Unspecified.
        Unspecified = 0,
        ///  Person.
        Person = 1,
        ///  Location.
        Location = 2,
        ///  Organization.
        Organization = 3,
        ///  Event.
        Event = 4,
        ///  Artwork.
        WorkOfArt = 5,
        ///  Consumer product.
        ConsumerGood = 6,
        ///  Other types of entities.
        Other = 7,
        ///  Phone number.
        ///
        ///  The metadata lists the phone number (formatted according to local
        ///  convention), plus whichever additional elements appear in the text:
        ///
        ///  * `number` - The actual number, broken down into sections according to
        ///  local convention.
        ///  * `national_prefix` - Country code, if detected.
        ///  * `area_code` - Region or area code, if detected.
        ///  * `extension` - Phone extension (to be dialed after connection), if
        ///  detected.
        PhoneNumber = 9,
        ///  Address.
        ///
        ///  The metadata identifies the street number and locality plus whichever
        ///  additional elements appear in the text:
        ///
        ///  * `street_number` - Street number.
        ///  * `locality` - City or town.
        ///  * `street_name` - Street/route name, if detected.
        ///  * `postal_code` - Postal code, if detected.
        ///  * `country` - Country, if detected.
        ///  * `broad_region` - Administrative area, such as the state, if detected.
        ///  * `narrow_region` - Smaller administrative area, such as county, if
        ///  detected.
        ///  * `sublocality` - Used in Asian addresses to demark a district within a
        ///  city, if detected.
        Address = 10,
        ///  Date.
        ///
        ///  The metadata identifies the components of the date:
        ///
        ///  * `year` - Four digit year, if detected.
        ///  * `month` - Two digit month number, if detected.
        ///  * `day` - Two digit day number, if detected.
        Date = 11,
        ///  Number.
        ///
        ///  The metadata is the number itself.
        Number = 12,
        ///  Price.
        ///
        ///  The metadata identifies the `value` and `currency`.
        Price = 13,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Person => "PERSON",
                Type::Location => "LOCATION",
                Type::Organization => "ORGANIZATION",
                Type::Event => "EVENT",
                Type::WorkOfArt => "WORK_OF_ART",
                Type::ConsumerGood => "CONSUMER_GOOD",
                Type::Other => "OTHER",
                Type::PhoneNumber => "PHONE_NUMBER",
                Type::Address => "ADDRESS",
                Type::Date => "DATE",
                Type::Number => "NUMBER",
                Type::Price => "PRICE",
            }
        }
    }
}
///  The data for an intent.
///  Represents a detected intent in the conversation, for example MAKES_PROMISE.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Intent {
    ///  The unique identifier of the intent.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    ///  The human-readable name of the intent.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
}
///  The data for a matched phrase matcher.
///  Represents information identifying a phrase matcher for a given match.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhraseMatchData {
    ///  The unique identifier (the resource name) of the phrase matcher.
    #[prost(string, tag="1")]
    pub phrase_matcher: ::prost::alloc::string::String,
    ///  The human-readable name of the phrase matcher.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
}
///  The data for a Dialogflow intent.
///  Represents a detected intent in the conversation, e.g. MAKES_PROMISE.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DialogflowIntent {
    ///  The human-readable name of the intent.
    #[prost(string, tag="1")]
    pub display_name: ::prost::alloc::string::String,
}
///  The data for an interruption annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterruptionData {
}
///  The data for a silence annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SilenceData {
}
///  The data for a hold annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HoldData {
}
///  The data for an entity mention annotation.
///  This represents a mention of an `Entity` in the conversation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityMentionData {
    ///  The key of this entity in conversation entities.
    ///  Can be used to retrieve the exact `Entity` this mention is attached to.
    #[prost(string, tag="1")]
    pub entity_unique_id: ::prost::alloc::string::String,
    ///  The type of the entity mention.
    #[prost(enumeration="entity_mention_data::MentionType", tag="2")]
    pub r#type: i32,
    ///  Sentiment expressed for this mention of the entity.
    #[prost(message, optional, tag="3")]
    pub sentiment: ::core::option::Option<SentimentData>,
}
/// Nested message and enum types in `EntityMentionData`.
pub mod entity_mention_data {
    ///  The supported types of mentions.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MentionType {
        ///  Unspecified.
        Unspecified = 0,
        ///  Proper noun.
        Proper = 1,
        ///  Common noun (or noun compound).
        Common = 2,
    }
    impl MentionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MentionType::Unspecified => "MENTION_TYPE_UNSPECIFIED",
                MentionType::Proper => "PROPER",
                MentionType::Common => "COMMON",
            }
        }
    }
}
///  The data for an intent match.
///  Represents an intent match for a text segment in the conversation. A text
///  segment can be part of a sentence, a complete sentence, or an utterance
///  with multiple sentences.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntentMatchData {
    ///  The id of the matched intent.
    ///  Can be used to retrieve the corresponding intent information.
    #[prost(string, tag="1")]
    pub intent_unique_id: ::prost::alloc::string::String,
}
///  The data for a sentiment annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SentimentData {
    ///  A non-negative number from 0 to infinity which represents the abolute
    ///  magnitude of sentiment regardless of score.
    #[prost(float, tag="1")]
    pub magnitude: f32,
    ///  The sentiment score between -1.0 (negative) and 1.0 (positive).
    #[prost(float, tag="2")]
    pub score: f32,
}
///  The issue model resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IssueModel {
    ///  Immutable. The resource name of the issue model.
    ///  Format:
    ///  projects/{project}/locations/{location}/issueModels/{issue_model}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  The representative name for the issue model.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    ///  Output only. The time at which this issue model was created.
    #[prost(message, optional, tag="3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The most recent time at which the issue model was updated.
    #[prost(message, optional, tag="4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. State of the model.
    #[prost(enumeration="issue_model::State", tag="5")]
    pub state: i32,
    ///  Configs for the input data that used to create the issue model.
    #[prost(message, optional, tag="6")]
    pub input_data_config: ::core::option::Option<issue_model::InputDataConfig>,
    ///  Output only. Immutable. The issue model's label statistics on its training data.
    #[prost(message, optional, tag="7")]
    pub training_stats: ::core::option::Option<IssueModelLabelStats>,
}
/// Nested message and enum types in `IssueModel`.
pub mod issue_model {
    ///  Configs for the input data used to create the issue model.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InputDataConfig {
        ///  Medium of conversations used in training data. This field is being
        ///  deprecated. To specify the medium to be used in training a new issue
        ///  model, set the `medium` field on `filter`.
        #[deprecated]
        #[prost(enumeration="super::conversation::Medium", tag="1")]
        pub medium: i32,
        ///  Output only. Number of conversations used in training. Output only.
        #[prost(int64, tag="2")]
        pub training_conversations_count: i64,
        ///  A filter to reduce the conversations used for training the model to a
        ///  specific subset.
        #[prost(string, tag="3")]
        pub filter: ::prost::alloc::string::String,
    }
    ///  State of the model.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        ///  Unspecified.
        Unspecified = 0,
        ///  Model is not deployed but is ready to deploy.
        Undeployed = 1,
        ///  Model is being deployed.
        Deploying = 2,
        ///  Model is deployed and is ready to be used. A model can only be used in
        ///  analysis if it's in this state.
        Deployed = 3,
        ///  Model is being undeployed.
        Undeploying = 4,
        ///  Model is being deleted.
        Deleting = 5,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Undeployed => "UNDEPLOYED",
                State::Deploying => "DEPLOYING",
                State::Deployed => "DEPLOYED",
                State::Undeploying => "UNDEPLOYING",
                State::Deleting => "DELETING",
            }
        }
    }
}
///  The issue resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Issue {
    ///  Immutable. The resource name of the issue.
    ///  Format:
    ///  projects/{project}/locations/{location}/issueModels/{issue_model}/issues/{issue}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  The representative name for the issue.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    ///  Output only. The time at which this issue was created.
    #[prost(message, optional, tag="3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The most recent time that this issue was updated.
    #[prost(message, optional, tag="4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
///  Aggregated statistics about an issue model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IssueModelLabelStats {
    ///  Number of conversations the issue model has analyzed at this point in time.
    #[prost(int64, tag="1")]
    pub analyzed_conversations_count: i64,
    ///  Number of analyzed conversations for which no issue was applicable at this
    ///  point in time.
    #[prost(int64, tag="2")]
    pub unclassified_conversations_count: i64,
    ///  Statistics on each issue. Key is the issue's resource name.
    #[prost(map="string, message", tag="3")]
    pub issue_stats: ::std::collections::HashMap<::prost::alloc::string::String, issue_model_label_stats::IssueStats>,
}
/// Nested message and enum types in `IssueModelLabelStats`.
pub mod issue_model_label_stats {
    ///  Aggregated statistics about an issue.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IssueStats {
        ///  Issue resource.
        ///  Format:
        ///  projects/{project}/locations/{location}/issueModels/{issue_model}/issues/{issue}
        #[prost(string, tag="1")]
        pub issue: ::prost::alloc::string::String,
        ///  Number of conversations attached to the issue at this point in time.
        #[prost(int64, tag="2")]
        pub labeled_conversations_count: i64,
        ///  Display name of the issue.
        #[prost(string, tag="3")]
        pub display_name: ::prost::alloc::string::String,
    }
}
///  The phrase matcher resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhraseMatcher {
    ///  The resource name of the phrase matcher.
    ///  Format:
    ///  projects/{project}/locations/{location}/phraseMatchers/{phrase_matcher}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Output only. Immutable. The revision ID of the phrase matcher.
    ///  A new revision is committed whenever the matcher is changed, except when it
    ///  is activated or deactivated. A server generated random ID will be used.
    ///  Example: locations/global/phraseMatchers/my-first-matcher@1234567
    #[prost(string, tag="2")]
    pub revision_id: ::prost::alloc::string::String,
    ///  The customized version tag to use for the phrase matcher. If not specified,
    ///  it will default to `revision_id`.
    #[prost(string, tag="3")]
    pub version_tag: ::prost::alloc::string::String,
    ///  Output only. The timestamp of when the revision was created. It is also the create time
    ///  when a new matcher is added.
    #[prost(message, optional, tag="4")]
    pub revision_create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  The human-readable name of the phrase matcher.
    #[prost(string, tag="5")]
    pub display_name: ::prost::alloc::string::String,
    ///  Required. The type of this phrase matcher.
    #[prost(enumeration="phrase_matcher::PhraseMatcherType", tag="6")]
    pub r#type: i32,
    ///  Applies the phrase matcher only when it is active.
    #[prost(bool, tag="7")]
    pub active: bool,
    ///  A list of phase match rule groups that are included in this matcher.
    #[prost(message, repeated, tag="8")]
    pub phrase_match_rule_groups: ::prost::alloc::vec::Vec<PhraseMatchRuleGroup>,
    ///  Output only. The most recent time at which the activation status was updated.
    #[prost(message, optional, tag="9")]
    pub activation_update_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  The role whose utterances the phrase matcher should be matched
    ///  against. If the role is ROLE_UNSPECIFIED it will be matched against any
    ///  utterances in the transcript.
    #[prost(enumeration="conversation_participant::Role", tag="10")]
    pub role_match: i32,
    ///  Output only. The most recent time at which the phrase matcher was updated.
    #[prost(message, optional, tag="11")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `PhraseMatcher`.
pub mod phrase_matcher {
    ///  Specifies how to combine each phrase match rule group to determine whether
    ///  there is a match.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PhraseMatcherType {
        ///  Unspecified.
        Unspecified = 0,
        ///  Must meet all phrase match rule groups or there is no match.
        AllOf = 1,
        ///  If any of the phrase match rule groups are met, there is a match.
        AnyOf = 2,
    }
    impl PhraseMatcherType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PhraseMatcherType::Unspecified => "PHRASE_MATCHER_TYPE_UNSPECIFIED",
                PhraseMatcherType::AllOf => "ALL_OF",
                PhraseMatcherType::AnyOf => "ANY_OF",
            }
        }
    }
}
///  A message representing a rule in the phrase matcher.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhraseMatchRuleGroup {
    ///  Required. The type of this phrase match rule group.
    #[prost(enumeration="phrase_match_rule_group::PhraseMatchRuleGroupType", tag="1")]
    pub r#type: i32,
    ///  A list of phase match rules that are included in this group.
    #[prost(message, repeated, tag="2")]
    pub phrase_match_rules: ::prost::alloc::vec::Vec<PhraseMatchRule>,
}
/// Nested message and enum types in `PhraseMatchRuleGroup`.
pub mod phrase_match_rule_group {
    ///  Specifies how to combine each phrase match rule for whether there is a
    ///  match.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PhraseMatchRuleGroupType {
        ///  Unspecified.
        Unspecified = 0,
        ///  Must meet all phrase match rules or there is no match.
        AllOf = 1,
        ///  If any of the phrase match rules are met, there is a match.
        AnyOf = 2,
    }
    impl PhraseMatchRuleGroupType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PhraseMatchRuleGroupType::Unspecified => "PHRASE_MATCH_RULE_GROUP_TYPE_UNSPECIFIED",
                PhraseMatchRuleGroupType::AllOf => "ALL_OF",
                PhraseMatchRuleGroupType::AnyOf => "ANY_OF",
            }
        }
    }
}
///  The data for a phrase match rule.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhraseMatchRule {
    ///  Required. The phrase to be matched.
    #[prost(string, tag="1")]
    pub query: ::prost::alloc::string::String,
    ///  Specifies whether the phrase must be missing from the transcript segment or
    ///  present in the transcript segment.
    #[prost(bool, tag="2")]
    pub negated: bool,
    ///  Provides additional information about the rule that specifies how to apply
    ///  the rule.
    #[prost(message, optional, tag="3")]
    pub config: ::core::option::Option<PhraseMatchRuleConfig>,
}
///  Configuration information of a phrase match rule.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhraseMatchRuleConfig {
    ///  The configuration of the phrase match rule.
    #[prost(oneof="phrase_match_rule_config::Config", tags="1")]
    pub config: ::core::option::Option<phrase_match_rule_config::Config>,
}
/// Nested message and enum types in `PhraseMatchRuleConfig`.
pub mod phrase_match_rule_config {
    ///  The configuration of the phrase match rule.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Config {
        ///  The configuration for the exact match rule.
        #[prost(message, tag="1")]
        ExactMatchConfig(super::ExactMatchConfig),
    }
}
///  Exact match configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExactMatchConfig {
    ///  Whether to consider case sensitivity when performing an exact match.
    #[prost(bool, tag="1")]
    pub case_sensitive: bool,
}
///  The settings resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Settings {
    ///  Immutable. The resource name of the settings resource.
    ///  Format:
    ///  projects/{project}/locations/{location}/settings
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Output only. The time at which the settings was created.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The time at which the settings were last updated.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  A language code to be applied to each transcript segment unless the segment
    ///  already specifies a language code. Language code defaults to "en-US" if it
    ///  is neither specified on the segment nor here.
    #[prost(string, tag="4")]
    pub language_code: ::prost::alloc::string::String,
    ///  The default TTL for newly-created conversations. If a conversation has a
    ///  specified expiration, that value will be used instead. Changing this
    ///  value will not change the expiration of existing conversations.
    ///  Conversations with no expire time persist until they are deleted.
    #[prost(message, optional, tag="5")]
    pub conversation_ttl: ::core::option::Option<::prost_types::Duration>,
    ///  A map that maps a notification trigger to a Pub/Sub topic. Each time a
    ///  specified trigger occurs, Insights will notify the corresponding Pub/Sub
    ///  topic.
    ///
    ///  Keys are notification triggers. Supported keys are:
    ///
    ///  * "all-triggers": Notify each time any of the supported triggers occurs.
    ///  * "create-analysis": Notify each time an analysis is created.
    ///  * "create-conversation": Notify each time a conversation is created.
    ///  * "export-insights-data": Notify each time an export is complete.
    ///  * "update-conversation": Notify each time a conversation is updated via
    ///  UpdateConversation.
    ///
    ///  Values are Pub/Sub topics. The format of each Pub/Sub topic is:
    ///  projects/{project}/topics/{topic}
    #[prost(map="string, string", tag="6")]
    pub pubsub_notification_settings: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  Default analysis settings.
    #[prost(message, optional, tag="7")]
    pub analysis_config: ::core::option::Option<settings::AnalysisConfig>,
}
/// Nested message and enum types in `Settings`.
pub mod settings {
    ///  Default configuration when creating Analyses in Insights.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AnalysisConfig {
        ///  Percentage of conversations created using Dialogflow runtime integration
        ///  to analyze automatically, between [0, 100].
        #[prost(double, tag="1")]
        pub runtime_integration_analysis_percentage: f64,
    }
}
///  An annotation that was generated during the customer and agent interaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeAnnotation {
    ///  The unique identifier of the annotation.
    ///  Format:
    ///  projects/{project}/locations/{location}/conversationDatasets/{dataset}/conversationDataItems/{data_item}/conversationAnnotations/{annotation}
    #[prost(string, tag="1")]
    pub annotation_id: ::prost::alloc::string::String,
    ///  The time at which this annotation was created.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  The boundary in the conversation where the annotation starts, inclusive.
    #[prost(message, optional, tag="3")]
    pub start_boundary: ::core::option::Option<AnnotationBoundary>,
    ///  The boundary in the conversation where the annotation ends, inclusive.
    #[prost(message, optional, tag="4")]
    pub end_boundary: ::core::option::Option<AnnotationBoundary>,
    ///  The feedback that the customer has about the answer in `data`.
    #[prost(message, optional, tag="5")]
    pub answer_feedback: ::core::option::Option<AnswerFeedback>,
    ///  The data in the annotation.
    #[prost(oneof="runtime_annotation::Data", tags="6, 7, 8, 9, 10")]
    pub data: ::core::option::Option<runtime_annotation::Data>,
}
/// Nested message and enum types in `RuntimeAnnotation`.
pub mod runtime_annotation {
    ///  The data in the annotation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        ///  Agent Assist Article Suggestion data.
        #[prost(message, tag="6")]
        ArticleSuggestion(super::ArticleSuggestionData),
        ///  Agent Assist FAQ answer data.
        #[prost(message, tag="7")]
        FaqAnswer(super::FaqAnswerData),
        ///  Agent Assist Smart Reply data.
        #[prost(message, tag="8")]
        SmartReply(super::SmartReplyData),
        ///  Agent Assist Smart Compose suggestion data.
        #[prost(message, tag="9")]
        SmartComposeSuggestion(super::SmartComposeSuggestionData),
        ///  Dialogflow interaction data.
        #[prost(message, tag="10")]
        DialogflowInteraction(super::DialogflowInteractionData),
    }
}
///  The feedback that the customer has about a certain answer in the
///  conversation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnswerFeedback {
    ///  The correctness level of an answer.
    #[prost(enumeration="answer_feedback::CorrectnessLevel", tag="1")]
    pub correctness_level: i32,
    ///  Indicates whether an answer or item was clicked by the human agent.
    #[prost(bool, tag="2")]
    pub clicked: bool,
    ///  Indicates whether an answer or item was displayed to the human agent in the
    ///  agent desktop UI.
    #[prost(bool, tag="3")]
    pub displayed: bool,
}
/// Nested message and enum types in `AnswerFeedback`.
pub mod answer_feedback {
    ///  The correctness level of an answer.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CorrectnessLevel {
        ///  Correctness level unspecified.
        Unspecified = 0,
        ///  Answer is totally wrong.
        NotCorrect = 1,
        ///  Answer is partially correct.
        PartiallyCorrect = 2,
        ///  Answer is fully correct.
        FullyCorrect = 3,
    }
    impl CorrectnessLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CorrectnessLevel::Unspecified => "CORRECTNESS_LEVEL_UNSPECIFIED",
                CorrectnessLevel::NotCorrect => "NOT_CORRECT",
                CorrectnessLevel::PartiallyCorrect => "PARTIALLY_CORRECT",
                CorrectnessLevel::FullyCorrect => "FULLY_CORRECT",
            }
        }
    }
}
///  Agent Assist Article Suggestion data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArticleSuggestionData {
    ///  Article title.
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    ///  Article URI.
    #[prost(string, tag="2")]
    pub uri: ::prost::alloc::string::String,
    ///  The system's confidence score that this article is a good match for this
    ///  conversation, ranging from 0.0 (completely uncertain) to 1.0 (completely
    ///  certain).
    #[prost(float, tag="3")]
    pub confidence_score: f32,
    ///  Map that contains metadata about the Article Suggestion and the document
    ///  that it originates from.
    #[prost(map="string, string", tag="4")]
    pub metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  The name of the answer record.
    ///  Format:
    ///  projects/{project}/locations/{location}/answerRecords/{answer_record}
    #[prost(string, tag="5")]
    pub query_record: ::prost::alloc::string::String,
    ///  The knowledge document that this answer was extracted from.
    ///  Format:
    ///  projects/{project}/knowledgeBases/{knowledge_base}/documents/{document}
    #[prost(string, tag="6")]
    pub source: ::prost::alloc::string::String,
}
///  Agent Assist frequently-asked-question answer data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaqAnswerData {
    ///  The piece of text from the `source` knowledge base document.
    #[prost(string, tag="1")]
    pub answer: ::prost::alloc::string::String,
    ///  The system's confidence score that this answer is a good match for this
    ///  conversation, ranging from 0.0 (completely uncertain) to 1.0 (completely
    ///  certain).
    #[prost(float, tag="2")]
    pub confidence_score: f32,
    ///  The corresponding FAQ question.
    #[prost(string, tag="3")]
    pub question: ::prost::alloc::string::String,
    ///  Map that contains metadata about the FAQ answer and the document that
    ///  it originates from.
    #[prost(map="string, string", tag="4")]
    pub metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  The name of the answer record.
    ///  Format:
    ///  projects/{project}/locations/{location}/answerRecords/{answer_record}
    #[prost(string, tag="5")]
    pub query_record: ::prost::alloc::string::String,
    ///  The knowledge document that this answer was extracted from.
    ///  Format:
    ///  projects/{project}/knowledgeBases/{knowledge_base}/documents/{document}.
    #[prost(string, tag="6")]
    pub source: ::prost::alloc::string::String,
}
///  Agent Assist Smart Reply data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmartReplyData {
    ///  The content of the reply.
    #[prost(string, tag="1")]
    pub reply: ::prost::alloc::string::String,
    ///  The system's confidence score that this reply is a good match for this
    ///  conversation, ranging from 0.0 (completely uncertain) to 1.0 (completely
    ///  certain).
    #[prost(double, tag="2")]
    pub confidence_score: f64,
    ///  Map that contains metadata about the Smart Reply and the document from
    ///  which it originates.
    #[prost(map="string, string", tag="3")]
    pub metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  The name of the answer record.
    ///  Format:
    ///  projects/{project}/locations/{location}/answerRecords/{answer_record}
    #[prost(string, tag="4")]
    pub query_record: ::prost::alloc::string::String,
}
///  Agent Assist Smart Compose suggestion data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmartComposeSuggestionData {
    ///  The content of the suggestion.
    #[prost(string, tag="1")]
    pub suggestion: ::prost::alloc::string::String,
    ///  The system's confidence score that this suggestion is a good match for this
    ///  conversation, ranging from 0.0 (completely uncertain) to 1.0 (completely
    ///  certain).
    #[prost(double, tag="2")]
    pub confidence_score: f64,
    ///  Map that contains metadata about the Smart Compose suggestion and the
    ///  document from which it originates.
    #[prost(map="string, string", tag="3")]
    pub metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  The name of the answer record.
    ///  Format:
    ///  projects/{project}/locations/{location}/answerRecords/{answer_record}
    #[prost(string, tag="4")]
    pub query_record: ::prost::alloc::string::String,
}
///  Dialogflow interaction data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DialogflowInteractionData {
    ///  The Dialogflow intent resource path. Format:
    ///  projects/{project}/agent/{agent}/intents/{intent}
    #[prost(string, tag="1")]
    pub dialogflow_intent_id: ::prost::alloc::string::String,
    ///  The confidence of the match ranging from 0.0 (completely uncertain) to 1.0
    ///  (completely certain).
    #[prost(float, tag="2")]
    pub confidence: f32,
}
///  The call participant speaking for a given utterance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversationParticipant {
    ///  Deprecated. Use `dialogflow_participant_name` instead.
    ///  The name of the Dialogflow participant. Format:
    ///  projects/{project}/locations/{location}/conversations/{conversation}/participants/{participant}
    #[deprecated]
    #[prost(string, tag="1")]
    pub dialogflow_participant: ::prost::alloc::string::String,
    ///  Obfuscated user ID from Dialogflow.
    #[prost(string, tag="3")]
    pub obfuscated_external_user_id: ::prost::alloc::string::String,
    ///  The role of the participant.
    #[prost(enumeration="conversation_participant::Role", tag="2")]
    pub role: i32,
    #[prost(oneof="conversation_participant::Participant", tags="5, 6")]
    pub participant: ::core::option::Option<conversation_participant::Participant>,
}
/// Nested message and enum types in `ConversationParticipant`.
pub mod conversation_participant {
    ///  The role of the participant.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Role {
        ///  Participant's role is not set.
        Unspecified = 0,
        ///  Participant is a human agent.
        HumanAgent = 1,
        ///  Participant is an automated agent.
        AutomatedAgent = 2,
        ///  Participant is an end user who conversed with the contact center.
        EndUser = 3,
        ///  Participant is either a human or automated agent.
        AnyAgent = 4,
    }
    impl Role {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Role::Unspecified => "ROLE_UNSPECIFIED",
                Role::HumanAgent => "HUMAN_AGENT",
                Role::AutomatedAgent => "AUTOMATED_AGENT",
                Role::EndUser => "END_USER",
                Role::AnyAgent => "ANY_AGENT",
            }
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Participant {
        ///  The name of the participant provided by Dialogflow. Format:
        ///  projects/{project}/locations/{location}/conversations/{conversation}/participants/{participant}
        #[prost(string, tag="5")]
        DialogflowParticipantName(::prost::alloc::string::String),
        ///  A user-specified ID representing the participant.
        #[prost(string, tag="6")]
        UserId(::prost::alloc::string::String),
    }
}
///  The View resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct View {
    ///  Immutable. The resource name of the view.
    ///  Format:
    ///  projects/{project}/locations/{location}/views/{view}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  The human-readable display name of the view.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    ///  Output only. The time at which this view was created.
    #[prost(message, optional, tag="3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The most recent time at which the view was updated.
    #[prost(message, optional, tag="4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  String with specific view properties.
    #[prost(string, tag="5")]
    pub value: ::prost::alloc::string::String,
}
///  The request for calculating conversation statistics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalculateStatsRequest {
    ///  Required. The location of the conversations.
    #[prost(string, tag="1")]
    pub location: ::prost::alloc::string::String,
    ///  A filter to reduce results to a specific subset. This field is useful for
    ///  getting statistics about conversations with specific properties.
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
}
///  The response for calculating conversation statistics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalculateStatsResponse {
    ///  The average duration of all conversations. The average is calculated using
    ///  only conversations that have a time duration.
    #[prost(message, optional, tag="1")]
    pub average_duration: ::core::option::Option<::prost_types::Duration>,
    ///  The average number of turns per conversation.
    #[prost(int32, tag="2")]
    pub average_turn_count: i32,
    ///  The total number of conversations.
    #[prost(int32, tag="3")]
    pub conversation_count: i32,
    ///  A map associating each smart highlighter display name with its respective
    ///  number of matches in the set of conversations.
    #[prost(map="string, int32", tag="4")]
    pub smart_highlighter_matches: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
    ///  A map associating each custom highlighter resource name with its respective
    ///  number of matches in the set of conversations.
    #[prost(map="string, int32", tag="5")]
    pub custom_highlighter_matches: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
    ///  A map associating each issue resource name with its respective number of
    ///  matches in the set of conversations. Key has the format:
    ///  `projects/<Project-ID>/locations/<Location-ID>/issueModels/<Issue-Model-ID>/issues/<Issue-ID>`
    ///  Deprecated, use `issue_matches_stats` field instead.
    #[prost(map="string, int32", tag="6")]
    pub issue_matches: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
    ///  A map associating each issue resource name with its respective number of
    ///  matches in the set of conversations. Key has the format:
    ///  `projects/<Project-ID>/locations/<Location-ID>/issueModels/<Issue-Model-ID>/issues/<Issue-ID>`
    #[prost(map="string, message", tag="8")]
    pub issue_matches_stats: ::std::collections::HashMap<::prost::alloc::string::String, issue_model_label_stats::IssueStats>,
    ///  A time series representing the count of conversations created over time
    ///  that match that requested filter criteria.
    #[prost(message, optional, tag="7")]
    pub conversation_count_time_series: ::core::option::Option<calculate_stats_response::TimeSeries>,
}
/// Nested message and enum types in `CalculateStatsResponse`.
pub mod calculate_stats_response {
    ///  A time series representing conversations over time.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TimeSeries {
        ///  The duration of each interval.
        #[prost(message, optional, tag="1")]
        pub interval_duration: ::core::option::Option<::prost_types::Duration>,
        ///  An ordered list of intervals from earliest to latest, where each interval
        ///  represents the number of conversations that transpired during the time
        ///  window.
        #[prost(message, repeated, tag="2")]
        pub points: ::prost::alloc::vec::Vec<time_series::Interval>,
    }
    /// Nested message and enum types in `TimeSeries`.
    pub mod time_series {
        ///  A single interval in a time series.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Interval {
            ///  The start time of this interval.
            #[prost(message, optional, tag="1")]
            pub start_time: ::core::option::Option<::prost_types::Timestamp>,
            ///  The number of conversations created in this interval.
            #[prost(int32, tag="2")]
            pub conversation_count: i32,
        }
    }
}
///  Metadata for a create analysis operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAnalysisOperationMetadata {
    ///  Output only. The time the operation was created.
    #[prost(message, optional, tag="1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The time the operation finished running.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The Conversation that this Analysis Operation belongs to.
    #[prost(string, tag="3")]
    pub conversation: ::prost::alloc::string::String,
}
///  Request to create a conversation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConversationRequest {
    ///  Required. The parent resource of the conversation.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Required. The conversation resource to create.
    #[prost(message, optional, tag="2")]
    pub conversation: ::core::option::Option<Conversation>,
    ///  A unique ID for the new conversation. This ID will become the final
    ///  component of the conversation's resource name. If no ID is specified, a
    ///  server-generated ID will be used.
    ///
    ///  This value should be 4-64 characters and must match the regular
    ///  expression `^\[a-z0-9-\]{4,64}$`. Valid characters are `\[a-z][0-9\]-`
    #[prost(string, tag="3")]
    pub conversation_id: ::prost::alloc::string::String,
}
///  Request to list conversations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConversationsRequest {
    ///  Required. The parent resource of the conversation.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  The maximum number of conversations to return in the response. A valid page
    ///  size ranges from 0 to 1,000 inclusive. If the page size is zero or
    ///  unspecified, a default page size of 100 will be chosen. Note that a call
    ///  might return fewer results than the requested page size.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    ///  The value returned by the last `ListConversationsResponse`. This value
    ///  indicates that this is a continuation of a prior `ListConversations` call
    ///  and that the system should return the next page of data.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    ///  A filter to reduce results to a specific subset. Useful for querying
    ///  conversations with specific properties.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    ///  The level of details of the conversation. Default is `BASIC`.
    #[prost(enumeration="ConversationView", tag="5")]
    pub view: i32,
}
///  The response of listing conversations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConversationsResponse {
    ///  The conversations that match the request.
    #[prost(message, repeated, tag="1")]
    pub conversations: ::prost::alloc::vec::Vec<Conversation>,
    ///  A token which can be sent as `page_token` to retrieve the next page. If
    ///  this field is set, it means there is another page available. If it is not
    ///  set, it means no other pages are available.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  The request to get a conversation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConversationRequest {
    ///  Required. The name of the conversation to get.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  The level of details of the conversation. Default is `FULL`.
    #[prost(enumeration="ConversationView", tag="2")]
    pub view: i32,
}
///  The request to update a conversation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConversationRequest {
    ///  Required. The new values for the conversation.
    #[prost(message, optional, tag="1")]
    pub conversation: ::core::option::Option<Conversation>,
    ///  The list of fields to be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
///  The request to delete a conversation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConversationRequest {
    ///  Required. The name of the conversation to delete.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  If set to true, all of this conversation's analyses will also be deleted.
    ///  Otherwise, the request will only succeed if the conversation has no
    ///  analyses.
    #[prost(bool, tag="2")]
    pub force: bool,
}
///  The request to create an analysis.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAnalysisRequest {
    ///  Required. The parent resource of the analysis.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Required. The analysis to create.
    #[prost(message, optional, tag="2")]
    pub analysis: ::core::option::Option<Analysis>,
}
///  The request to list analyses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAnalysesRequest {
    ///  Required. The parent resource of the analyses.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  The maximum number of analyses to return in the response. If this
    ///  value is zero, the service will select a default size. A call might return
    ///  fewer objects than requested. A non-empty `next_page_token` in the response
    ///  indicates that more data is available.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    ///  The value returned by the last `ListAnalysesResponse`; indicates
    ///  that this is a continuation of a prior `ListAnalyses` call and
    ///  the system should return the next page of data.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    ///  A filter to reduce results to a specific subset. Useful for querying
    ///  conversations with specific properties.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
}
///  The response to list analyses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAnalysesResponse {
    ///  The analyses that match the request.
    #[prost(message, repeated, tag="1")]
    pub analyses: ::prost::alloc::vec::Vec<Analysis>,
    ///  A token, which can be sent as `page_token` to retrieve the next page.
    ///  If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  The request to get an analysis.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAnalysisRequest {
    ///  Required. The name of the analysis to get.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  The request to delete an analysis.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAnalysisRequest {
    ///  Required. The name of the analysis to delete.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  The request to export insights.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportInsightsDataRequest {
    ///  Required. The parent resource to export data from.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  A filter to reduce results to a specific subset. Useful for exporting
    ///  conversations with specific properties.
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
    ///  A fully qualified KMS key name for BigQuery tables protected by CMEK.
    ///  Format:
    ///  projects/{project}/locations/{location}/keyRings/{keyring}/cryptoKeys/{key}/cryptoKeyVersions/{version}
    #[prost(string, tag="4")]
    pub kms_key: ::prost::alloc::string::String,
    ///  Options for what to do if the destination table already exists.
    #[prost(enumeration="export_insights_data_request::WriteDisposition", tag="5")]
    pub write_disposition: i32,
    ///  Exporter destination.
    #[prost(oneof="export_insights_data_request::Destination", tags="2")]
    pub destination: ::core::option::Option<export_insights_data_request::Destination>,
}
/// Nested message and enum types in `ExportInsightsDataRequest`.
pub mod export_insights_data_request {
    ///  A BigQuery Table Reference.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BigQueryDestination {
        ///  A project ID or number. If specified, then export will attempt to
        ///  write data to this project instead of the resource project. Otherwise,
        ///  the resource project will be used.
        #[prost(string, tag="3")]
        pub project_id: ::prost::alloc::string::String,
        ///  Required. The name of the BigQuery dataset that the snapshot result should be
        ///  exported to. If this dataset does not exist, the export call returns an
        ///  INVALID_ARGUMENT error.
        #[prost(string, tag="1")]
        pub dataset: ::prost::alloc::string::String,
        ///  The BigQuery table name to which the insights data should be written.
        ///  If this table does not exist, the export call returns an INVALID_ARGUMENT
        ///  error.
        #[prost(string, tag="2")]
        pub table: ::prost::alloc::string::String,
    }
    ///  Specifies the action that occurs if the destination table already exists.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum WriteDisposition {
        ///  Write disposition is not specified. Defaults to WRITE_TRUNCATE.
        Unspecified = 0,
        ///  If the table already exists, BigQuery will overwrite the table data and
        ///  use the schema from the load.
        WriteTruncate = 1,
        ///  If the table already exists, BigQuery will append data to the table.
        WriteAppend = 2,
    }
    impl WriteDisposition {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WriteDisposition::Unspecified => "WRITE_DISPOSITION_UNSPECIFIED",
                WriteDisposition::WriteTruncate => "WRITE_TRUNCATE",
                WriteDisposition::WriteAppend => "WRITE_APPEND",
            }
        }
    }
    ///  Exporter destination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        ///  Specified if sink is a BigQuery table.
        #[prost(message, tag="2")]
        BigQueryDestination(BigQueryDestination),
    }
}
///  Metadata for an export insights operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportInsightsDataMetadata {
    ///  Output only. The time the operation was created.
    #[prost(message, optional, tag="1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The time the operation finished running.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  The original request for export.
    #[prost(message, optional, tag="3")]
    pub request: ::core::option::Option<ExportInsightsDataRequest>,
    ///  Partial errors during export operation that might cause the operation
    ///  output to be incomplete.
    #[prost(message, repeated, tag="4")]
    pub partial_errors: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
}
///  Response for an export insights operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportInsightsDataResponse {
}
///  The request to create an issue model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIssueModelRequest {
    ///  Required. The parent resource of the issue model.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Required. The issue model to create.
    #[prost(message, optional, tag="2")]
    pub issue_model: ::core::option::Option<IssueModel>,
}
///  Metadata for creating an issue model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIssueModelMetadata {
    ///  Output only. The time the operation was created.
    #[prost(message, optional, tag="1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The time the operation finished running.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  The original request for creation.
    #[prost(message, optional, tag="3")]
    pub request: ::core::option::Option<CreateIssueModelRequest>,
}
///  The request to update an issue model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateIssueModelRequest {
    ///  Required. The new values for the issue model.
    #[prost(message, optional, tag="1")]
    pub issue_model: ::core::option::Option<IssueModel>,
    ///  The list of fields to be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
///  Request to list issue models.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIssueModelsRequest {
    ///  Required. The parent resource of the issue model.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
}
///  The response of listing issue models.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIssueModelsResponse {
    ///  The issue models that match the request.
    #[prost(message, repeated, tag="1")]
    pub issue_models: ::prost::alloc::vec::Vec<IssueModel>,
}
///  The request to get an issue model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIssueModelRequest {
    ///  Required. The name of the issue model to get.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  The request to delete an issue model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteIssueModelRequest {
    ///  Required. The name of the issue model to delete.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Metadata for deleting an issue model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteIssueModelMetadata {
    ///  Output only. The time the operation was created.
    #[prost(message, optional, tag="1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The time the operation finished running.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  The original request for deletion.
    #[prost(message, optional, tag="3")]
    pub request: ::core::option::Option<DeleteIssueModelRequest>,
}
///  The request to deploy an issue model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployIssueModelRequest {
    ///  Required. The issue model to deploy.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  The response to deploy an issue model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployIssueModelResponse {
}
///  Metadata for deploying an issue model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployIssueModelMetadata {
    ///  Output only. The time the operation was created.
    #[prost(message, optional, tag="1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The time the operation finished running.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  The original request for deployment.
    #[prost(message, optional, tag="3")]
    pub request: ::core::option::Option<DeployIssueModelRequest>,
}
///  The request to undeploy an issue model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeployIssueModelRequest {
    ///  Required. The issue model to undeploy.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  The response to undeploy an issue model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeployIssueModelResponse {
}
///  Metadata for undeploying an issue model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeployIssueModelMetadata {
    ///  Output only. The time the operation was created.
    #[prost(message, optional, tag="1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The time the operation finished running.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  The original request for undeployment.
    #[prost(message, optional, tag="3")]
    pub request: ::core::option::Option<UndeployIssueModelRequest>,
}
///  The request to get an issue.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIssueRequest {
    ///  Required. The name of the issue to get.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Request to list issues.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIssuesRequest {
    ///  Required. The parent resource of the issue.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
}
///  The response of listing issues.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIssuesResponse {
    ///  The issues that match the request.
    #[prost(message, repeated, tag="1")]
    pub issues: ::prost::alloc::vec::Vec<Issue>,
}
///  The request to update an issue.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateIssueRequest {
    ///  Required. The new values for the issue.
    #[prost(message, optional, tag="1")]
    pub issue: ::core::option::Option<Issue>,
    ///  The list of fields to be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
///  Request to get statistics of an issue model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalculateIssueModelStatsRequest {
    ///  Required. The resource name of the issue model to query against.
    #[prost(string, tag="1")]
    pub issue_model: ::prost::alloc::string::String,
}
///  Response of querying an issue model's statistics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalculateIssueModelStatsResponse {
    ///  The latest label statistics for the queried issue model. Includes results
    ///  on both training data and data labeled after deployment.
    #[prost(message, optional, tag="4")]
    pub current_stats: ::core::option::Option<IssueModelLabelStats>,
}
///  Request to create a phrase matcher.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePhraseMatcherRequest {
    ///  Required. The parent resource of the phrase matcher. Required. The location to create
    ///  a phrase matcher for.
    ///  Format: `projects/<Project ID>/locations/<Location ID>` or
    ///  `projects/<Project Number>/locations/<Location ID>`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Required. The phrase matcher resource to create.
    #[prost(message, optional, tag="2")]
    pub phrase_matcher: ::core::option::Option<PhraseMatcher>,
}
///  Request to list phrase matchers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPhraseMatchersRequest {
    ///  Required. The parent resource of the phrase matcher.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  The maximum number of phrase matchers to return in the response. If this
    ///  value is zero, the service will select a default size. A call might return
    ///  fewer objects than requested. A non-empty `next_page_token` in the response
    ///  indicates that more data is available.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    ///  The value returned by the last `ListPhraseMatchersResponse`. This value
    ///  indicates that this is a continuation of a prior `ListPhraseMatchers` call
    ///  and that the system should return the next page of data.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    ///  A filter to reduce results to a specific subset. Useful for querying
    ///  phrase matchers with specific properties.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
}
///  The response of listing phrase matchers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPhraseMatchersResponse {
    ///  The phrase matchers that match the request.
    #[prost(message, repeated, tag="1")]
    pub phrase_matchers: ::prost::alloc::vec::Vec<PhraseMatcher>,
    ///  A token, which can be sent as `page_token` to retrieve the next page.
    ///  If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  The request to get a a phrase matcher.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPhraseMatcherRequest {
    ///  Required. The name of the phrase matcher to get.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  The request to delete a phrase matcher.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePhraseMatcherRequest {
    ///  Required. The name of the phrase matcher to delete.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  The request to update a phrase matcher.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePhraseMatcherRequest {
    ///  Required. The new values for the phrase matcher.
    #[prost(message, optional, tag="1")]
    pub phrase_matcher: ::core::option::Option<PhraseMatcher>,
    ///  The list of fields to be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
///  The request to get project-level settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSettingsRequest {
    ///  Required. The name of the settings resource to get.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  The request to update project-level settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSettingsRequest {
    ///  Required. The new settings values.
    #[prost(message, optional, tag="1")]
    pub settings: ::core::option::Option<Settings>,
    ///  Required. The list of fields to be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
///  The request to create a view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateViewRequest {
    ///  Required. The parent resource of the view. Required. The location to create
    ///  a view for.
    ///  Format: `projects/<Project ID>/locations/<Location ID>` or
    ///  `projects/<Project Number>/locations/<Location ID>`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Required. The view resource to create.
    #[prost(message, optional, tag="2")]
    pub view: ::core::option::Option<View>,
}
///  The request to get a view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetViewRequest {
    ///  Required. The name of the view to get.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  The request to list views.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListViewsRequest {
    ///  Required. The parent resource of the views.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  The maximum number of views to return in the response. If this
    ///  value is zero, the service will select a default size. A call may return
    ///  fewer objects than requested. A non-empty `next_page_token` in the response
    ///  indicates that more data is available.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    ///  The value returned by the last `ListViewsResponse`; indicates
    ///  that this is a continuation of a prior `ListViews` call and
    ///  the system should return the next page of data.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
///  The response of listing views.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListViewsResponse {
    ///  The views that match the request.
    #[prost(message, repeated, tag="1")]
    pub views: ::prost::alloc::vec::Vec<View>,
    ///  A token, which can be sent as `page_token` to retrieve the next page.
    ///  If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  The request to update a view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateViewRequest {
    ///  Required. The new view.
    #[prost(message, optional, tag="1")]
    pub view: ::core::option::Option<View>,
    ///  The list of fields to be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
///  The request to delete a view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteViewRequest {
    ///  Required. The name of the view to delete.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Represents the options for viewing a conversation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConversationView {
    ///  The conversation view is not specified.
    ///
    ///  * Defaults to `FULL` in `GetConversationRequest`.
    ///  * Defaults to `BASIC` in `ListConversationsRequest`.
    Unspecified = 0,
    ///  Populates all fields in the conversation.
    Full = 2,
    ///  Populates all fields in the conversation except the transcript.
    Basic = 1,
}
impl ConversationView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConversationView::Unspecified => "CONVERSATION_VIEW_UNSPECIFIED",
            ConversationView::Full => "FULL",
            ConversationView::Basic => "BASIC",
        }
    }
}
/// Generated client implementations.
pub mod contact_center_insights_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// An API that lets users analyze and explore their business conversation data.
    #[derive(Debug, Clone)]
    pub struct ContactCenterInsightsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ContactCenterInsightsClient<tonic::transport::Channel> {
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
    impl<T> ContactCenterInsightsClient<T>
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
        ) -> ContactCenterInsightsClient<InterceptedService<T, F>>
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
            ContactCenterInsightsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a conversation.
        pub async fn create_conversation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConversationRequest>,
        ) -> Result<tonic::Response<super::Conversation>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/CreateConversation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a conversation.
        pub async fn update_conversation(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConversationRequest>,
        ) -> Result<tonic::Response<super::Conversation>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/UpdateConversation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a conversation.
        pub async fn get_conversation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConversationRequest>,
        ) -> Result<tonic::Response<super::Conversation>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/GetConversation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists conversations.
        pub async fn list_conversations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConversationsRequest>,
        ) -> Result<tonic::Response<super::ListConversationsResponse>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/ListConversations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a conversation.
        pub async fn delete_conversation(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConversationRequest>,
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/DeleteConversation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates an analysis. The long running operation is done when the analysis
        /// has completed.
        pub async fn create_analysis(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAnalysisRequest>,
        ) -> Result<
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/CreateAnalysis",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets an analysis.
        pub async fn get_analysis(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAnalysisRequest>,
        ) -> Result<tonic::Response<super::Analysis>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/GetAnalysis",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists analyses.
        pub async fn list_analyses(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAnalysesRequest>,
        ) -> Result<tonic::Response<super::ListAnalysesResponse>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/ListAnalyses",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes an analysis.
        pub async fn delete_analysis(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAnalysisRequest>,
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/DeleteAnalysis",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Export insights data to a destination defined in the request body.
        pub async fn export_insights_data(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportInsightsDataRequest>,
        ) -> Result<
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/ExportInsightsData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates an issue model.
        pub async fn create_issue_model(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateIssueModelRequest>,
        ) -> Result<
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/CreateIssueModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an issue model.
        pub async fn update_issue_model(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateIssueModelRequest>,
        ) -> Result<tonic::Response<super::IssueModel>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/UpdateIssueModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets an issue model.
        pub async fn get_issue_model(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIssueModelRequest>,
        ) -> Result<tonic::Response<super::IssueModel>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/GetIssueModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists issue models.
        pub async fn list_issue_models(
            &mut self,
            request: impl tonic::IntoRequest<super::ListIssueModelsRequest>,
        ) -> Result<tonic::Response<super::ListIssueModelsResponse>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/ListIssueModels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes an issue model.
        pub async fn delete_issue_model(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteIssueModelRequest>,
        ) -> Result<
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/DeleteIssueModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deploys an issue model. Returns an error if a model is already deployed.
        /// An issue model can only be used in analysis after it has been deployed.
        pub async fn deploy_issue_model(
            &mut self,
            request: impl tonic::IntoRequest<super::DeployIssueModelRequest>,
        ) -> Result<
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/DeployIssueModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Undeploys an issue model.
        /// An issue model can not be used in analysis after it has been undeployed.
        pub async fn undeploy_issue_model(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeployIssueModelRequest>,
        ) -> Result<
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/UndeployIssueModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets an issue.
        pub async fn get_issue(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIssueRequest>,
        ) -> Result<tonic::Response<super::Issue>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/GetIssue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists issues.
        pub async fn list_issues(
            &mut self,
            request: impl tonic::IntoRequest<super::ListIssuesRequest>,
        ) -> Result<tonic::Response<super::ListIssuesResponse>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/ListIssues",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an issue.
        pub async fn update_issue(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateIssueRequest>,
        ) -> Result<tonic::Response<super::Issue>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/UpdateIssue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets an issue model's statistics.
        pub async fn calculate_issue_model_stats(
            &mut self,
            request: impl tonic::IntoRequest<super::CalculateIssueModelStatsRequest>,
        ) -> Result<
            tonic::Response<super::CalculateIssueModelStatsResponse>,
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/CalculateIssueModelStats",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a phrase matcher.
        pub async fn create_phrase_matcher(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePhraseMatcherRequest>,
        ) -> Result<tonic::Response<super::PhraseMatcher>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/CreatePhraseMatcher",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a phrase matcher.
        pub async fn get_phrase_matcher(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPhraseMatcherRequest>,
        ) -> Result<tonic::Response<super::PhraseMatcher>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/GetPhraseMatcher",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists phrase matchers.
        pub async fn list_phrase_matchers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPhraseMatchersRequest>,
        ) -> Result<tonic::Response<super::ListPhraseMatchersResponse>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/ListPhraseMatchers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a phrase matcher.
        pub async fn delete_phrase_matcher(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePhraseMatcherRequest>,
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/DeletePhraseMatcher",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a phrase matcher.
        pub async fn update_phrase_matcher(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePhraseMatcherRequest>,
        ) -> Result<tonic::Response<super::PhraseMatcher>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/UpdatePhraseMatcher",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets conversation statistics.
        pub async fn calculate_stats(
            &mut self,
            request: impl tonic::IntoRequest<super::CalculateStatsRequest>,
        ) -> Result<tonic::Response<super::CalculateStatsResponse>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/CalculateStats",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets project-level settings.
        pub async fn get_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSettingsRequest>,
        ) -> Result<tonic::Response<super::Settings>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/GetSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates project-level settings.
        pub async fn update_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSettingsRequest>,
        ) -> Result<tonic::Response<super::Settings>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/UpdateSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a view.
        pub async fn create_view(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateViewRequest>,
        ) -> Result<tonic::Response<super::View>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/CreateView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a view.
        pub async fn get_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetViewRequest>,
        ) -> Result<tonic::Response<super::View>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/GetView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists views.
        pub async fn list_views(
            &mut self,
            request: impl tonic::IntoRequest<super::ListViewsRequest>,
        ) -> Result<tonic::Response<super::ListViewsResponse>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/ListViews",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a view.
        pub async fn update_view(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateViewRequest>,
        ) -> Result<tonic::Response<super::View>, tonic::Status> {
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/UpdateView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a view.
        pub async fn delete_view(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteViewRequest>,
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
                "/google.cloud.contactcenterinsights.v1.ContactCenterInsights/DeleteView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
