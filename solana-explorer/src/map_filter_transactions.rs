use crate::pb::sol::transactions::v1::{Instruction, Transaction, Transactions};
use anyhow::anyhow;
use serde::Deserialize;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction};

#[derive(Deserialize, Debug)]
struct TransactionFilterParams {
    signature: Option<String>,
}

#[substreams::handlers::map]
fn map_filter_transactions(params: String, blk: Block) -> Result<Transactions, Vec<substreams::errors::Error>> {
    let filters = parse_filters_from_params(params)?;

    let mut transactions: Vec<Transaction> = Vec::new();

    blk.transactions
        .iter()
        .filter(|tx| apply_filter(tx, &filters))
        .for_each(|tx| {
            let msg = tx.transaction.as_ref().unwrap().message.as_ref().unwrap();
            let acct_keys = tx.resolved_accounts();

            let insts: Vec<Instruction> = msg
                .instructions
                .iter()
                .map(|inst| Instruction {
                    program_id: bs58::encode(acct_keys[inst.program_id_index as usize].to_vec()).into_string(),
                    accounts: inst
                        .accounts
                        .iter()
                        .map(|acct| bs58::encode(acct_keys[*acct as usize].to_vec()).into_string())
                        .collect(),
                    data: bs58::encode(&inst.data).into_string(),
                })
                .collect();

            let t = Transaction {
                signatures: tx
                    .transaction
                    .as_ref()
                    .unwrap()
                    .signatures
                    .iter()
                    .map(|sig| bs58::encode(sig).into_string())
                    .collect(),
                instructions: insts,
            };
            transactions.push(t);
        });

    Ok(Transactions { transactions })
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

fn apply_filter(transaction: &&ConfirmedTransaction, filters: &TransactionFilterParams) -> bool {
    if filters.signature.is_none() {
        return true;
    }

    let mut found = false;

    transaction
        .transaction
        .as_ref()
        .unwrap()
        .signatures
        .iter()
        .for_each(|sig| {
            let xsig = bs58::encode(&sig).into_string();
            if xsig == filters.signature.clone().unwrap() {
                found = true;
            }
        });

    found
}
