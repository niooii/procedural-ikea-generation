use thiserror::Error;

pub type Result<T> = std::result::Result<T, PigError>;

#[derive(Error, Debug)]
pub enum PigError {
    #[error("Tile add error.")]
    TileAddError { why: String }
}