// this is the in-memory(for now) DB for holding local data in the node
use serde::{Deserialize, Serialize};
use sqlparser::ast::Value;

use super::errors::DatabaseError;
use crate::models::select_query::SelectQuery;
use crate::{db::errors::ValueParseError, models::insert_query::InsertQuery};

use std::collections::HashMap;
use std::cmp::Ordering;

use super::models::{number_value::NumberValueType, string_value::StringTypeValue};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TypeValue {
    StringTypeValue(StringTypeValue),
    NumberValueType(NumberValueType),
    NullValueType,
}

impl TryFrom<&Value> for TypeValue {
    type Error = ValueParseError;

    fn try_from(v: &Value) -> Result<Self, Self::Error> {
        match v {
            sqlparser::ast::Value::Number(s, _) => match s.parse::<f64>() {
                Ok(v) => Ok(TypeValue::NumberValueType(NumberValueType { value: v })),
                Err(e) => {
                    eprintln!("Error with parsing float: {v} error: {:?}", e);
                    Err(ValueParseError::IsNoneError())
                }
            },
            sqlparser::ast::Value::DoubleQuotedString(s) => {
                Ok(TypeValue::StringTypeValue(StringTypeValue { value: s.to_string() }))
            }
            sqlparser::ast::Value::SingleQuotedString(s) => {
                Ok(TypeValue::StringTypeValue(StringTypeValue { value: s.to_string() }))
            }
            _ => Err(ValueParseError::IsNoneError()),
        }
    }
}

impl PartialEq for TypeValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                &TypeValue::StringTypeValue(StringTypeValue { value: ref a }),
                &TypeValue::StringTypeValue(StringTypeValue { value: ref b }),
            ) => a == b,
            (
                &TypeValue::NumberValueType(NumberValueType { value: ref a }),
                &TypeValue::NumberValueType(NumberValueType { value: ref b }),
            ) => a == b,
            (&TypeValue::NullValueType, &TypeValue::NullValueType) => true,
            _ => false,
        }
    }
}

impl PartialOrd for TypeValue {
    fn partial_cmp(&self, other: &TypeValue) -> Option<Ordering> {
        match (self, other) {
            (
                &TypeValue::StringTypeValue(StringTypeValue { value: ref a }),
                &TypeValue::StringTypeValue(StringTypeValue { value: ref b }),
            ) => Some(a.cmp(b)),
            (
                &TypeValue::NumberValueType(NumberValueType { value: ref a }),
                &TypeValue::NumberValueType(NumberValueType { value: ref b }),
            ) => a.partial_cmp(b),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Db {
    // should include:
    // remote data -> this is data that is owned by another node, but is queried by this node.
    // local data -> this is data that is owned by this node but might be queried by another node.

    // naive implementation
    // structure:
    //     {
    //         table: [
    //             {/*row*/}
    //         ]
    //     }
    data: HashMap<String, Vec<HashMap<String, TypeValue>>>,
}

impl Db {
    pub fn new() -> Self {
        Db {
            data: HashMap::new(),
        }
    }

    pub fn query_data_by_select(query: SelectQuery) -> Vec<HashMap<String, TypeValue>> {
        // this shoud return a result from the DB depending on whether or not the query resulted in a data structure
        vec![]
    }

    pub fn insert(&mut self, query: InsertQuery) -> Result<(), DatabaseError> {
        // pub table_name: String,
        // pub columns: Vec<String>,
        // pub rows: Vec<Vec<String>>,
        match self.data.get_mut(&query.table_name) {
            Some(table) => {
                match insert_rows_into_table(table, &query.columns, &query.rows) {
                    Ok(_) => {
                        println!("Successfully inserted into table");
                    }
                    Err(e) => {
                        eprintln!("An error occurred with inserting the record: {:?}", e);
                    }
                };
                Ok(())
            }
            None => {
                let mut table: Vec<HashMap<String, TypeValue>> = vec![];

                match insert_rows_into_table(&mut table, &query.columns, &query.rows) {
                    Ok(_) => {
                        println!("Successfully inserted into table");
                    }
                    Err(e) => {
                        eprintln!("An error occurred with inserting the record: {:?}", e);
                    }
                };

                self.data.insert(query.table_name.to_string(), table);

                Ok(())
            }
        }
    }
}

pub fn insert_rows_into_table(
    table: &mut Vec<HashMap<String, TypeValue>>,
    columns: &Vec<String>,
    rows: &Vec<Vec<Option<TypeValue>>>,
) -> Result<(), DatabaseError> {
    for row in rows.iter() {
        let new_row = convert_row_to_hashmap(columns, row);

        table.push(new_row);
    }

    Ok(())
}

pub fn convert_row_to_hashmap(
    columns: &Vec<String>,
    row: &Vec<Option<TypeValue>>,
) -> HashMap<String, TypeValue> {
    let mut hmap = HashMap::new();

    for (i, column_name) in columns.iter().enumerate() {
        if let Some(tv) = row.get(i) {
            if let Some(v) = tv.as_ref() {
                hmap.insert(column_name.clone(), v.clone());
            } else {
                hmap.insert(column_name.clone(), TypeValue::NullValueType);
            }
        }
    }

    hmap
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn insert_rows_into_table_test() {
        let mut table: Vec<HashMap<String, TypeValue>> = vec![];
        let columns = vec![
            "id".to_string(),
            "first_name".to_string(),
            "last_name".to_string(),
        ];
        let rows = vec![vec![
            Some(TypeValue::NumberValueType(NumberValueType { value: 1.0 })),
            Some(TypeValue::StringTypeValue(StringTypeValue {
                value: "Cameron".to_string(),
            })),
            Some(TypeValue::StringTypeValue(StringTypeValue {
                value: "Harris".to_string(),
            })),
        ]];

        insert_rows_into_table(&mut table, &columns, &rows);

        assert_eq!(
            table,
            vec![HashMap::from([
                (
                    "id".to_string(),
                    TypeValue::NumberValueType(NumberValueType { value: 1.0 })
                ),
                (
                    "first_name".to_string(),
                    TypeValue::StringTypeValue(StringTypeValue {
                        value: "Cameron".to_string()
                    })
                ),
                (
                    "last_name".to_string(),
                    TypeValue::StringTypeValue(StringTypeValue {
                        value: "Harris".to_string()
                    })
                )
            ])]
        )
    }

    #[test]
    fn convert_row_to_hashmap_test() {
        let columns = vec![
            "id".to_string(),
            "first_name".to_string(),
            "last_name".to_string(),
        ];
        let row = vec![
            Some(TypeValue::NumberValueType(NumberValueType { value: 1.0 })),
            Some(TypeValue::StringTypeValue(StringTypeValue {
                value: "Cameron".to_string(),
            })),
            Some(TypeValue::StringTypeValue(StringTypeValue {
                value: "Harris".to_string(),
            })),
        ];

        let result = convert_row_to_hashmap(&columns, &row);

        assert_eq!(
            result,
            HashMap::from([
                (
                    "id".to_string(),
                    TypeValue::NumberValueType(NumberValueType { value: 1.0 })
                ),
                (
                    "first_name".to_string(),
                    TypeValue::StringTypeValue(StringTypeValue {
                        value: "Cameron".to_string()
                    })
                ),
                (
                    "last_name".to_string(),
                    TypeValue::StringTypeValue(StringTypeValue {
                        value: "Harris".to_string()
                    })
                )
            ])
        );
    }
}
