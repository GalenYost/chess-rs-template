use super::super::*;
use utils::ChessError;

#[test]
fn two_capture() -> Result<(), ChessError> {
    let mut board = board::Board::new();

    board.set(0, 0, Some(piece::Piece::Queen(piece::Color::White)));
    board.set(1, 1, Some(piece::Piece::Queen(piece::Color::Black)));
    board.set(1, 0, Some(piece::Piece::Queen(piece::Color::White)));
    board.set(0, 1, Some(piece::Piece::Queen(piece::Color::Black)));

    let queen_1_entity = piece::PieceEntity::new(
        piece::Piece::Queen(piece::Color::White),
        piece::Position { rank: 0, file: 0 },
    );
    let queen_2_entity = piece::PieceEntity::new(
        piece::Piece::Queen(piece::Color::White),
        piece::Position { rank: 1, file: 1 },
    );
    let queen_3_entity = piece::PieceEntity::new(
        piece::Piece::Queen(piece::Color::Black),
        piece::Position { rank: 1, file: 0 },
    );
    let queen_4_entity = piece::PieceEntity::new(
        piece::Piece::Queen(piece::Color::Black),
        piece::Position { rank: 0, file: 1 },
    );

    board.set_entity(0, 0, Some(queen_1_entity.clone()));
    board.set_entity(1, 1, Some(queen_2_entity));
    board.set_entity(1, 0, Some(queen_3_entity));
    board.set_entity(0, 1, Some(queen_4_entity));

    let legal = queen_1_entity.legal_moves(&board)?;

    assert!(
        legal.contains(&piece::Position { rank: 0, file: 1 })
            && legal.contains(&piece::Position { rank: 1, file: 1 })
    );
    Ok(())
}

#[test]
fn initial_position() -> Result<(), ChessError> {
    let mut board = board::Board::new();
    let template = utils::file_to_str("chess/templates/default.json")?;
    board.from_template(template)?;

    assert_eq!(board.get_entity(0, 3).unwrap().legal_moves(&board)?, vec![]);
    Ok(())
}
