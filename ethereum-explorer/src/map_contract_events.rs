use crate::pb::eth::event::v1::Event;
use crate::pb::eth::event::v1::Events;
use crate::util;
use anyhow::anyhow;
use anyhow::Ok;
use substreams::errors::Error;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
fn map_contract_events(contract_address: String, blk: Block) -> Result<Events, Error> {
    verify_parameter(&contract_address)?;

    let events: Vec<Event> = blk
        .logs()
        .filter(|log| log.address().to_vec() == Hex::decode(&contract_address).expect("already validated"))
        .map(|log| Event {
            address: Hex::encode(log.address()),
            topics: log.topics().into_iter().map(Hex::encode).collect(),
            tx_hash: Hex::encode(&log.receipt.transaction.hash),
        })
        .collect();

    Ok(Events { events })
}

fn verify_parameter(address: &String) -> Result<(), Error> {
    if !util::is_address_valid(&address) {
        return Err(anyhow!("Contract address ({}) is not valid", address));
    }

    Ok(())
}
