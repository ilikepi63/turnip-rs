use super::{insert_query::InsertQuery, select_query::SelectQuery};

pub enum Statement {
    Select(SelectQuery),
    Insert(InsertQuery),
}
