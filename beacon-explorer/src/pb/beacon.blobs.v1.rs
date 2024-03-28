// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Blobs {
    #[prost(uint64, tag="1")]
    pub slot: u64,
    #[prost(string, repeated, tag="2")]
    pub blobs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
// @@protoc_insertion_point(module)
