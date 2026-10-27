use std::path::PathBuf;

use serde::de::DeserializeOwned;

/// Directory holding the vector files, relative to this crate's root.
const VECTORS_DIR: &str = "../../vectors";

pub(crate) fn load<T: DeserializeOwned>(name: &str) -> T {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(VECTORS_DIR)
        .join(name);
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
    serde_json::from_slice(&bytes)
        .unwrap_or_else(|e| panic!("cannot parse {}: {e}", path.display()))
}
