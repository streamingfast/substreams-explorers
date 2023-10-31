use substreams_solana::pb::sf::solana::r#type::v1::Block;

#[substreams::handlers::map]
fn map_block_full(blk: Block) -> Result<Block, substreams::errors::Error> {
    Ok(blk)
}