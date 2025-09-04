use super::super::board::Board;
use super::super::utils::{step_moves, ChessError};
use super::{MoveMeta, PieceEntity, Position};

static STEPS: &[(i8, i8)] = &[
    (2, 1),
    (2, -1),
    (-2, 1),
    (-2, -1),
    (1, 2),
    (1, -2),
    (-1, 2),
    (-1, -2),
];

pub fn legal_moves(entity: &PieceEntity, board: &Board) -> Result<Vec<Position>, ChessError> {
    Ok(step_moves(board, entity.pos, STEPS, entity.piece.color())?)
}

pub fn on_move(
    entity: &mut PieceEntity,
    new_pos: Position,
    board: &mut Board,
) -> Result<MoveMeta, ChessError> {
    Ok(MoveMeta {
        piece: entity.piece.clone(),
        from: entity.pos,
        to: new_pos,
        castle: None,
        promotion: None,
        capture: match board.is_enemy(new_pos.rank, new_pos.file, entity.piece.color())? {
            true => Some(new_pos),
            false => None,
        },
    })
}
