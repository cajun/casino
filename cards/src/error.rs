use thiserror::Error;

#[derive(Error, Debug)]
pub enum CardError {
    #[error("The value of {0} is out of range")]
    ValueOutOfRange(i32),
}
