use substreams_solana::pb::sf::solana::r#type::v1::Block;
use crate::pb::sol::block::v1::BlockMeta;

#[substreams::handlers::map]
fn map_block_meta(blk: Block) -> Result<BlockMeta, substreams::errors::Error> {
    Ok(BlockMeta {
        slot: blk.slot,
        hash: blk.blockhash,
        parent_hash: blk.previous_blockhash,
    })
}