use super::super::board::Board;
use super::super::utils::{sliding_moves, step_moves, ChessError};
use super::{Castle, Color, MoveMeta, Piece, PieceEntity, Position};

static STEPS: &[(i8, i8)] = &[
    (1, 0),
    (-1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 1),
    (0, 1),
    (1, 1),
];

static CASTLE_DIRS: &[(i8, i8)] = &[(1, 0), (-1, 0)];

pub fn legal_moves(entity: &PieceEntity, board: &Board) -> Result<Vec<Position>, ChessError> {
    let mut moves = step_moves(board, entity.pos, STEPS, entity.piece.color())?;

    let has_moved = match entity.get_meta("moved") {
        Some(val) if *val == serde_json::Value::Bool(true) => true,
        _ => false,
    };
    if has_moved {
        return Ok(moves);
    }

    let sliding_moves = sliding_moves(board, entity.pos, CASTLE_DIRS, None)?;

    for mv in sliding_moves.iter() {
        if let Some(ent) = board.get_entity(mv.rank, mv.file) {
            if ent.piece != Piece::Rook(entity.piece.color()) {
                continue;
            }
            let rook_has_moved = match ent.get_meta("moved") {
                Some(val) if *val == serde_json::Value::Bool(true) => true,
                _ => false,
            };
            if rook_has_moved {
                continue;
            }

            let d_col = (mv.file as i8 - entity.pos.file as i8).signum();
            if let Some(castle_target) = entity.pos.shifted(d_col * 2, 0) {
                let path_squares = [entity.pos.shifted(d_col, 0).unwrap(), castle_target];

                let under_attack = {
                    let mut attacked = false;
                    for &sq in &path_squares {
                        if board.is_square_attacked(sq, entity.piece.color().opposite())? {
                            attacked = true;
                            break;
                        }
                    }
                    attacked
                };

                if !under_attack {
                    moves.push(castle_target);
                }
            }
        }
    }

    Ok(moves)
}

pub fn on_move(
    entity: &mut PieceEntity,
    new_pos: Position,
    board: &mut Board,
) -> Result<MoveMeta, ChessError> {
    let delta_col = new_pos.file as i8 - entity.pos.file as i8;

    let has_moved = match entity.get_meta("moved") {
        Some(val) if *val == serde_json::Value::Bool(true) => true,
        _ => false,
    };

    let is_castle = delta_col.abs() == 2 && !has_moved;

    let is_white = match entity.piece.color() {
        Color::White => true,
        Color::Black => false,
    };

    let castle_side: Castle = Castle {
        white_king: is_white && is_castle && new_pos.file > entity.pos.file,
        white_queen: is_white && is_castle && new_pos.file < entity.pos.file,
        black_king: !is_white && is_castle && new_pos.file > entity.pos.file,
        black_queen: !is_white && is_castle && new_pos.file < entity.pos.file,
    };

    entity.set_meta("moved", true);

    Ok(MoveMeta {
        piece: entity.piece.clone(),
        from: entity.pos,
        to: new_pos,
        castle: Some(castle_side),
        promotion: None,
        capture: match board.is_enemy(new_pos.rank, new_pos.file, entity.piece.color())? {
            true => Some(new_pos),
            false => None,
        },
    })
}
