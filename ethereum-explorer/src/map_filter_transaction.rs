mod pb;
use substreams_ethereum::pb::eth::v2::Block;
use substreams::Hex;
use crate::pb::eth::transaction::v1::TransactionOption;
use crate::pb::eth::transaction::v1::Transaction;


pub fn filter_by_transaction_hash(transaction_hash: String, blk: &Block) -> TransactionOption {
    let transaction_traces = &blk.transaction_traces;
    
    for transfer in transaction_traces {
        //let transferValue = transfer;
        let hash = &transfer.hash;
        let from = &transfer.from;
        let to = &transfer.to;
        
        if Hex(hash).to_string() == transaction_hash {
            let trans = Transaction { from: Hex(from).to_string(), to: Hex(to).to_string(), hash: Hex(hash).to_string() };
            return transfer_option_of(Some(trans));
        }
    }

    return empty_transfer_option()
}

fn transfer_option_of (transaction: Option<Transaction>) -> TransactionOption {
    return TransactionOption { transaction }
}

fn empty_transfer_option() -> TransactionOption {
    return transfer_option_of(None)
}