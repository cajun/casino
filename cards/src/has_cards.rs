use crate::card::Card;

pub trait HasCards {
    fn deal(&mut self) -> Option<Card>;
    fn cards_left(&self) -> usize;

    /// Shuffle the shoe
    fn shuffle(&mut self);
}
