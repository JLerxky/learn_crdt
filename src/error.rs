use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrdtError {
    #[error("unknown error")]
    Unknown,
}
