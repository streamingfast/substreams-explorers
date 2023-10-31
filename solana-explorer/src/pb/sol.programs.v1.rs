// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instructions {
    #[prost(message, repeated, tag="1")]
    pub instructions: ::prost::alloc::vec::Vec<Instruction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instruction {
    #[prost(string, tag="1")]
    pub program_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="3")]
    pub data: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub slot_number: u64,
    #[prost(string, tag="5")]
    pub block_hash: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
