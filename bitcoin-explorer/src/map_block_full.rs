use substreams_bitcoin::pb::btc::v1::Block;

#[substreams::handlers::map]
fn map_block_full(blk: Block) -> Result<Block, substreams::errors::Error> {
    Ok(blk)
}
