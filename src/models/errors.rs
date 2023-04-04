use sqlparser::{ast::Expr, parser::ParserError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
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

#[derive(Error, Debug, PartialEq)]
pub enum SelectQueryError {
    #[error("No Select query is present.")]
    IsNoneError(),

    #[error("Expression is not supported")]
    IsNotSupportedError(),
}
