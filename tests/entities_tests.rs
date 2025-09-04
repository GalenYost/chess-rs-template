use super::super::*;

#[test]
fn entities() -> () {
    let mut board = board::Board::new();
    assert!(board.get_entity(0, 0).is_none());

    board.set_entity(
        0,
        0,
        Some(piece::PieceEntity::new(
            piece::Piece::Rook(piece::Color::White),
            piece::Position { rank: 0, file: 0 },
        )),
    );

    assert!(board.get_entity(0, 0).is_some());
    assert_eq!(
        board.get_entity(0, 0).unwrap().piece,
        piece::Piece::Rook(piece::Color::White)
    );
}

#[test]
fn piece_entity_json() {
    let mut entity = piece::PieceEntity::new(
        piece::Piece::Rook(piece::Color::White),
        piece::Position { rank: 0, file: 0 },
    );

    entity.set_meta("test_data", 10);
    entity.set_meta("test_value_vec", vec!["spell1", "spell2", "spell3"]);

    serde_json::to_string(&entity).unwrap();
}
