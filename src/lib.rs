use hex::{decode, encode};

pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, String> {
    // TODO: Decode hex string into Vec<u8>, return error string on failure

    if hex_str.len() % 2 != 0 {
        return Err("Decoded hex string must have an even length".to_string());
    }
    // Check if the string has an even number of characters - hex needs pairs

    decode(hex_str).map_err(|error| format!("Invalid hex: {}", error))
    // Try to decode and handle any errors that come up
}

pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    // TODO: Reverse the byte order of input slice and return as Vec<u8>

    let mut result = Vec::from(bytes);
    result.reverse();
    result
    // Just flip the bytes around
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    // TODO: Implement conversion of bytes slice to hex string

    encode(bytes)
    // Turn bytes into a hex string
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    // TODO: Implement conversion of hex string to bytes vector

    decode(hex)
    // Same as decode_hex but with a different error type
}

pub fn swap_endian_u32(num: u32) -> [u8; 4] {
    // TODO: Implement little-endian byte swap for u32

    num.to_le_bytes()
    // Convert to little-endian bytes (backwards byte order)
}

pub fn parse_satoshis(input: &str) -> Result<u64, String> {
    // TODO: Parse input string to u64, return error string if invalid
    input
        .parse::<u64>()
        .map_err(|_| "Invalid satoshi amount".to_string())
    // Try to parse as a number, return error if it fails
}

pub enum ScriptType {
    P2PKH,
    P2WPKH,
    Unknown,
}

pub fn classify_script(script: &[u8]) -> ScriptType {
    // TODO: Match script pattern and return corresponding ScriptType

    match script {
        // P2PKH (Pay-to-Public-Key-Hash) pattern: OP_DUP OP_HASH160 <20-byte-push>
        [0x76, 0xa9, 0x14, ..] => ScriptType::P2PKH,

        // P2WPKH (Pay-to-Witness-Public-Key-Hash) pattern: OP_0 <20-byte-push>
        [0x00, 0x14, ..] => ScriptType::P2WPKH,

        // Any other pattern is unknown
        _ => ScriptType::Unknown,
    }
    // Look at the first few bytes to figure out what kind of script this is
}

// TODO: complete Outpoint tuple struct
pub struct Outpoint(pub String, pub u32);

pub fn read_pushdata(script: &[u8]) -> &[u8] {
    // TODO: Return the pushdata portion of the script slice (assumes pushdata starts at index 2)
    if script.len() > 2 { &script[2..] } else { &[] }
    // Skip the first 2 bytes and return the rest
}

pub trait Wallet {
    fn balance(&self) -> u64;
}

pub struct TestWallet {
    pub confirmed: u64,
}

impl Wallet for TestWallet {
    fn balance(&self) -> u64 {
        // TODO: Return the wallet's confirmed balance
        self.confirmed
        // Just return the balance we have stored
    }
}

pub fn apply_fee(balance: &mut u64, fee: u64) {
    // TODO: Subtract fee from mutable balance reference
    *balance = balance.saturating_sub(fee);
    // Subtract the fee, but don't go below zero
}

pub fn move_txid(txid: String) -> String {
    // TODO: Return formatted string including the txid for display or logging
    format!("txid: {}", txid)
    // Add a label to make it clear what this ID is
}

// TODO: Add necessary derive traits
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Opcode {
    OpChecksig,
    OpDup,
    OpInvalid,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        // TODO: Implement mapping from byte to Opcode variant
        match byte {
            0xac => Ok(Opcode::OpChecksig),
            0x76 => Ok(Opcode::OpDup),
            _ => Err(format!("Invalid opcode: 0x{:02x}", byte)),
        }
        // Match the byte to the right opcode, or complain if we don't know it
    }
}

// TODO: Add necessary derive traits
#[derive(Clone, Debug, PartialEq)]
pub struct UTXO {
    pub txid: Vec<u8>,
    pub vout: u32,
    pub value: u64,
}

pub fn consume_utxo(utxo: UTXO) -> UTXO {
    // TODO: Implement UTXO consumption logic (if any)
    utxo
    // For now, just pass it through unchanged
}
