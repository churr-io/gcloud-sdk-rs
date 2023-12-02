/// The create assessment request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAssessmentRequest {
    /// Required. The name of the project in which the assessment will be created,
    /// in the format `projects/{project_number}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The assessment details.
    #[prost(message, optional, tag = "2")]
    pub assessment: ::core::option::Option<Assessment>,
}
/// Describes an event in the lifecycle of a payment transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionEvent {
    /// Optional. The type of this transaction event.
    #[prost(enumeration = "transaction_event::TransactionEventType", tag = "1")]
    pub event_type: i32,
    /// Optional. The reason or standardized code that corresponds with this
    /// transaction event, if one exists. For example, a CHARGEBACK event with code
    /// 6005.
    #[prost(string, tag = "2")]
    pub reason: ::prost::alloc::string::String,
    /// Optional. The value that corresponds with this transaction event, if one
    /// exists. For example, a refund event where $5.00 was refunded. Currency is
    /// obtained from the original transaction data.
    #[prost(double, tag = "3")]
    pub value: f64,
    /// Optional. Timestamp when this transaction event occurred; otherwise assumed
    /// to be the time of the API call.
    #[prost(message, optional, tag = "4")]
    pub event_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `TransactionEvent`.
pub mod transaction_event {
    /// Enum that represents an event in the payment transaction lifecycle.
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
    pub enum TransactionEventType {
        /// Default, unspecified event type.
        Unspecified = 0,
        /// Indicates that the transaction is approved by the merchant. The
        /// accompanying reasons can include terms such as 'INHOUSE', 'ACCERTIFY',
        /// 'CYBERSOURCE', or 'MANUAL_REVIEW'.
        MerchantApprove = 1,
        /// Indicates that the transaction is denied and concluded due to risks
        /// detected by the merchant. The accompanying reasons can include terms such
        /// as 'INHOUSE',  'ACCERTIFY',  'CYBERSOURCE', or 'MANUAL_REVIEW'.
        MerchantDeny = 2,
        /// Indicates that the transaction is being evaluated by a human, due to
        /// suspicion or risk.
        ManualReview = 3,
        /// Indicates that the authorization attempt with the card issuer succeeded.
        Authorization = 4,
        /// Indicates that the authorization attempt with the card issuer failed.
        /// The accompanying reasons can include Visa's '54' indicating that the card
        /// is expired, or '82' indicating that the CVV is incorrect.
        AuthorizationDecline = 5,
        /// Indicates that the transaction is completed because the funds were
        /// settled.
        PaymentCapture = 6,
        /// Indicates that the transaction could not be completed because the funds
        /// were not settled.
        PaymentCaptureDecline = 7,
        /// Indicates that the transaction has been canceled. Specify the reason
        /// for the cancellation. For example, 'INSUFFICIENT_INVENTORY'.
        Cancel = 8,
        /// Indicates that the merchant has received a chargeback inquiry due to
        /// fraud for the transaction, requesting additional information before a
        /// fraud chargeback is officially issued and a formal chargeback
        /// notification is sent.
        ChargebackInquiry = 9,
        /// Indicates that the merchant has received a chargeback alert due to fraud
        /// for the transaction. The process of resolving the dispute without
        /// involving the payment network is started.
        ChargebackAlert = 10,
        /// Indicates that a fraud notification is issued for the transaction, sent
        /// by the payment instrument's issuing bank because the transaction appears
        /// to be fraudulent. We recommend including TC40 or SAFE data in the
        /// `reason` field for this event type. For partial chargebacks, we recommend
        /// that you include an amount in the `value` field.
        FraudNotification = 11,
        /// Indicates that the merchant is informed by the payment network that the
        /// transaction has entered the chargeback process due to fraud. Reason code
        /// examples include Discover's '6005' and '6041'. For partial chargebacks,
        /// we recommend that you include an amount in the `value` field.
        Chargeback = 12,
        /// Indicates that the transaction has entered the chargeback process due to
        /// fraud, and that the merchant has chosen to enter representment. Reason
        /// examples include Discover's '6005' and '6041'. For partial chargebacks,
        /// we recommend that you include an amount in the `value` field.
        ChargebackRepresentment = 13,
        /// Indicates that the transaction has had a fraud chargeback which was
        /// illegitimate and was reversed as a result. For partial chargebacks, we
        /// recommend that you include an amount in the `value` field.
        ChargebackReverse = 14,
        /// Indicates that the merchant has received a refund for a completed
        /// transaction. For partial refunds, we recommend that you include an amount
        /// in the `value` field. Reason example: 'TAX_EXEMPT' (partial refund of
        /// exempt tax)
        RefundRequest = 15,
        /// Indicates that the merchant has received a refund request for this
        /// transaction, but that they have declined it. For partial refunds, we
        /// recommend that you include an amount in the `value` field. Reason
        /// example: 'TAX_EXEMPT' (partial refund of exempt tax)
        RefundDecline = 16,
        /// Indicates that the completed transaction was refunded by the merchant.
        /// For partial refunds, we recommend that you include an amount in the
        /// `value` field. Reason example: 'TAX_EXEMPT' (partial refund of exempt
        /// tax)
        Refund = 17,
        /// Indicates that the completed transaction was refunded by the merchant,
        /// and that this refund was reversed. For partial refunds, we recommend that
        /// you include an amount in the `value` field.
        RefundReverse = 18,
    }
    impl TransactionEventType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TransactionEventType::Unspecified => "TRANSACTION_EVENT_TYPE_UNSPECIFIED",
                TransactionEventType::MerchantApprove => "MERCHANT_APPROVE",
                TransactionEventType::MerchantDeny => "MERCHANT_DENY",
                TransactionEventType::ManualReview => "MANUAL_REVIEW",
                TransactionEventType::Authorization => "AUTHORIZATION",
                TransactionEventType::AuthorizationDecline => "AUTHORIZATION_DECLINE",
                TransactionEventType::PaymentCapture => "PAYMENT_CAPTURE",
                TransactionEventType::PaymentCaptureDecline => "PAYMENT_CAPTURE_DECLINE",
                TransactionEventType::Cancel => "CANCEL",
                TransactionEventType::ChargebackInquiry => "CHARGEBACK_INQUIRY",
                TransactionEventType::ChargebackAlert => "CHARGEBACK_ALERT",
                TransactionEventType::FraudNotification => "FRAUD_NOTIFICATION",
                TransactionEventType::Chargeback => "CHARGEBACK",
                TransactionEventType::ChargebackRepresentment => {
                    "CHARGEBACK_REPRESENTMENT"
                }
                TransactionEventType::ChargebackReverse => "CHARGEBACK_REVERSE",
                TransactionEventType::RefundRequest => "REFUND_REQUEST",
                TransactionEventType::RefundDecline => "REFUND_DECLINE",
                TransactionEventType::Refund => "REFUND",
                TransactionEventType::RefundReverse => "REFUND_REVERSE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TRANSACTION_EVENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "MERCHANT_APPROVE" => Some(Self::MerchantApprove),
                "MERCHANT_DENY" => Some(Self::MerchantDeny),
                "MANUAL_REVIEW" => Some(Self::ManualReview),
                "AUTHORIZATION" => Some(Self::Authorization),
                "AUTHORIZATION_DECLINE" => Some(Self::AuthorizationDecline),
                "PAYMENT_CAPTURE" => Some(Self::PaymentCapture),
                "PAYMENT_CAPTURE_DECLINE" => Some(Self::PaymentCaptureDecline),
                "CANCEL" => Some(Self::Cancel),
                "CHARGEBACK_INQUIRY" => Some(Self::ChargebackInquiry),
                "CHARGEBACK_ALERT" => Some(Self::ChargebackAlert),
                "FRAUD_NOTIFICATION" => Some(Self::FraudNotification),
                "CHARGEBACK" => Some(Self::Chargeback),
                "CHARGEBACK_REPRESENTMENT" => Some(Self::ChargebackRepresentment),
                "CHARGEBACK_REVERSE" => Some(Self::ChargebackReverse),
                "REFUND_REQUEST" => Some(Self::RefundRequest),
                "REFUND_DECLINE" => Some(Self::RefundDecline),
                "REFUND" => Some(Self::Refund),
                "REFUND_REVERSE" => Some(Self::RefundReverse),
                _ => None,
            }
        }
    }
}
/// The request message to annotate an Assessment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateAssessmentRequest {
    /// Required. The resource name of the Assessment, in the format
    /// `projects/{project_number}/assessments/{assessment_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The annotation that will be assigned to the Event. This field can
    /// be left empty to provide reasons that apply to an event without concluding
    /// whether the event is legitimate or fraudulent.
    #[prost(enumeration = "annotate_assessment_request::Annotation", tag = "2")]
    pub annotation: i32,
    /// Optional. Reasons for the annotation that are assigned to the event.
    #[prost(
        enumeration = "annotate_assessment_request::Reason",
        repeated,
        packed = "false",
        tag = "3"
    )]
    pub reasons: ::prost::alloc::vec::Vec<i32>,
    /// Optional. Unique stable hashed user identifier to apply to the assessment.
    /// This is an alternative to setting the `hashed_account_id` in
    /// `CreateAssessment`, for example, when the account identifier is not yet
    /// known in the initial request. It is recommended that the identifier is
    /// hashed using hmac-sha256 with stable secret.
    #[prost(bytes = "vec", tag = "4")]
    pub hashed_account_id: ::prost::alloc::vec::Vec<u8>,
    /// Optional. If the assessment is part of a payment transaction, provide
    /// details on payment lifecycle events that occur in the transaction.
    #[prost(message, optional, tag = "5")]
    pub transaction_event: ::core::option::Option<TransactionEvent>,
}
/// Nested message and enum types in `AnnotateAssessmentRequest`.
pub mod annotate_assessment_request {
    /// Enum that represents the types of annotations.
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
    pub enum Annotation {
        /// Default unspecified type.
        Unspecified = 0,
        /// Provides information that the event turned out to be legitimate.
        Legitimate = 1,
        /// Provides information that the event turned out to be fraudulent.
        Fraudulent = 2,
        /// Provides information that the event was related to a login event in which
        /// the user typed the correct password. Deprecated, prefer indicating
        /// CORRECT_PASSWORD through the reasons field instead.
        PasswordCorrect = 3,
        /// Provides information that the event was related to a login event in which
        /// the user typed the incorrect password. Deprecated, prefer indicating
        /// INCORRECT_PASSWORD through the reasons field instead.
        PasswordIncorrect = 4,
    }
    impl Annotation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Annotation::Unspecified => "ANNOTATION_UNSPECIFIED",
                Annotation::Legitimate => "LEGITIMATE",
                Annotation::Fraudulent => "FRAUDULENT",
                Annotation::PasswordCorrect => "PASSWORD_CORRECT",
                Annotation::PasswordIncorrect => "PASSWORD_INCORRECT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ANNOTATION_UNSPECIFIED" => Some(Self::Unspecified),
                "LEGITIMATE" => Some(Self::Legitimate),
                "FRAUDULENT" => Some(Self::Fraudulent),
                "PASSWORD_CORRECT" => Some(Self::PasswordCorrect),
                "PASSWORD_INCORRECT" => Some(Self::PasswordIncorrect),
                _ => None,
            }
        }
    }
    /// Enum that represents potential reasons for annotating an assessment.
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
    pub enum Reason {
        /// Default unspecified reason.
        Unspecified = 0,
        /// Indicates that the transaction had a chargeback issued with no other
        /// details. When possible, specify the type by using CHARGEBACK_FRAUD or
        /// CHARGEBACK_DISPUTE instead.
        Chargeback = 1,
        /// Indicates that the transaction had a chargeback issued related to an
        /// alleged unauthorized transaction from the cardholder's perspective (for
        /// example, the card number was stolen).
        ChargebackFraud = 8,
        /// Indicates that the transaction had a chargeback issued related to the
        /// cardholder having provided their card details but allegedly not being
        /// satisfied with the purchase (for example, misrepresentation, attempted
        /// cancellation).
        ChargebackDispute = 9,
        /// Indicates that the completed payment transaction was refunded by the
        /// seller.
        Refund = 10,
        /// Indicates that the completed payment transaction was determined to be
        /// fraudulent by the seller, and was cancelled and refunded as a result.
        RefundFraud = 11,
        /// Indicates that the payment transaction was accepted, and the user was
        /// charged.
        TransactionAccepted = 12,
        /// Indicates that the payment transaction was declined, for example due to
        /// invalid card details.
        TransactionDeclined = 13,
        /// Indicates the transaction associated with the assessment is suspected of
        /// being fraudulent based on the payment method, billing details, shipping
        /// address or other transaction information.
        PaymentHeuristics = 2,
        /// Indicates that the user was served a 2FA challenge. An old assessment
        /// with `ENUM_VALUES.INITIATED_TWO_FACTOR` reason that has not been
        /// overwritten with `PASSED_TWO_FACTOR` is treated as an abandoned 2FA flow.
        /// This is equivalent to `FAILED_TWO_FACTOR`.
        InitiatedTwoFactor = 7,
        /// Indicates that the user passed a 2FA challenge.
        PassedTwoFactor = 3,
        /// Indicates that the user failed a 2FA challenge.
        FailedTwoFactor = 4,
        /// Indicates the user provided the correct password.
        CorrectPassword = 5,
        /// Indicates the user provided an incorrect password.
        IncorrectPassword = 6,
        /// Indicates that the user sent unwanted and abusive messages to other users
        /// of the platform, such as spam, scams, phishing, or social engineering.
        SocialSpam = 14,
    }
    impl Reason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Reason::Unspecified => "REASON_UNSPECIFIED",
                Reason::Chargeback => "CHARGEBACK",
                Reason::ChargebackFraud => "CHARGEBACK_FRAUD",
                Reason::ChargebackDispute => "CHARGEBACK_DISPUTE",
                Reason::Refund => "REFUND",
                Reason::RefundFraud => "REFUND_FRAUD",
                Reason::TransactionAccepted => "TRANSACTION_ACCEPTED",
                Reason::TransactionDeclined => "TRANSACTION_DECLINED",
                Reason::PaymentHeuristics => "PAYMENT_HEURISTICS",
                Reason::InitiatedTwoFactor => "INITIATED_TWO_FACTOR",
                Reason::PassedTwoFactor => "PASSED_TWO_FACTOR",
                Reason::FailedTwoFactor => "FAILED_TWO_FACTOR",
                Reason::CorrectPassword => "CORRECT_PASSWORD",
                Reason::IncorrectPassword => "INCORRECT_PASSWORD",
                Reason::SocialSpam => "SOCIAL_SPAM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REASON_UNSPECIFIED" => Some(Self::Unspecified),
                "CHARGEBACK" => Some(Self::Chargeback),
                "CHARGEBACK_FRAUD" => Some(Self::ChargebackFraud),
                "CHARGEBACK_DISPUTE" => Some(Self::ChargebackDispute),
                "REFUND" => Some(Self::Refund),
                "REFUND_FRAUD" => Some(Self::RefundFraud),
                "TRANSACTION_ACCEPTED" => Some(Self::TransactionAccepted),
                "TRANSACTION_DECLINED" => Some(Self::TransactionDeclined),
                "PAYMENT_HEURISTICS" => Some(Self::PaymentHeuristics),
                "INITIATED_TWO_FACTOR" => Some(Self::InitiatedTwoFactor),
                "PASSED_TWO_FACTOR" => Some(Self::PassedTwoFactor),
                "FAILED_TWO_FACTOR" => Some(Self::FailedTwoFactor),
                "CORRECT_PASSWORD" => Some(Self::CorrectPassword),
                "INCORRECT_PASSWORD" => Some(Self::IncorrectPassword),
                "SOCIAL_SPAM" => Some(Self::SocialSpam),
                _ => None,
            }
        }
    }
}
/// Empty response for AnnotateAssessment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateAssessmentResponse {}
/// Password leak verification info.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PasswordLeakVerification {
    /// Optional. Scrypt hash of the username+password that the customer wants to
    /// verify against a known password leak.
    #[prost(bytes = "vec", tag = "1")]
    pub hashed_user_credentials: ::prost::alloc::vec::Vec<u8>,
    /// Output only. Whether or not the user's credentials are present in a known
    /// leak.
    #[prost(bool, tag = "2")]
    pub credentials_leaked: bool,
    /// Optional. The username part of the user credentials for which we want to
    /// trigger a leak check in canonicalized form. This is the same data used to
    /// create the hashed_user_credentials on the customer side.
    #[prost(string, tag = "3")]
    pub canonicalized_username: ::prost::alloc::string::String,
}
/// A reCAPTCHA Enterprise assessment resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Assessment {
    /// Output only. The resource name for the Assessment in the format
    /// `projects/{project_number}/assessments/{assessment_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The event being assessed.
    #[prost(message, optional, tag = "2")]
    pub event: ::core::option::Option<Event>,
    /// Output only. Legitimate event score from 0.0 to 1.0.
    /// (1.0 means very likely legitimate traffic while 0.0 means very likely
    /// non-legitimate traffic).
    #[prost(float, tag = "3")]
    pub score: f32,
    /// Output only. Properties of the provided event token.
    #[prost(message, optional, tag = "4")]
    pub token_properties: ::core::option::Option<TokenProperties>,
    /// Output only. Reasons contributing to the risk analysis verdict.
    #[prost(
        enumeration = "assessment::ClassificationReason",
        repeated,
        packed = "false",
        tag = "5"
    )]
    pub reasons: ::prost::alloc::vec::Vec<i32>,
    /// Information about the user's credentials used to check for leaks.
    /// This feature is part of the Early Access Program (EAP). Exercise caution,
    /// and do not deploy integrations based on this feature in a production
    /// environment.
    #[prost(message, optional, tag = "7")]
    pub password_leak_verification: ::core::option::Option<PasswordLeakVerification>,
    /// Assessment returned by account defender when a hashed_account_id is
    /// provided.
    #[prost(message, optional, tag = "8")]
    pub account_defender_assessment: ::core::option::Option<AccountDefenderAssessment>,
    /// Assessment returned by Fraud Prevention when TransactionData is provided.
    #[prost(message, optional, tag = "11")]
    pub fraud_prevention_assessment: ::core::option::Option<FraudPreventionAssessment>,
}
/// Nested message and enum types in `Assessment`.
pub mod assessment {
    /// Reasons contributing to the risk analysis verdict.
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
    pub enum ClassificationReason {
        /// Default unspecified type.
        Unspecified = 0,
        /// Interactions matched the behavior of an automated agent.
        Automation = 1,
        /// The event originated from an illegitimate environment.
        UnexpectedEnvironment = 2,
        /// Traffic volume from the event source is higher than normal.
        TooMuchTraffic = 3,
        /// Interactions with the site were significantly different than expected
        /// patterns.
        UnexpectedUsagePatterns = 4,
        /// Too little traffic has been received from this site thus far to generate
        /// quality risk analysis.
        LowConfidenceScore = 5,
        /// The request matches behavioral characteristics of a carding attack.
        SuspectedCarding = 6,
        /// The request matches behavioral characteristics of chargebacks for fraud.
        SuspectedChargeback = 7,
    }
    impl ClassificationReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ClassificationReason::Unspecified => "CLASSIFICATION_REASON_UNSPECIFIED",
                ClassificationReason::Automation => "AUTOMATION",
                ClassificationReason::UnexpectedEnvironment => "UNEXPECTED_ENVIRONMENT",
                ClassificationReason::TooMuchTraffic => "TOO_MUCH_TRAFFIC",
                ClassificationReason::UnexpectedUsagePatterns => {
                    "UNEXPECTED_USAGE_PATTERNS"
                }
                ClassificationReason::LowConfidenceScore => "LOW_CONFIDENCE_SCORE",
                ClassificationReason::SuspectedCarding => "SUSPECTED_CARDING",
                ClassificationReason::SuspectedChargeback => "SUSPECTED_CHARGEBACK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CLASSIFICATION_REASON_UNSPECIFIED" => Some(Self::Unspecified),
                "AUTOMATION" => Some(Self::Automation),
                "UNEXPECTED_ENVIRONMENT" => Some(Self::UnexpectedEnvironment),
                "TOO_MUCH_TRAFFIC" => Some(Self::TooMuchTraffic),
                "UNEXPECTED_USAGE_PATTERNS" => Some(Self::UnexpectedUsagePatterns),
                "LOW_CONFIDENCE_SCORE" => Some(Self::LowConfidenceScore),
                "SUSPECTED_CARDING" => Some(Self::SuspectedCarding),
                "SUSPECTED_CHARGEBACK" => Some(Self::SuspectedChargeback),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// Optional. The user response token provided by the reCAPTCHA Enterprise
    /// client-side integration on your site.
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    /// Optional. The site key that was used to invoke reCAPTCHA Enterprise on your
    /// site and generate the token.
    #[prost(string, tag = "2")]
    pub site_key: ::prost::alloc::string::String,
    /// Optional. The user agent present in the request from the user's device
    /// related to this event.
    #[prost(string, tag = "3")]
    pub user_agent: ::prost::alloc::string::String,
    /// Optional. The IP address in the request from the user's device related to
    /// this event.
    #[prost(string, tag = "4")]
    pub user_ip_address: ::prost::alloc::string::String,
    /// Optional. The expected action for this type of event. This should be the
    /// same action provided at token generation time on client-side platforms
    /// already integrated with recaptcha enterprise.
    #[prost(string, tag = "5")]
    pub expected_action: ::prost::alloc::string::String,
    /// Optional. Unique stable hashed user identifier for the request. The
    /// identifier must be hashed using hmac-sha256 with stable secret.
    #[prost(bytes = "vec", tag = "6")]
    pub hashed_account_id: ::prost::alloc::vec::Vec<u8>,
    /// Optional. Data describing a payment transaction to be assessed. Sending
    /// this data enables reCAPTCHA Enterprise Fraud Prevention and the
    /// FraudPreventionAssessment component in the response.
    #[prost(message, optional, tag = "13")]
    pub transaction_data: ::core::option::Option<TransactionData>,
}
/// Transaction data associated with a payment protected by reCAPTCHA Enterprise.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionData {
    /// Unique identifier for the transaction. This custom identifier can be used
    /// to reference this transaction in the future, for example, labeling a refund
    /// or chargeback event. Two attempts at the same transaction should use the
    /// same transaction id.
    #[prost(string, optional, tag = "11")]
    pub transaction_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The payment method for the transaction. The allowed values are:
    ///
    /// * credit-card
    /// * debit-card
    /// * gift-card
    /// * processor-{name} (If a third-party is used, for example,
    /// processor-paypal)
    /// * custom-{name} (If an alternative method is used, for example,
    /// custom-crypto)
    #[prost(string, tag = "1")]
    pub payment_method: ::prost::alloc::string::String,
    /// The Bank Identification Number - generally the first 6 or 8 digits of the
    /// card.
    #[prost(string, tag = "2")]
    pub card_bin: ::prost::alloc::string::String,
    /// The last four digits of the card.
    #[prost(string, tag = "3")]
    pub card_last_four: ::prost::alloc::string::String,
    /// The currency code in ISO-4217 format.
    #[prost(string, tag = "4")]
    pub currency_code: ::prost::alloc::string::String,
    /// The decimal value of the transaction in the specified currency.
    #[prost(double, tag = "5")]
    pub value: f64,
    /// The value of shipping in the specified currency. 0 for free or no shipping.
    #[prost(double, tag = "12")]
    pub shipping_value: f64,
    /// Destination address if this transaction involves shipping a physical item.
    #[prost(message, optional, tag = "6")]
    pub shipping_address: ::core::option::Option<transaction_data::Address>,
    /// Address associated with the payment method when applicable.
    #[prost(message, optional, tag = "7")]
    pub billing_address: ::core::option::Option<transaction_data::Address>,
    /// Information about the user paying/initiating the transaction.
    #[prost(message, optional, tag = "8")]
    pub user: ::core::option::Option<transaction_data::User>,
    /// Information about the user or users fulfilling the transaction.
    #[prost(message, repeated, tag = "13")]
    pub merchants: ::prost::alloc::vec::Vec<transaction_data::User>,
    /// Items purchased in this transaction.
    #[prost(message, repeated, tag = "14")]
    pub items: ::prost::alloc::vec::Vec<transaction_data::Item>,
    /// Information about the payment gateway's response to the transaction.
    #[prost(message, optional, tag = "10")]
    pub gateway_info: ::core::option::Option<transaction_data::GatewayInfo>,
}
/// Nested message and enum types in `TransactionData`.
pub mod transaction_data {
    /// Structured address format for billing and shipping addresses.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Address {
        /// The recipient name, potentially including information such as "care of".
        #[prost(string, tag = "1")]
        pub recipient: ::prost::alloc::string::String,
        /// The first lines of the address. The first line generally contains the
        /// street name and number, and further lines may include information such as
        /// an apartment number.
        #[prost(string, repeated, tag = "2")]
        pub address: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The town/city of the address.
        #[prost(string, tag = "3")]
        pub locality: ::prost::alloc::string::String,
        /// The state, province, or otherwise administrative area of the address.
        #[prost(string, tag = "4")]
        pub administrative_area: ::prost::alloc::string::String,
        /// The CLDR country/region of the address.
        #[prost(string, tag = "5")]
        pub region_code: ::prost::alloc::string::String,
        /// The postal or ZIP code of the address.
        #[prost(string, tag = "6")]
        pub postal_code: ::prost::alloc::string::String,
    }
    /// Details about a user's account involved in the transaction.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct User {
        /// Unique account identifier for this user. If using account defender,
        /// this should match the hashed_account_id field. Otherwise, a unique and
        /// persistent identifier for this account.
        #[prost(string, tag = "6")]
        pub account_id: ::prost::alloc::string::String,
        /// The epoch milliseconds of the user's account creation.
        #[prost(int64, tag = "1")]
        pub creation_ms: i64,
        /// The email address of the user.
        #[prost(string, tag = "2")]
        pub email: ::prost::alloc::string::String,
        /// Whether the email has been verified to be accessible by the user (OTP or
        /// similar).
        #[prost(bool, tag = "3")]
        pub email_verified: bool,
        /// The phone number of the user, with country code.
        #[prost(string, tag = "4")]
        pub phone_number: ::prost::alloc::string::String,
        /// Whether the phone number has been verified to be accessible by the user
        /// (OTP or similar).
        #[prost(bool, tag = "5")]
        pub phone_verified: bool,
    }
    /// Line items being purchased in this transaction.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Item {
        /// The full name of the item.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The value per item that the user is paying, in the transaction currency,
        /// after discounts.
        #[prost(double, tag = "2")]
        pub value: f64,
        /// The quantity of this item that is being purchased.
        #[prost(int64, tag = "3")]
        pub quantity: i64,
        /// When a merchant is specified, its corresponding account_id. Necessary to
        /// populate marketplace-style transactions.
        #[prost(string, tag = "4")]
        pub merchant_account_id: ::prost::alloc::string::String,
    }
    /// Details about the transaction from the gateway.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GatewayInfo {
        /// Name of the gateway service (for example, stripe, square, paypal).
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Gateway response code describing the state of the transaction.
        #[prost(string, tag = "2")]
        pub gateway_response_code: ::prost::alloc::string::String,
        /// AVS response code from the gateway
        /// (available only when reCAPTCHA Enterprise is called after authorization).
        #[prost(string, tag = "3")]
        pub avs_response_code: ::prost::alloc::string::String,
        /// CVV response code from the gateway
        /// (available only when reCAPTCHA Enterprise is called after authorization).
        #[prost(string, tag = "4")]
        pub cvv_response_code: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenProperties {
    /// Whether the provided user response token is valid. When valid = false, the
    /// reason could be specified in invalid_reason or it could also be due to
    /// a user failing to solve a challenge or a sitekey mismatch (i.e the sitekey
    /// used to generate the token was different than the one specified in the
    /// assessment).
    #[prost(bool, tag = "1")]
    pub valid: bool,
    /// Reason associated with the response when valid = false.
    #[prost(enumeration = "token_properties::InvalidReason", tag = "2")]
    pub invalid_reason: i32,
    /// The timestamp corresponding to the generation of the token.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The hostname of the page on which the token was generated.
    #[prost(string, tag = "4")]
    pub hostname: ::prost::alloc::string::String,
    /// Action name provided at token generation.
    #[prost(string, tag = "5")]
    pub action: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TokenProperties`.
pub mod token_properties {
    /// Enum that represents the types of invalid token reasons.
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
    pub enum InvalidReason {
        /// Default unspecified type.
        Unspecified = 0,
        /// If the failure reason was not accounted for.
        UnknownInvalidReason = 1,
        /// The provided user verification token was malformed.
        Malformed = 2,
        /// The user verification token had expired.
        Expired = 3,
        /// The user verification had already been seen.
        Dupe = 4,
        /// The user verification token did not match the provided site key.
        /// This may be a configuration error (for example, development keys used in
        /// production) or end users trying to use verification tokens from other
        /// sites.
        SiteMismatch = 5,
        /// The user verification token was not present.  It is a required input.
        Missing = 6,
        /// A retriable error (such as network failure) occurred on the browser.
        /// Could easily be simulated by an attacker.
        BrowserError = 7,
    }
    impl InvalidReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InvalidReason::Unspecified => "INVALID_REASON_UNSPECIFIED",
                InvalidReason::UnknownInvalidReason => "UNKNOWN_INVALID_REASON",
                InvalidReason::Malformed => "MALFORMED",
                InvalidReason::Expired => "EXPIRED",
                InvalidReason::Dupe => "DUPE",
                InvalidReason::SiteMismatch => "SITE_MISMATCH",
                InvalidReason::Missing => "MISSING",
                InvalidReason::BrowserError => "BROWSER_ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INVALID_REASON_UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN_INVALID_REASON" => Some(Self::UnknownInvalidReason),
                "MALFORMED" => Some(Self::Malformed),
                "EXPIRED" => Some(Self::Expired),
                "DUPE" => Some(Self::Dupe),
                "SITE_MISMATCH" => Some(Self::SiteMismatch),
                "MISSING" => Some(Self::Missing),
                "BROWSER_ERROR" => Some(Self::BrowserError),
                _ => None,
            }
        }
    }
}
/// Assessment for Fraud Prevention.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FraudPreventionAssessment {
    /// Probability (0-1) of this transaction being fraudulent. Summarizes the
    /// combined risk of attack vectors below.
    #[prost(float, tag = "1")]
    pub transaction_risk: f32,
    /// Assessment of this transaction for risk of a stolen instrument.
    #[prost(message, optional, tag = "2")]
    pub stolen_instrument_verdict: ::core::option::Option<
        fraud_prevention_assessment::StolenInstrumentVerdict,
    >,
    /// Assessment of this transaction for risk of being part of a card testing
    /// attack.
    #[prost(message, optional, tag = "3")]
    pub card_testing_verdict: ::core::option::Option<
        fraud_prevention_assessment::CardTestingVerdict,
    >,
    /// Assessment of this transaction for behavioral trust.
    #[prost(message, optional, tag = "4")]
    pub behavioral_trust_verdict: ::core::option::Option<
        fraud_prevention_assessment::BehavioralTrustVerdict,
    >,
}
/// Nested message and enum types in `FraudPreventionAssessment`.
pub mod fraud_prevention_assessment {
    /// Information about stolen instrument fraud, where the user is not the
    /// legitimate owner of the instrument being used for the purchase.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StolenInstrumentVerdict {
        /// Probability (0-1) of this transaction being executed with a stolen
        /// instrument.
        #[prost(float, tag = "1")]
        pub risk: f32,
    }
    /// Information about card testing fraud, where an adversary is testing
    /// fraudulently obtained cards or brute forcing their details.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CardTestingVerdict {
        /// Probability (0-1) of this transaction attempt being part of a card
        /// testing attack.
        #[prost(float, tag = "1")]
        pub risk: f32,
    }
    /// Information about behavioral trust of the transaction.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BehavioralTrustVerdict {
        /// Probability (0-1) of this transaction attempt being executed in a
        /// behaviorally trustworthy way.
        #[prost(float, tag = "1")]
        pub trust: f32,
    }
}
/// Account defender risk assessment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountDefenderAssessment {
    /// Labels for this request.
    #[prost(
        enumeration = "account_defender_assessment::AccountDefenderLabel",
        repeated,
        tag = "1"
    )]
    pub labels: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `AccountDefenderAssessment`.
pub mod account_defender_assessment {
    /// Labels returned by account defender for this request.
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
    pub enum AccountDefenderLabel {
        /// Default unspecified type.
        Unspecified = 0,
        /// The request matches a known good profile for the user.
        ProfileMatch = 1,
        /// The request is potentially a suspicious login event and should be further
        /// verified either via multi-factor authentication or another system.
        SuspiciousLoginActivity = 2,
        /// The request matched a profile that previously had suspicious account
        /// creation behavior. This could mean this is a fake account.
        SuspiciousAccountCreation = 3,
        /// The account in the request has a high number of related accounts. It does
        /// not necessarily imply that the account is bad but could require
        /// investigating.
        RelatedAccountsNumberHigh = 4,
    }
    impl AccountDefenderLabel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AccountDefenderLabel::Unspecified => "ACCOUNT_DEFENDER_LABEL_UNSPECIFIED",
                AccountDefenderLabel::ProfileMatch => "PROFILE_MATCH",
                AccountDefenderLabel::SuspiciousLoginActivity => {
                    "SUSPICIOUS_LOGIN_ACTIVITY"
                }
                AccountDefenderLabel::SuspiciousAccountCreation => {
                    "SUSPICIOUS_ACCOUNT_CREATION"
                }
                AccountDefenderLabel::RelatedAccountsNumberHigh => {
                    "RELATED_ACCOUNTS_NUMBER_HIGH"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACCOUNT_DEFENDER_LABEL_UNSPECIFIED" => Some(Self::Unspecified),
                "PROFILE_MATCH" => Some(Self::ProfileMatch),
                "SUSPICIOUS_LOGIN_ACTIVITY" => Some(Self::SuspiciousLoginActivity),
                "SUSPICIOUS_ACCOUNT_CREATION" => Some(Self::SuspiciousAccountCreation),
                "RELATED_ACCOUNTS_NUMBER_HIGH" => Some(Self::RelatedAccountsNumberHigh),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod recaptcha_enterprise_service_v1_beta1_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to determine the likelihood an event is legitimate.
    #[derive(Debug, Clone)]
    pub struct RecaptchaEnterpriseServiceV1Beta1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RecaptchaEnterpriseServiceV1Beta1Client<tonic::transport::Channel> {
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
    impl<T> RecaptchaEnterpriseServiceV1Beta1Client<T>
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
        ) -> RecaptchaEnterpriseServiceV1Beta1Client<InterceptedService<T, F>>
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
            RecaptchaEnterpriseServiceV1Beta1Client::new(
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
        /// Creates an Assessment of the likelihood an event is legitimate.
        pub async fn create_assessment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAssessmentRequest>,
        ) -> std::result::Result<tonic::Response<super::Assessment>, tonic::Status> {
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
                "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/CreateAssessment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1",
                        "CreateAssessment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Annotates a previously created Assessment to provide additional information
        /// on whether the event turned out to be authentic or fradulent.
        pub async fn annotate_assessment(
            &mut self,
            request: impl tonic::IntoRequest<super::AnnotateAssessmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AnnotateAssessmentResponse>,
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
                "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/AnnotateAssessment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1",
                        "AnnotateAssessment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
