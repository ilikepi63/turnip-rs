use sqlparser::parser::ParserError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StatementError {
    #[error("Could not correctly parse the query.")]
    ParserError(#[from] ParserError),

    #[error("No Statement could be found for the given query.")]
    StatementNotFoundError(),

    #[error("Not implemented")]
    NotImplementedError(),

    #[error("No `into` parameter specified for Select Query. Each Select needs to have an into parameter specified.")]
    NoIntoSpecifiedForSelect(),
}
