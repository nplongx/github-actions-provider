//! Deterministic content hashing used by provider contracts.

use serde::Serialize;
use sha2::{Digest, Sha256};

/// SHA-256 over the compact serde JSON representation of a typed value.
pub(crate) fn content_sha256<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    let serialized = serde_json::to_vec(value)?;
    Ok(hex::encode(Sha256::digest(serialized)))
}
