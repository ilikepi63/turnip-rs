// this is going to be a Read-Eval-Print-Loop for turnip, which will work by putting in

use sqlparser::parser::Parser;
use sqlparser::{
    ast::{
        SetExpr::Select,
        Statement::{Insert, Query},
    },
    dialect::GenericDialect,
};
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    while let Some(Ok(line)) = iterator.next() {
        let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...

        let ast = Parser::parse_sql(&dialect, &line).unwrap_or_else(|_| {
            println!("Well this failed miserably.");
            vec![]
        });

        // println!("\nAST: {:?}", ast);

        for statement in ast.iter() {
            match statement {
                Query(query) => {
                    match &*query.body {
                        Select(select) => {
                            // println!("Projection: {:?}", select.projection);

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
                    println!("\n{:?}", after_columns);
                    println!("\n{:?}", columns);
                    println!("\n{:?}", source);
                    println!("\n{:?} ", partitioned);
                }
                _ => {
                    println!("Found something else");
                }
            }
        }
    }

    Ok(())
}
