/// input
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct CommandRequest {
    #[prost(oneof = "command_request::Data", tags = "1, 2, 3")]
    pub data: ::core::option::Option<command_request::Data>,
}
/// Nested message and enum types in `CommandRequest`.
pub mod command_request {
    #[derive(PartialOrd, Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag = "1")]
        Set(super::Set),
        #[prost(message, tag = "2")]
        Get(super::Get),
        #[prost(message, tag = "3")]
        Del(super::Del),
    }
}
/// output
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct CommandResponse {
    #[prost(oneof = "command_response::Data", tags = "1")]
    pub data: ::core::option::Option<command_response::Data>,
}
/// Nested message and enum types in `CommandResponse`.
pub mod command_response {
    #[derive(PartialOrd, Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag = "1")]
        Value(super::Value),
    }
}
/// set key = value
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct Set {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Value>,
}
/// get key
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct Get {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
/// delete key
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct Del {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
/// value
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(oneof = "value::Value", tags = "1, 2, 3, 4, 5")]
    pub value: ::core::option::Option<value::Value>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[derive(PartialOrd, Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag = "1")]
        String(::prost::alloc::string::String),
        #[prost(bytes, tag = "2")]
        Binary(::prost::bytes::Bytes),
        #[prost(int64, tag = "3")]
        Integer(i64),
        #[prost(double, tag = "4")]
        Float(f64),
        #[prost(bool, tag = "5")]
        Bool(bool),
    }
}
