use cards::prelude::Shoe;
use player::{House, Player};

#[derive(Debug, PartialEq, Clone)]
pub enum Progress {
    Starting,
    Playing,
    Done,
}

impl std::fmt::Display for Progress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Progress::Starting => write!(f, "Starting"),
            Progress::Playing => write!(f, "Playing"),
            Progress::Done => write!(f, "Done"),
        }
    }
}

impl Default for Progress {
    fn default() -> Self {
        Progress::Starting
    }
}

#[derive(Default, Clone)]
pub struct GameState {
    pub progress: Progress,
    pub house: House,
    pub players: Vec<Player>,
    pub shoe: Shoe,
}

#[cfg(test)]
mod tests {
    use super::GameState;

    #[test]
    fn default_game_state() {
        let state: GameState = Default::default();
        assert_eq!(0, state.players.len())
    }
}
