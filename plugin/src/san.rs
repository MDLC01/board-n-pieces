use crate::model::{Color, File, Piece, PieceKind, Position, Rank, Square, SquareContent};
use crate::utils::{CharExt, Finite, Name, StrExt};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::{fmt, iter};

#[derive(Debug, Default, Copy, Clone)]
enum EnPassantMetadata {
    #[default]
    /// Nothing relevant to report.
    Nothing,
    /// In case of a two-square pawn move, this is the file of the square that was skipped.
    SkipFile(File),
    /// In case of an en passant capture, this is the file of the square that was captured.
    EnPassantCaptureFile(File),
}

impl EnPassantMetadata {
    fn skip_file(self) -> Option<File> {
        match self {
            Self::SkipFile(file) => Some(file),
            _ => None,
        }
    }

    fn en_passant_capture_file(self) -> Option<File> {
        match self {
            Self::EnPassantCaptureFile(file) => Some(file),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Move<S = Square> {
    from: S,
    to: S,
    en_passant_metadata: EnPassantMetadata,
    /// `true` if, and only if, this move removes the ability to kingside castle.
    removes_kingside_castling_ability: bool,
    /// `true` if, and only if, this move removes the ability to queenside castle.
    removes_queenside_castling_ability: bool,
}

impl Move {
    pub fn apply(
        self,
        position: &Position,
        piece: PieceKind,
        capture: bool,
        promotion: Option<PieceKind>,
    ) -> Position {
        let mut new_board = position.board.clone();
        new_board[self.from] = SquareContent::Empty;
        let final_piece = Piece::new(position.active, promotion.unwrap_or(piece));
        new_board[self.to] = SquareContent::Piece(final_piece);
        if let Some(capture_file) = self.en_passant_metadata.en_passant_capture_file() {
            let capture_square =
                Square::new(capture_file, position.active.en_passant_capture_rank());
            new_board[capture_square] = SquareContent::Empty
        }

        let mut castling_availabilities = position.castling_availabilities;
        if self.removes_kingside_castling_ability {
            if position.active == Color::White {
                castling_availabilities.white_kingside = false
            } else {
                castling_availabilities.black_kingside = false
            }
        }
        if self.removes_queenside_castling_ability {
            if position.active == Color::White {
                castling_availabilities.white_queenside = false
            } else {
                castling_availabilities.black_queenside = false
            }
        }

        let halfmove = if capture || piece == PieceKind::Pawn {
            0
        } else {
            position.halfmove + 1
        };

        Position {
            board: new_board,
            active: position.active.flip(),
            castling_availabilities,
            en_passant_target_file: self.en_passant_metadata.skip_file(),
            halfmove,
            fullmove: position.next_fullmove(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct LocalSquare {
    color: Color,
    local_file: File,
    local_rank: Rank,
}

impl LocalSquare {
    fn from_absolute(color: Color, absolute_square: Square) -> Self {
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

    pub fn to_absolute(self) -> Square {
        let local_square = Square::new(self.local_file, self.local_rank);
        match self.color {
            Color::White => local_square,
            Color::Black => local_square.transpose(),
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
        value.to_absolute()
    }
}

type LocalMove = Move<LocalSquare>;

impl LocalMove {
    fn new(from: LocalSquare, to: LocalSquare) -> Self {
        Self {
            from,
            to,
            en_passant_metadata: EnPassantMetadata::default(),
            removes_kingside_castling_ability: false,
            removes_queenside_castling_ability: false,
        }
    }

    fn with_skipped_square(self, square: LocalSquare) -> Self {
        Self {
            en_passant_metadata: EnPassantMetadata::SkipFile(square.to_absolute().file()),
            ..self
        }
    }

    fn with_en_passant_capture(self, square: LocalSquare) -> Self {
        Self {
            en_passant_metadata: EnPassantMetadata::EnPassantCaptureFile(
                square.to_absolute().file(),
            ),
            ..self
        }
    }

    fn with_removed_kingside_castling_availability(self) -> Self {
        Self {
            removes_kingside_castling_ability: true,
            ..self
        }
    }

    fn with_removed_queenside_castling_availability(self) -> Self {
        Self {
            removes_queenside_castling_ability: true,
            ..self
        }
    }

    fn with_removed_castling_availabilities(self) -> Self {
        Self {
            removes_kingside_castling_ability: true,
            removes_queenside_castling_ability: true,
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
            removes_kingside_castling_ability: value.removes_kingside_castling_ability,
            removes_queenside_castling_ability: value.removes_queenside_castling_ability,
        }
    }
}

fn generate_affine_moves<'a>(
    position: &'a Position,
    departure: LocalSquare,
    mut next: impl FnMut(LocalSquare) -> Option<LocalSquare> + 'a,
) -> impl Iterator<Item = LocalMove> + 'a {
    let mut state = Some(departure);
    iter::from_fn(move || {
        state.and_then(|square| match next(square) {
            None => {
                state = None;
                None
            }
            Some(destination) => {
                state = if position.at(destination.into()).is_occupied() {
                    None
                } else {
                    Some(destination)
                };
                Some(LocalMove::new(departure, destination))
            }
        })
    })
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

/// Returns all valid moves pieces of a specific kind can make in a specific position. This does not
/// include castling moves, and does not exclude moves that put the king in a check position.
fn valid_moves(position: &Position, piece_kind: PieceKind) -> Vec<Move> {
    let mut moves = Vec::new();
    for global_square in Square::all() {
        let departure = LocalSquare::from_absolute(position.active, global_square);
        if !position
            .at(departure.into())
            .is(Piece::new(position.active, piece_kind))
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

                    if destination.local_rank == Rank::Six
                        && position.en_passant_target_file == Some(destination.to_absolute().file())
                    {
                        let square = destination.backward().unwrap();
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

            #[rustfmt::skip]
            PieceKind::Bishop => moves.extend(
                iter::empty()
                    .chain(generate_affine_moves(position, departure, LocalSquare::forward_left))
                    .chain(generate_affine_moves(position, departure, LocalSquare::forward_right))
                    .chain(generate_affine_moves(position, departure, LocalSquare::backward_left))
                    .chain(generate_affine_moves(position, departure, LocalSquare::backward_right)),
            ),

            #[rustfmt::skip]
            PieceKind::Rook => moves.extend(
                iter::empty()
                    .chain(generate_affine_moves(position, departure, LocalSquare::forward))
                    .chain(generate_affine_moves(position, departure, LocalSquare::backward))
                    .chain(generate_affine_moves(position, departure, LocalSquare::left))
                    .chain(generate_affine_moves(position, departure, LocalSquare::right))
                    .map(|m| {
                        let departure_file = Square::from(m.from).file();
                        if departure_file == File::H {
                            m.with_removed_kingside_castling_availability()
                        } else if departure_file == File::A {
                            m.with_removed_queenside_castling_availability()
                        } else {
                            m
                        }
                    }),
            ),

            #[rustfmt::skip]
            PieceKind::Queen => moves.extend(
                iter::empty()
                    .chain(generate_affine_moves(position, departure, LocalSquare::forward))
                    .chain(generate_affine_moves(position, departure, LocalSquare::backward))
                    .chain(generate_affine_moves(position, departure, LocalSquare::left))
                    .chain(generate_affine_moves(position, departure, LocalSquare::right))
                    .chain(generate_affine_moves(position, departure, LocalSquare::forward_left))
                    .chain(generate_affine_moves(position, departure, LocalSquare::forward_right))
                    .chain(generate_affine_moves(position, departure, LocalSquare::backward_left))
                    .chain(generate_affine_moves(position, departure, LocalSquare::backward_right)),
            ),

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
                    .map(|destination| {
                        LocalMove::new(departure, destination)
                            .with_removed_castling_availabilities()
                    }),
            ),
        }
    }

    moves.into_iter().map(|m| m.into()).collect()
}

#[derive(Debug, Copy, Clone)]
pub enum Side {
    King,
    Queen,
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
    Castle(Side),
}

impl AlgebraicTurn {
    fn to_indexed_string(self, index: usize) -> String {
        format!(
            "{}{} {}",
            index / 2 + 1,
            if index % 2 == 0 { "." } else { "..." },
            self
        )
    }

    pub fn apply(self, turn_index: usize, initial_position: &Position) -> crate::Result<Position> {
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
                let mut possible_new_positions = valid_moves(initial_position, piece)
                    .into_iter()
                    // Filter moves that do not match the algebraic notation.
                    .filter(|m| {
                        m.to.file() == destination_file
                            && m.to.rank() == destination_rank
                            && departure_file.is_none_or(|file| m.from.file() == file)
                            && departure_rank.is_none_or(|rank| m.from.rank() == rank)
                    })
                    // Get corresponding positions.
                    .map(|m| m.apply(initial_position, piece, capture, promotion))
                    // Filter moves that put the king in a check position.
                    .filter(|new_position| {
                        let king = Piece::new(initial_position.active, PieceKind::King);
                        PieceKind::iter()
                            .flat_map(|piece| valid_moves(new_position, piece))
                            .all(|m| !new_position.at(m.to).is(king))
                    });

                let Some(new_position) = possible_new_positions.next() else {
                    Err(format!(
                        "illegal move: {}",
                        self.to_indexed_string(turn_index)
                    ))?
                };
                if possible_new_positions.next().is_some() {
                    Err(format!(
                        "ambiguous move: {}",
                        self.to_indexed_string(turn_index)
                    ))?
                }

                Ok(new_position)
            }

            Self::Castle(side) => {
                let rank = if initial_position.active == Color::White {
                    Rank::One
                } else {
                    Rank::Eight
                };
                let king = Piece::new(initial_position.active, PieceKind::King);
                let rook = Piece::new(initial_position.active, PieceKind::Rook);

                let requirements = match side {
                    Side::King => {
                        initial_position
                            .castling_availabilities
                            .kingside_for(initial_position.active)
                            && initial_position.at(Square::new(File::E, rank)).is(king)
                            && initial_position.at(Square::new(File::F, rank)).is_empty()
                            && initial_position.at(Square::new(File::G, rank)).is_empty()
                            && initial_position.at(Square::new(File::H, rank)).is(rook)
                    }
                    Side::Queen => {
                        initial_position
                            .castling_availabilities
                            .queenside_for(initial_position.active)
                            && initial_position.at(Square::new(File::A, rank)).is(rook)
                            && initial_position.at(Square::new(File::B, rank)).is_empty()
                            && initial_position.at(Square::new(File::C, rank)).is_empty()
                            && initial_position.at(Square::new(File::D, rank)).is_empty()
                            && initial_position.at(Square::new(File::E, rank)).is(king)
                    }
                };
                if !requirements {
                    Err(format!(
                        "illegal move: {}",
                        self.to_indexed_string(turn_index)
                    ))?
                }

                let mut new_board = initial_position.board.clone();
                match side {
                    Side::King => {
                        new_board[Square::new(File::E, rank)] = SquareContent::Empty;
                        new_board[Square::new(File::F, rank)] = SquareContent::Piece(rook);
                        new_board[Square::new(File::G, rank)] = SquareContent::Piece(king);
                        new_board[Square::new(File::H, rank)] = SquareContent::Empty;
                    }
                    Side::Queen => {
                        new_board[Square::new(File::A, rank)] = SquareContent::Empty;
                        new_board[Square::new(File::C, rank)] = SquareContent::Piece(king);
                        new_board[Square::new(File::D, rank)] = SquareContent::Piece(rook);
                        new_board[Square::new(File::E, rank)] = SquareContent::Empty;
                    }
                }

                Ok(Position {
                    board: new_board,
                    active: initial_position.active.flip(),
                    castling_availabilities: initial_position
                        .castling_availabilities
                        .remove_for(initial_position.active),
                    en_passant_target_file: None,
                    halfmove: initial_position.halfmove + 1,
                    fullmove: initial_position.next_fullmove(),
                })
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
            Self::Castle(Side::King) => write!(f, "0-0"),
            Self::Castle(Side::Queen) => write!(f, "0-0-0"),
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
            return Ok(Self::Castle(Side::King));
        }

        if source == "0-0-0" || source == "O-O-O" {
            return Ok(Self::Castle(Side::Queen));
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
pub enum Mark {
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
pub enum Annotation {
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

#[derive(Debug, Copy, Clone)]
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

impl AnnotatedAlgebraicTurn {
    pub fn apply(self, turn_index: usize, position: &Position) -> crate::Result<Position> {
        self.turn.apply(turn_index, position)
    }
}

pub fn parse_turn(s: &str) -> crate::Result<AnnotatedAlgebraicTurn> {
    s.parse()
}
