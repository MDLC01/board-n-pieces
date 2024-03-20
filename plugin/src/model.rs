use crate::abi;
use crate::abi::{Abi, ByteAbi};
use std::borrow::Cow;
use std::iter;
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone)]
pub struct File(u8);

impl ByteAbi for File {
    fn descriptor() -> Cow<'static, str> {
        "file".into()
    }

    fn from_byte(byte: u8) -> abi::Result<Self> {
        if !byte.is_ascii_lowercase() {
            Err("Invalid file")?
        }
        Ok(Self(byte))
    }

    fn to_byte(self) -> u8 {
        self.0
    }
}

impl File {
    fn index(self) -> usize {
        (self.0 - b'a') as usize
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Rank(u8);

impl ByteAbi for Rank {
    fn descriptor() -> Cow<'static, str> {
        "rank".into()
    }

    fn from_byte(byte: u8) -> abi::Result<Self> {
        if !(b'1'..=b'9').contains(&byte) {
            Err("Invalid rank")?
        }
        Ok(Self(byte))
    }

    fn to_byte(self) -> u8 {
        self.0
    }
}

impl Rank {
    fn index(self) -> usize {
        (self.0 - b'1') as usize
    }
}

#[derive(Debug, Copy, Clone, Abi)]
#[abi(name = "square")]
pub struct Square {
    file: File,
    rank: Rank,
}

#[derive(Debug, Copy, Clone)]
pub enum Color {
    White,
    Black,
}

impl ByteAbi for Color {
    fn descriptor() -> Cow<'static, str> {
        "color".into()
    }

    fn from_byte(byte: u8) -> abi::Result<Self> {
        match byte {
            b'w' => Ok(Self::White),
            b'b' => Ok(Self::Black),
            _ => Err(format!("Invalid color: {}", char::from(byte))),
        }
    }

    fn to_byte(self) -> u8 {
        match self {
            Self::White => b'w',
            Self::Black => b'b',
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub enum PieceKind {
    #[default]
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl ByteAbi for PieceKind {
    fn descriptor() -> Cow<'static, str> {
        "piece kind".into()
    }

    fn from_byte(byte: u8) -> abi::Result<Self> {
        match byte {
            b'P' => Ok(Self::Pawn),
            b'N' => Ok(Self::Knight),
            b'B' => Ok(Self::Bishop),
            b'R' => Ok(Self::Rook),
            b'Q' => Ok(Self::Queen),
            b'K' => Ok(Self::King),
            _ => Err("Invalid piece kind".into()),
        }
    }

    fn to_byte(self) -> u8 {
        match self {
            Self::Pawn => b'P',
            Self::Knight => b'N',
            Self::Bishop => b'B',
            Self::Rook => b'R',
            Self::Queen => b'Q',
            Self::King => b'K',
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    kind: PieceKind,
    color: Color,
}

impl Piece {
    pub fn new(kind: PieceKind, color: Color) -> Self {
        Self { kind, color }
    }
}

impl ByteAbi for Piece {
    fn descriptor() -> Cow<'static, str> {
        "piece".into()
    }

    fn from_byte(byte: u8) -> abi::Result<Self> {
        if byte.is_ascii_uppercase() {
            Ok(Self {
                kind: PieceKind::from_byte(byte)?,
                color: Color::White,
            })
        } else {
            Ok(Self {
                kind: PieceKind::from_byte(byte.to_ascii_uppercase())?,
                color: Color::Black,
            })
        }
    }

    fn to_byte(self) -> u8 {
        match self.color {
            Color::White => self.kind.to_byte(),
            Color::Black => self.kind.to_byte().to_ascii_lowercase(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SquareContent {
    Empty,
    Piece(Piece),
}

impl ByteAbi for SquareContent {
    fn descriptor() -> Cow<'static, str> {
        "square content".into()
    }

    fn from_byte(byte: u8) -> abi::Result<Self> {
        match Piece::from_byte(byte) {
            Ok(piece) => Ok(Self::Piece(piece)),
            Err(_) => Ok(Self::Empty),
        }
    }

    fn to_byte(self) -> u8 {
        match self {
            SquareContent::Empty => 0,
            SquareContent::Piece(piece) => piece.to_byte(),
        }
    }
}

#[derive(Debug)]
pub struct Board {
    /// The squares of the board, stored in rank-major order.
    squares: Vec<Vec<SquareContent>>,
}

impl Board {
    pub fn new(squares: Vec<Vec<SquareContent>>) -> Self {
        Self { squares }
    }
    pub fn width(&self) -> u32 {
        self.squares[0].len() as u32
    }

    pub fn height(&self) -> u32 {
        self.squares.len() as u32
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

/// The first byte indicates the amount of files. Remaining bytes contain the content of each square.
impl Abi for Board {
    fn descriptor() -> Cow<'static, str> {
        "board".into()
    }

    fn from_bytes(bytes: &mut impl Iterator<Item = u8>) -> abi::Result<Self> {
        let width = u32::from_bytes(bytes)?;
        if width == 0 {
            Err("Board width should be non-null")?
        }
        let height = u32::from_bytes(bytes)?;
        Ok(Self {
            squares: bytes
                .map(SquareContent::from_byte)
                .take((width * height) as usize)
                .collect::<abi::Result<Vec<_>>>()?
                .chunks_exact(width as usize)
                .map(<[_]>::to_vec)
                .collect::<Vec<_>>(),
        })
    }

    fn to_bytes(self) -> Vec<u8> {
        iter::empty()
            .chain(self.width().to_bytes())
            .chain(self.height().to_bytes())
            .chain(
                self.squares
                    .iter()
                    .flat_map(|rank| rank.iter().map(|square| square.to_byte())),
            )
            .collect()
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
    const WHITE_KING_MASK: u8 = 0b0001;
    const WHITE_QUEEN_MASK: u8 = 0b0010;
    const BLACK_KING_MASK: u8 = 0b0100;
    const BLACK_QUEEN_MASK: u8 = 0b1000;

    pub const ALL: Self = Self {
        white_king_side: true,
        white_queen_side: true,
        black_king_side: true,
        black_queen_side: true,
    };
}

impl ByteAbi for CastlingAvailabilities {
    fn descriptor() -> Cow<'static, str> {
        "castling availabilities".into()
    }

    fn from_byte(byte: u8) -> abi::Result<Self> {
        Ok(Self {
            white_king_side: byte & Self::WHITE_KING_MASK != 0,
            white_queen_side: byte & Self::WHITE_QUEEN_MASK != 0,
            black_king_side: byte & Self::BLACK_KING_MASK != 0,
            black_queen_side: byte & Self::BLACK_QUEEN_MASK != 0,
        })
    }

    fn to_byte(self) -> u8 {
        let mut byte = 0b0000;
        if self.white_king_side {
            byte |= Self::WHITE_KING_MASK
        }
        if self.white_queen_side {
            byte |= Self::WHITE_QUEEN_MASK
        }
        if self.black_king_side {
            byte |= Self::BLACK_KING_MASK
        }
        if self.black_queen_side {
            byte |= Self::BLACK_QUEEN_MASK
        }
        byte
    }
}

#[derive(Debug, Abi)]
#[abi(name = "position")]
pub struct Position {
    pub board: Board,
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
}

impl Default for Position {
    fn default() -> Self {
        #![allow(non_snake_case)]
        let P = SquareContent::Piece(Piece::new(PieceKind::Pawn, Color::White));
        let N = SquareContent::Piece(Piece::new(PieceKind::Knight, Color::White));
        let B = SquareContent::Piece(Piece::new(PieceKind::Bishop, Color::White));
        let R = SquareContent::Piece(Piece::new(PieceKind::Rook, Color::White));
        let Q = SquareContent::Piece(Piece::new(PieceKind::Queen, Color::White));
        let K = SquareContent::Piece(Piece::new(PieceKind::King, Color::White));
        let p = SquareContent::Piece(Piece::new(PieceKind::Pawn, Color::Black));
        let n = SquareContent::Piece(Piece::new(PieceKind::Knight, Color::Black));
        let b = SquareContent::Piece(Piece::new(PieceKind::Bishop, Color::Black));
        let r = SquareContent::Piece(Piece::new(PieceKind::Rook, Color::Black));
        let q = SquareContent::Piece(Piece::new(PieceKind::Queen, Color::Black));
        let k = SquareContent::Piece(Piece::new(PieceKind::King, Color::Black));

        Self::default_with_board(Board {
            squares: vec![
                vec![R, N, B, Q, K, B, N, R],
                vec![P, P, P, P, P, P, P, P],
                vec![SquareContent::Empty; 8],
                vec![SquareContent::Empty; 8],
                vec![SquareContent::Empty; 8],
                vec![SquareContent::Empty; 8],
                vec![p, p, p, p, p, p, p, p],
                vec![r, n, b, q, k, b, n, r],
            ],
        })
    }
}
