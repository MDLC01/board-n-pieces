mod fen;
mod model;
mod pgn;
mod san;
mod utils;

use crate::fen::{fen, parse_fen};
use crate::model::Position;
use crate::pgn::PgnGame;
use crate::san::parse_turn;
use crate::utils::SliceExt;
use std::iter;
use wasm_minimal_protocol::{initiate_protocol, wasm_func};

initiate_protocol!();

pub type Result<T> = std::result::Result<T, String>;

fn serialize_position_sequence(positions: impl IntoIterator<Item = Position>) -> Vec<u8> {
    positions
        .into_iter()
        .flat_map(|position| iter::once(0).chain(fen(position).into_bytes()))
        .skip(1)
        .collect()
}

#[wasm_func]
pub fn replay_game(starting_position: &[u8], turns: &[u8]) -> Result<Vec<u8>> {
    let starting_position = std::str::from_utf8(starting_position)
        .map_err(|_| "internal error: FEN should be valid UTF-8")?;
    let turns = turns.split_on(0);
    let mut positions = Vec::with_capacity(turns.size_hint().0 + 1);
    positions.push(parse_fen(starting_position)?);
    for (i, turn) in turns.enumerate() {
        let Ok(turn) = std::str::from_utf8(turn) else {
            Err("internal error: each turn should be a valid UTF-8 string")?
        };
        positions.push(parse_turn(turn)?.apply(i, positions.last().unwrap())?);
    }
    Ok(serialize_position_sequence(positions))
}

#[wasm_func]
pub fn game_from_pgn(pgn: &[u8]) -> Result<Vec<u8>> {
    let Ok(pgn) = std::str::from_utf8(pgn) else {
        // The specification actually requires that PGN used ASCII, but we allow UTF-8 because this
        // is today's world standard.
        Err("internal error: PGN should be a valid UTF-8 string")?
    };
    let game = pgn.parse::<PgnGame>()?;
    let mut positions = Vec::with_capacity(game.len() + 1);
    positions.push(game.starting_position);
    for (i, turn) in game.turns.iter().enumerate() {
        positions.push(turn.apply(i, positions.last().unwrap())?)
    }
    Ok(serialize_position_sequence(positions))
}
