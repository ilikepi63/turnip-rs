use sqlparser::ast::Expr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Could not insert the record")]
    InsertError(),
}

#[derive(Error, Debug)]
pub enum ValueParseError {
    #[error("No Select query is present.")]
    IsNoneError(),

    #[error("Expression is not supported")]
    IsNotSupportedError(Expr, String),
}
