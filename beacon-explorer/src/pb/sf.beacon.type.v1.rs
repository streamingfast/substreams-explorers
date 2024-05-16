// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(uint32, tag="1")]
    pub version: u32,
    #[prost(enumeration="Spec", tag="2")]
    pub spec: i32,
    #[prost(uint64, tag="3")]
    pub slot: u64,
    #[prost(uint64, tag="4")]
    pub parent_slot: u64,
    #[prost(bytes="vec", tag="5")]
    pub root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub parent_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub state_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="8")]
    pub proposer_index: u64,
    #[prost(bytes="vec", tag="9")]
    pub body_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="30")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="31")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(oneof="block::Body", tags="20, 21, 22, 23, 24")]
    pub body: ::core::option::Option<block::Body>,
}
/// Nested message and enum types in `Block`.
pub mod block {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Body {
        #[prost(message, tag="20")]
        Phase0(super::Phase0Body),
        #[prost(message, tag="21")]
        Altair(super::AltairBody),
        #[prost(message, tag="22")]
        Bellatrix(super::BellatrixBody),
        #[prost(message, tag="23")]
        Capella(super::CapellaBody),
        #[prost(message, tag="24")]
        Deneb(super::DenebBody),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Phase0Body {
    #[prost(bytes="vec", tag="1")]
    pub rando_reveal: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub eth1_data: ::core::option::Option<Eth1Data>,
    #[prost(bytes="vec", tag="3")]
    pub graffiti: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="4")]
    pub proposer_slashings: ::prost::alloc::vec::Vec<ProposerSlashing>,
    #[prost(message, repeated, tag="5")]
    pub attester_slashings: ::prost::alloc::vec::Vec<AttesterSlashing>,
    #[prost(message, repeated, tag="6")]
    pub attestations: ::prost::alloc::vec::Vec<Attestation>,
    #[prost(message, repeated, tag="7")]
    pub deposits: ::prost::alloc::vec::Vec<Deposit>,
    #[prost(message, repeated, tag="8")]
    pub voluntary_exits: ::prost::alloc::vec::Vec<SignedVoluntaryExit>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AltairBody {
    #[prost(bytes="vec", tag="1")]
    pub rando_reveal: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub eth1_data: ::core::option::Option<Eth1Data>,
    #[prost(bytes="vec", tag="3")]
    pub graffiti: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="4")]
    pub proposer_slashings: ::prost::alloc::vec::Vec<ProposerSlashing>,
    #[prost(message, repeated, tag="5")]
    pub attester_slashings: ::prost::alloc::vec::Vec<AttesterSlashing>,
    #[prost(message, repeated, tag="6")]
    pub attestations: ::prost::alloc::vec::Vec<Attestation>,
    #[prost(message, repeated, tag="7")]
    pub deposits: ::prost::alloc::vec::Vec<Deposit>,
    #[prost(message, repeated, tag="8")]
    pub voluntary_exits: ::prost::alloc::vec::Vec<SignedVoluntaryExit>,
    #[prost(message, optional, tag="9")]
    pub sync_aggregate: ::core::option::Option<SyncAggregate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BellatrixBody {
    #[prost(bytes="vec", tag="1")]
    pub rando_reveal: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub eth1_data: ::core::option::Option<Eth1Data>,
    #[prost(bytes="vec", tag="3")]
    pub graffiti: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="4")]
    pub proposer_slashings: ::prost::alloc::vec::Vec<ProposerSlashing>,
    #[prost(message, repeated, tag="5")]
    pub attester_slashings: ::prost::alloc::vec::Vec<AttesterSlashing>,
    #[prost(message, repeated, tag="6")]
    pub attestations: ::prost::alloc::vec::Vec<Attestation>,
    #[prost(message, repeated, tag="7")]
    pub deposits: ::prost::alloc::vec::Vec<Deposit>,
    #[prost(message, repeated, tag="8")]
    pub voluntary_exits: ::prost::alloc::vec::Vec<SignedVoluntaryExit>,
    #[prost(message, optional, tag="9")]
    pub sync_aggregate: ::core::option::Option<SyncAggregate>,
    #[prost(message, optional, tag="10")]
    pub execution_payload: ::core::option::Option<BellatrixExecutionPayload>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapellaBody {
    #[prost(bytes="vec", tag="1")]
    pub rando_reveal: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub eth1_data: ::core::option::Option<Eth1Data>,
    #[prost(bytes="vec", tag="3")]
    pub graffiti: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="4")]
    pub proposer_slashings: ::prost::alloc::vec::Vec<ProposerSlashing>,
    #[prost(message, repeated, tag="5")]
    pub attester_slashings: ::prost::alloc::vec::Vec<AttesterSlashing>,
    #[prost(message, repeated, tag="6")]
    pub attestations: ::prost::alloc::vec::Vec<Attestation>,
    #[prost(message, repeated, tag="7")]
    pub deposits: ::prost::alloc::vec::Vec<Deposit>,
    #[prost(message, repeated, tag="8")]
    pub voluntary_exits: ::prost::alloc::vec::Vec<SignedVoluntaryExit>,
    #[prost(message, optional, tag="9")]
    pub sync_aggregate: ::core::option::Option<SyncAggregate>,
    #[prost(message, optional, tag="10")]
    pub execution_payload: ::core::option::Option<CapellaExecutionPayload>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenebBody {
    #[prost(bytes="vec", tag="1")]
    pub rando_reveal: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub eth1_data: ::core::option::Option<Eth1Data>,
    #[prost(bytes="vec", tag="3")]
    pub graffiti: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="4")]
    pub proposer_slashings: ::prost::alloc::vec::Vec<ProposerSlashing>,
    #[prost(message, repeated, tag="5")]
    pub attester_slashings: ::prost::alloc::vec::Vec<AttesterSlashing>,
    #[prost(message, repeated, tag="6")]
    pub attestations: ::prost::alloc::vec::Vec<Attestation>,
    #[prost(message, repeated, tag="7")]
    pub deposits: ::prost::alloc::vec::Vec<Deposit>,
    #[prost(message, repeated, tag="8")]
    pub voluntary_exits: ::prost::alloc::vec::Vec<SignedVoluntaryExit>,
    #[prost(message, optional, tag="9")]
    pub sync_aggregate: ::core::option::Option<SyncAggregate>,
    #[prost(message, optional, tag="10")]
    pub execution_payload: ::core::option::Option<DenebExecutionPayload>,
    #[prost(message, repeated, tag="11")]
    pub bls_to_execution_changes: ::prost::alloc::vec::Vec<SignedBlsToExecutionChange>,
    #[prost(bytes="vec", repeated, tag="12")]
    pub blob_kzg_commitments: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag="20")]
    pub embedded_blobs: ::prost::alloc::vec::Vec<Blob>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Eth1Data {
    #[prost(bytes="vec", tag="1")]
    pub deposit_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub deposit_count: u64,
    #[prost(bytes="vec", tag="3")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposerSlashing {
    #[prost(message, optional, tag="1")]
    pub signed_header_1: ::core::option::Option<SignedBeaconBlockHeader>,
    #[prost(message, optional, tag="2")]
    pub signed_header_2: ::core::option::Option<SignedBeaconBlockHeader>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttesterSlashing {
    #[prost(message, optional, tag="1")]
    pub attestation_1: ::core::option::Option<IndexedAttestation>,
    #[prost(message, optional, tag="2")]
    pub attestation_2: ::core::option::Option<IndexedAttestation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attestation {
    #[prost(bytes="vec", tag="1")]
    pub aggregation_bits: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub data: ::core::option::Option<AttestationData>,
    #[prost(bytes="vec", tag="3")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deposit {
    #[prost(bytes="vec", repeated, tag="1")]
    pub proof: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="2")]
    pub data: ::core::option::Option<DepositData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedVoluntaryExit {
    #[prost(message, optional, tag="1")]
    pub message: ::core::option::Option<VoluntaryExit>,
    #[prost(bytes="vec", tag="2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncAggregate {
    #[prost(bytes="vec", tag="1")]
    pub sync_commitee_bits: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub sync_comittee_signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BellatrixExecutionPayload {
    #[prost(bytes="vec", tag="1")]
    pub parent_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub fee_recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub state_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub receipts_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub logs_bloom: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub prev_randao: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="7")]
    pub block_number: u64,
    #[prost(uint64, tag="8")]
    pub gas_limit: u64,
    #[prost(uint64, tag="9")]
    pub gas_used: u64,
    #[prost(message, optional, tag="10")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bytes="vec", tag="11")]
    pub extra_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="12")]
    pub base_fee_per_gas: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="13")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="14")]
    pub transactions: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapellaExecutionPayload {
    #[prost(bytes="vec", tag="1")]
    pub parent_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub fee_recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub state_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub receipts_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub logs_bloom: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub prev_randao: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="7")]
    pub block_number: u64,
    #[prost(uint64, tag="8")]
    pub gas_limit: u64,
    #[prost(uint64, tag="9")]
    pub gas_used: u64,
    #[prost(message, optional, tag="10")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bytes="vec", tag="11")]
    pub extra_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="12")]
    pub base_fee_per_gas: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="13")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="14")]
    pub transactions: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag="15")]
    pub withdrawals: ::prost::alloc::vec::Vec<Withdrawal>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenebExecutionPayload {
    #[prost(bytes="vec", tag="1")]
    pub parent_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub fee_recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub state_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub receipts_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub logs_bloom: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub prev_randao: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="7")]
    pub block_number: u64,
    #[prost(uint64, tag="8")]
    pub gas_limit: u64,
    #[prost(uint64, tag="9")]
    pub gas_used: u64,
    #[prost(message, optional, tag="10")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bytes="vec", tag="11")]
    pub extra_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="12")]
    pub base_fee_per_gas: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="13")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="14")]
    pub transactions: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag="15")]
    pub withdrawals: ::prost::alloc::vec::Vec<Withdrawal>,
    #[prost(uint64, tag="16")]
    pub blob_gas_used: u64,
    #[prost(uint64, tag="17")]
    pub excess_blob_gas: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedBlsToExecutionChange {
    #[prost(message, optional, tag="1")]
    pub message: ::core::option::Option<BlsToExecutionChange>,
    #[prost(bytes="vec", tag="2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlsToExecutionChange {
    #[prost(uint64, tag="1")]
    pub validator_index: u64,
    #[prost(bytes="vec", tag="2")]
    pub from_bls_pub_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub to_execution_address: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Withdrawal {
    #[prost(uint64, tag="1")]
    pub withdrawal_index: u64,
    #[prost(uint64, tag="2")]
    pub validator_index: u64,
    #[prost(bytes="vec", tag="3")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="4")]
    pub gwei: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoluntaryExit {
    #[prost(uint64, tag="1")]
    pub epoch: u64,
    #[prost(uint64, tag="2")]
    pub validator_index: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositData {
    #[prost(bytes="vec", tag="1")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub withdrawal_credentials: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="3")]
    pub gwei: u64,
    #[prost(bytes="vec", tag="4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexedAttestation {
    #[prost(uint64, repeated, tag="1")]
    pub attesting_indices: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag="2")]
    pub data: ::core::option::Option<AttestationData>,
    #[prost(bytes="vec", tag="3")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttestationData {
    #[prost(uint64, tag="1")]
    pub slot: u64,
    #[prost(uint64, tag="2")]
    pub committee_index: u64,
    #[prost(bytes="vec", tag="3")]
    pub beacon_block_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="4")]
    pub source: ::core::option::Option<Checkpoint>,
    #[prost(message, optional, tag="5")]
    pub target: ::core::option::Option<Checkpoint>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Checkpoint {
    #[prost(uint64, tag="1")]
    pub epoch: u64,
    #[prost(bytes="vec", tag="2")]
    pub root: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedBeaconBlockHeader {
    #[prost(message, optional, tag="1")]
    pub message: ::core::option::Option<BeaconBlockHeader>,
    #[prost(bytes="vec", tag="2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeaconBlockHeader {
    #[prost(uint64, tag="1")]
    pub slot: u64,
    #[prost(uint64, tag="2")]
    pub proposer_index: u64,
    #[prost(bytes="vec", tag="3")]
    pub parent_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub state_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub body_root: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Blob {
    #[prost(uint64, tag="1")]
    pub index: u64,
    #[prost(bytes="vec", tag="2")]
    pub blob: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub kzg_commitment: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub kzg_proof: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="5")]
    pub kzg_commitment_inclusion_proof: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Spec {
    Unspecified = 0,
    Phase0 = 1,
    Altair = 2,
    Bellatrix = 3,
    Capella = 4,
    Deneb = 5,
}
impl Spec {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Spec::Unspecified => "UNSPECIFIED",
            Spec::Phase0 => "PHASE0",
            Spec::Altair => "ALTAIR",
            Spec::Bellatrix => "BELLATRIX",
            Spec::Capella => "CAPELLA",
            Spec::Deneb => "DENEB",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "PHASE0" => Some(Self::Phase0),
            "ALTAIR" => Some(Self::Altair),
            "BELLATRIX" => Some(Self::Bellatrix),
            "CAPELLA" => Some(Self::Capella),
            "DENEB" => Some(Self::Deneb),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
