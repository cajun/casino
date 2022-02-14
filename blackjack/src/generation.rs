use crate::game_state::GameState;
use std::time::SystemTime;

/// Generation will contain and maintain the history of the game state.  It will keep this history
/// in a tree structure.
#[derive(Debug)]
pub struct Generation {
    state: GameState,
    timestamp: SystemTime,
    children: Vec<Self>,
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
    pub fn new(state: GameState) -> Self {
        Generation {
            state,
            timestamp: SystemTime::now(),
            ..Default::default()
        }
    }

    /// append_generation will take in a new GameState and append it to the current list of states
    /// on this generation.  Right now this is a private method.  It is possible to create trees of
    /// generations using this method.
    ///
    /// * `state`: GameState to be added to this generation
    pub(super) fn append_generation(&mut self, state: GameState) {
        let generation = Generation::new(state);
        self.children.push(generation);
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
        let current = self.mut_current_generation();
        current.append_generation(state);
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
    /// assert_eq!(1, generation.number_of_branches());
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
    /// assert_eq!(1, generation.branches().len());
    /// ```
    pub fn branches(&self) -> &Vec<Generation> {
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
    pub fn current_branch(&self) -> Option<&Self> {
        self.children
            .iter()
            .max_by(|a, b| a.timestamp.cmp(&b.timestamp))
    }

    /// current generation will traverse all generations to discover the current generation.  When
    /// evaluating this will give the most up-to-date information on the game state.  If the
    /// history forks this method will still pull the most up-to-date generation until a new
    /// generation is created.
    ///
    /// Example:
    /// ```
    /// use blackjack::prelude::{ Generation, GameState, Progress };
    ///
    /// let mut generation: Generation = Default::default();
    ///
    /// let current = generation.current_generation();
    /// ```
    pub fn current_generation(&self) -> &Generation {
        if let Some(branch) = self.current_branch() {
            branch.current_generation()
        } else {
            self
        }
    }

    /// Return a mutable branch.  This will default to the current branch.
    fn mut_current_branch(&mut self) -> Option<&mut Self> {
        self.children
            .iter_mut()
            .max_by(|a, b| a.timestamp.cmp(&b.timestamp))
    }

    /// Return the current generation as mutable.
    fn mut_current_generation(&mut self) -> &mut Self {
        if self.children.is_empty() {
            return self;
        }

        self.mut_current_branch()
            .expect("If there are children then there must be a current branch")
            .mut_current_generation()
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
    use crate::game_state::{GameState, Progress};

    #[test]
    fn can_add_a_generation() {
        let mut generation: Generation = Default::default();
        generation.add_generation(Default::default());
        assert_eq!(1, generation.number_of_branches());
    }

    #[test]
    fn can_add_a_custom_game_state_to_a_generation() {
        let gs: GameState = Default::default();
        let generation = Generation::new(gs);

        assert_eq!(0, generation.number_of_branches());
        assert_eq!(Progress::Starting, generation.state.progress);
    }

    #[test]
    fn can_get_the_current_branch() {
        let mut generation: Generation = Default::default();

        assert_eq!(0, generation.number_of_branches());

        let gs = GameState {
            players: vec![Default::default()],
            ..Default::default()
        };

        generation.append_generation(gs);

        assert_eq!(1, generation.number_of_branches());

        let gs2 = GameState {
            players: vec![Default::default(), Default::default()],
            ..Default::default()
        };
        generation.append_generation(gs2);

        assert_eq!(2, generation.number_of_branches());

        let maybe = generation.current_branch();
        assert!(maybe.is_some());
        assert_eq!(2, maybe.unwrap().state.players.len());
    }
}
