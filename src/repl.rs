use db::data::Db;
// this is going to be a Read-Eval-Print-Loop for turnip, which will work by putting in
use models::insert_query::InsertQuery;
use models::select_query::SelectQuery;
use runtime::TurnipRuntime;

use postcard::{from_bytes, to_vec};
use sqlparser::parser::Parser;
use sqlparser::{
    ast::Statement::{Insert, Query},
    dialect::GenericDialect,
};
use std::collections::HashMap;
use std::io::{self, BufRead};

mod db;
mod models;
mod runtime;
mod server;

#[tokio::main]
async fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut db = Db::new();

    // the runtime
    let mut runtime = TurnipRuntime::new("8080");

    runtime.run();

    let messenger = runtime
        .get_messenger()
        .expect("Could not get the messenger from the runtime");

    if let Ok(mut receiver) = runtime.get_receiver() {
        println!("Creating the receiver");

        tokio::spawn(async move {
            while let Ok(msg) = receiver.recv().await {
                let out: SelectQuery = match from_bytes(&msg) {
                    Ok(v) => v,
                    Err(e) => return,
                };

                println!("here: {:?}", out);

                // if let Ok(res) = String::from_utf8(msg) {
                //     let res = res.trim_matches(char::from(0));

                //     println!("{}: {:?}", res.len(), res);
                // }
            }
        });
    };

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
                        Ok(select) => match to_vec::<_, 32>(&select) {
                            Ok(result) => {
                                let cloned_messenger = messenger.clone();
                                tokio::spawn(async move {
                                    cloned_messenger.write_all(result.to_vec()).await
                                });
                            }
                            Err(e) => {
                                eprintln!("An Error ocurred trying to serialize data: {:?}", e);
                            }
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

                    if let Ok(query) = insert_query {
                        db.insert(query);
                    }
                }
                _ => {
                    println!("Found something else");
                }
            }
        }
    }

    Ok(())
}
