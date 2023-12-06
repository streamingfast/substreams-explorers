use crate::pb::btc::transaction::v1::{Transaction, Transactions};
use substreams_bitcoin::pb::btc::v1::{Block};

#[substreams::handlers::map]
fn map_transactions(blk: Block) -> Result<Transactions, Vec<substreams::errors::Error>> {
    let transactions: Vec<Transaction> = blk
        .transactions()
        .map(|trans| Transaction {
            hash: trans.txid.clone(),
            vin_count: trans.vin.len() as u64,
            vout_count: trans.vout.len() as u64,
            btc_value: trans.vout.iter().map(|vout| vout.value).sum(),
        })
        .collect();

    Ok(Transactions { transactions })
}