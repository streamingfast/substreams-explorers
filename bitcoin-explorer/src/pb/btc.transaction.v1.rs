// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transactions {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(string, tag="1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub vin_count: u64,
    #[prost(uint64, tag="3")]
    pub vout_count: u64,
    #[prost(double, tag="4")]
    pub btc_value: f64,
}
// @@protoc_insertion_point(module)
