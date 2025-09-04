use super::piece::{Castle, Color, MoveMeta, Piece, PieceEntity, Position};
use super::utils::{all_positions, mask, ChessError, FILES};
use rayon::prelude::*;

#[derive(Default, Debug, Clone)]
struct Pieces {
    white_pawns: u64,
    white_rooks: u64,
    white_knights: u64,
    white_bishops: u64,
    white_king: u64,
    white_queen: u64,

    black_pawns: u64,
    black_rooks: u64,
    black_knights: u64,
    black_bishops: u64,
    black_king: u64,
    black_queen: u64,
}

#[derive(Debug, Clone)]
pub struct Board {
    positions: Pieces,
    turn: Color,
    castling: Castle,
    passant_target: Option<Position>,
    fullmove: u8,
    halfmove: u8,
    entities: [Option<PieceEntity>; 64],
    history: Vec<MoveMeta>,
    // clock:
}

impl Board {
    pub fn new() -> Self {
        Self {
            positions: Pieces::default(),
            turn: Color::default(),
            castling: Castle::default(),
            passant_target: None,
            fullmove: 1,
            halfmove: 0,
            entities: std::array::from_fn(|_| None),
            history: Vec::new(),
        }
    }

    pub fn get_passant_target(&self) -> Option<Position> {
        self.passant_target
    }

    pub fn set_passant_target(&mut self, target: Option<Position>) -> () {
        self.passant_target = target;
    }

    pub fn get(&self, rank: usize, file: usize) -> Result<Option<Piece>, ChessError> {
        let mask = mask(rank, file)?;
        let boards = [
            (self.positions.white_pawns, Piece::Pawn(Color::White)),
            (self.positions.white_rooks, Piece::Rook(Color::White)),
            (self.positions.white_knights, Piece::Knight(Color::White)),
            (self.positions.white_bishops, Piece::Bishop(Color::White)),
            (self.positions.white_king, Piece::King(Color::White)),
            (self.positions.white_queen, Piece::Queen(Color::White)),
            (self.positions.black_pawns, Piece::Pawn(Color::Black)),
            (self.positions.black_rooks, Piece::Rook(Color::Black)),
            (self.positions.black_knights, Piece::Knight(Color::Black)),
            (self.positions.black_bishops, Piece::Bishop(Color::Black)),
            (self.positions.black_king, Piece::King(Color::Black)),
            (self.positions.black_queen, Piece::Queen(Color::Black)),
        ];

        let mut found: (bool, Option<Piece>) = (false, None);

        for (bitboard, piece) in boards {
            if bitboard & mask != 0 {
                if found.0 {
                    let prev = found.1.unwrap();
                    return Err(ChessError::InternalError(format!(
                        "[ERROR]: two pieces on the same spot: {} with {} on {}:{}",
                        prev.fen_char(),
                        piece.fen_char(),
                        rank,
                        file
                    )));
                }
                found = (true, Some(piece));
            }
        }

        Ok(found.1)
    }

    pub fn set(&mut self, rank: usize, file: usize, p: Option<Piece>) -> Result<(), ChessError> {
        let mask = mask(rank, file)?;

        self.positions.white_pawns &= !mask;
        self.positions.white_rooks &= !mask;
        self.positions.white_knights &= !mask;
        self.positions.white_bishops &= !mask;
        self.positions.white_king &= !mask;
        self.positions.white_queen &= !mask;

        self.positions.black_pawns &= !mask;
        self.positions.black_rooks &= !mask;
        self.positions.black_knights &= !mask;
        self.positions.black_bishops &= !mask;
        self.positions.black_king &= !mask;
        self.positions.black_queen &= !mask;

        if let Some(piece) = p {
            match piece {
                Piece::Pawn(Color::White) => self.positions.white_pawns |= mask,
                Piece::Knight(Color::White) => self.positions.white_knights |= mask,
                Piece::Bishop(Color::White) => self.positions.white_bishops |= mask,
                Piece::Rook(Color::White) => self.positions.white_rooks |= mask,
                Piece::Queen(Color::White) => self.positions.white_queen |= mask,
                Piece::King(Color::White) => self.positions.white_king |= mask,

                Piece::Pawn(Color::Black) => self.positions.black_pawns |= mask,
                Piece::Knight(Color::Black) => self.positions.black_knights |= mask,
                Piece::Bishop(Color::Black) => self.positions.black_bishops |= mask,
                Piece::Rook(Color::Black) => self.positions.black_rooks |= mask,
                Piece::Queen(Color::Black) => self.positions.black_queen |= mask,
                Piece::King(Color::Black) => self.positions.black_king |= mask,
            }
        }

        Ok(())
    }

    pub fn get_king_pos(&self, color: Color) -> Result<Option<Position>, ChessError> {
        Ok(all_positions().find(|&pos| {
            self.get(pos.rank, pos.file)
                .map_or(false, |p| p == Some(Piece::King(color)))
        }))
    }

    pub fn get_entity(&self, rank: usize, file: usize) -> Option<&PieceEntity> {
        let idx = Position { rank, file }.to_index();
        self.entities[idx].as_ref()
    }

    pub fn get_entity_mut(&mut self, rank: usize, file: usize) -> Option<&mut PieceEntity> {
        let idx = Position { rank, file }.to_index();
        self.entities[idx].as_mut()
    }

    pub fn set_entity(&mut self, rank: usize, file: usize, entity: Option<PieceEntity>) -> () {
        let idx = Position { rank, file }.to_index();
        self.entities[idx] = entity;
    }

    pub fn is_enemy(&self, rank: usize, file: usize, color: Color) -> Result<bool, ChessError> {
        match self.get(rank, file)? {
            Some(p) => Ok(p.color() != color),
            None => Ok(false),
        }
    }

    pub fn is_empty(&self, rank: usize, file: usize) -> Result<bool, ChessError> {
        match self.get(rank, file)? {
            Some(_p) => Ok(false),
            None => Ok(true),
        }
    }

    pub fn can_move_to(&self, pos: Position, color: Option<Color>) -> Result<bool, ChessError> {
        match (self.get(pos.rank, pos.file)?, color) {
            (None, _) => Ok(true),
            (Some(p), Some(c)) => Ok(p.color() != c),
            (Some(_), None) => Ok(true),
        }
    }

    pub fn is_square_attacked(
        &self,
        from: Position,
        attacker_color: Color,
    ) -> Result<bool, ChessError> {
        let attacked = all_positions()
            .par_bridge()
            .filter_map(|pos| self.get_entity(pos.rank, pos.file))
            .filter(|e| e.piece.color() == attacker_color)
            .try_fold(
                || false,
                |acc, attacker| {
                    if acc {
                        Ok(acc)
                    } else {
                        let moves = attacker.legal_moves(self)?;
                        Ok(moves.contains(&from))
                    }
                },
            )
            .try_reduce(|| false, |a, b| Ok(a || b))?;

        Ok(attacked)
    }

    pub fn generate_fen(&self) -> Result<String, ChessError> {
        let mut board_chars: Vec<char> = Vec::new();

        for rank in (0..8).rev() {
            let mut empty_count = 0;
            for file in 0..8 {
                if let Some(piece) = self.get(rank, file)? {
                    if empty_count > 0 {
                        for digit in empty_count.to_string().chars() {
                            board_chars.push(digit);
                        }
                        empty_count = 0;
                    }
                    board_chars.push(piece.fen_char());
                } else {
                    empty_count += 1;
                }
            }
            if empty_count > 0 {
                for digit in empty_count.to_string().chars() {
                    board_chars.push(digit);
                }
            }
            if rank > 0 {
                board_chars.push('/');
            }
        }

        Ok(format!(
            "{} {} {} {} {} {}",
            board_chars.into_iter().collect::<String>(),
            self.turn.fen_char(),
            self.castling.as_fen(),
            if let Some(p_target) = self.passant_target.clone() {
                format!("{}{}", p_target.rank, FILES[p_target.file].1)
            } else {
                '-'.to_string()
            },
            self.halfmove,
            self.fullmove,
        ))
    }

    pub fn from_fen(&mut self, fen: String) -> Result<(), ChessError> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 6 {
            return Err(ChessError::ParseError(
                "invalid FEN, not enough parts".into(),
            ));
        }

        let (placement, turn, castling, passant, halfmove, fullmove) =
            (parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]);

        for (rank_idx, row) in placement.split('/').enumerate() {
            let mut file_idx = 0;
            for ch in row.chars() {
                if ch.is_digit(10) {
                    file_idx += ch.to_digit(10).unwrap() as usize;
                } else {
                    let rank = 7 - rank_idx;
                    let file = file_idx;

                    let piece = Piece::from_fen(ch);
                    self.set(rank, file, Some(piece));

                    file_idx += 1;
                }
            }
        }

        self.turn = Color::from_fen(turn);

        self.castling.from_fen(castling.to_string());
        self.passant_target = match passant {
            "-" => None,
            _ => Some(Position::from_fen(passant)),
        };

        self.halfmove = halfmove
            .parse::<u8>()
            .map_err(|_| ChessError::ParseError("halfmove clock parse error".into()))?;

        self.fullmove = fullmove
            .parse::<u8>()
            .map_err(|_| ChessError::ParseError("fullmove clock parse error".into()))?;

        Ok(())
    }

    pub fn from_template(&mut self, template: String) -> Result<(), ChessError> {
        let vec: Vec<Option<PieceEntity>> = serde_json::from_str(&template)
            .map_err(|err| ChessError::ParseError(format!("fail parsing template: {}", err)))?;

        for entity in vec.iter() {
            if let Some(ent) = entity {
                self.set(ent.pos.rank, ent.pos.file, Some(ent.piece));
                self.set_entity(ent.pos.rank, ent.pos.file, Some(ent.clone()));
            }
        }

        Ok(())
    }
}
