use crate::{card::Card, deck::Deck, error::CardError, has_cards::HasCards};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone)]
pub struct Shoe {
    cards: Vec<Card>,
}

impl Shoe {
    /// NOTE: A deck is a standard deck without jokers.   It has four suits and Ace through King.
    pub fn new(number_of_decks: i32) -> Result<Shoe, CardError> {
        let mut cards = vec![];

        for _ in 0..number_of_decks {
            cards.append(&mut Deck::new()?.cards);
        }

        Ok(Shoe { cards })
    }
}

impl HasCards for Shoe {
    fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    fn cards_left(&self) -> usize {
        self.cards.len()
    }

    /// Shuffle the cards
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

impl Default for Shoe {
    fn default() -> Self {
        Shoe::new(7).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Shoe;
    use crate::has_cards::HasCards;

    #[test]
    fn check_shoe() {
        let maybe = Shoe::new(7);
        assert!(maybe.is_ok());
        let shoe = maybe.unwrap();
        assert_eq!(52 * 7, shoe.cards.len());
    }

    #[test]
    fn default_shoe() {
        let shoe: Shoe = Default::default();
        assert_eq!(52 * 7, shoe.cards.len());
    }

    #[test]
    fn shuffle_shoe() {
        let maybe = Shoe::new(7);
        assert!(maybe.is_ok());
        let mut shoe = maybe.unwrap();
        shoe.shuffle();

        assert_eq!(52 * 7, shoe.cards.len());
    }
}
