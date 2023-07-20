mod pb;

#[path = "map_block_meta.rs"]
mod block_meta;

#[path = "map_filter_transaction.rs"]
mod filter_transaction;

#[path = "map_contract_events.rs"]
mod contract_events;


use pb::eth::transaction::v1::TransactionOption;
use substreams_ethereum::pb::eth::v2::Block;
use pb::eth::block_meta::v1::BlockMeta;
use crate::pb::eth::event::v1::Events;

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_block_meta(blk: Block) -> Result<BlockMeta, substreams::errors::Error> {
    let block_meta = block_meta::map_block_meta(&blk);

    Ok(block_meta)
}

#[substreams::handlers::map]
pub fn map_filter_transaction(transaction_hash: String, blk: Block) -> Result<TransactionOption, substreams::errors::Error> {
    let filtered_transaction = filter_transaction::filter_by_transaction_hash(transaction_hash, &blk);

    Ok(filtered_transaction)
}

#[substreams::handlers::map]
fn map_contract_events(contract_address: String, blk: Block) -> Result<Events, substreams::errors::Error> {
    let events: Events = contract_events::map_contract_events(contract_address, &blk);

    Ok(events)
}

