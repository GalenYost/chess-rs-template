use super::super::*;
use piece::Position;
use utils::ChessError;

#[test]
fn is_in_check() -> Result<(), ChessError> {
    let mut board = board::Board::new();

    board.set(2, 3, Some(piece::Piece::King(piece::Color::White)));
    board.set(7, 3, Some(piece::Piece::Queen(piece::Color::Black)));

    let king_entity = piece::PieceEntity::new(
        piece::Piece::King(piece::Color::White),
        piece::Position { rank: 2, file: 3 },
    );
    let queen_entity = piece::PieceEntity::new(
        piece::Piece::Queen(piece::Color::Black),
        piece::Position { rank: 7, file: 3 },
    );

    board.set_entity(2, 3, Some(king_entity));
    board.set_entity(7, 3, Some(queen_entity));

    assert!(rules::is_in_check(&board, piece::Color::White)?);
    Ok(())
}

#[test]
fn cant_walk_to_check() -> Result<(), ChessError> {
    let mut board = board::Board::new();

    board.set(2, 2, Some(piece::Piece::King(piece::Color::White)));
    board.set(7, 3, Some(piece::Piece::Queen(piece::Color::Black)));

    let king_entity = piece::PieceEntity::new(
        piece::Piece::King(piece::Color::White),
        piece::Position { rank: 2, file: 2 },
    );
    let queen_entity = piece::PieceEntity::new(
        piece::Piece::Queen(piece::Color::Black),
        piece::Position { rank: 7, file: 3 },
    );

    board.set_entity(2, 2, Some(king_entity.clone()));
    board.set_entity(7, 3, Some(queen_entity));

    let mut king_moves = piece::king::legal_moves(&king_entity, &board)?;
    rules::filter_moves(
        &mut board,
        &mut king_moves,
        piece::Position { rank: 2, file: 2 },
        piece::Color::White,
    );

    assert!(king_moves.contains(&piece::Position { rank: 1, file: 1 }));
    assert!(king_moves.contains(&piece::Position { rank: 2, file: 1 }));
    assert!(king_moves.contains(&piece::Position { rank: 3, file: 1 }));
    assert!(king_moves.contains(&piece::Position { rank: 3, file: 2 }));
    assert!(king_moves.contains(&piece::Position { rank: 1, file: 2 }));

    assert!(!king_moves.contains(&piece::Position { rank: 1, file: 3 }));
    assert!(!king_moves.contains(&piece::Position { rank: 2, file: 3 }));
    assert!(!king_moves.contains(&piece::Position { rank: 3, file: 3 }));
    Ok(())
}

#[test]
fn is_checkmate() -> Result<(), ChessError> {
    let mut board = board::Board::new();

    board.set(0, 1, Some(piece::Piece::King(piece::Color::White)));
    board.set(1, 1, Some(piece::Piece::Queen(piece::Color::Black)));

    let king_entity = piece::PieceEntity::new(
        piece::Piece::King(piece::Color::White),
        piece::Position { rank: 0, file: 1 },
    );
    let queen_entity = piece::PieceEntity::new(
        piece::Piece::Queen(piece::Color::Black),
        piece::Position { rank: 1, file: 1 },
    );

    board.set_entity(0, 1, Some(king_entity.clone()));
    board.set_entity(1, 1, Some(queen_entity.clone()));

    assert!(!rules::is_checkmate(&mut board, piece::Color::White)?);

    board.set(2, 1, Some(piece::Piece::Queen(piece::Color::Black)));

    let mut new_queen_entity = queen_entity.clone();
    new_queen_entity.pos = Position { rank: 2, file: 1 };
    board.set_entity(2, 1, Some(new_queen_entity));

    assert!(rules::is_checkmate(&mut board, piece::Color::White)?);
    Ok(())
}

#[test]
fn is_stalemate() -> Result<(), ChessError> {
    let mut board = board::Board::new();

    board.set(0, 1, Some(piece::Piece::King(piece::Color::White)));

    board.set(1, 7, Some(piece::Piece::Queen(piece::Color::Black)));
    board.set(1, 7, Some(piece::Piece::Queen(piece::Color::Black)));
    board.set(7, 0, Some(piece::Piece::Queen(piece::Color::Black)));
    board.set(7, 2, Some(piece::Piece::Queen(piece::Color::Black)));

    let king_entity = piece::PieceEntity::new(
        piece::Piece::King(piece::Color::White),
        piece::Position { rank: 0, file: 1 },
    );
    let queen_entity_1 = piece::PieceEntity::new(
        piece::Piece::Queen(piece::Color::Black),
        piece::Position { rank: 1, file: 7 },
    );
    let queen_entity_2 = piece::PieceEntity::new(
        piece::Piece::Queen(piece::Color::Black),
        piece::Position { rank: 7, file: 0 },
    );
    let queen_entity_3 = piece::PieceEntity::new(
        piece::Piece::Queen(piece::Color::Black),
        piece::Position { rank: 7, file: 2 },
    );

    board.set_entity(0, 1, Some(king_entity.clone()));
    board.set_entity(1, 7, Some(queen_entity_1.clone()));
    board.set_entity(7, 0, Some(queen_entity_2.clone()));
    board.set_entity(7, 2, Some(queen_entity_3.clone()));

    assert!(!rules::is_checkmate(&mut board, piece::Color::White)?);
    assert!(rules::is_stalemate(&mut board, piece::Color::White)?);
    Ok(())
}
