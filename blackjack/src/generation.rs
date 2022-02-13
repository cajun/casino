use crate::game_state::GameState;
use std::time::SystemTime;

/// Generation will contain and maintain the history of the game state.  It will keep this history
/// in a tree structure.
#[derive(Debug)]
pub struct Generation {
    state: GameState,
    timestamp: SystemTime,
    children: Vec<Box<Generation>>,
}

impl Default for Generation {
    fn default() -> Self {
        Self {
            state: Default::default(),
            timestamp: SystemTime::now(),
            children: Default::default(),
        }
    }
}

impl Generation {
    /// Create a new generation with a given state.  A generation can never be created with an
    /// empty state.
    ///
    /// * `state`: What state should this generation keep track of
    fn new(state: GameState) -> Self {
        Generation {
            state,
            timestamp: SystemTime::now(),
            ..Default::default()
        }
    }

    /// Add a new GameState to the list of game states on this generation.  This will allow each
    /// generation to have multiple possible outcomes.
    ///
    /// Example:
    /// ```
    /// use blackjack::prelude::{ Generation, GameState };
    ///
    /// let mut generation: Generation = Default::default();
    /// let game_state: GameState = Default::default();
    ///
    /// generation.add_generation(game_state);
    ///
    /// assert_eq!(1, generation.number_of_branches());
    /// ```
    ///
    /// * `state`: The GameState that needs to be added to this generation.
    pub fn add_generation(&mut self, state: GameState) {
        let generation = Generation::new(state);
        self.children.push(Box::new(generation));
    }

    /// Return the number of possible branches which occur in this generation.
    ///
    /// Example:
    /// ```
    /// use blackjack::prelude::{ Generation, GameState };
    ///
    /// let mut generation: Generation = Default::default();
    ///
    /// generation.add_generation(Default::default());
    /// generation.add_generation(Default::default());
    ///
    /// assert_eq!(2, generation.number_of_branches());
    /// ```
    pub fn number_of_branches(&self) -> usize {
        self.children.len()
    }

    /// In some cases you may want to switch to a different path in history.  Using branches will
    /// allow access to all possible paths which could occur in history.
    ///
    /// Example:
    /// ```
    /// use blackjack::prelude::{ Generation, GameState };
    ///
    /// let mut generation: Generation = Default::default();
    ///
    /// generation.add_generation(Default::default());
    /// generation.add_generation(Default::default());
    ///
    /// assert_eq!(2, generation.branches().len());
    /// ```
    pub fn branches(&self) -> &Vec<Box<Generation>> {
        &self.children
    }

    /// Gets the current generation for the game state.
    ///
    /// Example:
    /// ```
    /// use blackjack::prelude::{ Generation, GameState, Progress };
    ///
    /// let mut generation: Generation = Default::default();
    ///
    /// let mut game_state: GameState = Default::default();
    /// game_state.progress = Progress::Done;
    ///
    /// generation.add_generation(Default::default());
    /// generation.add_generation(game_state);
    ///
    /// let current_branch = generation.current_branch().unwrap();
    /// assert_eq!(Progress::Done, current_branch.current_state().progress);
    /// ```
    pub fn current_branch(&self) -> Option<&Box<Generation>> {
        self.children
            .iter()
            .max_by(|a, b| a.timestamp.cmp(&b.timestamp))
    }

    /// Gets the current state for the game.  This will traverse the most current state and return
    /// that state.
    ///
    /// Example:
    /// ```
    /// use blackjack::prelude::{ Generation, GameState, Progress };
    ///
    /// let mut generation: Generation = Default::default();
    ///
    /// let mut game_state: GameState = Default::default();
    /// game_state.progress = Progress::Done;
    ///
    /// generation.add_generation(Default::default());
    /// generation.add_generation(game_state);
    ///
    /// let current_branch = generation.current_branch().unwrap();
    /// assert_eq!(Progress::Done, current_branch.current_state().progress);
    /// ```
    pub fn current_state(&self) -> &GameState {
        if let Some(branch) = self.current_branch() {
            branch.current_state()
        } else {
            &self.state
        }
    }
}

#[cfg(test)]
mod test {
    use super::Generation;
    use crate::game_state::GameState;

    #[test]
    fn can_add_a_generation() {
        let mut generation: Generation = Default::default();
        generation.add_generation(Default::default());
        assert_eq!(1, generation.number_of_branches());
    }
}
