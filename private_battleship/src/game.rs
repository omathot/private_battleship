use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum UserError {
    #[error("Can't play now, enemy's turn")]
    EnemyTurn,
    #[error("Cannot start game")]
    CannotStartGame,
    #[error("Invalid Input")]
    InvalidInput,
}
