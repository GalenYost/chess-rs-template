use super::board::Board;
use super::piece::{Color, Piece, Position};
use super::utils::{all_positions, ChessError};

pub fn is_in_check(board: &Board, color: Color) -> Result<bool, ChessError> {
    let king_pos = board
        .get_king_pos(color)?
        .ok_or_else(|| ChessError::InternalError(format!("{:?} king not found", color)))?;

    for pos in all_positions() {
        if let Some(p) = board.get_entity(pos.rank, pos.file) {
            if p.piece != Piece::King(color) && p.legal_moves(board)?.contains(&king_pos) {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

pub fn filter_moves(
    board: &mut Board,
    moves: &mut Vec<Position>,
    from: Position,
    color: Color,
) -> Result<(), ChessError> {
    let mut filtered = Vec::with_capacity(moves.len());

    for &mv in moves.iter() {
        let mut b_clone = board.clone();

        if let Some(p_ent) = board.get_entity(from.rank, from.file) {
            let mut new_p = p_ent.clone();
            new_p.pos = mv;

            b_clone.set(from.rank, from.file, None)?;
            b_clone.set_entity(from.rank, from.file, None);
            b_clone.set(mv.rank, mv.file, Some(new_p.piece.clone()))?;
            b_clone.set_entity(mv.rank, mv.file, Some(new_p));

            if !is_in_check(&b_clone, color)? {
                filtered.push(mv);
            }
        }
    }

    *moves = filtered;
    Ok(())
}

pub fn is_checkmate(board: &mut Board, color: Color) -> Result<bool, ChessError> {
    if !is_in_check(board, color)? {
        return Ok(false);
    }

    let piece_positions: Vec<Position> = all_positions()
        .filter(|from| {
            board
                .get_entity(from.rank, from.file)
                .map_or(false, |p| p.piece.color() == color)
        })
        .collect();

    for pos in piece_positions {
        let entity = board.get_entity(pos.rank, pos.file).unwrap();
        let mut m = entity.legal_moves(board)?;
        filter_moves(board, &mut m, entity.pos, color)?;
        if !m.is_empty() {
            return Ok(false);
        }
    }

    Ok(true)
}

pub fn is_stalemate(board: &mut Board, color: Color) -> Result<bool, ChessError> {
    if is_in_check(board, color)? {
        return Ok(false);
    }

    let piece_positions: Vec<Position> = all_positions()
        .filter(|from| {
            board
                .get_entity(from.rank, from.file)
                .map_or(false, |p| p.piece.color() == color)
        })
        .collect();

    for pos in piece_positions {
        let entity = board.get_entity(pos.rank, pos.file).unwrap();
        let mut m = entity.legal_moves(board)?;
        filter_moves(board, &mut m, entity.pos, color)?;
        if !m.is_empty() {
            return Ok(false);
        }
    }

    Ok(true)
}
