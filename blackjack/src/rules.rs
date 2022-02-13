use crate::{
    error::RuleError,
    game_state::{GameState, Progress},
};

struct Rules {
    game_state: Vec<GameState>,
}

impl Default for Rules {
    fn default() -> Self {
        Self {
            game_state: vec![Default::default()],
        }
    }
}

impl Rules {
    /// add_player will add a new player to the table.   The current state must be in starting for
    /// this action to be done.
    fn add_player(&mut self) -> Result<(), RuleError> {
        if !self.is_starting() {
            return Err(RuleError::InvalidState(self.current_progress()));
        }

        let mut gs = self
            .current_state()
            .expect("Should always have game state")
            .clone();

        gs.players.push(Default::default());
        self.game_state.push(gs);
        Ok(())
    }

    /// Change the state from starting to playing.   This should only occur when the game state is
    /// in the starting state.
    fn start_playing(&mut self) -> Result<(), RuleError> {
        if !self.is_starting() {
            return Err(RuleError::InvalidState(self.current_progress()));
        }

        let mut gs = self
            .current_state()
            .expect("Should always have game state")
            .clone();

        gs.progress = Progress::Playing;
        self.game_state.push(gs);
        Ok(())
    }

    fn done_playing(&mut self) -> Result<(), RuleError> {
        if !self.is_playing() {
            return Err(RuleError::InvalidState(self.current_progress()));
        }

        let mut gs = self
            .current_state()
            .expect("Should always have game state")
            .clone();

        gs.progress = Progress::Done;
        self.game_state.push(gs);
        Ok(())
    }

    /// Check the the current progress of the blackjack game.
    fn current_progress(&self) -> Progress {
        self.current_state()
            .map_or(Progress::Starting, |x| x.progress.clone())
    }

    /// is_starting is a check to determine if the game is in the starting state.
    fn is_starting(&self) -> bool {
        self.current_progress() == Progress::Starting
    }

    /// is_playing is a check to determine if the game is in the playing state.
    fn is_playing(&self) -> bool {
        self.current_progress() == Progress::Playing
    }

    /// is_done is a check to determine if the game is in the done state.
    fn is_done(&self) -> bool {
        self.current_progress() == Progress::Done
    }

    /// current_state pull the current state of the game.
    fn current_state(&self) -> Option<&GameState> {
        self.game_state.last()
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
        assert_eq!(1, rules.game_state.len())
    }

    #[test]
    fn default_games_are_in_the_starting_state() {
        let rules: Rules = Default::default();
        assert_eq!(Progress::Starting, rules.current_progress())
    }

    #[test]
    fn games_should_move_to_playing_state() {
        let mut rules: Rules = Default::default();
        let maybe = rules.start_playing();
        assert!(maybe.is_ok());
        assert_eq!(Progress::Playing, rules.current_progress())
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
        assert_eq!(Progress::Done, rules.current_progress())
    }

    #[test]
    fn adding_player_to_rules() {
        let mut rules: Rules = Default::default();
        rules.add_player().unwrap();
        assert_eq!(2, rules.game_state.len());
        assert_eq!(1, rules.current_state().unwrap().players.len());
    }
}
