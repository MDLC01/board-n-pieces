use crate::abi;
use crate::abi::{Abi, ByteAbi};
use crate::model::{Board, CastlingAvailabilities, Color, Piece, Position, Square, SquareContent};
use crate::utils::SliceExt;
use std::iter;
use std::str::FromStr;

fn parse_board(fen: &[u8]) -> abi::Result<Board> {
    let mut squares = Vec::new();
    for fen_rank in fen.split_on(b'/') {
        let fen_rank: &[u8] = fen_rank;
        let mut rank = Vec::new();
        for c in fen_rank {
            if let Some(n) = (*c as char).to_digit(10) {
                rank.extend(iter::repeat(SquareContent::Empty).take(n as usize))
            } else {
                rank.push(SquareContent::Piece(Piece::from_byte(*c)?))
            }
        }
        squares.push(rank)
    }
    squares.reverse();
    Ok(Board::new(squares))
}

fn parse_castling_availabilities(fen: &[u8]) -> abi::Result<CastlingAvailabilities> {
    Ok(CastlingAvailabilities {
        white_king_side: fen.contains(&b'K'),
        white_queen_side: fen.contains(&b'Q'),
        black_king_side: fen.contains(&b'k'),
        black_queen_side: fen.contains(&b'q'),
    })
}

fn parse_en_passant_target_square(fen: &[u8]) -> abi::Result<Option<Square>> {
    if fen == b"-" {
        return Ok(None);
    }
    Ok(Some(Square::from_slice(fen)?))
}

fn parse_int(fen: &[u8]) -> abi::Result<u32> {
    u32::from_str(
        std::str::from_utf8(fen)
            .map_err(|_| "Unreachable: FEN was already checked to be valid ASCII.")?,
    )
    .map_err(|err| err.to_string())
}

pub fn parse_fen(fen: &[u8]) -> abi::Result<Position> {
    if !fen.is_ascii() {
        Err("Invalid FEN: expected ASCII")?
    }

    let mut parts = fen.split_on(b' ').peekable();

    let board = parse_board(parts.next().ok_or("Invalid FEN: missing board info")?)?;

    if parts.peek().is_none() {
        return Ok(Position::default_with_board(board));
    }

    let active = Color::from_slice(parts.next().ok_or("Invalid FEN: missing active player")?)?;

    let castling_availabilities = parse_castling_availabilities(
        parts
            .next()
            .ok_or("Invalid FEN: missing castling availabilities")?,
    )?;

    let en_passant_target_square = parse_en_passant_target_square(
        parts
            .next()
            .ok_or("Invalid FEN: missing en passant target square")?,
    )?;

    let halfmove = parse_int(parts.next().ok_or("Invalid FEN: expected fullmove")?)?;

    let fullmove = parse_int(parts.next().ok_or("Invalid FEN: expected fullmove")?)?;

    if parts.next().is_some() {
        Err("Invalid FEN: too many parts")?
    }

    Ok(Position {
        board,
        active,
        castling_availabilities,
        en_passant_target_square,
        halfmove,
        fullmove,
    })
}
