mod pb;
use crate::pb::eth::event::v1::Events;
use crate::pb::eth::event::v1::Event;
use substreams_ethereum::pb::eth::v2::Block;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Log;
use substreams_ethereum::pb::eth::v2::TransactionTrace;

pub fn map_contract_events(contract_address: String, blk: &Block) -> Events {
    let mut events: Vec<Event> = Vec::new();

    for tr in &blk.transaction_traces {
        let to = Hex(&tr.to).to_string();

        if to == contract_address {
            //substreams::log::info!(hash);

            let transaction_events = &mut get_transaction_events(&tr);
            events.append(transaction_events);

            substreams::log::info!("//////////////////");
        }
    }

    return Events { events }
}

fn get_transaction_events(transaction: &TransactionTrace) -> Vec<Event> {
    let mut transaction_events: Vec<Event> = Vec::new();

    for log in &transaction.receipt().receipt.logs {
        let address = Hex(&log.address).to_string();
        let topics = get_log_topics(&log);

        transaction_events.push(create_event_from(address, topics, &transaction.hash))
    }

    return transaction_events;
}

fn get_log_topics(log: &Log) -> Vec<String> {
    let mut topics: Vec<String> = Vec::new();

    for topic in &log.topics {
        let topic_string = Hex(topic).to_string();
        topics.push(topic_string)
    }

    return topics;
}

fn create_event_from(address: String, topics: Vec<String>, hash: &Vec<u8>) -> Event {
    let hash_as_string = Hex(hash).to_string();

    return Event { 
        address, 
        topics, 
        tx_hash: hash_as_string 
    }
}