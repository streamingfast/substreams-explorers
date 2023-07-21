use substreams::Hex;

pub fn hexadecimal_to_string(hex: &Vec<u8>) -> String {
    return Hex::encode(hex);
}

pub fn is_transaction_hash_valid(hash: &String) -> bool {
    // A transaction hash is always 64 hexadecimal characters
    if hash.len() != 64 {
        return false;
    }
    
    return true;
}

pub fn is_address_valid(address: &String) -> bool {
     // An address is always 40 hexadecimal characters
    if address.len() != 40  {
       return false;
    }

    return true;
}
