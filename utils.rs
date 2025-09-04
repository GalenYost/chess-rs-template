use super::board::Board;
use super::piece::{Color, Position};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChessError {
    InvalidPosition(String),
    IllegalMove(String),
    ParseError(String),
    InternalError(String),
}

pub static FILES: [(usize, char); 8] = [
    (0, 'a'),
    (1, 'b'),
    (2, 'c'),
    (3, 'd'),
    (4, 'e'),
    (5, 'f'),
    (6, 'g'),
    (7, 'h'),
];

pub fn mask(rank: usize, file: usize) -> Result<u64, ChessError> {
    if rank >= 8 || file >= 8 {
        return Err(ChessError::ParseError(String::from(
            "rank and file expected to be in range 0-8",
        )));
    }
    Ok(1u64 << (rank * 8 + file))
}

pub fn unmask(masked: u64) -> Result<(usize, usize), ChessError> {
    if masked == 0 || !masked.is_power_of_two() {
        return Err(ChessError::ParseError(format!(
            "unmask expects a single‐bit mask, got {:#x}",
            masked
        )));
    }

    let idx = masked.trailing_zeros() as usize;
    let rank = idx / 8;
    let file = idx % 8;
    Ok((rank, file))
}

pub fn validate_pos(rank: i8, file: i8) -> bool {
    (0..8).contains(&rank) && (0..8).contains(&file)
}

pub fn file_to_str(path: &str) -> Result<String, ChessError> {
    Ok(std::fs::read_to_string(std::path::Path::new(path))
        .map_err(|e| ChessError::ParseError(e.to_string()))?)
}

pub fn all_positions() -> impl Iterator<Item = Position> {
    (0..8).flat_map(|rank| (0..8).map(move |file| Position { rank, file }))
}

pub fn sliding_moves(
    board: &Board,
    from: Position,
    dirs: &[(i8, i8)],
    color: Option<Color>,
) -> Result<Vec<Position>, ChessError> {
    let mut moves = Vec::new();

    for &(dx, dy) in dirs {
        let mut current = from.clone();
        while let Some(next_pos) = current.shifted(dx, dy) {
            if board.is_empty(next_pos.rank, next_pos.file)? {
                moves.push(next_pos);
                current = next_pos;
                continue;
            }
            if board.can_move_to(next_pos, color)? {
                moves.push(next_pos);
            }
            break;
        }
    }

    Ok(moves)
}

pub fn step_moves(
    board: &Board,
    from: Position,
    deltas: &[(i8, i8)],
    color: Color,
) -> Result<Vec<Position>, ChessError> {
    let mut moves = Vec::new();

    for &(dx, dy) in deltas {
        if let Some(to) = from.shifted(dx, dy) {
            if board.can_move_to(to, Some(color))? {
                moves.push(to);
            }
        }
    }

    Ok(moves)
}
