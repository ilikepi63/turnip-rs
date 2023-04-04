use crate::db::data::TypeValue;
use serde::{Deserialize, Serialize};
use sqlparser::ast::Statement::{Insert, Query};
use sqlparser::ast::{
    BinaryOperator, Expr, SelectItem,
    SetExpr::{self, Select},
    TableFactor,
};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::collections::HashMap;

use super::errors::{SelectQueryError, StatementError};
use super::expression::Expression;

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectQuery {
    pub into: String,
    pub projection: Vec<String>,
    pub from: String,
    pub constraints: Option<Expression>,
}

impl TryFrom<&SetExpr> for SelectQuery {
    type Error = StatementError;

    fn try_from(value: &SetExpr) -> Result<Self, Self::Error> {
        if let Select(select) = value {
            println!("Selection:{:#?}", select.selection);

            let into = match match &select.into {
                Some(v) => Ok(v),
                None => Err(StatementError::NoIntoSpecifiedForSelect()),
            }?
            .name
            .0
            .first()
            {
                Some(v) => Ok(v.value.clone()),
                None => Err(StatementError::NoIntoSpecifiedForSelect()),
            }?;

            let projection: Vec<String> = select
                .projection
                .iter()
                .map(|v| match v {
                    SelectItem::UnnamedExpr(expr) => match expr {
                        sqlparser::ast::Expr::Identifier(ident) => ident.value.clone(),
                        _ => "".to_string(),
                    },
                    _ => "*".to_string(),
                })
                .collect();

            let from = match select.from.first() {
                Some(v) => match &v.relation {
                    TableFactor::Table {
                        name,
                        alias: _,
                        args: _,
                        with_hints: _,
                    } => match name.0.first() {
                        Some(v) => Ok(v.value.clone()),
                        None => Err(StatementError::NotImplementedError()),
                    },
                    _ => Err(StatementError::NotImplementedError()),
                },
                None => Err(StatementError::NotImplementedError()),
            }?;

            let constraints = match &select.selection {
                Some(r) => match Expression::try_from(r){
                    Ok(result) => Some(result),
                    Err(_) => None
                },
                None => None,
            };

            return Ok(SelectQuery {
                into,
                projection: projection,
                from,
                constraints,
            });
        } else {
            return Err(StatementError::NotImplementedError());
        }
    }
}
