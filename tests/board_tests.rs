use super::super::*;
use utils::ChessError;

#[test]
fn get_set() -> Result<(), ChessError> {
    let mut board = board::Board::new();
    assert_eq!(board.get(0, 0)?, None);

    board.set(0, 0, Some(piece::Piece::Rook(piece::Color::White)));

    assert_eq!(
        board.get(0, 0)?,
        Some(piece::Piece::Rook(piece::Color::White))
    );
    Ok(())
}

#[test]
fn fen() -> Result<(), ChessError> {
    let mut board = board::Board::new();
    assert_eq!(board.generate_fen()?, "8/8/8/8/8/8/8/8 w - - 0 1");

    board.set(0, 0, Some(piece::Piece::Rook(piece::Color::White)));
    board.set_entity(
        0,
        0,
        Some(piece::PieceEntity::new(
            piece::Piece::Rook(piece::Color::White),
            piece::Position { rank: 0, file: 0 },
        )),
    );

    assert_eq!(board.generate_fen()?, "8/8/8/8/8/8/8/R7 w - - 0 1");
    Ok(())
}

#[test]
fn template_parsing() -> Result<(), ChessError> {
    let mut board = board::Board::new();
    let template = utils::file_to_str("chess/templates/default.json")?;
    let _ = board.from_template(template)?;

    assert_eq!(
        board.get(7, 4)?,
        Some(piece::Piece::King(piece::Color::Black))
    );
    assert_eq!(
        board.get(0, 4)?,
        Some(piece::Piece::King(piece::Color::White))
    );
    assert_eq!(
        board.get(0, 0)?,
        Some(piece::Piece::Rook(piece::Color::White))
    );
    assert_eq!(
        board.get(7, 7)?,
        Some(piece::Piece::Rook(piece::Color::Black))
    );
    assert_eq!(
        board.get(0, 3)?,
        Some(piece::Piece::Queen(piece::Color::White))
    );
    assert_eq!(
        board.get(7, 3)?,
        Some(piece::Piece::Queen(piece::Color::Black))
    );
    Ok(())
}
