use super::super::*;
use utils::ChessError;

#[test]
fn one_capture() -> Result<(), ChessError> {
    let mut board = board::Board::new();

    board.set(0, 0, Some(piece::Piece::Bishop(piece::Color::White)));
    board.set(1, 1, Some(piece::Piece::Bishop(piece::Color::Black)));

    let bishop_1_entity = piece::PieceEntity::new(
        piece::Piece::Bishop(piece::Color::White),
        piece::Position { rank: 0, file: 0 },
    );
    let bishop_2_entity = piece::PieceEntity::new(
        piece::Piece::Bishop(piece::Color::Black),
        piece::Position { rank: 1, file: 1 },
    );

    board.set_entity(0, 0, Some(bishop_1_entity.clone()));
    board.set_entity(1, 1, Some(bishop_2_entity));

    let legal = bishop_1_entity.legal_moves(&board)?;

    assert!(legal.contains(&piece::Position { rank: 1, file: 1 }));
    Ok(())
}

#[test]
fn initial_position() -> Result<(), ChessError> {
    let mut board = board::Board::new();
    let template = utils::file_to_str("chess/templates/default.json")?;
    board.from_template(template)?;

    assert_eq!(board.get_entity(0, 2).unwrap().legal_moves(&board)?, vec![]);
    Ok(())
}
