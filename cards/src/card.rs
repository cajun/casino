use crate::error::CardError;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Suit {
    Clubs,
    Hearts,
    Spades,
    Diamonds,
}

#[derive(Debug, Copy, Clone)]
pub struct Card {
    value: i32,
    suit: Suit,
}

impl Card {
    /// Creating a new card
    ///
    /// * `value`: should be the value between 1 and 13
    /// * `suit`: should be a valid enum for a card
    pub fn new(value: i32, suit: Suit) -> Result<Card, CardError> {
        if !(1..=13).contains(&value) {
            return Err(CardError::ValueOutOfRange(value));
        }
        Ok(Card { value, suit })
    }

    /// face will return the string face value of the card.  This is a standard deck which will
    /// have an Ace, Jack, Queen, and King
    pub fn rank(&self) -> String {
        match self.value {
            1 => "Ace".to_owned(),
            11 => "Jack".to_owned(),
            12 => "Queen".to_owned(),
            13 => "King".to_owned(),
            _ => self.value.to_string(),
        }
    }

    /// face will return the string face value of the card.  This is a standard deck which will
    /// have an Ace, Jack, Queen, and King
    pub fn value(&self) -> i32 {
        match self.value {
            1 => 1,
            11 => 10,
            12 => 10,
            13 => 10,
            _ => self.value,
        }
    }

    /// show the suit for the card
    pub fn suit(&self) -> Suit {
        self.suit
    }
}

#[cfg(test)]
mod tests {
    use super::{Card, Suit};

    #[test]
    fn card_value() {
        let card = Card::new(1, Suit::Clubs).unwrap();
        assert_eq!(card.value, 1);
        assert_eq!(card.suit, Suit::Clubs);
    }

    #[test]
    fn card_out_of_range() {
        let card = Card::new(1, Suit::Clubs);
        assert!(card.is_ok());
        let card = Card::new(0, Suit::Clubs);
        assert!(card.is_err());
        let card = Card::new(13, Suit::Clubs);
        assert!(card.is_ok());
        let card = Card::new(14, Suit::Clubs);
        assert!(card.is_err());
    }

    #[test]
    fn card_rank() {
        let card = Card::new(1, Suit::Clubs).unwrap();
        assert_eq!(card.rank(), "Ace");
        let card = Card::new(5, Suit::Clubs).unwrap();
        assert_eq!(card.rank(), "5");
        let card = Card::new(11, Suit::Clubs).unwrap();
        assert_eq!(card.rank(), "Jack");
        let card = Card::new(12, Suit::Clubs).unwrap();
        assert_eq!(card.rank(), "Queen");
        let card = Card::new(13, Suit::Clubs).unwrap();
        assert_eq!(card.rank(), "King");
    }
}
