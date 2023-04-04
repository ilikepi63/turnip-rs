use crate::db::models::{number_value::NumberValueType, string_value::StringTypeValue};
use serde::{Deserialize, Serialize};
use sqlparser::ast::{
    Expr, Ident, SetExpr,
    Statement::{self, Insert},
    Values,
};

use crate::db::data::TypeValue;

use super::errors::StatementError;

fn expr_to_value(i: &Expr) -> Option<TypeValue> {
    match i {
        Expr::Identifier(Ident { value, quote_style }) => match quote_style {
            Some(_) => Some(TypeValue::StringTypeValue(StringTypeValue {
                value: value.to_string(),
            })),
            // todo: Also make a place for boolean values here
            None => match value.parse::<f64>() {
                Ok(v) => Some(TypeValue::NumberValueType(NumberValueType { value: v })),
                Err(e) => {
                    eprintln!("Error with parsing float: {value} error: {:?}", e);
                    None
                }
            },
        },
        Expr::Value(value) => match value {
            sqlparser::ast::Value::Number(s, _) => match s.parse::<f64>() {
                Ok(v) => Some(TypeValue::NumberValueType(NumberValueType { value: v })),
                Err(e) => {
                    eprintln!("Error with parsing float: {value} error: {:?}", e);
                    None
                }
            },
            _ => None,
        },
        _ => None,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertQuery {
    pub table_name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Option<TypeValue>>>,
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
                    .map(|row| row.iter().map(|v| expr_to_value(v)).collect())
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
