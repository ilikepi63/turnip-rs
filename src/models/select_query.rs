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
    #[serde(skip_deserializing, skip_serializing)]
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

// pub fn evaluate(
//     values: &HashMap<String, TypeValue>,
//     selection: Option<Expr>,
// ) -> Result<bool, SelectQueryError> {
//     match selection {
//         Some(expr) => {
//             match expr {
//                 Expr::BinaryOp { left, op, right } => {
//                     match op {
//                         // binaries
//                         BinaryOperator::Gt => {
//                             match (resolve(&left, values), resolve(&right, values)) {
//                                 (Ok(tv1), Ok(tv2)) => Ok(tv1 > tv2),
//                                 _ => Err(SelectQueryError::IsNotSupportedError()),
//                             }
//                         }
//                         BinaryOperator::Lt => {
//                             match (resolve(&left, values), resolve(&right, values)) {
//                                 (Ok(tv1), Ok(tv2)) => Ok(tv1 < tv2),
//                                 _ => Err(SelectQueryError::IsNotSupportedError()),
//                             }
//                         }
//                         BinaryOperator::GtEq => {
//                             match (resolve(&left, values), resolve(&right, values)) {
//                                 (Ok(tv1), Ok(tv2)) => Ok(tv1 >= tv2),
//                                 _ => Err(SelectQueryError::IsNotSupportedError()),
//                             }
//                         }
//                         BinaryOperator::LtEq => {
//                             match (resolve(&left, values), resolve(&right, values)) {
//                                 (Ok(tv1), Ok(tv2)) => Ok(tv1 <= tv2),
//                                 _ => Err(SelectQueryError::IsNotSupportedError()),
//                             }
//                         }
//                         BinaryOperator::Eq => {
//                             println!("EQUATING TWO VARIABLES, LEFT: {}, RIGHT: {} ", left, right);
//                             match (resolve(&left, values), resolve(&right, values)) {
//                                 (Ok(tv1), Ok(tv2)) => Ok(tv1 == tv2),
//                                 _ => Err(SelectQueryError::IsNotSupportedError()),
//                             }
//                         }
//                         BinaryOperator::NotEq => {
//                             match (resolve(&left, values), resolve(&right, values)) {
//                                 (Ok(tv1), Ok(tv2)) => Ok(tv1 != tv2),
//                                 _ => Err(SelectQueryError::IsNotSupportedError()),
//                             }
//                         }

//                         // compounds
//                         BinaryOperator::And => {
//                             println!(
//                                 "Doing the and operation: left: {:?}, right: {:?}",
//                                 left, right
//                             );
//                             let result = evaluate(values, Some(*right));

//                             println!("RESULT: {:?}", result);

//                             match (evaluate(values, Some(*left)), result) {
//                                 (Ok(v1), Ok(v2)) => Ok(v1 && v2),
//                                 _ => Ok(false),
//                             }
//                         }
//                         BinaryOperator::Or => match (
//                             evaluate(values, Some(*left)),
//                             evaluate(values, Some(*right)),
//                         ) {
//                             (Ok(v1), Ok(v2)) => Ok(v1 || v2),
//                             _ => Ok(false),
//                         },
//                         _ => Err(SelectQueryError::IsNotSupportedError()),
//                     }
//                 }
//                 _ => Err(SelectQueryError::IsNotSupportedError()),
//             }
//         }
//         None => Err(SelectQueryError::IsNoneError()),
//     }
// }

// pub fn resolve(
//     expr: &Expr,
//     values: &HashMap<String, TypeValue>,
// ) -> Result<TypeValue, SelectQueryError> {
//     match expr {
//         Expr::Identifier(i) => {
//             let value = values.get(&i.value);

//             match value {
//                 Some(v) => Ok(v.to_owned()),
//                 None => Err(SelectQueryError::IsNotSupportedError()),
//             }
//         }
//         Expr::Value(value) => match TypeValue::try_from(value) {
//             Ok(r) => Ok(r),
//             Err(err) => Err(SelectQueryError::IsNotSupportedError()),
//         },
//         _ => Err(SelectQueryError::IsNotSupportedError()),
//     }
// }

// // select * into customer_cache from customer where x = 1 and u = 2 and y = 3;

// #[cfg(test)]
// mod tests {
//     use crate::db::models::number_value::NumberValueType;

//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;

//     #[test]
//     fn evaluate_statement() {
//         let sql = "select * into customer_cache from customer where x = 1 and u = 2 and y = 3;";

//         let dialect = GenericDialect {};

//         let ast = Parser::parse_sql(&dialect, sql).expect("Error with parsing the sql");

//         for statement in ast.iter() {
//             match statement {
//                 Query(query) => {
//                     let select_query = SelectQuery::try_from(&*query.body);

//                     let values = HashMap::from([
//                         (
//                             "x".to_string(),
//                             TypeValue::NumberValueType(NumberValueType { value: 1.0 }),
//                         ),
//                         (
//                             "u".to_string(),
//                             TypeValue::NumberValueType(NumberValueType { value: 2.0 }),
//                         ),
//                         (
//                             "y".to_string(),
//                             TypeValue::NumberValueType(NumberValueType { value: 3.0 }),
//                         ),
//                     ]);

//                     match select_query {
//                         Ok(select) => assert_eq!(evaluate(&values, select.constraints), Ok(true)),
//                         Err(e) => {
//                             eprintln!("Error with getting the Statement: {:?}", e);
//                             panic!("No Select Statement found.")
//                         }
//                     }
//                 }
//                 _ => panic!("No Select Statement found."),
//             }
//         }
//     }

//     #[test]
//     fn evaluate_statement_false() {
//         let sql = "select * into customer_cache from customer where x = 1 and u = 2  and y = 3;";

//         let dialect = GenericDialect {};

//         let ast = Parser::parse_sql(&dialect, sql).expect("Error with parsing the sql");

//         for statement in ast.iter() {
//             match statement {
//                 Query(query) => {
//                     let select_query = SelectQuery::try_from(&*query.body);

//                     let values = HashMap::from([
//                         (
//                             "x".to_string(),
//                             TypeValue::NumberValueType(NumberValueType { value: 1.0 }),
//                         ),
//                         (
//                             "u".to_string(),
//                             TypeValue::NumberValueType(NumberValueType { value: 2.0 }),
//                         ),
//                     ]);

//                     match select_query {
//                         Ok(select) => assert_eq!(evaluate(&values, select.constraints), Ok(false)),
//                         Err(e) => {
//                             eprintln!("Error with getting the Statement: {:?}", e);
//                             panic!("No Select Statement found.")
//                         }
//                     }
//                 }
//                 _ => panic!("No Select Statement found."),
//             }
//         }
//     }

//     #[test]
//     fn evaluate_statement_comparison() {
//         let sql = "select * into customer_cache from customer where x = 1 and u = 2  and y > 3;";

//         let dialect = GenericDialect {};

//         let ast = Parser::parse_sql(&dialect, sql).expect("Error with parsing the sql");

//         for statement in ast.iter() {
//             match statement {
//                 Query(query) => {
//                     let select_query = SelectQuery::try_from(&*query.body);

//                     let values = HashMap::from([
//                         (
//                             "x".to_string(),
//                             TypeValue::NumberValueType(NumberValueType { value: 1.0 }),
//                         ),
//                         (
//                             "u".to_string(),
//                             TypeValue::NumberValueType(NumberValueType { value: 2.0 }),
//                         ),
//                         (
//                             "y".to_string(),
//                             TypeValue::NumberValueType(NumberValueType { value: 4.0 }),
//                         ),
//                     ]);

//                     match select_query {
//                         Ok(select) => assert_eq!(evaluate(&values, select.constraints), Ok(true)),
//                         Err(e) => {
//                             eprintln!("Error with getting the Statement: {:?}", e);
//                             panic!("No Select Statement found.")
//                         }
//                     }
//                 }
//                 _ => panic!("No Select Statement found."),
//             }
//         }
//     }

//     #[test]
//     fn evaluate_statement_comparison_false() {
//         let sql = "select * into customer_cache from customer where x = 1 and u = 2  and y > 3;";

//         let dialect = GenericDialect {};

//         let ast = Parser::parse_sql(&dialect, sql).expect("Error with parsing the sql");

//         for statement in ast.iter() {
//             match statement {
//                 Query(query) => {
//                     let select_query = SelectQuery::try_from(&*query.body);

//                     let values = HashMap::from([
//                         (
//                             "x".to_string(),
//                             TypeValue::NumberValueType(NumberValueType { value: 1.0 }),
//                         ),
//                         (
//                             "u".to_string(),
//                             TypeValue::NumberValueType(NumberValueType { value: 2.0 }),
//                         ),
//                         (
//                             "y".to_string(),
//                             TypeValue::NumberValueType(NumberValueType { value: 2.0 }),
//                         ),
//                     ]);

//                     match select_query {
//                         Ok(select) => assert_eq!(evaluate(&values, select.constraints), Ok(false)),
//                         Err(e) => {
//                             eprintln!("Error with getting the Statement: {:?}", e);
//                             panic!("No Select Statement found.")
//                         }
//                     }
//                 }
//                 _ => panic!("No Select Statement found."),
//             }
//         }
//     }
// }
