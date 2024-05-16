use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, Clone, Copy)]
pub enum Error {
    #[error("duplicated vector query")]
    DuplicatedVectorQuery,
    #[error("duplicated param")]
    DuplicatedParam,
    #[error("empty query body")]
    EmptyQueryBody,
}
