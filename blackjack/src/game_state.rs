use cards::prelude::Shoe;
use player::{House, Player};

/// Progress will let you know where you are in the game.  It will help enforce that certain
/// actions can only occur when the game is in a given state.
#[derive(Debug, PartialEq, Clone)]
pub enum Progress {
    Starting,
    Playing,
    Done,
}

/// Prrogress needs a standard way to be printed out.  That's what Display is for.
impl std::fmt::Display for Progress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Progress::Starting => write!(f, "Starting"),
            Progress::Playing => write!(f, "Playing"),
            Progress::Done => write!(f, "Done"),
        }
    }
}

/// When creating a Progress there should be a starting point.
impl Default for Progress {
    fn default() -> Self {
        Progress::Starting
    }
}

/// GameState keeps track of the important things about the game.  As games are added this game
/// state could be updated to include more generic items about that state.
#[derive(Default, Clone, Debug, PartialEq)]
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
