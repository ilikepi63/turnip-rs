use serde::{Deserialize, Serialize};
use sqlparser::ast::{
    Expr, Ident, SetExpr,
    Statement::{self, Insert},
    Values,
};

use super::errors::StatementError;

fn expr_to_string(i: &Expr) -> String {
    match i {
        Expr::Identifier(Ident {
            value,
            quote_style: _,
        }) => value.clone(),
        _ => "".to_string(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertQuery {
    pub table_name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl TryFrom<&Statement> for InsertQuery {
    type Error = StatementError;

    fn try_from(value: &Statement) -> Result<Self, Self::Error> {
        if let Insert {
            or: _,
            into: _,
            table_name,
            after_columns: _,
            columns,
            source,
            overwrite: _,
            partitioned: _,
            table: _,
            on: _,
            returning: _,
        } = value
        {
            let rows = match &*source.body {
                SetExpr::Values(Values {
                    explicit_row: _,
                    rows,
                }) => Ok(rows
                    .iter()
                    .map(|row| row.iter().map(|v| expr_to_string(v)).collect())
                    .collect()),
                _ => Err(StatementError::NotImplementedError()),
            }?;

            let columns: Vec<String> = columns.iter().map(|i| i.value.clone()).collect();

            return Ok(InsertQuery {
                table_name: match table_name.0.first() {
                    Some(v) => Ok(v.value.clone()),
                    None => Err(StatementError::NotImplementedError()),
                }?,
                columns,
                rows,
            });
        } else {
            Err(StatementError::NotImplementedError())
        }
    }
}
