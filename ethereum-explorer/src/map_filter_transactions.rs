use crate::pb::eth::transaction::v1::{Transaction, Transactions};
use crate::util;
use serde::Deserialize;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{Block, TransactionTraceStatus, TransactionTrace};
use anyhow::anyhow;

#[derive(Deserialize)]
struct TransactionFilterParams {
    to: Option<String>,
    from: Option<String>,
}

#[substreams::handlers::map]
fn map_filter_transactions(params: String, blk: Block) -> Result<Transactions, Vec<substreams::errors::Error>> {
    let filters: TransactionFilterParams = serde_qs::from_str(&params).unwrap();
    verify_filter_params(&filters)?;

    let transactions: Vec<Transaction> = blk.transactions()
        .filter(|trans| apply_filter(&trans, &filters))
        .map(|trans| Transaction {
            from: Hex::encode(&trans.from),
            to: Hex::encode(&trans.to),
            hash: Hex::encode(&trans.hash),
        }).collect();
    

    Ok(Transactions {
        transactions,
    })
}

fn verify_filter_params(params: &TransactionFilterParams) -> Result<(), Vec<substreams::errors::Error>> {
    let mut errors: Vec<substreams::errors::Error> = Vec::new();

    if params.from.is_some() && !util::is_address_valid(&params.from.as_ref().unwrap()) {
        errors.push(anyhow!("'from' address is not valid"));
    }

    if params.to.is_some() && !util::is_address_valid(&params.to.as_ref().unwrap()) {
        errors.push(anyhow!("'to' address is not valid"));
    }

    if errors.len() > 0 {
        return Err(errors)
    }

    return Ok(())
}

fn apply_filter(transaction: &TransactionTrace, filters: &TransactionFilterParams) -> bool {
    if !filter_by_parameter(&filters.from, &transaction.from)
        || !filter_by_parameter(&filters.to, &transaction.to)
        || transaction.status != (TransactionTraceStatus::Succeeded as i32) {
        return false;
    }

    return true;
}

fn filter_by_parameter(parameter: &Option<String>, transaction_field: &Vec<u8>) -> bool {
    if parameter.is_none() {
        return true;
    }

    let parameter_as_vec = &Hex::decode(parameter.as_ref().unwrap()).expect("already verified");
    if parameter_as_vec == transaction_field {
        return true;
    }

    return false;
}
