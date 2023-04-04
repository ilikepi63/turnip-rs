
use serde::{Deserialize, Serialize};

use sqlparser::ast::{
    SelectItem,
    SetExpr::{self, Select},
    TableFactor,
};




use super::errors::{StatementError};
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

            Ok(SelectQuery {
                into,
                projection,
                from,
                constraints,
            })
        } else {
            Err(StatementError::NotImplementedError())
        }
    }
}
