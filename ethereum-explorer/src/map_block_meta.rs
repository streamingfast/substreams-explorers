use crate::pb::eth::block_meta::v1::BlockMeta;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
fn map_block_meta(blk: Block) -> Result<BlockMeta, substreams::errors::Error> {
    let header = blk.header.as_ref().unwrap();

    Ok(BlockMeta {
        number: blk.number,
        hash: Hex::encode(&blk.hash),
        parent_hash: Hex::encode(&header.parent_hash),
    })
}
