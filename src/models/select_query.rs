use sqlparser::ast::{
    Query, SelectItem,
    SetExpr::{self, Select},
    Statement, TableFactor, TableWithJoins,
};

use super::errors::StatementError;

#[derive(Debug)]
pub struct SelectQuery {
    pub into: String,
    pub projection: Vec<SelectItem>,
    pub from: String,
}

impl TryFrom<&SetExpr> for SelectQuery {
    type Error = StatementError;

    fn try_from(value: &SetExpr) -> Result<Self, Self::Error> {
        if let Select(select) = value {
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

            let projection = &select.projection;

            let from = match select.from.first() {
                Some(v) => match &v.relation {
                    TableFactor::Table {
                        name,
                        alias,
                        args,
                        with_hints,
                    } => match name.0.first() {
                        Some(v) => Ok(v.value.clone()),
                        None => Err(StatementError::NotImplementedError()),
                    },
                    _ => Err(StatementError::NotImplementedError()),
                },
                None => Err(StatementError::NotImplementedError()),
            }?;

            return Ok(SelectQuery {
                into,
                projection: projection.to_vec(),
                from,
            });
        } else {
            return Err(StatementError::NotImplementedError());
        }
    }
}
