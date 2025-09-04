use super::super::*;
use utils::ChessError;

#[test]
fn no_moves() -> Result<(), ChessError> {
    let mut board = board::Board::new();

    board.set(0, 0, Some(piece::Piece::Knight(piece::Color::White)));
    board.set(1, 2, Some(piece::Piece::Knight(piece::Color::White)));
    board.set(2, 1, Some(piece::Piece::Knight(piece::Color::White)));

    let knight_1_entity = piece::PieceEntity::new(
        piece::Piece::Knight(piece::Color::White),
        piece::Position { rank: 0, file: 0 },
    );
    let knight_2_entity = piece::PieceEntity::new(
        piece::Piece::Knight(piece::Color::White),
        piece::Position { rank: 1, file: 2 },
    );
    let knight_3_entity = piece::PieceEntity::new(
        piece::Piece::Knight(piece::Color::White),
        piece::Position { rank: 2, file: 1 },
    );

    board.set_entity(0, 0, Some(knight_1_entity.clone()));
    board.set_entity(1, 2, Some(knight_2_entity));
    board.set_entity(2, 1, Some(knight_3_entity));

    assert_eq!(knight_1_entity.legal_moves(&board)?, vec![]);
    Ok(())
}

#[test]
fn one_capture() -> Result<(), ChessError> {
    let mut board = board::Board::new();

    board.set(0, 0, Some(piece::Piece::Knight(piece::Color::White)));
    board.set(1, 2, Some(piece::Piece::Knight(piece::Color::White)));
    board.set(2, 1, Some(piece::Piece::Knight(piece::Color::Black)));

    let knight_1_entity = piece::PieceEntity::new(
        piece::Piece::Knight(piece::Color::White),
        piece::Position { rank: 0, file: 0 },
    );
    let knight_2_entity = piece::PieceEntity::new(
        piece::Piece::Knight(piece::Color::White),
        piece::Position { rank: 1, file: 2 },
    );
    let knight_3_entity = piece::PieceEntity::new(
        piece::Piece::Knight(piece::Color::Black),
        piece::Position { rank: 2, file: 1 },
    );

    board.set_entity(0, 0, Some(knight_1_entity.clone()));
    board.set_entity(1, 2, Some(knight_2_entity));
    board.set_entity(2, 1, Some(knight_3_entity));

    assert_eq!(
        knight_1_entity.legal_moves(&board)?,
        vec![piece::Position { rank: 2, file: 1 }]
    );
    Ok(())
}

#[test]
fn initial_position() -> Result<(), ChessError> {
    let mut board = board::Board::new();
    let template = utils::file_to_str("chess/templates/default.json")?;
    board.from_template(template)?;

    assert!(board
        .get_entity(0, 1)
        .unwrap()
        .legal_moves(&board)?
        .contains(&piece::Position { rank: 2, file: 2 },));

    assert!(board
        .get_entity(0, 1)
        .unwrap()
        .legal_moves(&board)?
        .contains(&piece::Position { rank: 2, file: 0 }));

    assert!(board
        .get_entity(0, 6)
        .unwrap()
        .legal_moves(&board)?
        .contains(&piece::Position { rank: 2, file: 5 }));

    assert!(board
        .get_entity(0, 6)
        .unwrap()
        .legal_moves(&board)?
        .contains(&piece::Position { rank: 2, file: 7 }));

    Ok(())
}
