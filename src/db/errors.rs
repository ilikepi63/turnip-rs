use sqlparser::parser::ParserError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Could not insert the record")]
    InsertError(),
}
