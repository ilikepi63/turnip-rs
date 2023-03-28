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

        // println!("\nAST: {:?}", ast);

        for statement in ast.iter() {
            match statement {
                Query(query) => {
                    match &*query.body {
                        Select(select) => {
                            println!("Into: {:?}", select.into);

                            for projection in select.projection.iter() {
                                println!("{:?}", projection);
                            }

                            for from in select.from.iter() {
                                println!("{:?}", from);
                            }

                            // println!("from: {:?}", select.from);
                        }

                        _ => {
                            println!("Nothing to see here");
                        }
                    }
                }
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
                } => {
                    println!("{:?}", table_name);
                    // println!("\n{:?}", after_columns);
                    println!("\n{:?}", columns);
                    println!("\n{:?}", source);
                    // println!("\n{:?} ", partitioned);
                }
                _ => {
                    println!("Found something else");
                }
            }
        }
    }

    Ok(())
}
