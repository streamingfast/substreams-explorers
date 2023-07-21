use substreams_ethereum::pb::eth::v2::Block;
use crate::pb::eth::block_meta::v1::BlockMeta;
use substreams::Hex;

#[substreams::handlers::map]
fn map_block_meta(blk: Block) -> Result<BlockMeta, substreams::errors::Error> {
    let header = blk.header.as_ref().unwrap();

    let hash_string = Hex(&blk.hash).to_string();
    let parent_hash_string = Hex(&header.parent_hash).to_string();

    Ok(BlockMeta {
         number: blk.number, 
         hash: hash_string, 
         parent_hash: parent_hash_string 
    })
}