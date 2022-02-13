use crate::{
    error::RuleError,
    game_state::{GameState, Progress},
    generation::Generation,
};

/// Rules will be the hub for blackjack.  In the future Traits "might" be broken out from this impl
/// , but I'm not sure at the momentA.
pub struct Rules {
    generation: Generation,
}

/// A default rule will have the game in the starting state
impl Default for Rules {
    fn default() -> Self {
        Self {
            generation: Generation::new(Default::default()),
        }
    }
}

impl Rules {
    /// add_player will add a new player to the table.   The current state must be in starting for
    /// this action to be done.
    ///
    /// Example:
    /// ```
    /// use blackjack::prelude::Rules;
    ///
    /// let mut rule: Rules = Default::default();
    /// rule.add_player();
    /// rule.add_player();
    ///
    /// assert_eq!(2, rule.current_state().players.len());
    /// ```
    pub fn add_player(&mut self) -> Result<(), RuleError> {
        if !self.is_starting() {
            return Err(RuleError::InvalidState(self.current_progress()));
        }

        let mut gs = self.current_state().clone();

        gs.players.push(Default::default());
        self.generation.add_generation(gs);
        Ok(())
    }

    /// Change the state from starting to playing.   This should only occur when the game state is
    /// in the starting state.
    ///
    /// Example:
    /// ```
    /// use blackjack::prelude::{ Progress, Rules };
    ///
    /// let mut rule: Rules = Default::default();
    /// rule.add_player();
    /// rule.add_player();
    /// assert!(rule.start_playing().is_ok());
    ///
    /// assert_eq!(&Progress::Playing, rule.current_progress());
    /// ```
    pub fn start_playing(&mut self) -> Result<(), RuleError> {
        if !self.is_starting() {
            return Err(RuleError::InvalidState(self.current_progress()));
        }

        let mut gs = self.current_state().clone();

        gs.progress = Progress::Playing;
        self.generation.add_generation(gs);
        Ok(())
    }

    /// This will mark the game as done playing
    ///
    /// Example:
    /// ```
    /// use blackjack::prelude::{ Progress, Rules };
    ///
    /// let mut rule: Rules = Default::default();
    /// rule.add_player();
    /// rule.add_player();
    /// assert!(rule.start_playing().is_ok());
    /// assert!(rule.done_playing().is_ok());
    ///
    /// assert_eq!(&Progress::Done, rule.current_progress());
    /// ```
    pub fn done_playing(&mut self) -> Result<(), RuleError> {
        if !self.is_playing() {
            return Err(RuleError::InvalidState(self.current_progress()));
        }

        let mut gs = self.current_state().clone();

        gs.progress = Progress::Done;
        self.generation.add_generation(gs);
        Ok(())
    }

    /// This will create a new game, but only after the current game is done
    ///
    /// Example:
    /// ```
    /// use blackjack::prelude::{ Progress, Rules };
    ///
    /// let mut rule: Rules = Default::default();
    /// rule.add_player();
    /// rule.add_player();
    /// assert!(rule.start_playing().is_ok());
    /// assert!(rule.done_playing().is_ok());
    /// assert!(rule.new_game().is_ok());
    ///
    /// assert_eq!(&Progress::Starting, rule.current_progress());
    /// ```
    pub fn new_game(&mut self) -> Result<(), RuleError> {
        if !self.is_done() {
            return Err(RuleError::InvalidState(self.current_progress()));
        }

        let mut gs = self.current_state().clone();

        gs.progress = Progress::Starting;
        self.generation.add_generation(gs);
        Ok(())
    }

    /// Check the current progress of the blackjack game.
    ///
    /// Example:
    /// ```
    /// use blackjack::prelude::{ Progress, Rules };
    ///
    /// let rule: Rules = Default::default();
    ///
    /// assert_eq!(&Progress::Starting, rule.current_progress());
    pub fn current_progress(&self) -> &Progress {
        &self.current_state().progress
    }

    /// is_starting is a check to determine if the game is in the starting state.
    /// Check the current progress of the blackjack game.
    ///
    /// Example:
    /// ```
    /// use blackjack::prelude::Rules;
    ///
    /// let rule: Rules = Default::default();
    ///
    /// assert_eq!(true, rule.is_starting());
    /// assert_eq!(false, rule.is_playing());
    /// assert_eq!(false, rule.is_done());
    pub fn is_starting(&self) -> bool {
        self.current_progress() == &Progress::Starting
    }

    /// is_playing is a check to determine if the game is in the playing state.
    ///
    /// Example:
    /// ```
    /// use blackjack::prelude::Rules;
    ///
    /// let mut rule: Rules = Default::default();
    /// assert!(rule.start_playing().is_ok());
    ///
    /// assert_eq!(false, rule.is_starting());
    /// assert_eq!(true, rule.is_playing());
    /// assert_eq!(false, rule.is_done());
    pub fn is_playing(&self) -> bool {
        self.current_progress() == &Progress::Playing
    }

    /// is_done is a check to determine if the game is in the done state.
    ///
    /// Example:
    /// ```
    /// use blackjack::prelude::Rules;
    ///
    /// let mut rule: Rules = Default::default();
    /// assert!(rule.start_playing().is_ok());
    /// assert!(rule.done_playing().is_ok());
    ///
    /// assert_eq!(false, rule.is_starting());
    /// assert_eq!(false, rule.is_playing());
    /// assert_eq!(true, rule.is_done());
    pub fn is_done(&self) -> bool {
        self.current_progress() == &Progress::Done
    }

    /// current_state pull the current state of the game.
    /// Example:
    /// ```
    /// use blackjack::prelude::{ Progress, Rules };
    ///
    /// let mut rule: Rules = Default::default();
    /// let game_state = rule.current_state();
    ///
    /// assert!(game_state.players.is_empty());
    /// assert_eq!(Progress::Starting, game_state.progress);
    ///
    pub fn current_state(&self) -> &GameState {
        self.generation.current_state()
    }
}

impl Iterator for Rules {
    type Item = GameState;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Rules;
    use crate::game_state::Progress;

    #[test]
    fn default_rules() {
        let rules: Rules = Default::default();
        assert_eq!(0, rules.generation.branches().len())
    }

    #[test]
    fn default_games_are_in_the_starting_state() {
        let rules: Rules = Default::default();
        assert_eq!(&Progress::Starting, rules.current_progress())
    }

    #[test]
    fn games_should_move_to_playing_state() {
        let mut rules: Rules = Default::default();
        let maybe = rules.start_playing();
        assert!(maybe.is_ok());
        assert_eq!(&Progress::Playing, rules.current_progress())
    }

    #[test]
    fn games_should_move_to_done_state() {
        let mut rules: Rules = Default::default();
        let maybe_fail = rules.done_playing();
        assert!(maybe_fail.is_err());
        let maybe_ok = rules.start_playing();
        assert!(maybe_ok.is_ok());
        let maybe = rules.done_playing();
        assert!(maybe.is_ok());
        assert_eq!(&Progress::Done, rules.current_progress())
    }

    #[test]
    fn adding_player_to_rules() {
        let mut rules: Rules = Default::default();
        rules.add_player().unwrap();
        assert_eq!(1, rules.generation.branches().len());
        assert_eq!(1, rules.current_state().players.len());
    }
}
