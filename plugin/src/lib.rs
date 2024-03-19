mod abi;
mod fen;
mod model;
mod utils;

use crate::abi::Abi;
use wasm_minimal_protocol::{initiate_protocol, wasm_func};

initiate_protocol!();

#[wasm_func]
pub fn parse_fen(fen: &[u8]) -> abi::Result<Vec<u8>> {
    Ok(fen::parse_fen(fen)?.to_vec())
}
