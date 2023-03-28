// this is going to be a Read-Eval-Print-Loop for turnip, which will work by putting in

use models::errors::StatementError;
use models::insert_query::InsertQuery;
use models::select_query::SelectQuery;
use models::statement::Statement;
use sqlparser::parser::Parser;
use sqlparser::{
    ast::{
        SetExpr::Select,
        Statement::{Insert, Query},
    },
    dialect::GenericDialect,
};
use std::error::Error;
use std::io::{self, BufRead, Write};

use crate::models::insert_query;

mod models;

pub fn string_to_query(q: String) -> Result<Statement, StatementError> {
    let dialect = GenericDialect {};

    let ast = Parser::parse_sql(&dialect, &q)?;

    let statement = ast.first();

    if statement.is_none() {
        return Err(StatementError::StatementNotFoundError());
    }

    let unwrapped_statement = statement.unwrap();

    let result = match unwrapped_statement {
        Query(query) => match &*query.body {
            select => Ok(Statement::Select(SelectQuery::try_from(select)?)),
            _ => Err(StatementError::NotImplementedError()),
        },
        Insert {
            or,
            into,
            table_name,
            after_columns,
            columns,
            source,
            overwrite,
            partitioned,
            table,
            on,
            returning,
        } => Ok(Statement::Insert(InsertQuery::try_from(
            unwrapped_statement,
        )?)),
        _ => Err(StatementError::NotImplementedError()),
    };

    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    // let mut stdout = io::stdout();
    let mut iterator = stdin.lock().lines();

    while let Some(Ok(line)) = iterator.next() {
        let dialect = GenericDialect {};

        let ast = Parser::parse_sql(&dialect, &line).unwrap_or_else(|e| {
            eprintln!("{:?}", e);
            vec![]
        });

        for statement in ast.iter() {
            match statement {
                Query(query) => {
                    let select_query = SelectQuery::try_from(&*query.body);
                    println!("Select!: {:?}", select_query)
                }
                Insert {
                    or: _,
                    into: _,
                    table_name: _,
                    after_columns: _,
                    columns: _,
                    source: _,
                    overwrite: _,
                    partitioned: _,
                    table: _,
                    on: _,
                    returning: _,
                } => {
                    let insert_query = InsertQuery::try_from(statement);

                    println!("Insert! {:?}", insert_query);
                }
                _ => {
                    println!("Found something else");
                }
            }
        }
    }

    Ok(())
}
