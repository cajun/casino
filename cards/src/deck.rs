use crate::{
    card::{Card, Suit},
    error::CardError,
    has_cards::HasCards,
};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, PartialEq)]
pub struct Deck {
    pub(crate) cards: Vec<Card>,
}

impl Deck {
    /// NOTE: A deck is a standard deck without jokers.   It has four suits and Ace through King.
    pub fn new() -> Result<Deck, CardError> {
        let mut cards = vec![];

        cards.append(&mut create_cards(Suit::Clubs)?);
        cards.append(&mut create_cards(Suit::Diamonds)?);
        cards.append(&mut create_cards(Suit::Hearts)?);
        cards.append(&mut create_cards(Suit::Spades)?);

        Ok(Deck { cards })
    }
}

impl HasCards for Deck {
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

impl Default for Deck {
    fn default() -> Self {
        Deck::new().unwrap()
    }
}

fn create_cards(suit: Suit) -> Result<Vec<Card>, CardError> {
    (1..=13)
        .into_iter()
        .map(|value| Card::new(value, suit))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::Deck;
    use crate::has_cards::HasCards;

    #[test]
    fn check_deck() {
        let maybe = Deck::new();
        assert!(maybe.is_ok());
        let deck = maybe.unwrap();
        assert_eq!(52, deck.cards.len());
    }

    #[test]
    fn shuffle_deck() {
        let maybe = Deck::new();
        assert!(maybe.is_ok());
        let mut deck = maybe.unwrap();
        deck.shuffle();

        assert_eq!(52, deck.cards.len());
    }
}
