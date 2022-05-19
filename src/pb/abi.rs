/// input
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InboudCommand {
    #[prost(oneof="inboud_command::Data", tags="1, 2")]
    pub data: ::core::option::Option<inboud_command::Data>,
}
/// Nested message and enum types in `InboudCommand`.
pub mod inboud_command {
    #[derive(PartialOrd)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag="1")]
        Set(super::Set),
        #[prost(message, tag="2")]
        Get(super::Get),
    }
}
/// output
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutboundCommand {
}
/// set key = value
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Set {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub value: ::core::option::Option<Value>,
}
/// get key
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Get {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
}
/// value
#[derive(PartialOrd)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(oneof="value::Value", tags="1, 2, 3, 4, 5")]
    pub value: ::core::option::Option<value::Value>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[derive(PartialOrd)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag="1")]
        String(::prost::alloc::string::String),
        #[prost(bytes, tag="2")]
        Binary(::prost::bytes::Bytes),
        #[prost(int64, tag="3")]
        Integer(i64),
        #[prost(double, tag="4")]
        Float(f64),
        #[prost(bool, tag="5")]
        Bool(bool),
    }
}
