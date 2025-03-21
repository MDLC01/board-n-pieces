use crate::utils::{CharExt, Finite, FromChar, Name, cartesian_product};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub fn new(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::A),
            1 => Some(Self::B),
            2 => Some(Self::C),
            3 => Some(Self::D),
            4 => Some(Self::E),
            5 => Some(Self::F),
            6 => Some(Self::G),
            7 => Some(Self::H),
            _ => None,
        }
    }

    pub fn index(self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
            Self::D => 3,
            Self::E => 4,
            Self::F => 5,
            Self::G => 6,
            Self::H => 7,
        }
    }

    pub fn mirror(self) -> Self {
        match self {
            Self::A => Self::H,
            Self::B => Self::G,
            Self::C => Self::F,
            Self::D => Self::E,
            Self::E => Self::D,
            Self::F => Self::C,
            Self::G => Self::B,
            Self::H => Self::A,
        }
    }
}

impl Finite for File {
    fn values() -> [Self; 8] {
        [
            Self::A,
            Self::B,
            Self::C,
            Self::D,
            Self::E,
            Self::F,
            Self::G,
            Self::H,
        ]
    }
}

impl Name for File {
    fn name(&self) -> String {
        match self {
            Self::A => "a".into(),
            Self::B => "b".into(),
            Self::C => "c".into(),
            Self::D => "d".into(),
            Self::E => "e".into(),
            Self::F => "f".into(),
            Self::G => "g".into(),
            Self::H => "h".into(),
        }
    }
}

impl FromChar for File {
    type Err = String;

    fn from_char(c: char) -> crate::Result<Self> {
        match c {
            'a' => Ok(Self::A),
            'b' => Ok(Self::B),
            'c' => Ok(Self::C),
            'd' => Ok(Self::D),
            'e' => Ok(Self::E),
            'f' => Ok(Self::F),
            'g' => Ok(Self::G),
            'h' => Ok(Self::H),
            c => Err(format!("invalid file: {c}"))?,
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Rank {
    pub fn new(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::One),
            1 => Some(Self::Two),
            2 => Some(Self::Three),
            3 => Some(Self::Four),
            4 => Some(Self::Five),
            5 => Some(Self::Six),
            6 => Some(Self::Seven),
            7 => Some(Self::Eight),
            _ => None,
        }
    }

    pub fn index(self) -> usize {
        match self {
            Self::One => 0,
            Self::Two => 1,
            Self::Three => 2,
            Self::Four => 3,
            Self::Five => 4,
            Self::Six => 5,
            Self::Seven => 6,
            Self::Eight => 7,
        }
    }

    pub fn mirror(self) -> Self {
        match self {
            Self::One => Self::Eight,
            Self::Two => Self::Seven,
            Self::Three => Self::Six,
            Self::Four => Self::Five,
            Self::Five => Self::Four,
            Self::Six => Self::Three,
            Self::Seven => Self::Two,
            Self::Eight => Self::One,
        }
    }
}

impl Finite for Rank {
    fn values() -> [Self; 8] {
        [
            Self::One,
            Self::Two,
            Self::Three,
            Self::Four,
            Self::Five,
            Self::Six,
            Self::Seven,
            Self::Eight,
        ]
    }
}

impl Name for Rank {
    fn name(&self) -> String {
        match self {
            Self::One => "1".into(),
            Self::Two => "2".into(),
            Self::Three => "3".into(),
            Self::Four => "4".into(),
            Self::Five => "5".into(),
            Self::Six => "6".into(),
            Self::Seven => "7".into(),
            Self::Eight => "8".into(),
        }
    }
}

impl FromChar for Rank {
    type Err = String;

    fn from_char(c: char) -> crate::Result<Self> {
        match c {
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            c => Err(format!("invalid rank: {c}"))?,
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Square {
    file: File,
    rank: Rank,
}

impl Square {
    /// Returns an iterator over all the squares of a chessboard, in no particular order.
    pub fn all() -> impl Iterator<Item = Self> {
        cartesian_product(File::values(), Rank::values()).map(|(file, rank)| Self { file, rank })
    }

    pub fn new(file: File, rank: Rank) -> Self {
        Self { file, rank }
    }

    pub fn file(self) -> File {
        self.file
    }

    pub fn rank(self) -> Rank {
        self.rank
    }

    /// Returns the coordinate of this square from the other player's point of view.
    pub fn transpose(self) -> Self {
        Self {
            file: self.file.mirror(),
            rank: self.rank.mirror(),
        }
    }
}

impl Finite for Square {
    fn values() -> impl IntoIterator<Item = Self> {
        cartesian_product(File::values(), Rank::values()).map(|(file, rank)| Self { file, rank })
    }
}

impl Name for Square {
    fn name(&self) -> String {
        format!("{}{}", self.file.name(), self.rank.name())
    }
}

impl FromStr for Square {
    type Err = String;

    fn from_str(s: &str) -> crate::Result<Self> {
        let [f, r] = s.chars().collect::<Vec<_>>()[..] else {
            Err(format!("invalid square: {s}"))?
        };
        Ok(Self::new(f.parse()?, r.parse()?))
    }
}

impl Debug for Square {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Square({})", self)
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn flip(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }

    /// Returns the rank pawns of this color have to target to capture en passant.
    pub fn en_passant_target_rank(self) -> Rank {
        match self {
            Self::White => Rank::Six,
            Self::Black => Rank::Three,
        }
    }

    /// Returns the rank in which pawns of the other color can be captured en passant.
    pub fn en_passant_capture_rank(self) -> Rank {
        match self {
            Self::White => Rank::Five,
            Self::Black => Rank::Four,
        }
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum PieceKind {
    #[default]
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Finite for PieceKind {
    fn values() -> impl IntoIterator<Item = Self> {
        [
            Self::Pawn,
            Self::Knight,
            Self::Bishop,
            Self::Rook,
            Self::Queen,
            Self::King,
        ]
    }
}

impl FromChar for PieceKind {
    type Err = String;

    fn from_char(c: char) -> crate::Result<Self> {
        match c {
            'P' => Ok(Self::Pawn),
            'N' => Ok(Self::Knight),
            'B' => Ok(Self::Bishop),
            'R' => Ok(Self::Rook),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            c => Err(format!("invalid piece kind: {}", c))?,
        }
    }
}

impl Display for PieceKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Pawn => write!(f, "P"),
            Self::Knight => write!(f, "N"),
            Self::Bishop => write!(f, "B"),
            Self::Rook => write!(f, "R"),
            Self::Queen => write!(f, "Q"),
            Self::King => write!(f, "K"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
}

impl Piece {
    pub fn new(color: Color, kind: PieceKind) -> Self {
        Self { color, kind }
    }

    /// Flips the color of this piece.
    pub fn flip(self) -> Self {
        Self {
            color: self.color.flip(),
            kind: self.kind,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SquareContent {
    Empty,
    Piece(Piece),
}

impl SquareContent {
    pub fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }

    pub fn is_occupied(self) -> bool {
        !self.is_empty()
    }

    pub fn is(self, piece: Piece) -> bool {
        match self {
            Self::Empty => false,
            Self::Piece(p) => p == piece,
        }
    }

    pub fn map(self, f: impl FnOnce(Piece) -> Piece) -> Self {
        match self {
            Self::Empty => Self::Empty,
            Self::Piece(p) => Self::Piece(f(p)),
        }
    }

    pub fn flip(self) -> Self {
        self.map(Piece::flip)
    }
}

#[derive(Debug, Clone)]
pub struct Board<const WIDTH: usize = 8, const HEIGHT: usize = 8> {
    /// The squares of the board, in file-major order.
    squares: [[SquareContent; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> Board<WIDTH, HEIGHT> {
    pub fn new(squares: [[SquareContent; WIDTH]; HEIGHT]) -> Self {
        Self { squares }
    }

    /// Mirrors the board vertically (i.e., along a horizontal axis).
    pub fn mirror(&self) -> Self {
        let mut squares = self.squares;
        squares.reverse();
        Self { squares }
    }

    /// Flips the colors of the pieces.
    pub fn flip(&self) -> Self {
        Self {
            squares: self.squares.map(|r| r.map(SquareContent::flip)),
        }
    }
}

impl Index<Square> for Board {
    type Output = SquareContent;

    fn index(&self, index: Square) -> &Self::Output {
        &self.squares[index.rank.index()][index.file.index()]
    }
}

impl IndexMut<Square> for Board {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self.squares[index.rank.index()][index.file.index()]
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CastlingAvailabilities {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl CastlingAvailabilities {
    pub const ALL: Self = Self {
        white_kingside: true,
        white_queenside: true,
        black_kingside: true,
        black_queenside: true,
    };

    pub fn flip(self) -> Self {
        Self {
            white_kingside: self.black_kingside,
            white_queenside: self.black_queenside,
            black_kingside: self.white_kingside,
            black_queenside: self.white_queenside,
        }
    }

    pub fn kingside_for(self, color: Color) -> bool {
        match color {
            Color::White => self.white_kingside,
            Color::Black => self.black_kingside,
        }
    }

    pub fn queenside_for(self, color: Color) -> bool {
        match color {
            Color::White => self.white_queenside,
            Color::Black => self.black_queenside,
        }
    }

    pub fn remove_for(self, color: Color) -> Self {
        match color {
            Color::White => Self {
                white_kingside: false,
                white_queenside: false,
                ..self
            },
            Color::Black => Self {
                black_kingside: false,
                black_queenside: false,
                ..self
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    pub board: Board,
    /// The color that will play next.
    pub active: Color,
    pub castling_availabilities: CastlingAvailabilities,
    pub en_passant_target_file: Option<File>,
    pub halfmove: u32,
    pub fullmove: u32,
}

impl Position {
    pub fn default_with_board(board: Board) -> Self {
        Self {
            board,
            active: Color::White,
            castling_availabilities: CastlingAvailabilities::ALL,
            en_passant_target_file: None,
            halfmove: 0,
            fullmove: 1,
        }
    }

    pub fn at(&self, square: Square) -> SquareContent {
        self.board[square]
    }

    pub fn next_fullmove(&self) -> u32 {
        if self.active == Color::Black {
            self.fullmove + 1
        } else {
            self.fullmove
        }
    }

    /// Inverts this position, mirroring it, and flipping the piece colors.
    pub fn invert(&self) -> Self {
        Self {
            board: self.board.mirror().flip(),
            active: self.active.flip(),
            castling_availabilities: self.castling_availabilities.flip(),
            en_passant_target_file: self.en_passant_target_file.map(File::mirror),
            halfmove: self.halfmove,
            fullmove: self.fullmove,
        }
    }
}

impl Default for Position {
    /// Returns the standard starting position.
    fn default() -> Self {
        #![allow(non_snake_case)]
        let P = SquareContent::Piece(Piece::new(Color::White, PieceKind::Pawn));
        let N = SquareContent::Piece(Piece::new(Color::White, PieceKind::Knight));
        let B = SquareContent::Piece(Piece::new(Color::White, PieceKind::Bishop));
        let R = SquareContent::Piece(Piece::new(Color::White, PieceKind::Rook));
        let Q = SquareContent::Piece(Piece::new(Color::White, PieceKind::Queen));
        let K = SquareContent::Piece(Piece::new(Color::White, PieceKind::King));
        let p = SquareContent::Piece(Piece::new(Color::Black, PieceKind::Pawn));
        let n = SquareContent::Piece(Piece::new(Color::Black, PieceKind::Knight));
        let b = SquareContent::Piece(Piece::new(Color::Black, PieceKind::Bishop));
        let r = SquareContent::Piece(Piece::new(Color::Black, PieceKind::Rook));
        let q = SquareContent::Piece(Piece::new(Color::Black, PieceKind::Queen));
        let k = SquareContent::Piece(Piece::new(Color::Black, PieceKind::King));

        Self::default_with_board(Board {
            squares: [
                [R, N, B, Q, K, B, N, R],
                [P, P, P, P, P, P, P, P],
                [SquareContent::Empty; 8],
                [SquareContent::Empty; 8],
                [SquareContent::Empty; 8],
                [SquareContent::Empty; 8],
                [p, p, p, p, p, p, p, p],
                [r, n, b, q, k, b, n, r],
            ],
        })
    }
}
