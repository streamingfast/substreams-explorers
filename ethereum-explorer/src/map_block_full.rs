use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
fn map_block_full(blk: Block) -> Result<Block, substreams::errors::Error> {
    Ok(blk)
}
