pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use pawn::Promotion;

use super::board::Board;
use super::utils::ChessError;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Copy)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum Color {
    #[default]
    White,
    Black,
}

impl Color {
    pub fn fen_char(&self) -> char {
        match self {
            Color::White => 'w',
            Color::Black => 'b',
        }
    }

    pub fn from_fen(str: &str) -> Self {
        match str {
            "w" => Color::White,
            "b" => Color::Black,
            _ => Color::White,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Copy)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum Piece {
    Pawn(Color),
    Knight(Color),
    Rook(Color),
    Bishop(Color),
    King(Color),
    Queen(Color),
}

impl Piece {
    pub fn fen_char(&self) -> char {
        match self {
            Piece::Pawn(Color::White) => 'P',
            Piece::Rook(Color::White) => 'R',
            Piece::Knight(Color::White) => 'N',
            Piece::Bishop(Color::White) => 'B',
            Piece::King(Color::White) => 'K',
            Piece::Queen(Color::White) => 'Q',

            Piece::Pawn(Color::Black) => 'p',
            Piece::Rook(Color::Black) => 'r',
            Piece::Knight(Color::Black) => 'n',
            Piece::Bishop(Color::Black) => 'b',
            Piece::King(Color::Black) => 'k',
            Piece::Queen(Color::Black) => 'q',
        }
    }

    pub fn from_fen(fen: char) -> Self {
        match fen {
            'P' => Piece::Pawn(Color::White),
            'R' => Piece::Rook(Color::White),
            'N' => Piece::Knight(Color::White),
            'B' => Piece::Bishop(Color::White),
            'Q' => Piece::Queen(Color::White),
            'K' => Piece::King(Color::White),

            'p' => Piece::Pawn(Color::Black),
            'r' => Piece::Rook(Color::Black),
            'n' => Piece::Knight(Color::Black),
            'b' => Piece::Bishop(Color::Black),
            'q' => Piece::Queen(Color::Black),
            'k' => Piece::King(Color::Black),

            _ => Piece::Pawn(Color::White),
        }
    }

    pub fn color(&self) -> Color {
        match *self {
            Piece::Pawn(c) => c,
            Piece::Knight(c) => c,
            Piece::Bishop(c) => c,
            Piece::Rook(c) => c,
            Piece::Queen(c) => c,
            Piece::King(c) => c,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct Position {
    pub rank: usize,
    pub file: usize,
}

impl Position {
    pub fn shifted(&self, dx: i8, dy: i8) -> Option<Self> {
        let new_file = self.file as i8 + dx;
        let new_rank = self.rank as i8 + dy;

        if (0..=7).contains(&new_file) && (0..=7).contains(&new_rank) {
            Some(Position {
                file: new_file as usize,
                rank: new_rank as usize,
            })
        } else {
            None
        }
    }

    pub fn to_bitboard(self) -> u64 {
        1u64 << self.to_index()
    }

    pub fn from_index(index: usize) -> Self {
        Position {
            file: index % 8,
            rank: index / 8,
        }
    }

    pub fn to_index(self) -> usize {
        self.rank * 8 + self.file
    }

    pub fn from_fen(slice: &str) -> Self {
        let file_char = slice.chars().nth(0).unwrap();
        let rank_char = slice.chars().nth(1).unwrap();

        let file = file_char as usize - 'a' as usize;
        let rank = rank_char.to_digit(10).unwrap() as usize - 1;

        Position { rank, file }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct Castle {
    pub white_king: bool,
    pub white_queen: bool,
    pub black_king: bool,
    pub black_queen: bool,
}

impl Castle {
    pub fn as_fen(&self) -> String {
        let mut vec: Vec<char> = vec![];

        if self.white_king {
            vec[0] = 'K';
        }
        if self.white_queen {
            vec[1] = 'Q';
        }
        if self.black_king {
            vec[2] = 'k';
        }
        if self.black_queen {
            vec[3] = 'q';
        }

        if vec.is_empty() {
            return "-".to_string();
        } else {
            vec.into_iter().collect()
        }
    }

    pub fn from_fen(&mut self, castling_slice: String) -> Result<(), ChessError> {
        for ch in castling_slice.chars() {
            match ch {
                'K' => self.white_king = true,
                'Q' => self.white_queen = true,
                'k' => self.black_king = true,
                'q' => self.black_queen = true,
                _ => {
                    return Err(ChessError::ParseError(
                        "couldnt parse castling rights into char".into(),
                    ))
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct MoveMeta {
    pub piece: Piece,
    pub from: Position,
    pub to: Position,
    pub castle: Option<Castle>,
    pub promotion: Option<Piece>,
    pub capture: Option<Position>,
}

pub type PieceEntityData = HashMap<String, serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct PieceEntity {
    pub piece: Piece,
    pub pos: Position,
    pub data: PieceEntityData,
}

impl PieceEntity {
    pub fn new(piece: Piece, pos: Position) -> Self {
        Self {
            piece,
            pos,
            data: HashMap::new(),
        }
    }

    pub fn set_meta<V: Into<serde_json::Value>>(&mut self, key: impl Into<String>, value: V) {
        self.data.insert(key.into(), value.into());
    }

    pub fn get_meta(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }

    pub fn legal_moves(&self, board: &Board) -> Result<Vec<Position>, ChessError> {
        Ok(match self.piece {
            Piece::Pawn(_c) => pawn::legal_moves(self, board)?,
            Piece::Knight(_c) => knight::legal_moves(self, board)?,
            Piece::Bishop(_c) => bishop::legal_moves(self, board)?,
            Piece::King(_c) => king::legal_moves(self, board)?,
            Piece::Rook(_c) => rook::legal_moves(self, board)?,
            Piece::Queen(_c) => queen::legal_moves(self, board)?,
        })
    }

    pub fn on_move(
        &mut self,
        new_pos: Position,
        board: &mut Board,
        promotion: Option<Promotion>,
    ) -> Result<MoveMeta, ChessError> {
        Ok(match self.piece {
            Piece::Pawn(_c) => pawn::on_move(self, new_pos, board, promotion)?,
            Piece::Knight(_c) => knight::on_move(self, new_pos, board)?,
            Piece::Bishop(_c) => bishop::on_move(self, new_pos, board)?,
            Piece::King(_c) => king::on_move(self, new_pos, board)?,
            Piece::Rook(_c) => rook::on_move(self, new_pos, board)?,
            Piece::Queen(_c) => queen::on_move(self, new_pos, board)?,
        })
    }
}
