use substreams::Hex;
use crate::pb::beacon::block_meta::v1::BlockMeta;
use crate::pb::sf::beacon::r#type::v1::Block;

#[substreams::handlers::map]
fn map_block_meta(blk: Block) -> Result<BlockMeta, substreams::errors::Error> {

    Ok(BlockMeta {
        slot: blk.slot,
        root: Hex::encode(&blk.root),
        state_root: Hex::encode(&blk.state_root),
        spec: blk.spec,
        timestamp: blk.timestamp,
    })
}
