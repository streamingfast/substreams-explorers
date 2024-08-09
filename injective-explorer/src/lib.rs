mod pb;

use crate::pb::cosmos::authz::v1beta1::MsgExec;
use crate::pb::cosmos::v1::MsgExecList;
use crate::pb::cosmos::v1::MsgExecMeta;
use crate::pb::sf::cosmos::r#type::v2::Block;
use cosmrs::Any;
use cosmrs::Tx;
use prost::Message;
use substreams::errors::Error;

#[substreams::handlers::map]
pub fn map_msg_exec_messages(block: Block) -> Result<MsgExecList, Error> {
    // Mutable list to add the output of the Substreams
    let mut msg_execs: Vec<MsgExecMeta> = Vec::new();

    // Iterate over the transactions in the block
    for tx in block.txs.iter() {
        let tx_as_u8: &[u8] = &tx[..];

        // Decode the transaction bytes into a Rust object
        if let Ok(transaction) = Tx::from_bytes(tx_as_u8) {

            // Iterate over the transaction messages
            for message in transaction.body.messages {

                // Extract ONLY messages of type MsgExec. If any, add them to the list.
                if let Some(msg_exec_meta) = extract_msg_exec_message(&message) {
                    msg_execs.push(msg_exec_meta);
                }
            }
        }
    }

    Ok(MsgExecList { msgs: msg_execs })
}

fn extract_msg_exec_message(message: &Any) -> Option<MsgExecMeta> {
    let message_bytes: &[u8] = &message.value[..];

    // Check if the message is of type MsgExec
    if &message.type_url != "/cosmos.authz.v1beta1.MsgExec" {
        return None;
    }
    
    // Decode the bytes of the bytes into the MsgExec Protobuf
    if let Ok(msg_exec) = MsgExec::decode(message_bytes) {
        let msgs_string: Vec<String> = msg_exec.msgs.iter().map(|ms| ms.type_url.to_owned()).collect();

        // Create the output object of the Substreams
        return Some(MsgExecMeta {
            grantee: msg_exec.grantee,
            msg_strings: msgs_string,
        });
    }

    return None;
}
