// @generated
/// MsgExec attempts to execute the provided messages using
/// authorizations granted to the grantee. Each message should have only
/// one signer corresponding to the granter of the authorization.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExec {
    #[prost(string, tag="1")]
    pub grantee: ::prost::alloc::string::String,
    /// Execute Msg.
    /// The x/authz will try to find a grant matching (msg.signers\[0\], grantee, MsgTypeURL(msg))
    /// triple and validate it.
    #[prost(message, repeated, tag="2")]
    pub msgs: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
// @@protoc_insertion_point(module)
