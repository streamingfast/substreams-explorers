use crate::pb::btc::block_meta::v1::BlockMeta;
use substreams_bitcoin::pb::btc::v1::Block;

#[substreams::handlers::map]
fn map_block_meta(blk: Block) -> Result<BlockMeta, substreams::errors::Error> {
    Ok(BlockMeta {
        number: blk.height as u64,
        hash: blk.hash,
        parent_hash: blk.previous_hash,
    })
}
