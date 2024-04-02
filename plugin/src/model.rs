use crate::utils::cartesian_product;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct File(u8);

impl File {
    pub fn new(index: usize) -> Option<Self> {
        if (0..8).contains(&index) {
            Some(Self(index as u8))
        } else {
            None
        }
    }

    pub fn index(self) -> usize {
        self.0 as usize
    }
}

impl FromStr for File {
    type Err = String;

    fn from_str(s: &str) -> crate::Result<Self> {
        match s {
            "a" => Ok(Self(0)),
            "b" => Ok(Self(1)),
            "c" => Ok(Self(2)),
            "d" => Ok(Self(3)),
            "e" => Ok(Self(4)),
            "f" => Ok(Self(5)),
            "g" => Ok(Self(6)),
            "h" => Ok(Self(7)),
            _ => Err("Invalid file")?,
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", (self.0 + b'a') as char)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rank(u8);

impl Rank {
    pub fn new(index: usize) -> Option<Self> {
        if (0..8).contains(&index) {
            Some(Self(index as u8))
        } else {
            None
        }
    }

    pub fn index(self) -> usize {
        self.0 as usize
    }
}

impl FromStr for Rank {
    type Err = String;

    fn from_str(s: &str) -> crate::Result<Self> {
        match s {
            "1" => Ok(Self(0)),
            "2" => Ok(Self(1)),
            "3" => Ok(Self(2)),
            "4" => Ok(Self(3)),
            "5" => Ok(Self(4)),
            "6" => Ok(Self(5)),
            "7" => Ok(Self(6)),
            "8" => Ok(Self(7)),
            _ => Err("Invalid file")?,
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", (self.0 + b'1') as char)
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
        cartesian_product(0..8, 0..8).map(|(file, rank)| Self {
            file: File(file),
            rank: Rank(rank),
        })
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
            file: File(7 - self.file.0),
            rank: Rank(7 - self.rank.0),
        }
    }

    pub fn name(self) -> String {
        format!("{}{}", self.file, self.rank)
    }
}

impl FromStr for Square {
    type Err = String;

    fn from_str(s: &str) -> crate::Result<Self> {
        let [f, r] = s.chars().collect::<Vec<_>>()[..] else {
            Err("Invalid square")?
        };
        Ok(Self::new(f.to_string().parse()?, r.to_string().parse()?))
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
    pub fn other(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
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

impl FromStr for PieceKind {
    type Err = String;

    fn from_str(s: &str) -> crate::Result<Self> {
        match s {
            "P" => Ok(Self::Pawn),
            "N" => Ok(Self::Knight),
            "B" => Ok(Self::Bishop),
            "R" => Ok(Self::Rook),
            "Q" => Ok(Self::Queen),
            "K" => Ok(Self::King),
            _ => Err("Invalid piece kind")?,
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

    pub fn is(self, color: Color, kind: PieceKind) -> bool {
        match self {
            Self::Empty => false,
            Self::Piece(piece) => piece.color == color && piece.kind == kind,
        }
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
    pub white_king_side: bool,
    pub white_queen_side: bool,
    pub black_king_side: bool,
    pub black_queen_side: bool,
}

impl CastlingAvailabilities {
    pub const ALL: Self = Self {
        white_king_side: true,
        white_queen_side: true,
        black_king_side: true,
        black_queen_side: true,
    };

    pub fn king_side_for(self, color: Color) -> bool {
        match color {
            Color::White => self.white_king_side,
            Color::Black => self.black_king_side,
        }
    }

    pub fn queen_side_for(self, color: Color) -> bool {
        match color {
            Color::White => self.white_queen_side,
            Color::Black => self.black_queen_side,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    pub board: Board,
    /// The color that will play next.
    pub active: Color,
    pub castling_availabilities: CastlingAvailabilities,
    pub en_passant_target_square: Option<Square>,
    pub halfmove: u32,
    pub fullmove: u32,
}

impl Position {
    pub fn default_with_board(board: Board) -> Self {
        Self {
            board,
            active: Color::White,
            castling_availabilities: CastlingAvailabilities::ALL,
            en_passant_target_square: None,
            halfmove: 0,
            fullmove: 1,
        }
    }

    pub fn at(&self, square: Square) -> SquareContent {
        self.board[square]
    }
}

impl Default for Position {
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
