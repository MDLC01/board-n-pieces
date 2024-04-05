mod fen;
mod san;
mod model;
mod utils;

use crate::fen::{fen, parse_fen};
use crate::san::parse_turn;
use crate::utils::SliceExt;
use std::iter;
use wasm_minimal_protocol::{initiate_protocol, wasm_func};

initiate_protocol!();

pub type Result<T> = std::result::Result<T, String>;

#[wasm_func]
pub fn replay_game(starting_position: &[u8], turns: &[u8]) -> Result<Vec<u8>> {
    let turns = turns.split_on(0);
    let mut positions = Vec::with_capacity(turns.size_hint().0 + 1);
    positions.push(parse_fen(starting_position)?);
    for (i, turn) in turns.enumerate() {
        let Ok(turn) = std::str::from_utf8(turn) else {
            Err("Internal error: each turn should be a valid UTF-8 string")?
        };
        positions.push(parse_turn(turn)?.apply(i, positions.last().unwrap())?);
    }
    Ok(positions
        .into_iter()
        .flat_map(|position| iter::once(0).chain(fen(position).into_bytes()))
        .skip(1)
        .collect())
}
