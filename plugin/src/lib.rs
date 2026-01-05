#![allow(refining_impl_trait)]

mod fen;
mod model;
mod pgn;
mod san;
mod utils;

use crate::fen::{fen, parse_fen};
use crate::model::{Movement, Position};
use crate::pgn::PgnGame;
use crate::san::parse_turn;
use crate::utils::SliceExt;
use std::iter;
use wasm_minimal_protocol::{initiate_protocol, wasm_func};

initiate_protocol!();

pub type Result<T> = std::result::Result<T, String>;

fn serialize_game(
    positions: impl IntoIterator<Item = Position>,
    movements: impl IntoIterator<Item = Movement>,
) -> Vec<u8> {
    let serialized_positions = positions
        .into_iter()
        .flat_map(|position| iter::once(0).chain(fen(position).into_bytes()))
        .skip(1);
    let serialized_movements = movements
        .into_iter()
        .flat_map(|m| iter::once(0).chain(m.to_string().into_bytes()))
        .skip(1);
    serialized_positions
        .chain(iter::once(0xff))
        .chain(serialized_movements)
        .collect()
}

#[wasm_func]
pub fn invert_position(position: &[u8]) -> Result<Vec<u8>> {
    let position =
        std::str::from_utf8(position).map_err(|_| "internal error: FEN should be valid UTF-8")?;
    Ok(fen(parse_fen(position)?.invert()).into_bytes())
}

#[wasm_func]
pub fn replay_game(starting_position: &[u8], turns: &[u8]) -> Result<Vec<u8>> {
    let starting_position = std::str::from_utf8(starting_position)
        .map_err(|_| "internal error: FEN should be valid UTF-8")?;
    let turns = turns.split_on(0);
    let mut positions = Vec::with_capacity(turns.size_hint().0 + 1);
    let mut movements = Vec::with_capacity(turns.size_hint().0);
    positions.push(parse_fen(starting_position)?);
    for (i, turn) in turns.enumerate() {
        let Ok(turn) = std::str::from_utf8(turn) else {
            Err("internal error: each turn should be a valid UTF-8 string")?
        };
        let (position, movement) = parse_turn(turn)?.apply(i, positions.last().unwrap())?;
        positions.push(position);
        movements.push(movement);
    }
    Ok(serialize_game(positions, movements))
}

#[wasm_func]
pub fn game_from_pgn(pgn: &[u8]) -> Result<Vec<u8>> {
    let Ok(pgn) = std::str::from_utf8(pgn) else {
        // The specification actually requires that PGN uses ASCII, but we allow UTF-8 because this
        // is today's world standard.
        Err("internal error: PGN should be a valid UTF-8 string")?
    };
    let game = pgn.parse::<PgnGame>()?;
    let mut positions = Vec::with_capacity(game.len() + 1);
    let mut movements = Vec::with_capacity(game.len());
    positions.push(game.starting_position);
    for (i, turn) in game.turns.iter().enumerate() {
        let (position, movement) = turn.apply(i, positions.last().unwrap())?;
        positions.push(position);
        movements.push(movement);
    }
    Ok(serialize_game(positions, movements))
}
