use crate::game_state::Progress;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuleError {
    #[error("Game state is in {0}.")]
    InvalidState(Progress),
}
