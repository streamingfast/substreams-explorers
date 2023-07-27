pub fn is_address_valid(address: &String) -> bool {
    // An address is always 40 hexadecimal characters (or 2 more character with 0x prefix)
    if address.len() != 40 && address.len() != 42 {
        return false;
    }

    true
}
