///  The access log entry definition of online prediction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnlinePredictionLogEntry {
    ///  The resource name of the endpoint as referred to in the original request.
    ///  For example, projects/12323/locations/us-central1/endpoints/123.
    #[prost(string, tag="1")]
    pub endpoint: ::prost::alloc::string::String,
    ///  The ID of the deployed model used to serve this predict request.
    #[prost(string, tag="2")]
    pub deployed_model_id: ::prost::alloc::string::String,
    ///  The number of instances in the prediction request.
    #[prost(int64, tag="3")]
    pub instance_count: i64,
    ///  The number of successfully predicted instances in the response.
    ///  Populated when prediction succeeds.
    #[prost(int64, tag="4")]
    pub prediction_count: i64,
    ///  The error code and message.
    ///  Populated when prediction fails.
    #[prost(message, optional, tag="5")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
