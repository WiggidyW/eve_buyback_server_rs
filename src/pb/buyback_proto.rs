// @generated
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqItem {
    #[prost(uint32, tag = "1")]
    pub type_id: u32,
    #[prost(uint32, tag = "2")]
    pub quantity: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyReq {
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<ReqItem>,
    #[prost(string, tag = "2")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub location: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckReq {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub language: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepItem {
    #[prost(uint32, tag = "1")]
    pub type_id: u32,
    #[prost(uint32, tag = "2")]
    pub parent_type_id: u32,
    #[prost(double, tag = "3")]
    pub quantity: f64,
    #[prost(string, tag = "4")]
    #[serde(skip_serializing, skip_deserializing)]
    pub name: ::prost::alloc::string::String,
    #[prost(double, tag = "5")]
    pub price_per: f64,
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag = "7")]
    pub accepted: bool,
    #[prost(message, optional, tag = "8")]
    pub meta: ::core::option::Option<::prost_wkt_types::Struct>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rep {
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<RepItem>,
    #[prost(string, tag = "2")]
    pub hash: ::prost::alloc::string::String,
    #[prost(double, tag = "3")]
    pub sum: f64,
    #[prost(uint64, tag = "4")]
    pub timestamp: u64,
    #[prost(string, tag = "5")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub location: ::prost::alloc::string::String,
}
include!("buyback_proto.tonic.rs");
// @@protoc_insertion_point(module)
