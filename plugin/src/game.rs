use crate::model::{Color, File, Piece, PieceKind, Position, Rank, Square, SquareContent};
use crate::utils::OptionExt;
use std::fmt::{Display, Formatter};
use std::{fmt, iter};

#[derive(Debug, Copy, Clone)]
struct Move {
    from: Square,
    to: Square,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{}", self.from, self.to)
    }
}

#[derive(Debug, Copy, Clone)]
struct LocalSquare {
    color: Color,
    local_file: File,
    local_rank: Rank,
}

impl LocalSquare {
    fn new(color: Color, absolute_square: Square) -> Self {
        let local_square = match color {
            Color::White => absolute_square,
            Color::Black => absolute_square.transpose(),
        };
        Self {
            color,
            local_file: local_square.file(),
            local_rank: local_square.rank(),
        }
    }

    pub fn forward(self) -> Option<Self> {
        self.local_rank
            .index()
            .checked_add(1)
            .and_then(Rank::new)
            .map(|local_rank| Self { local_rank, ..self })
    }

    pub fn backward(self) -> Option<Self> {
        self.local_rank
            .index()
            .checked_sub(1)
            .and_then(Rank::new)
            .map(|local_rank| Self { local_rank, ..self })
    }

    pub fn left(self) -> Option<Self> {
        self.local_file
            .index()
            .checked_sub(1)
            .and_then(File::new)
            .map(|local_file| Self { local_file, ..self })
    }

    pub fn right(self) -> Option<Self> {
        self.local_file
            .index()
            .checked_add(1)
            .and_then(File::new)
            .map(|local_file| Self { local_file, ..self })
    }

    pub fn forward_left(self) -> Option<Self> {
        self.forward().and_then(Self::left)
    }

    pub fn forward_right(self) -> Option<Self> {
        self.forward().and_then(Self::right)
    }

    pub fn backward_left(self) -> Option<Self> {
        self.backward().and_then(Self::left)
    }

    pub fn backward_right(self) -> Option<Self> {
        self.backward().and_then(Self::right)
    }
}

impl From<LocalSquare> for Square {
    fn from(value: LocalSquare) -> Self {
        let local_square = Self::new(value.local_file, value.local_rank);
        match value.color {
            Color::White => local_square,
            Color::Black => local_square.transpose(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct LocalMove {
    from: LocalSquare,
    to: LocalSquare,
}

impl LocalMove {
    fn new(from: LocalSquare, to: LocalSquare) -> Self {
        Self { from, to }
    }
}

impl From<LocalMove> for Move {
    fn from(value: LocalMove) -> Self {
        Self {
            from: value.from.into(),
            to: value.to.into(),
        }
    }
}

fn generate_affine_moves(
    moves: &mut Vec<LocalMove>,
    position: &Position,
    departure: LocalSquare,
    mut next: impl FnMut(LocalSquare) -> Option<LocalSquare>,
) {
    let mut destination = departure;
    while let Some(s) = next(destination) {
        destination = s;
        moves.push(LocalMove::new(departure, destination));
        if !position.at(destination.into()).is_empty() {
            break;
        }
    }
}

macro_rules! gen_move {
    ($moves:ident, $departure:ident, [$( $step:expr ), *]) => {
        if let Some(destination) = Some($departure)
            $( .and_then($step) )*
        {
            $moves.push(LocalMove::new($departure, destination))
        }
    }
}

/// Returns all valid moves pieces of a specific kind can make in a specific position, not including
/// castling moves.
/// Squares in the returned move are relative to the active player.
fn valid_moves(position: &Position, piece_kind: PieceKind) -> crate::Result<Vec<Move>> {
    let mut moves = Vec::new();
    for global_square in Square::all() {
        let departure = LocalSquare::new(position.active, global_square);
        if !position
            .at(departure.into())
            .is(position.active, piece_kind)
        {
            continue;
        }
        match piece_kind {
            PieceKind::Pawn => {
                // Forward pawn move.
                if let Some(destination) = departure.forward() {
                    if position.at(destination.into()).is_empty() {
                        moves.push(LocalMove::new(departure, destination));
                    }
                }

                // Initial double pawn move.
                if let Some(destination) = departure.forward().and_then(LocalSquare::forward) {
                    if position.at(destination.into()).is_empty() {
                        moves.push(LocalMove::new(departure, destination));
                    }
                }

                // Pawn capture.
                for destination in departure
                    .forward_left()
                    .into_iter()
                    .chain(departure.forward_right())
                {
                    if !position.at(destination.into()).is_empty()
                        || position
                            .en_passant_target_square
                            .is_some_and(|t| t == destination.into())
                    {
                        moves.push(LocalMove::new(departure, destination));
                    }
                }
            }

            PieceKind::Knight => {
                gen_move!(
                    moves,
                    departure,
                    [
                        LocalSquare::forward,
                        LocalSquare::forward,
                        LocalSquare::left
                    ]
                );
                gen_move!(
                    moves,
                    departure,
                    [
                        LocalSquare::forward,
                        LocalSquare::forward,
                        LocalSquare::right
                    ]
                );
                gen_move!(
                    moves,
                    departure,
                    [
                        LocalSquare::backward,
                        LocalSquare::backward,
                        LocalSquare::left
                    ]
                );
                gen_move!(
                    moves,
                    departure,
                    [
                        LocalSquare::backward,
                        LocalSquare::backward,
                        LocalSquare::right
                    ]
                );
                gen_move!(
                    moves,
                    departure,
                    [LocalSquare::left, LocalSquare::left, LocalSquare::forward]
                );
                gen_move!(
                    moves,
                    departure,
                    [LocalSquare::right, LocalSquare::right, LocalSquare::forward]
                );
                gen_move!(
                    moves,
                    departure,
                    [LocalSquare::left, LocalSquare::left, LocalSquare::backward]
                );
                gen_move!(
                    moves,
                    departure,
                    [
                        LocalSquare::right,
                        LocalSquare::right,
                        LocalSquare::backward
                    ]
                );
            }

            PieceKind::Bishop => {
                generate_affine_moves(&mut moves, position, departure, LocalSquare::forward_left);
                generate_affine_moves(&mut moves, position, departure, LocalSquare::forward_right);
                generate_affine_moves(&mut moves, position, departure, LocalSquare::backward_left);
                generate_affine_moves(&mut moves, position, departure, LocalSquare::backward_right);
            }

            PieceKind::Rook => {
                generate_affine_moves(&mut moves, position, departure, LocalSquare::forward);
                generate_affine_moves(&mut moves, position, departure, LocalSquare::backward);
                generate_affine_moves(&mut moves, position, departure, LocalSquare::left);
                generate_affine_moves(&mut moves, position, departure, LocalSquare::right);
            }

            PieceKind::Queen => {
                generate_affine_moves(&mut moves, position, departure, LocalSquare::forward);
                generate_affine_moves(&mut moves, position, departure, LocalSquare::backward);
                generate_affine_moves(&mut moves, position, departure, LocalSquare::left);
                generate_affine_moves(&mut moves, position, departure, LocalSquare::right);
                generate_affine_moves(&mut moves, position, departure, LocalSquare::forward_left);
                generate_affine_moves(&mut moves, position, departure, LocalSquare::forward_right);
                generate_affine_moves(&mut moves, position, departure, LocalSquare::backward_left);
                generate_affine_moves(&mut moves, position, departure, LocalSquare::backward_right);
            }

            PieceKind::King => moves.extend(
                // FIXME: This should not include moves that put the king in a check position.
                iter::empty()
                    .chain(departure.forward())
                    .chain(departure.backward())
                    .chain(departure.left())
                    .chain(departure.right())
                    .chain(departure.forward_left())
                    .chain(departure.forward_right())
                    .chain(departure.backward_left())
                    .chain(departure.backward_right())
                    .map(|destination| LocalMove::new(departure, destination)),
            ),
        }
    }
    Ok(moves.into_iter().map(|m| m.into()).collect())
}

#[derive(Debug, Copy, Clone)]
pub enum AlgebraicTurn {
    Normal {
        destination_file: File,
        destination_rank: Rank,
        piece: PieceKind,
        departure_file: Option<File>,
        departure_rank: Option<Rank>,
        capture: bool,
        promotion: Option<PieceKind>,
    },
    KingSideCastle,
    QueenSideCastle,
}

impl AlgebraicTurn {
    pub fn apply(self, turn_index: usize, position: &Position) -> crate::Result<Position> {
        match self {
            Self::Normal {
                destination_file,
                destination_rank,
                piece,
                departure_file,
                departure_rank,
                capture,
                promotion,
            } => {
                let valid_moves = valid_moves(position, piece)?;
                let mut possible_moves = valid_moves.clone().into_iter().filter(|m| {
                    m.to.file() == destination_file
                        && m.to.rank() == destination_rank
                        && departure_file.is_none_or(|file| m.from.file() == file)
                        && departure_rank.is_none_or(|rank| m.from.rank() == rank)
                });
                let Some(turn) = possible_moves.next() else {
                    Err(format!("Turn #{} is impossible.", turn_index + 1))?
                };
                if possible_moves.next().is_some() {
                    Err("Ambiguous move.")?
                }

                // TODO: Update flags such as castling availabilities.
                // TODO: Support en passant.
                let mut new_board = position.board.clone();
                new_board[turn.from] = SquareContent::Empty;
                new_board[turn.to] =
                    SquareContent::Piece(Piece::new(position.active, promotion.unwrap_or(piece)));

                Ok(Position {
                    board: new_board,

                    active: position.active.other(),

                    // TODO: Update that properly.
                    castling_availabilities: position.castling_availabilities,

                    // TODO: Update that properly.
                    en_passant_target_square: position.en_passant_target_square,

                    halfmove: if capture || piece == PieceKind::Pawn {
                        0
                    } else {
                        position.halfmove
                    },

                    fullmove: if position.active == Color::Black {
                        position.fullmove + 1
                    } else {
                        position.fullmove
                    },
                })
            }

            Self::KingSideCastle => {
                if !position
                    .castling_availabilities
                    .king_side_for(position.active)
                {
                    Err("Not allowed to castle king-side.")?
                }
                // TODO
                Err("Castling moves not implemented yet.")?
            }

            Self::QueenSideCastle => {
                if !position
                    .castling_availabilities
                    .queen_side_for(position.active)
                {
                    Err("Not able to castle king-side.")?
                }
                // TODO
                Err("Castling moves not implemented yet.")?
            }
        }
    }
}

macro_rules! file {
    () => {
        b'a'..=b'h'
    };
}

macro_rules! rank {
    () => {
        b'1'..=b'8'
    };
}

macro_rules! capture {
    () => {
        b'x' | b':'
    };
}

macro_rules! piece {
    () => {
        b'P' | b'N' | b'B' | b'R' | b'Q' | b'K'
    };
}

pub fn parse_algebraic_turn(source: &[u8]) -> crate::Result<AlgebraicTurn> {
    // TODO: Support markers for check (+), checkmate (#) and comments (??, !!, ?, !, !?, ?!)
    // TODO: Maybe this could be rewritten as a sort of RR1 thing by starting from the right.

    let capture = source.iter().any(|c| matches!(c, capture!()));

    match source {
        // "e4", "xe4" (pawn move + optional capture)
        [f @ file!(), r @ rank!()] | [capture!(), f @ file!(), r @ rank!()] => {
            Ok(AlgebraicTurn::Normal {
                destination_file: (*f as char).to_string().parse()?,
                destination_rank: (*r as char).to_string().parse()?,
                piece: PieceKind::Pawn,
                departure_file: None,
                departure_rank: None,
                capture,
                promotion: None,
            })
        }

        // "exd5" (disambiguated pawn move + capture)
        [f0 @ file!(), capture!(), f @ file!(), r @ rank!()] => Ok(AlgebraicTurn::Normal {
            destination_file: (*f as char).to_string().parse()?,
            destination_rank: (*r as char).to_string().parse()?,
            piece: PieceKind::Pawn,
            departure_file: Some((*f0 as char).to_string().parse()?),
            departure_rank: None,
            capture,
            promotion: None,
        }),

        // "e8Q", "xe8Q" (pawn move + optional capture + promote)
        [f @ file!(), r @ rank!(), p @ piece!()]
        | [f @ file!(), r @ rank!(), b'=', p @ piece!()]
        | [f @ file!(), r @ rank!(), b'/', p @ piece!()]
        | [f @ file!(), r @ rank!(), b'(', p @ piece!(), b')']
        | [capture!(), f @ file!(), r @ rank!(), p @ piece!()]
        | [capture!(), f @ file!(), r @ rank!(), b'=', p @ piece!()]
        | [capture!(), f @ file!(), r @ rank!(), b'/', p @ piece!()]
        | [capture!(), f @ file!(), r @ rank!(), b'(', p @ piece!(), b')'] => {
            Ok(AlgebraicTurn::Normal {
                destination_file: (*f as char).to_string().parse()?,
                destination_rank: (*r as char).to_string().parse()?,
                piece: PieceKind::Pawn,
                departure_file: None,
                departure_rank: None,
                capture,
                promotion: Some((*p as char).to_string().parse()?),
            })
        }

        // "exd5Q" (disambiguated pawn move + capture + promote)
        [f0 @ file!(), capture!(), f @ file!(), r @ rank!(), p @ piece!()]
        | [f0 @ file!(), capture!(), f @ file!(), r @ rank!(), b'=', p @ piece!()]
        | [f0 @ file!(), capture!(), f @ file!(), r @ rank!(), b'/', p @ piece!()]
        | [f0 @ file!(), capture!(), f @ file!(), r @ rank!(), b'(', p @ piece!(), b')'] => {
            Ok(AlgebraicTurn::Normal {
                destination_file: (*f as char).to_string().parse()?,
                destination_rank: (*r as char).to_string().parse()?,
                piece: PieceKind::Pawn,
                departure_file: Some((*f0 as char).to_string().parse()?),
                departure_rank: None,
                capture,
                promotion: Some((*p as char).to_string().parse()?),
            })
        }

        // "Be5", "Bxe5" (piece move)
        [p @ piece!(), f @ file!(), r @ rank!()]
        | [p @ piece!(), capture!(), f @ file!(), r @ rank!()] => Ok(AlgebraicTurn::Normal {
            destination_file: (*f as char).to_string().parse()?,
            destination_rank: (*r as char).to_string().parse()?,
            piece: (*p as char).to_string().parse()?,
            departure_file: None,
            departure_rank: None,
            capture,
            promotion: None,
        }),

        // "Rdf8", "Rdxf8"
        [p @ piece!(), f0 @ file!(), f @ file!(), r @ rank!()]
        | [p @ piece!(), f0 @ file!(), capture!(), f @ file!(), r @ rank!()] => {
            Ok(AlgebraicTurn::Normal {
                destination_file: (*f as char).to_string().parse()?,
                destination_rank: (*r as char).to_string().parse()?,
                piece: (*p as char).to_string().parse()?,
                departure_file: Some((*f0 as char).to_string().parse()?),
                departure_rank: None,
                capture,
                promotion: None,
            })
        }

        // "R1a3", "R1xa3"
        [p @ piece!(), r0 @ rank!(), f @ file!(), r @ rank!()]
        | [p @ piece!(), r0 @ rank!(), capture!(), f @ file!(), r @ rank!()] => {
            Ok(AlgebraicTurn::Normal {
                destination_file: (*f as char).to_string().parse()?,
                destination_rank: (*r as char).to_string().parse()?,
                piece: (*p as char).to_string().parse()?,
                departure_file: None,
                departure_rank: Some((*r0 as char).to_string().parse()?),
                capture,
                promotion: None,
            })
        }

        // "Qh4e1", "Qh4xe1"
        [p @ piece!(), f0 @ file!(), r0 @ rank!(), f @ file!(), r @ rank!()]
        | [p @ piece!(), f0 @ file!(), r0 @ rank!(), capture!(), f @ file!(), r @ rank!()] => {
            Ok(AlgebraicTurn::Normal {
                destination_file: (*f as char).to_string().parse()?,
                destination_rank: (*r as char).to_string().parse()?,
                piece: (*p as char).to_string().parse()?,
                departure_file: Some((*f0 as char).to_string().parse()?),
                departure_rank: Some((*r0 as char).to_string().parse()?),
                capture,
                promotion: None,
            })
        }

        b"0-0" | b"O-O" => Ok(AlgebraicTurn::KingSideCastle),

        b"0-0-0" | b"O-O-O" => Ok(AlgebraicTurn::QueenSideCastle),

        _ => Err("Invalid algebraic notation")?,
    }
}
