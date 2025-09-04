use super::super::*;
use utils::ChessError;

#[test]
fn masking() -> Result<(), ChessError> {
    assert_eq!(utils::mask(1, 1)?, 1u64 << 9);
    assert_eq!(utils::mask(6, 1)?, 1u64 << 49);

    assert_eq!(utils::unmask(1u64 << 9)?, (1, 1));
    assert_eq!(utils::unmask(1u64 << 49)?, (6, 1));
    Ok(())
}

#[test]
fn file_reading() -> Result<(), ChessError> {
    let result = utils::file_to_str("chess/templates/none.json");
    assert!(result.is_err());
    Ok(())
}
