// this is going to be a Read-Eval-Print-Loop for turnip, which will work by putting in
use models::insert_query::InsertQuery;
use models::select_query::SelectQuery;
use runtime::TurnipRuntime;

use postcard::to_vec;
use sqlparser::parser::Parser;
use sqlparser::{
    ast::Statement::{Insert, Query},
    dialect::GenericDialect,
};
use std::io::{self, BufRead};

mod models;
mod runtime;
mod server;
mod db;

#[tokio::main]
async fn main() -> io::Result<()> {
    let stdin = io::stdin();

    // this is the command line
    while let Some(Ok(line)) = stdin.lock().lines().next() {
        let dialect = GenericDialect {};

        let ast = Parser::parse_sql(&dialect, &line).unwrap_or_else(|e| {
            eprintln!("{:?}", e);
            vec![]
        });

        for statement in ast.iter() {
            match statement {
                Query(query) => {
                    let select_query = SelectQuery::try_from(&*query.body);

                        match select_query {
                            Ok(select) => {

                                println!("{:?}",select);
                         
                            },
                            Err(e) => {
                                eprintln!("Error with getting the Statement: {:?}", e);
                            }
                        }
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
                    // with insert, we are only interested in sharing data with known nodes that are interested in it.
                }
                _ => {
                    println!("Found something else");
                }
            }
        }
    }

    Ok(())
}
