use cards::prelude::Card;

#[derive(Debug, Default, Clone)]
pub struct Hand {
    cards: Vec<Card>,
}

#[derive(Debug, Default, Clone)]
pub struct Player {
    pub hand: Hand,
}

#[derive(Debug, Default, Clone)]
pub struct House {
    pub hand: Hand,
}

/// Many things can Handle cards.  This trait will allow common ways to handle cardv
pub trait HandleCards {
    fn cards(&self) -> &Vec<Card>;
    fn number_of_cards(&self) -> usize;
    fn recieve(&mut self, card: Card);
    fn show_card(&self) -> Option<&Card>;
    fn show_hand(&self) -> &Vec<Card>;
    fn trash(&mut self) -> Vec<Card>;
    fn trash_card(&mut self) -> Option<Card>;
}

/// A Hand knows how to handle cards.
///
/// # Example
/// ```
/// use cards::prelude::{ Card, Suit };
/// use player::{ Hand, HandleCards };
///
/// let mut hand: Hand = Default::default();
/// hand.recieve(Card::new(1,Suit::Clubs).unwrap());
/// assert!(hand.show_card().is_some());
/// assert_eq!(hand.show_hand().len(), 1);
/// assert!(hand.trash_card().is_some());
/// assert_eq!(hand.show_hand().len(), 0);
/// ```
impl HandleCards for Hand {
    fn cards(&self) -> &Vec<Card> {
        &self.cards
    }

    fn number_of_cards(&self) -> usize {
        self.cards.len()
    }

    fn recieve(&mut self, card: Card) {
        self.cards.push(card)
    }

    fn show_card(&self) -> Option<&Card> {
        self.cards.first()
    }

    fn show_hand(&self) -> &Vec<Card> {
        self.cards()
    }

    fn trash(&mut self) -> Vec<Card> {
        let trashing = self.cards.to_vec();
        self.cards.clear();
        trashing
    }

    fn trash_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::{Hand, HandleCards};
    use cards::prelude::{Card, Suit};

    #[test]
    fn hand_has_cards() {
        let mut hand: Hand = Default::default();
        assert_eq!(hand.number_of_cards(), 0);
        hand.recieve(Card::new(1, Suit::Clubs).unwrap());
        hand.recieve(Card::new(1, Suit::Diamonds).unwrap());
        assert_eq!(hand.number_of_cards(), 2);
        assert_eq!(hand.cards().len(), 2);
        assert!(hand.show_card().is_some());
        assert!(hand.show_card().is_some());
        assert_eq!(hand.show_card().unwrap().rank(), "Ace");
        assert_eq!(hand.show_card().unwrap().value(), 1);
    }
}
