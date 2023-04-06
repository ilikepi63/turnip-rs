use serde::{Deserialize, Serialize};


use crate::models::{select_query::SelectQuery, insert_query::InsertQuery};

#[derive(Deserialize, Serialize)]
pub enum Message{
    Select(SelectQuery),
    Insert(InsertQuery)
}