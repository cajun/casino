use crate::game_state::Progress;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuleError<'a> {
    #[error("Game state is in {0}.")]
    InvalidState(&'a Progress),
}
