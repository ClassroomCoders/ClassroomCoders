use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug)]
pub enum CodecError {
    Encode(Box<dyn std::error::Error + Send + Sync>),
    Decode(Box<dyn std::error::Error + Send + Sync>),
}

impl std::fmt::Display for CodecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodecError::Encode(e) => writeln!(f, "Failed to encode: {}", e),
            CodecError::Decode(e) => writeln!(f, "Failed to decode: {}", e),
        }
    }
}

pub fn encode<T: Serialize>(v: &T) -> Result<Vec<u8>, CodecError> {
    serde_json::to_vec(v).map_err(|err| CodecError::Encode(err.into()))
}
pub fn decode<T: DeserializeOwned>(b: &[u8]) -> Result<T, CodecError> {
    serde_json::from_slice(b).map_err(|err| CodecError::Decode(err.into()))
}
