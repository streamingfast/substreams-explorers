use substreams_ethereum::pb::eth::v2::{Block, TransactionTraceStatus};
use crate::pb::eth::transaction::v1::{Transaction, Transactions};
use serde::Deserialize;
use substreams::errors::Error;
use crate::util;

#[derive(Deserialize)]
struct TransactionFilterParams {
    hash: Option<String>,
    to: Option<String>,
    from: Option<String>
}

#[substreams::handlers::map]
pub fn map_filter_transactions(params: String, blk: Block) -> Result<Transactions, Vec<substreams::errors::Error>> {
    let filters: TransactionFilterParams = serde_qs::from_str(&params).unwrap();
    let errors = verify_filter_params(&filters);
    if errors.len() > 0 {
        return Err(errors)
    }

    let mut filtered_transactions: Vec<Transaction> = Vec::new();
    
    for transaction in &blk.transaction_traces {
        let tx_hash = util::hexadecimal_to_string(&transaction.hash);
        let tx_from = util::hexadecimal_to_string(&transaction.from);
        let tx_to = util::hexadecimal_to_string(&transaction.to);
        let mut current_transaction_filtered = true;

        if !filter_by_parameter(&filters.hash, &tx_hash) || 
            !filter_by_parameter(&filters.from, &tx_from) ||
            !filter_by_parameter(&filters.to, &tx_to) ||
            transaction.status != (TransactionTraceStatus::Succeeded as i32) {
            current_transaction_filtered = false
        }

        if current_transaction_filtered {
            let trans = Transaction { from: tx_from, to: tx_to, hash: tx_hash };
            filtered_transactions.push(trans)
        }
    }
    
    Ok(Transactions { transactions: filtered_transactions })
}

fn verify_filter_params(params: &TransactionFilterParams) -> Vec<substreams::errors::Error> {
    let mut errors: Vec<substreams::errors::Error> = Vec::new();

    if params.hash.is_some()
        && !util::is_transaction_hash_valid(&params.hash.as_ref().unwrap()) {
        errors.push(Error::Unexpected(String::from("Transaction hash is not valid")));
    }

    if params.from.is_some()
        && !util::is_address_valid(&params.from.as_ref().unwrap()) {
        errors.push(Error::Unexpected(String::from("'from' address is not valid")));
    }

    if params.to.is_some()
        && !util::is_address_valid(&params.to.as_ref().unwrap()) {
        errors.push(Error::Unexpected(String::from("'to' address is not valid")));
     }

    return errors;
}

fn filter_by_parameter(parameter: &Option<String>, transaction_field: &String) -> bool {
    if parameter.is_none() {
        return true;
    }

    if parameter.as_ref().unwrap() == transaction_field {
        return true
    }

    return false;
} 