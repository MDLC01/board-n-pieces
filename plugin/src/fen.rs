use crate::model::{
    Board, CastlingAvailabilities, Color, File, Piece, PieceKind, Position, Rank, Square,
    SquareContent,
};
use crate::utils::SliceExt;
use std::str::FromStr;

fn parse_piece(fen: u8) -> crate::Result<Piece> {
    match fen {
        b'P' => Ok(Piece::new(Color::White, PieceKind::Pawn)),
        b'N' => Ok(Piece::new(Color::White, PieceKind::Knight)),
        b'B' => Ok(Piece::new(Color::White, PieceKind::Bishop)),
        b'R' => Ok(Piece::new(Color::White, PieceKind::Rook)),
        b'Q' => Ok(Piece::new(Color::White, PieceKind::Queen)),
        b'K' => Ok(Piece::new(Color::White, PieceKind::King)),
        b'p' => Ok(Piece::new(Color::Black, PieceKind::Pawn)),
        b'n' => Ok(Piece::new(Color::Black, PieceKind::Knight)),
        b'b' => Ok(Piece::new(Color::Black, PieceKind::Bishop)),
        b'r' => Ok(Piece::new(Color::Black, PieceKind::Rook)),
        b'q' => Ok(Piece::new(Color::Black, PieceKind::Queen)),
        b'k' => Ok(Piece::new(Color::Black, PieceKind::King)),
        _ => Err("Invalid piece")?,
    }
}

fn parse_board(fen: &[u8]) -> crate::Result<Board> {
    let mut squares = [[SquareContent::Empty; 8]; 8];
    for (rank_index, fen_rank) in fen.split_on(b'/').enumerate() {
        let fen_rank: &[u8] = fen_rank;
        let mut file_index = 0;
        for c in fen_rank {
            if let Some(n) = (*c as char).to_digit(10) {
                file_index += n as usize
            } else {
                squares[rank_index][file_index] = SquareContent::Piece(parse_piece(*c)?);
                file_index += 1;
            }
        }
    }
    squares.reverse();
    Ok(Board::new(squares))
}

fn parse_castling_availabilities(fen: &[u8]) -> crate::Result<CastlingAvailabilities> {
    Ok(CastlingAvailabilities {
        white_king_side: fen.contains(&b'K'),
        white_queen_side: fen.contains(&b'Q'),
        black_king_side: fen.contains(&b'k'),
        black_queen_side: fen.contains(&b'q'),
    })
}

fn parse_en_passant_target_square(fen: &[u8]) -> crate::Result<Option<Square>> {
    if fen == b"-" {
        return Ok(None);
    }
    let [f, r] = fen else { Err("Invalid square")? };
    let file = File::new((*f - b'a') as usize).ok_or("Invalid file")?;
    let rank = Rank::new((*r - b'1') as usize).ok_or("Invalid rank")?;
    Ok(Some(Square::new(file, rank)))
}

fn parse_int(fen: &[u8]) -> crate::Result<u32> {
    u32::from_str(
        std::str::from_utf8(fen)
            .map_err(|_| "Unreachable: FEN was already checked to be valid ASCII.")?,
    )
    .map_err(|err| err.to_string())
}

/// Parses Forsyth–Edwards Notation (FEN) into a position.
pub fn parse_fen(fen: &[u8]) -> crate::Result<Position> {
    if !fen.is_ascii() {
        Err("Invalid FEN: expected ASCII")?
    }

    let mut parts = fen.split_on(b' ').peekable();

    let board = parse_board(parts.next().ok_or("Invalid FEN: missing board info")?)?;

    if parts.peek().is_none() {
        return Ok(Position::default_with_board(board));
    }

    let active = match parts.next() {
        Some(b"w") => Color::White,
        Some(b"b") => Color::Black,
        Some(_) => Err("Invalid active player")?,
        None => Err("Invalid FEN: missing active player")?,
    };

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

fn fen_piece(piece: Piece) -> String {
    let mut s = piece.kind.to_string();
    match piece.color {
        Color::White => s.make_ascii_uppercase(),
        Color::Black => s.make_ascii_lowercase(),
    }
    s
}

fn fen_board(board: Board) -> String {
    let mut s = String::new();
    for r in (0..8).rev() {
        let mut empty_streak = 0;
        for f in 0..8 {
            match board[Square::new(File::new(f).unwrap(), Rank::new(r).unwrap())] {
                SquareContent::Empty => empty_streak += 1,
                SquareContent::Piece(piece) => {
                    if empty_streak > 0 {
                        s.push_str(&empty_streak.to_string());
                        empty_streak = 0
                    }
                    s.push_str(&fen_piece(piece))
                }
            }
        }
        if empty_streak > 0 {
            s.push_str(&empty_streak.to_string())
        }
        if r != 0 {
            s.push('/')
        }
    }
    s
}

fn fen_color(color: Color) -> &'static str {
    match color {
        Color::White => "w",
        Color::Black => "b",
    }
}

fn fen_castling_availabilities(castling_availabilities: CastlingAvailabilities) -> String {
    let mut s = String::new();
    if castling_availabilities.white_king_side {
        s.push('K')
    }
    if castling_availabilities.white_queen_side {
        s.push('Q')
    }
    if castling_availabilities.black_king_side {
        s.push('k')
    }
    if castling_availabilities.black_queen_side {
        s.push('q')
    }
    if s.is_empty() {
        s.push('-')
    }
    s
}

/// Converts a position to Forsyth–Edwards Notation (FEN).
pub fn fen(position: Position) -> String {
    format!(
        "{} {} {} {} {} {}",
        fen_board(position.board),
        fen_color(position.active),
        fen_castling_availabilities(position.castling_availabilities),
        match position.en_passant_target_square {
            None => "-".to_string(),
            Some(square) => square.name(),
        },
        position.halfmove,
        position.fullmove,
    )
}
