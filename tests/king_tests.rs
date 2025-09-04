use super::super::*;
use utils::ChessError;

#[test]
fn castle_no_barrier() -> Result<(), ChessError> {
    let mut board = board::Board::new();

    board.set(0, 4, Some(piece::Piece::King(piece::Color::White)))?;
    board.set(0, 0, Some(piece::Piece::Rook(piece::Color::White)))?;

    let mut king_entity = piece::PieceEntity::new(
        piece::Piece::King(piece::Color::White),
        piece::Position { rank: 0, file: 4 },
    );
    king_entity.set_meta("moved", false);
    let mut rook_entity = piece::PieceEntity::new(
        piece::Piece::Rook(piece::Color::White),
        piece::Position { rank: 0, file: 0 },
    );
    rook_entity.set_meta("moved", false);

    board.set_entity(0, 4, Some(king_entity.clone()));
    board.set_entity(0, 0, Some(rook_entity));

    let legal = king_entity.legal_moves(&board)?;

    assert!(legal.contains(&piece::Position { rank: 0, file: 2 }));
    Ok(())
}

#[test]
fn castle_w_barrier() -> Result<(), ChessError> {
    let mut board = board::Board::new();

    board.set(0, 4, Some(piece::Piece::King(piece::Color::White)))?;
    board.set(0, 0, Some(piece::Piece::Rook(piece::Color::White)))?;
    board.set(7, 3, Some(piece::Piece::Queen(piece::Color::Black)))?;

    let mut king_entity = piece::PieceEntity::new(
        piece::Piece::King(piece::Color::White),
        piece::Position { rank: 0, file: 4 },
    );
    king_entity.set_meta("moved", false);
    let mut rook_entity = piece::PieceEntity::new(
        piece::Piece::Rook(piece::Color::White),
        piece::Position { rank: 0, file: 0 },
    );
    rook_entity.set_meta("moved", false);
    let queen_entity = piece::PieceEntity::new(
        piece::Piece::Queen(piece::Color::Black),
        piece::Position { rank: 7, file: 3 },
    );

    board.set_entity(0, 4, Some(king_entity.clone()));
    board.set_entity(0, 0, Some(rook_entity));
    board.set_entity(7, 3, Some(queen_entity));

    let legal = king_entity.legal_moves(&board)?;

    assert!(!legal.contains(&piece::Position { rank: 0, file: 2 }));
    Ok(())
}
