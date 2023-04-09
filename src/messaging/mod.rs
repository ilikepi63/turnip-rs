use serde::{Deserialize, Serialize};

use crate::models::{insert_query::InsertQuery, select_query::SelectQuery};

#[derive(Deserialize, Serialize)]
pub enum Message {
    Select(SelectQuery),
    Insert(InsertQuery),
}
