use crate::model::{Color, File, Piece, PieceKind, Position, Rank, Square, SquareContent};
use crate::utils::{CharExt, Finite, Name, OptionExt, StrExt};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::{fmt, iter};

#[derive(Debug, Default, Copy, Clone)]
enum EnPassantMetadata {
    #[default]
    /// Nothing relevant to report.
    Nothing,
    /// In case of a two-square pawn move, this is the square that was skipped.
    SkippedSquare(Square),
    /// In case of an en passant capture, this is the square that was captures.
    EnPassantCaptureSquare(Square),
}

impl EnPassantMetadata {
    fn skipped_square(self) -> Option<Square> {
        match self {
            Self::SkippedSquare(square) => Some(square),
            _ => None,
        }
    }

    fn en_passant_capture_square(self) -> Option<Square> {
        match self {
            Self::EnPassantCaptureSquare(square) => Some(square),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Move {
    from: Square,
    to: Square,
    en_passant_metadata: EnPassantMetadata,
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
    en_passant_metadata: EnPassantMetadata,
}

impl LocalMove {
    fn new(from: LocalSquare, to: LocalSquare) -> Self {
        Self {
            from,
            to,
            en_passant_metadata: EnPassantMetadata::default(),
        }
    }

    fn with_skipped_square(self, square: LocalSquare) -> Self {
        Self {
            en_passant_metadata: EnPassantMetadata::SkippedSquare(square.into()),
            ..self
        }
    }

    fn with_en_passant_capture(self, square: LocalSquare) -> Self {
        Self {
            en_passant_metadata: EnPassantMetadata::EnPassantCaptureSquare(square.into()),
            ..self
        }
    }
}

impl From<LocalMove> for Move {
    fn from(value: LocalMove) -> Self {
        Self {
            from: value.from.into(),
            to: value.to.into(),
            en_passant_metadata: value.en_passant_metadata,
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
        if position.at(destination.into()).is_occupied() {
            break;
        }
    }
}

macro_rules! generate_composite_move {
    ($moves:ident, $departure:ident, [$( $step:ident ), *]) => {
        if let Some(destination) = Some($departure)
            $( .and_then(LocalSquare::$step) )*
        {
            $moves.push(LocalMove::new($departure, destination))
        }
    }
}

/// Returns all valid moves pieces of a specific kind can make in a specific position, not including
/// castling moves.
fn valid_moves(position: &Position, piece_kind: PieceKind) -> crate::Result<Vec<Move>> {
    // FIXME: We should not include moves that put the king in a check position.

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

                // Initial two-square pawn move.
                if let Some(skipped) = departure.forward() {
                    if let Some(destination) = skipped.forward() {
                        if departure.local_rank == Rank::Two
                            && position.at(skipped.into()).is_empty()
                            && position.at(destination.into()).is_empty()
                        {
                            moves.push(
                                LocalMove::new(departure, destination).with_skipped_square(skipped),
                            )
                        }
                    }
                }

                // Capture with pawn.
                for destination in departure
                    .forward_left()
                    .into_iter()
                    .chain(departure.forward_right())
                {
                    if position.at(destination.into()).is_occupied() {
                        moves.push(LocalMove::new(departure, destination))
                    }

                    if position.en_passant_target_square == Some(destination.into()) {
                        let square = destination.backward().ok_or(format!(
                            "internal error: unexpected en passant target square in {}",
                            Square::from(destination),
                        ))?;
                        moves.push(
                            LocalMove::new(departure, destination).with_en_passant_capture(square),
                        )
                    }
                }
            }

            PieceKind::Knight => {
                generate_composite_move!(moves, departure, [forward, forward, left]);
                generate_composite_move!(moves, departure, [forward, forward, right]);
                generate_composite_move!(moves, departure, [backward, backward, left]);
                generate_composite_move!(moves, departure, [backward, backward, right]);
                generate_composite_move!(moves, departure, [left, left, forward]);
                generate_composite_move!(moves, departure, [right, right, forward]);
                generate_composite_move!(moves, departure, [left, left, backward]);
                generate_composite_move!(moves, departure, [right, right, backward]);
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
        let turn_string = format!(
            "{}{} {}",
            turn_index / 2 + 1,
            if turn_index % 2 == 0 { "." } else { "..." },
            self
        );
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
                // FIXME: When a turn is disambiguated using the rank of departure, this means it cannot be
                //  disambiguated using the departure file, which is an additional information that we don't
                //  currently take into account.
                let mut possible_moves = valid_moves.clone().into_iter().filter(|m| {
                    m.to.file() == destination_file
                        && m.to.rank() == destination_rank
                        && departure_file.is_none_or(|file| m.from.file() == file)
                        && departure_rank.is_none_or(|rank| m.from.rank() == rank)
                });
                let Some(turn) = possible_moves.next() else {
                    Err(format!("illegal move: {turn_string}"))?
                };
                if possible_moves.next().is_some() {
                    Err(format!("ambiguous move: {turn_string}"))?
                }

                // TODO: Update flags such as castling availabilities.
                let mut new_board = position.board.clone();
                new_board[turn.from] = SquareContent::Empty;
                new_board[turn.to] =
                    SquareContent::Piece(Piece::new(position.active, promotion.unwrap_or(piece)));
                if let Some(capture) = turn.en_passant_metadata.en_passant_capture_square() {
                    new_board[capture] = SquareContent::Empty
                }

                Ok(Position {
                    board: new_board,

                    active: position.active.other(),

                    // TODO: Update that properly.
                    castling_availabilities: position.castling_availabilities,

                    en_passant_target_square: turn.en_passant_metadata.skipped_square(),

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
                    Err("not allowed to castle king-side")?
                }
                // TODO
                Err("castling moves not implemented yet")?
            }

            Self::QueenSideCastle => {
                if !position
                    .castling_availabilities
                    .queen_side_for(position.active)
                {
                    Err("not able to castle king-side")?
                }
                // TODO
                Err("castling moves not implemented yet")?
            }
        }
    }
}

impl Display for AlgebraicTurn {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
                let piece_text = match piece {
                    PieceKind::Pawn => "".into(),
                    piece => piece.to_string(),
                };
                let capture_text = match capture {
                    true => "x",
                    false => "",
                };
                let departure_file_text = match departure_file {
                    None => "".into(),
                    Some(file) => file.name(),
                };
                let departure_rank_text = match departure_rank {
                    None => "".into(),
                    Some(rank) => rank.name(),
                };
                let promote_text = match promotion {
                    None => "".into(),
                    Some(piece) => format!("={piece}"),
                };
                write!(f, "{piece_text}{departure_file_text}{departure_rank_text}{capture_text}{destination_file}{destination_rank}{promote_text}")
            }
            Self::KingSideCastle => write!(f, "0-0"),
            Self::QueenSideCastle => write!(f, "0-0-0"),
        }
    }
}

fn parse_finite<T: Finite + Name>(source: &str) -> (&str, Option<T>) {
    for x in T::values() {
        if let Some(prefix) = source.strip_suffix(&x.name()) {
            return (prefix, Some(x));
        }
    }
    (source, None)
}

fn parse_promotion(source: &str) -> crate::Result<(&str, Option<PieceKind>)> {
    // Parenthesized promotion (e.g., "e8(Q)").
    if let Some(prefix) = source.strip_suffix(')') {
        let Some((prefix, c)) = prefix.split_last_char() else {
            Err(format!("invalid SAN: {:?}", source))?
        };
        let Some(prefix) = prefix.strip_suffix('(') else {
            Err(format!("invalid SAN: {:?}", source))?
        };
        let promotion = c.parse()?;
        return Ok((prefix, Some(promotion)));
    }

    // Promotion with or without indicating symbol (e.g., "e8=Q", "e8/Q", "e8Q").
    if let Some((prefix, c)) = source.split_last_char() {
        if let Ok(promotion) = c.parse() {
            return if let Some(prefix) = prefix.strip_suffix('=') {
                Ok((prefix, Some(promotion)))
            } else if let Some(prefix) = prefix.strip_suffix('/') {
                Ok((prefix, Some(promotion)))
            } else {
                Ok((prefix, Some(promotion)))
            };
        }
    }

    // No promotion.
    Ok((source, None))
}

fn parse_capture(source: &str) -> (&str, bool) {
    match source
        .strip_suffix('x')
        .or_else(|| source.strip_suffix(':'))
        .or_else(|| source.strip_suffix('×'))
    {
        None => (source, false),
        Some(prefix) => (prefix, true),
    }
}

fn parse_piece(source: &str) -> (&str, PieceKind) {
    match source.split_last_char() {
        Some((prefix, 'N')) => (prefix, PieceKind::Knight),
        Some((prefix, 'B')) => (prefix, PieceKind::Bishop),
        Some((prefix, 'R')) => (prefix, PieceKind::Rook),
        Some((prefix, 'Q')) => (prefix, PieceKind::Queen),
        Some((prefix, 'K')) => (prefix, PieceKind::King),
        _ => (source, PieceKind::Pawn),
    }
}

impl FromStr for AlgebraicTurn {
    type Err = String;

    fn from_str(source: &str) -> crate::Result<Self> {
        if source == "0-0" || source == "O-O" {
            return Ok(Self::KingSideCastle);
        }

        if source == "0-0-0" || source == "O-O-O" {
            return Ok(Self::QueenSideCastle);
        }

        // TODO: Support pawn moves containing only file information (minimal algebraic notation).

        // TODO: Support "e.p." suffix.

        // TODO: Support capture indicators at the end.

        let (s, promotion) = parse_promotion(source)?;

        let (s, optional_destination_rank) = parse_finite(s);
        let destination_rank = optional_destination_rank.ok_or(format!("invalid SAN: {source}"))?;

        let (s, optional_destination_file) = parse_finite(s);
        let destination_file = optional_destination_file.ok_or(format!("invalid SAN: {source}"))?;

        let (s, capture) = parse_capture(s);

        let (s, departure_rank) = parse_finite(s);

        let (s, departure_file) = parse_finite(s);

        let (s, piece) = parse_piece(s);

        if !s.is_empty() {
            Err(format!("invalid SAN: {source}"))?
        }

        Ok(AlgebraicTurn::Normal {
            destination_file,
            destination_rank,
            piece,
            departure_file,
            departure_rank,
            capture,
            promotion,
        })
    }
}

#[derive(Debug, Copy, Clone)]
enum Mark {
    Check,
    Checkmate,
}

impl Finite for Mark {
    fn values() -> [Self; 2] {
        [Self::Check, Self::Checkmate]
    }
}

impl Name for Mark {
    fn name(&self) -> String {
        match self {
            Self::Check => "+".into(),
            Self::Checkmate => "#".into(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Annotation {
    Blunder,
    Mistake,
    Dubious,
    Interesting,
    Good,
    Brilliant,
}

impl Finite for Annotation {
    fn values() -> [Self; 6] {
        [
            Self::Blunder,
            Self::Mistake,
            Self::Dubious,
            Self::Interesting,
            Self::Good,
            Self::Brilliant,
        ]
    }
}

impl Name for Annotation {
    fn name(&self) -> String {
        match self {
            Self::Blunder => "??".into(),
            Self::Mistake => "?".into(),
            Self::Dubious => "?!".into(),
            Self::Interesting => "!?".into(),
            Self::Good => "!".into(),
            Self::Brilliant => "!!".into(),
        }
    }
}

#[derive(Debug)]
pub struct AnnotatedAlgebraicTurn {
    turn: AlgebraicTurn,
    #[allow(unused)]
    mark: Option<Mark>,
    #[allow(unused)]
    annotation: Option<Annotation>,
}

impl FromStr for AnnotatedAlgebraicTurn {
    type Err = String;

    fn from_str(s: &str) -> crate::Result<Self> {
        let (s, annotation) = parse_finite(s);
        let (s, mark) = parse_finite(s);
        let turn = s.parse()?;

        Ok(Self {
            turn,
            mark,
            annotation,
        })
    }
}

pub fn parse_turn(s: &str) -> crate::Result<AlgebraicTurn> {
    Ok(s.parse::<AnnotatedAlgebraicTurn>()?.turn)
}
