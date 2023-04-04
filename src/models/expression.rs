use crate::db::data::TypeValue;
use crate::db::models::number_value::NumberValueType;
use crate::db::models::string_value::StringTypeValue;

use serde::{Deserialize, Serialize};

use sqlparser::ast::{
    BinaryOperator, Expr,
    Value,
};


use std::collections::HashMap;
use std::convert::TryFrom;
use thiserror::Error;

// use sqlparser::tokenizer::Token::{Number, SingleQuotedString, SingleQuotedByteStringLiteral, };

// conversion error
#[derive(Error, Debug, PartialEq)]
pub enum ExpressionConversionError {
    #[error("Conversion Error")]
    StandardError(),
}

#[derive(Error, Debug, PartialEq)]
pub enum ExpressionEvaluationError {
    #[error("Evaluation Error")]
    StandardError(),
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum Expression {
    BinaryOp(Box<Expression>, Box<Expression>, ExpressionBinaryOperator),
    Value(ExpressionValue),
    Identifier(ExpressionIdentifier),
}

pub fn compare(
    left: &Expression,
    right: &Expression,
    eval: fn(TypeValue, TypeValue) -> bool,
    values: &HashMap<String, TypeValue>,
) -> Result<bool, ExpressionEvaluationError> {
    match (left.resolve(values), right.resolve(values)) {
        (Ok(tv1), Ok(tv2)) => {
            println!("Comparing!: {:?} {:?}", tv1, tv2);   
            Ok(eval(tv1, tv2))
        },
        _ => Err(ExpressionEvaluationError::StandardError()),
    }
}

impl Expression {
    pub fn evaluate(
        &self,
        values: &HashMap<String, TypeValue>,
    ) -> Result<bool, ExpressionEvaluationError> {
        match self {
            Expression::BinaryOp(left, right, op) => {
                match op {
                    ExpressionBinaryOperator::Gt => {
                        compare(left, right, |tv1, tv2| tv1 > tv2, values)
                    }
                    ExpressionBinaryOperator::Lt => {
                        compare(left, right, |tv1, tv2| tv1 < tv2, values)
                    }
                    ExpressionBinaryOperator::GtEq => {
                        compare(left, right, |tv1, tv2| tv1 >= tv2, values)
                    }
                    ExpressionBinaryOperator::LtEq => {
                        compare(left, right, |tv1, tv2| tv1 <= tv2, values)
                    }
                    ExpressionBinaryOperator::Eq => {
                        compare(left, right, |tv1, tv2| tv1 == tv2, values)
                    }
                    ExpressionBinaryOperator::NotEq => {
                        compare(left, right, |tv1, tv2| tv1 != tv2, values)
                    }

                    // compounds
                    ExpressionBinaryOperator::And => {
                        let result = right.evaluate(values);

                        match (left.evaluate(values), result) {
                            (Ok(v1), Ok(v2)) => Ok(v1 && v2),
                            _ => Ok(false),
                        }
                    }
                    ExpressionBinaryOperator::Or => {
                        match (left.evaluate(values), right.evaluate(values)) {
                            (Ok(v1), Ok(v2)) => Ok(v1 || v2),
                            _ => Ok(false),
                        }
                    }
                    _ => Err(ExpressionEvaluationError::StandardError()),
                }
            }
            _ => Err(ExpressionEvaluationError::StandardError()),
        }
    }

    pub fn resolve(
        &self,
        values: &HashMap<String, TypeValue>,
    ) -> Result<TypeValue, ExpressionEvaluationError> {
        match self {
            Expression::Identifier(i) => {
                let value = values.get(&i.value);

                match value {
                    Some(v) => Ok(v.to_owned()),
                    None => Err(ExpressionEvaluationError::StandardError()),
                }
            }
            Expression::Value(value) => match ((*value).clone()).try_into() {
                Ok(r) => Ok(r),
                Err(_) => Err(ExpressionEvaluationError::StandardError()),
            },
            _ => Err(ExpressionEvaluationError::StandardError()),
        }
    }
}

impl TryFrom<&Expr> for Expression {
    type Error = ExpressionConversionError;

    fn try_from(expr: &Expr) -> Result<Self, Self::Error> {
        Ok(match expr {
            Expr::BinaryOp { left, op, right } => Expression::BinaryOp(
                match Expression::try_from(&**left) {
                    Ok(r) => Box::new(r),
                    Err(e) => return Err(e),
                },
                match Expression::try_from(&**right) {
                    Ok(r) => Box::new(r),
                    Err(e) => return Err(e),
                },
                match op {
                    BinaryOperator::Gt => ExpressionBinaryOperator::Gt,
                    BinaryOperator::Lt => ExpressionBinaryOperator::Lt,
                    BinaryOperator::GtEq => ExpressionBinaryOperator::GtEq,
                    BinaryOperator::LtEq => ExpressionBinaryOperator::LtEq,
                    BinaryOperator::Eq => ExpressionBinaryOperator::Eq,
                    BinaryOperator::NotEq => ExpressionBinaryOperator::NotEq,
                    BinaryOperator::And => ExpressionBinaryOperator::And,
                    BinaryOperator::Or => ExpressionBinaryOperator::Or,
                    _ => {
                        eprintln!("Error found at Binary Operator: {:?}", op);
                        return Err(ExpressionConversionError::StandardError());
                    }
                },
            ),
            Expr::Identifier(i) => Expression::Identifier(ExpressionIdentifier { value: i.value.to_string() }),
            Expr::Value(value) => Expression::Value(match value {
                Value::Number(s, _) => ExpressionValue::Number(s.to_string() ),
                Value::SingleQuotedString(s) => ExpressionValue::String(s.to_string() ),
                Value::DollarQuotedString(s) => ExpressionValue::String(s.value.to_string() ),
                Value::EscapedStringLiteral(s) => ExpressionValue::String(s.to_string() ),

                Value::SingleQuotedByteStringLiteral(s) => ExpressionValue::String(s.to_string() ),
                Value::DoubleQuotedByteStringLiteral(s) => ExpressionValue::String(s.to_string() ),
                Value::RawStringLiteral(s) => ExpressionValue::String(s.to_string() ),

                Value::NationalStringLiteral(s) => ExpressionValue::String(s.to_string() ),
                Value::HexStringLiteral(s) => ExpressionValue::String(s.to_string() ),
                Value::DoubleQuotedString(s) => ExpressionValue::String(s.to_string() ),
                _ => {
                    eprintln!("Error found at Converting value expression: {:?}", value);
                    return Err(ExpressionConversionError::StandardError());
                } // NULL? BOOL?
            }),
            _ => {
                eprintln!("Error found at Expression: {:?}", expr);
                return Err(ExpressionConversionError::StandardError());
            }
        })

        // Err(ExpressionConversionError::StandardError())
    }
}

// #[derive(Deserialize, Serialize, PartialEq, Debug)]
// pub struct ExpressionBinaryOp {
//     left: Box<Expression>,
//     right: Box<Expression>,
//     op: ExpressionBinaryOperator,
// }

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub enum ExpressionValue {
    Number(String),
    String(String),
}

impl TryInto<TypeValue> for ExpressionValue {
    type Error = ExpressionEvaluationError;

    fn try_into(self) -> Result<TypeValue, Self::Error> {
        match self {
            ExpressionValue::Number(s) => match s.parse::<f64>() {
                Ok(v) => Ok(TypeValue::NumberValueType(NumberValueType { value: v })),
                Err(e) => {
                    eprintln!("Error with parsing float: {s} error: {:?}", e);
                    Err(ExpressionEvaluationError::StandardError())
                }
            },
            ExpressionValue::String(s) => Ok(TypeValue::StringTypeValue(StringTypeValue {
                value: s,
            })),
            _ => Err(ExpressionEvaluationError::StandardError()),
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct ExpressionIdentifier {
    value: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum ExpressionBinaryOperator {
    Gt,
    Lt,
    GtEq,
    LtEq,
    Eq,
    NotEq,
    And,
    Or,
    Xor,
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn convert_expression() {
        let sql = "select * into customer_cache from customer where x = 1 and u = 2 and y = 3;";

        let dialect = GenericDialect {};

        let ast = Parser::parse_sql(&dialect, sql).expect("Error with parsing the sql");

        for statement in ast.iter() {
            match statement {
                Query(query) => {
                    let select_query = SelectQuery::try_from(&*query.body);

                    let result = Ok(Expression::BinaryOp(
                        Box::new(Expression::BinaryOp(
                            Box::new(Expression::BinaryOp(
                                Box::new(Expression::Identifier(ExpressionIdentifier {
                                    value: "x".to_string(),
                                })),
                                Box::new(Expression::Value(ExpressionValue::Number("1".to_string()))),
                                ExpressionBinaryOperator::Eq,
                            )),
                            Box::new(Expression::BinaryOp(
                                Box::new(Expression::Identifier(ExpressionIdentifier {
                                    value: "u".to_string(),
                                })),
                                Box::new(Expression::Value(ExpressionValue::Number("2".to_string()))),
                                ExpressionBinaryOperator::Eq,
                            )),
                            ExpressionBinaryOperator::And,
                        )),
                        Box::new(Expression::BinaryOp(
                            Box::new(Expression::Identifier(ExpressionIdentifier {
                                value: "y".to_string(),
                            })),
                            Box::new(Expression::Value(ExpressionValue::Number("3".to_string()))),
                            ExpressionBinaryOperator::Eq,
                        )),
                        ExpressionBinaryOperator::And,
                    ));

                    match select_query {
                        Ok(select) => match select.constraints {
                            Some(c) => assert_eq!(Expression::try_from(c), result),
                            None => panic!("No Select Statement found."),
                        },
                        Err(e) => {
                            eprintln!("Error with getting the Statement: {:?}", e);
                            panic!("No Select Statement found.")
                        }
                    }
                }
                _ => panic!("No Select Statement found."),
            }
        }
    }

    #[test]
    fn evaluate_statement() {
        let sql = "select * into customer_cache from customer where x = 1 and u = 2 and y = 3;";

        let dialect = GenericDialect {};

        let ast = Parser::parse_sql(&dialect, sql).expect("Error with parsing the sql");

        for statement in ast.iter() {
            match statement {
                Query(query) => {
                    let select_query = SelectQuery::try_from(&*query.body);

                    let values = HashMap::from([
                        (
                            "x".to_string(),
                            TypeValue::NumberValueType(NumberValueType { value: 1.0 }),
                        ),
                        (
                            "u".to_string(),
                            TypeValue::NumberValueType(NumberValueType { value: 2.0 }),
                        ),
                        (
                            "y".to_string(),
                            TypeValue::NumberValueType(NumberValueType { value: 3.0 }),
                        ),
                    ]);

                    match select_query {
                        Ok(select) => assert_eq!(select.constraints.expect("No Constraints").evaluate(&values), Ok(true)),
                        Err(e) => {
                            eprintln!("Error with getting the Statement: {:?}", e);
                            panic!("No Select Statement found.")
                        }
                    }
                }
                _ => panic!("No Select Statement found."),
            }
        }
    }

    #[test]
    fn evaluate_statement_false() {
        let sql = "select * into customer_cache from customer where x = 1 and u = 2  and y = 3;";

        let dialect = GenericDialect {};

        let ast = Parser::parse_sql(&dialect, sql).expect("Error with parsing the sql");

        for statement in ast.iter() {
            match statement {
                Query(query) => {
                    let select_query = SelectQuery::try_from(&*query.body);

                    let values = HashMap::from([
                        (
                            "x".to_string(),
                            TypeValue::NumberValueType(NumberValueType { value: 1.0 }),
                        ),
                        (
                            "u".to_string(),
                            TypeValue::NumberValueType(NumberValueType { value: 2.0 }),
                        ),
                    ]);

                    match select_query {
                        Ok(select) => assert_eq!(select.constraints.expect("No Constraints").evaluate(&values), Ok(false)),
                        Err(e) => {
                            eprintln!("Error with getting the Statement: {:?}", e);
                            panic!("No Select Statement found.")
                        }
                    }
                }
                _ => panic!("No Select Statement found."),
            }
        }
    }

    #[test]
    fn evaluate_statement_comparison() {
        let sql = "select * into customer_cache from customer where x = 1 and u = 2  and y > 3;";

        let dialect = GenericDialect {};

        let ast = Parser::parse_sql(&dialect, sql).expect("Error with parsing the sql");

        for statement in ast.iter() {
            match statement {
                Query(query) => {
                    let select_query = SelectQuery::try_from(&*query.body);

                    let values = HashMap::from([
                        (
                            "x".to_string(),
                            TypeValue::NumberValueType(NumberValueType { value: 1.0 }),
                        ),
                        (
                            "u".to_string(),
                            TypeValue::NumberValueType(NumberValueType { value: 2.0 }),
                        ),
                        (
                            "y".to_string(),
                            TypeValue::NumberValueType(NumberValueType { value: 4.0 }),
                        ),
                    ]);

                    match select_query {
                        Ok(select) => assert_eq!(select.constraints.expect("No Constraints").evaluate(&values), Ok(true)),
                        Err(e) => {
                            eprintln!("Error with getting the Statement: {:?}", e);
                            panic!("No Select Statement found.")
                        }
                    }
                }
                _ => panic!("No Select Statement found."),
            }
        }
    }

    #[test]
    fn evaluate_statement_comparison_false() {
        let sql = "select * into customer_cache from customer where x = 1 and u = 2  and y > 3;";

        let dialect = GenericDialect {};

        let ast = Parser::parse_sql(&dialect, sql).expect("Error with parsing the sql");

        for statement in ast.iter() {
            match statement {
                Query(query) => {
                    let select_query = SelectQuery::try_from(&*query.body);

                    let values = HashMap::from([
                        (
                            "x".to_string(),
                            TypeValue::NumberValueType(NumberValueType { value: 1.0 }),
                        ),
                        (
                            "u".to_string(),
                            TypeValue::NumberValueType(NumberValueType { value: 2.0 }),
                        ),
                        (
                            "y".to_string(),
                            TypeValue::NumberValueType(NumberValueType { value: 2.0 }),
                        ),
                    ]);

                    match select_query {
                        Ok(select) => assert_eq!(select.constraints.expect("No Constraints").evaluate(&values), Ok(false)),
                        Err(e) => {
                            eprintln!("Error with getting the Statement: {:?}", e);
                            panic!("No Select Statement found.")
                        }
                    }
                }
                _ => panic!("No Select Statement found."),
            }
        }
    }
}
