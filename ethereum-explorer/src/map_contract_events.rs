use crate::pb::eth::event::v1::Events;
use crate::pb::eth::event::v1::Event;
use substreams_ethereum::pb::eth::v2::Block;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Log;
use substreams_ethereum::pb::eth::v2::TransactionTrace;
use substreams::errors::Error;
use crate::util;

#[substreams::handlers::map]
fn map_contract_events(contract_address: String, blk: Block) -> Result<Events, Error> {
   let error = verify_parameter(&contract_address);
   if error.is_some() {
    return Err(error.unwrap());
   }

    let mut events: Vec<Event> = Vec::new();
    let contract_address_as_vec = match Hex::decode(&contract_address) {
        Ok(address) => address,
        Err(error) => return Err(Error::Unexpected(error.to_string())),
    };

    for transaction in &blk.transaction_traces {
        if transaction.to == contract_address_as_vec {
            let transaction_events = get_transaction_events(&transaction);
            events.extend(transaction_events);
        }
    }

    Ok(Events { events })
}

fn verify_parameter(contract_address: &String) -> Option<Error> {
    if !util::is_address_valid(contract_address) {
        return Some(Error::Unexpected(String::from("Contract address is not valid")))
    }

    return None
}

fn get_transaction_events(transaction: &TransactionTrace) -> Vec<Event> {
    let mut transaction_events: Vec<Event> = Vec::new();

    for log in &transaction.receipt().receipt.logs {
        let address = util::hexadecimal_to_string(&log.address);
        let topics = get_log_topics(&log);

        let event = create_event_from(address, topics, util::hexadecimal_to_string(&transaction.hash));
        transaction_events.push(event)
    }

    return transaction_events;
}

fn get_log_topics(log: &Log) -> Vec<String> {
    let mut topics: Vec<String> = Vec::new();

    for topic in &log.topics {
        let topic_string = util::hexadecimal_to_string(topic);
        topics.push(topic_string)
    }

    return topics;
}

fn create_event_from(address: String, topics: Vec<String>, hash: String) -> Event {

    return Event { 
        address, 
        topics, 
        tx_hash: hash 
    }
}