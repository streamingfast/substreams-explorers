use anyhow::anyhow;
use serde::Deserialize;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, CompiledInstruction};
use crate::pb::sol::programs::v1::{Instruction, Instructions};

#[derive(Deserialize, Debug)]
struct TransactionFilterParams {
    program_id: Option<String>,
}


#[substreams::handlers::map]
fn map_filter_instructions(params: String, blk: Block) -> Result<Instructions, Vec<substreams::errors::Error>> {
    let filters = parse_filters_from_params(params)?;

    let mut instructions : Vec<Instruction> = Vec::new();

    blk.transactions.iter().for_each(|tx| {
        let msg = tx.transaction.clone().unwrap().message.unwrap();
        tx.transaction.clone().unwrap().
        let acct_keys = msg.account_keys.as_slice();
        let insts : Vec<Instruction> = msg.instructions.iter()
            .filter(|inst| apply_filter(inst, &filters, acct_keys.to_vec()))
            .map(|inst| {
            Instruction {
                slot_number: blk.slot,
                block_hash: blk.blockhash.clone(),
                program_id: bs58::encode(acct_keys[inst.program_id_index as usize].to_vec()).into_string(),
                accounts: inst.accounts.iter().map(|acct| bs58::encode(acct_keys[*acct as usize].to_vec()).into_string()).collect(),
                data: bs58::encode(inst.data.clone()).into_string(),
            }
        }).collect();
        instructions.extend(insts);
    });

    Ok(Instructions { instructions })
}

fn parse_filters_from_params(params: String) -> Result<TransactionFilterParams, Vec<substreams::errors::Error>> {
    let parsed_result = serde_qs::from_str(&params);
    if parsed_result.is_err() {
        return Err(Vec::from([anyhow!("Unexpected error while parsing parameters")]));
    }

    let filters = parsed_result.unwrap();
    //todo: verify_filters(&filters)?;

    Ok(filters)
}

fn apply_filter(instruction: &CompiledInstruction, filters: &TransactionFilterParams, account_keys: Vec<Vec<u8>>) -> bool {
    if filters.program_id.is_none() {
        return true;
    }
    let program_id_filter = filters.program_id.clone().unwrap();

    let program_account_key = account_keys.get(instruction.program_id_index as usize);
    if program_account_key.is_none() {
        return false;
    }
    let program_account_key_val = bs58::encode(program_account_key.unwrap()).into_string();

    //check if account key value is equal to the program id
    if program_account_key_val != program_id_filter {
        return false;
    }

    true
}