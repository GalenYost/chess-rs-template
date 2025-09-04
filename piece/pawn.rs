use super::super::board::Board;
use super::super::utils::{validate_pos, ChessError};
use super::{Color, MoveMeta, Piece, PieceEntity, Position};

pub fn legal_moves(entity: &PieceEntity, board: &Board) -> Result<Vec<Position>, ChessError> {
    let mut moves = Vec::new();

    let dir = match entity.piece.color() {
        Color::White => 1,
        Color::Black => -1,
    };

    let rank_forward = entity.pos.rank as i8 + dir;
    if board.is_empty(rank_forward as usize, entity.pos.file)?
        && validate_pos(rank_forward, entity.pos.file as i8)
    {
        moves.push(Position {
            rank: rank_forward as usize,
            file: entity.pos.file,
        });

        if let Some(serde_json::Value::Bool(moved_data)) = entity.get_meta("moved") {
            if *moved_data == false {
                let two_forward = rank_forward + dir;
                if validate_pos(two_forward, entity.pos.file as i8) {
                    if board.is_empty(two_forward as usize, entity.pos.file)? {
                        moves.push(Position {
                            rank: two_forward as usize,
                            file: entity.pos.file,
                        });
                    }
                }
            }
        }
    }

    for d_file in [-1, 1] {
        let file = entity.pos.file as i8 + d_file;
        if validate_pos(rank_forward, file)
            && board.is_enemy(rank_forward as usize, file as usize, entity.piece.color())?
        {
            moves.push(Position {
                rank: rank_forward as usize,
                file: file as usize,
            });
        }

        if let Some(p_target) = board.get_passant_target() {
            let file = p_target.file as i8 + d_file;

            if !validate_pos(entity.pos.rank as i8, file) {
                continue;
            }

            if entity.pos.rank as i8 != p_target.rank as i8 + dir {
                continue;
            }

            if board.is_enemy(entity.pos.rank, file as usize, entity.piece.color())?
                && board.is_empty(p_target.rank, p_target.file)?
            {
                if let Some(entity) = board.get_entity(entity.pos.rank, file as usize) {
                    match entity.piece {
                        Piece::Pawn(_c) => {
                            moves.push(Position {
                                rank: p_target.rank,
                                file: p_target.file,
                            });
                        }
                        _ => continue,
                    };
                }
            }
        }
    }

    Ok(moves)
}

pub struct Promotion {
    pub pawn_pos: Position,
    pub new_piece: Piece,
}

pub fn on_move(
    entity: &mut PieceEntity,
    new_pos: Position,
    board: &mut Board,
    promotion: Option<Promotion>,
) -> Result<MoveMeta, ChessError> {
    if (entity.pos.rank as i8 - new_pos.rank as i8).abs() == 2 {
        for d_file in [-1, 1] {
            let file = entity.pos.file as i8 + d_file;
            if validate_pos(new_pos.rank as i8, file) {
                match board.get(new_pos.rank, file as usize)? {
                    Some(p) => {
                        let p_target_rank: usize = match entity.piece.color() {
                            Color::White => (new_pos.rank as i8 - 1) as usize,
                            Color::Black => (new_pos.rank as i8 + 1) as usize,
                        };
                        match p {
                            Piece::Pawn(_c) => board.set_passant_target(Some(Position {
                                rank: p_target_rank,
                                file: entity.pos.file,
                            })),
                            _ => continue,
                        }
                    }
                    None => continue,
                }
            }
        }
    }

    let prev_piece = entity.piece.clone();
    if let Some(promotion_data) = &promotion {
        entity.piece = promotion_data.new_piece.clone();
    }

    entity.set_meta("moved", true);

    Ok(MoveMeta {
        piece: prev_piece,
        from: entity.pos,
        to: new_pos,
        castle: None,
        promotion: match promotion {
            Some(promotion) => Some(promotion.new_piece),
            None => None,
        },
        capture: match board.is_enemy(new_pos.rank, new_pos.file, entity.piece.color())? {
            true => Some(new_pos),
            false => None,
        },
    })
}
