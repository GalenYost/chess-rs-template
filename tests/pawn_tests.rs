use super::super::*;
use utils::ChessError;

#[test]
fn pawn_legal_moves() -> Result<(), ChessError> {
    let mut board = board::Board::new();

    board.set(1, 1, Some(piece::Piece::Pawn(piece::Color::White)));
    board.set(2, 2, Some(piece::Piece::Pawn(piece::Color::White)));
    board.set(3, 3, Some(piece::Piece::Pawn(piece::Color::Black)));

    let mut entity1 = piece::PieceEntity::new(
        piece::Piece::Pawn(piece::Color::White),
        piece::Position { rank: 1, file: 1 },
    );
    entity1.set_meta("moved", false);

    let mut entity2 = piece::PieceEntity::new(
        piece::Piece::Pawn(piece::Color::White),
        piece::Position { rank: 2, file: 2 },
    );
    entity2.set_meta("moved", true);

    let mut entity3 = piece::PieceEntity::new(
        piece::Piece::Pawn(piece::Color::Black),
        piece::Position { rank: 3, file: 3 },
    );
    entity3.set_meta("moved", true);

    board.set_entity(1, 1, Some(entity1.clone()));
    board.set_entity(2, 2, Some(entity2.clone()));
    board.set_entity(3, 3, Some(entity3.clone()));

    // moving for entity1 (two forward)
    assert_eq!(
        piece::pawn::legal_moves(&entity1, &board)?,
        vec![
            piece::Position { rank: 2, file: 1 },
            piece::Position { rank: 3, file: 1 }
        ]
    );

    // moving for entity2 (one forward + capture on 3:3 (right))
    assert_eq!(
        piece::pawn::legal_moves(&entity2, &board)?,
        vec![
            piece::Position { rank: 3, file: 2 },
            piece::Position { rank: 3, file: 3 },
        ]
    );

    // moving for entity3 (one forward + capture on 2:2 (right))
    assert_eq!(
        piece::pawn::legal_moves(&entity3, &board)?,
        vec![
            piece::Position { rank: 2, file: 3 },
            piece::Position { rank: 2, file: 2 },
        ]
    );
    Ok(())
}

#[test]
fn pawn_on_move() -> Result<(), ChessError> {
    let mut board = board::Board::new();

    board.set(6, 1, Some(piece::Piece::Pawn(piece::Color::White)));
    let mut pawn_entity = piece::PieceEntity::new(
        piece::Piece::Pawn(piece::Color::White),
        piece::Position { rank: 6, file: 1 },
    );

    let on_move_result = piece::pawn::on_move(
        &mut pawn_entity,
        piece::Position { rank: 7, file: 1 },
        &mut board,
        Some(piece::pawn::Promotion {
            pawn_pos: piece::Position { rank: 6, file: 1 },
            new_piece: piece::Piece::Queen(piece::Color::White),
        }),
    )?;

    assert_eq!(
        on_move_result,
        piece::MoveMeta {
            piece: piece::Piece::Pawn(piece::Color::White),
            from: piece::Position { rank: 6, file: 1 },
            to: piece::Position { rank: 7, file: 1 },
            castle: None,
            promotion: Some(piece::Piece::Queen(piece::Color::White)),
            capture: None,
        }
    );
    Ok(())
}

#[test]
fn on_move_passant_target_set() -> Result<(), ChessError> {
    let mut board = board::Board::new();

    board.set(1, 1, Some(piece::Piece::Pawn(piece::Color::White)));
    board.set(3, 2, Some(piece::Piece::Pawn(piece::Color::Black)));

    let _pawn_entity_white = piece::PieceEntity::new(
        piece::Piece::Pawn(piece::Color::White),
        piece::Position { rank: 1, file: 1 },
    );
    let mut pawn_entity_black = piece::PieceEntity::new(
        piece::Piece::Pawn(piece::Color::Black),
        piece::Position { rank: 3, file: 2 },
    );

    let on_move_result = piece::pawn::on_move(
        &mut pawn_entity_black,
        piece::Position { rank: 1, file: 2 },
        &mut board,
        None,
    )?;

    assert_eq!(
        board.get_passant_target(),
        Some(piece::Position { rank: 2, file: 2 })
    );

    assert_eq!(
        on_move_result,
        piece::MoveMeta {
            piece: piece::Piece::Pawn(piece::Color::Black),
            from: piece::Position { rank: 3, file: 2 },
            to: piece::Position { rank: 1, file: 2 },
            castle: None,
            promotion: None,
            capture: None,
        }
    );
    Ok(())
}
