use super::super::*;
use utils::ChessError;

#[test]
fn two_pieces_on_same_square() -> Result<(), ChessError> {
    let mut board = board::Board::new();

    board.set(0, 0, Some(piece::Piece::Rook(piece::Color::White)));
    board.set(0, 0, Some(piece::Piece::Queen(piece::Color::White)));

    assert_eq!(
        board.get(0, 0)?,
        Some(piece::Piece::Queen(piece::Color::White))
    );
    Ok(())
}
