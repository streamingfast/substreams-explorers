use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction};

const VOTE_INSTRUCTION: &str = "Vote111111111111111111111111111111111111111";

#[substreams::handlers::map]
fn map_block_full_no_vote(block: Block) -> Result<Block, substreams::errors::Error> {
    let mut block_cloned: Block = block.clone();

    let filtered_transactions: Vec<ConfirmedTransaction> = block.transactions_owned()
        .filter(|confirmed_trx| {
            let mut valid = false;
            let accounts = confirmed_trx.resolved_accounts_as_strings();

            if let Some(trx) = &confirmed_trx.transaction {
                let instructions = trx.message.clone().unwrap().instructions;

                if instructions.len() > 1 {
                    valid = true
                }

                let instruction = &instructions[0];
                let instruction_id = &accounts[instruction.program_id_index as usize];

                if instruction_id != &VOTE_INSTRUCTION.to_string() {
                    valid = true
                }
            }

            valid
        }).collect();

    block_cloned.transactions = filtered_transactions;

    Ok(block_cloned)
}
