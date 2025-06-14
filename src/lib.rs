use hex::{decode, FromHexError};

pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {

    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }

    let tx_byte = match decode(raw_tx_hex) {
        Ok(tx_byte) => tx_byte,
        Err(e) => match e {
            FromHexError::InvalidHexCharacter { .. } => {
                return Err("Hex decode error".to_string());
            },
            FromHexError::OddLength => {
                return Err("Odd length hex string".to_string());
            },
            FromHexError::InvalidStringLength => {
                return Err("Transaction data too short".to_string());
            },
        },
    };

    let version = tx_byte[0] as u32;
    Ok(version)
}
