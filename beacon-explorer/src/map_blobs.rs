use substreams::Hex;
use crate::pb::beacon::blobs::v1::Blobs;
use crate::pb::sf::beacon::r#type::v1::{block::Body::*, Block};

#[substreams::handlers::map]
fn map_blobs(blk: Block) -> Result<Blobs, substreams::errors::Error> {

    Ok(Blobs {
        slot: blk.slot,
        blobs: match blk.body {
            Some(Deneb(body)) => body.embedded_blobs.into_iter().map(|b| Hex::encode(&b.blob)).collect(),
            _ => vec![],
        },
    })
}
