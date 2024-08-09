// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecMeta {
    #[prost(string, tag="1")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub msg_strings: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecList {
    #[prost(message, repeated, tag="1")]
    pub msgs: ::prost::alloc::vec::Vec<MsgExecMeta>,
}
// @@protoc_insertion_point(module)
