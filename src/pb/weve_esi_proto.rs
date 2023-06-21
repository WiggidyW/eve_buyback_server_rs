// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(string, tag="2")]
    pub token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketOrder {
    #[prost(int64, tag="1")]
    pub quantity: i64,
    #[prost(double, tag="2")]
    pub price: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketOrdersRep {
    /// no particular order
    #[prost(message, repeated, tag="1")]
    pub inner: ::prost::alloc::vec::Vec<MarketOrder>,
}
/// Request and Response Types
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketOrdersReq {
    #[prost(uint64, tag="1")]
    pub location_id: u64,
    #[prost(uint32, tag="2")]
    pub type_id: u32,
    #[prost(string, tag="3")]
    pub token: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub buy: bool,
}
/// uint32 is typeid
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdjustedPriceRep {
    #[prost(map="uint32, double", tag="1")]
    pub inner: ::std::collections::HashMap<u32, f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdjustedPriceReq {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemIndex {
    #[prost(double, tag="1")]
    pub manufacturing: f64,
    #[prost(double, tag="2")]
    pub research_te: f64,
    #[prost(double, tag="3")]
    pub research_me: f64,
    #[prost(double, tag="4")]
    pub copying: f64,
    #[prost(double, tag="5")]
    pub invention: f64,
    #[prost(double, tag="6")]
    pub reactions: f64,
}
/// uint32 is systemid
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemIndexRep {
    #[prost(map="uint32, message", tag="1")]
    pub inner: ::std::collections::HashMap<u32, SystemIndex>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemIndexReq {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndustryJob {
    #[prost(uint64, tag="1")]
    pub location_id: u64,
    #[prost(uint64, tag="2")]
    pub character_id: u64,
    /// unix timestamp of when it started
    #[prost(uint64, tag="3")]
    pub start: u64,
    /// unix timestamp of when it finishes
    #[prost(uint64, tag="4")]
    pub finish: u64,
    /// always 1.0 unless invention
    #[prost(double, tag="5")]
    pub probability: f64,
    /// 0 if ME/TE research
    #[prost(uint32, tag="6")]
    pub product_id: u32,
    #[prost(uint32, tag="7")]
    pub blueprint_id: u32,
    /// blueprint ME
    #[prost(int32, tag="8")]
    pub material_efficiency: i32,
    /// blueprint TE
    #[prost(int32, tag="9")]
    pub time_efficiency: i32,
    #[prost(int32, tag="10")]
    pub activity: i32,
    #[prost(int32, tag="11")]
    pub runs: i32,
    #[prost(bool, tag="12")]
    pub is_bpc: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndustryJobsRep {
    #[prost(message, repeated, tag="1")]
    pub inner: ::prost::alloc::vec::Vec<IndustryJob>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndustryJobsReq {
    #[prost(message, repeated, tag="1")]
    pub characters: ::prost::alloc::vec::Vec<Entity>,
    #[prost(message, repeated, tag="2")]
    pub corporations: ::prost::alloc::vec::Vec<Entity>,
}
/// assets and blueprints
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    #[prost(uint64, tag="1")]
    pub entity_id: u64,
    #[prost(int64, tag="2")]
    pub quantity: i64,
    #[prost(int32, tag="3")]
    pub runs: i32,
    #[prost(int32, tag="4")]
    pub material_efficiency: i32,
    #[prost(int32, tag="5")]
    pub time_efficiency: i32,
    /// may include BPC or Singleton // NEVERMIND
    #[prost(string, repeated, tag="6")]
    pub flags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeAssets {
    #[prost(message, repeated, tag="1")]
    pub inner: ::prost::alloc::vec::Vec<Asset>,
}
/// uint32 is typeid
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationAssets {
    #[prost(map="uint32, message", tag="1")]
    pub inner: ::std::collections::HashMap<u32, TypeAssets>,
}
/// only includes station assets, uint64 is locationid
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetsRep {
    #[prost(map="uint64, message", tag="1")]
    pub inner: ::std::collections::HashMap<u64, LocationAssets>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetsReq {
    #[prost(message, repeated, tag="1")]
    pub characters: ::prost::alloc::vec::Vec<Entity>,
    #[prost(message, repeated, tag="2")]
    pub corporations: ::prost::alloc::vec::Vec<Entity>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Skills {
    #[prost(map="uint32, uint32", tag="1")]
    pub inner: ::std::collections::HashMap<u32, u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkillsRep {
    #[prost(map="uint64, message", tag="1")]
    pub inner: ::std::collections::HashMap<u64, Skills>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkillsReq {
    #[prost(message, repeated, tag="1")]
    pub characters: ::prost::alloc::vec::Vec<Entity>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveOrder {
    #[prost(bool, tag="1")]
    pub buy: bool,
    #[prost(double, tag="2")]
    pub price: f64,
    /// bool buy = 1;
    /// uint64 location_id = 2;
    /// uint32 type_id = 3;
    /// double price = 4;
    /// int64 quantity = 5;
    #[prost(int64, tag="3")]
    pub quantity: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeActiveOrders {
    #[prost(message, repeated, tag="1")]
    pub inner: ::prost::alloc::vec::Vec<ActiveOrder>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationActiveOrders {
    #[prost(map="uint32, message", tag="1")]
    pub inner: ::std::collections::HashMap<u32, TypeActiveOrders>,
}
/// shouldn't include duplicates, uint64 is locationid
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveOrdersRep {
    #[prost(map="uint64, message", tag="1")]
    pub inner: ::std::collections::HashMap<u64, LocationActiveOrders>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveOrdersReq {
    #[prost(message, repeated, tag="1")]
    pub characters: ::prost::alloc::vec::Vec<Entity>,
    #[prost(message, repeated, tag="2")]
    pub corporations: ::prost::alloc::vec::Vec<Entity>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(bool, tag="1")]
    pub buy: bool,
    #[prost(double, tag="2")]
    pub price: f64,
    #[prost(int64, tag="3")]
    pub quantity: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeTransactions {
    #[prost(message, repeated, tag="1")]
    pub inner: ::prost::alloc::vec::Vec<Transaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationTransactions {
    #[prost(map="uint32, message", tag="1")]
    pub inner: ::std::collections::HashMap<u32, TypeTransactions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionsRep {
    #[prost(map="uint64, message", tag="1")]
    pub inner: ::std::collections::HashMap<u64, LocationTransactions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionsReq {
    #[prost(message, repeated, tag="1")]
    pub characters: ::prost::alloc::vec::Vec<Entity>,
    #[prost(message, repeated, tag="2")]
    pub corporations: ::prost::alloc::vec::Vec<Entity>,
    #[prost(uint64, tag="3")]
    pub since: u64,
}
include!("weve_esi_proto.tonic.rs");
// @@protoc_insertion_point(module)
