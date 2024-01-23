use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction};

static VOTE_INSTRUCTION: &[u8] = b"Vote111111111111111111111111111111111111111";

#[substreams::handlers::map]
fn map_block_without_votes(block: Block) -> Result<Block, substreams::errors::Error> {
    let mut block_cloned: Block = block.clone();

    let filtered_transactions: Vec<ConfirmedTransaction> = block.transactions_owned()
        .filter(|trx| {
            let meta = match trx.meta.as_ref() {
                Some(meta) => meta,
                None => return false,
            };
            if meta.err.is_some() {
                return false;
            }
            let transaction = match trx.transaction.as_ref() {
                Some(transaction) => transaction,
                None => return false,
            };
            let message = transaction.message.as_ref().expect("Message is missing");

            if message.account_keys.contains(&VOTE_INSTRUCTION.to_vec()) {
                return false;
            }

            true
        }).collect();

    block_cloned.transactions = filtered_transactions;

    Ok(block_cloned)
}
