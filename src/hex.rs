#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HexError {
    InvalidHexCharacter { c: char, index: usize },
    InvalidStringLength,
}

impl std::error::Error for HexError {}

impl std::fmt::Display for HexError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            HexError::InvalidHexCharacter { c, index } => {
                write!(f, "Invalid character {c:?} at position {index}")
            }
            HexError::InvalidStringLength => write!(f, "Invalid string length"),
        }
    }
}

const fn val(c: u8, idx: usize) -> Result<u8, HexError> {
    match c {
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'0'..=b'9' => Ok(c - b'0'),
        _ => Err(HexError::InvalidHexCharacter {
            c: c as char,
            index: idx,
        }),
    }
}

/// Decode single u8 hex string
pub fn decode_to_u8<T: AsRef<[u8]>>(data: T) -> Result<u8, HexError> {
    let data = data.as_ref();

    if data.len() != 2 {
        return Err(HexError::InvalidStringLength);
    }

    let byte = val(data[0], 0)? << 4 | val(data[1], 1)?;

    Ok(byte)
}
