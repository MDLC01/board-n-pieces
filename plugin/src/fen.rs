use crate::model::{
    Board, CastlingAvailabilities, Color, File, Piece, PieceKind, Position, Rank, Square,
    SquareContent,
};
use crate::utils::Name;
use std::str::FromStr;

fn parse_piece(fen: char) -> crate::Result<Piece> {
    match fen {
        'P' => Ok(Piece::new(Color::White, PieceKind::Pawn)),
        'N' => Ok(Piece::new(Color::White, PieceKind::Knight)),
        'B' => Ok(Piece::new(Color::White, PieceKind::Bishop)),
        'R' => Ok(Piece::new(Color::White, PieceKind::Rook)),
        'Q' => Ok(Piece::new(Color::White, PieceKind::Queen)),
        'K' => Ok(Piece::new(Color::White, PieceKind::King)),
        'p' => Ok(Piece::new(Color::Black, PieceKind::Pawn)),
        'n' => Ok(Piece::new(Color::Black, PieceKind::Knight)),
        'b' => Ok(Piece::new(Color::Black, PieceKind::Bishop)),
        'r' => Ok(Piece::new(Color::Black, PieceKind::Rook)),
        'q' => Ok(Piece::new(Color::Black, PieceKind::Queen)),
        'k' => Ok(Piece::new(Color::Black, PieceKind::King)),
        c => Err(format!("invalid piece: {c}"))?,
    }
}

fn parse_board(fen: &str) -> crate::Result<Board> {
    let mut squares = [[SquareContent::Empty; 8]; 8];
    for (rank_index, fen_rank) in fen.split('/').enumerate() {
        let mut file_index = 0;
        for c in fen_rank.chars() {
            if let Some(n) = c.to_digit(10) {
                file_index += n as usize
            } else {
                squares[rank_index][file_index] = SquareContent::Piece(parse_piece(c)?);
                file_index += 1;
            }
        }
    }
    squares.reverse();
    Ok(Board::new(squares))
}

fn parse_castling_availabilities(fen: &str) -> crate::Result<CastlingAvailabilities> {
    Ok(CastlingAvailabilities {
        white_king_side: fen.contains('K'),
        white_queen_side: fen.contains('Q'),
        black_king_side: fen.contains('k'),
        black_queen_side: fen.contains('q'),
    })
}

fn parse_en_passant_target_square(fen: &str) -> crate::Result<Option<Square>> {
    if fen == "-" {
        return Ok(None);
    }
    Ok(Some(fen.parse()?))
}

fn parse_int(fen: &str) -> crate::Result<u32> {
    u32::from_str(fen).map_err(|err| err.to_string())
}

/// Parses Forsyth–Edwards Notation (FEN) into a position.
pub fn parse_fen(fen: &[u8]) -> crate::Result<Position> {
    let fen = std::str::from_utf8(fen).map_err(|_| "internal error: FEN should be valid UTF-8")?;

    let mut parts = fen.split(' ');

    let board = parse_board(parts.next().ok_or("invalid FEN: missing board info")?)?;

    let active = match parts.next() {
        Some("w") => Color::White,
        Some("b") => Color::Black,
        Some(p) => Err(format!("invalid active player: {p}"))?,
        None => return Ok(Position::default_with_board(board)),
    };

    let castling_availabilities = parse_castling_availabilities(
        parts
            .next()
            .ok_or("invalid FEN: missing castling availabilities")?,
    )?;

    let en_passant_target_square = parse_en_passant_target_square(
        parts
            .next()
            .ok_or("invalid FEN: missing en passant target square")?,
    )?;

    let halfmove = parse_int(parts.next().ok_or("invalid FEN: missing fullmove")?)?;

    let fullmove = parse_int(parts.next().ok_or("invalid FEN: missing fullmove")?)?;

    if parts.next().is_some() {
        Err("invalid FEN: too many parts")?
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
