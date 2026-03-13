use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("Couldn't parse string: {0}")]
    ParseError(String),
}
