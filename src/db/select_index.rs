// this file will hold all of the selects that have been made by other nodes
use std::collections::HashMap;
use std::error::Error;

use crate::models::{insert_query::InsertQuery, select_query::SelectQuery};

pub struct SelectIndex {
    // get by relation -> projection -> constraints
    // get by relation -> contraints -> projection
    data: HashMap<String, String>,
}

impl SelectIndex {
    // access pattern to get all the addresses subscribed to a
    pub fn get_addr_for_insert(insert_query: InsertQuery) -> Result<Vec<String>, Box<dyn Error>> {
        Ok(vec![])
    }

    // insert a select statement, happens when either this node or another node asks to query a subset of data
    pub fn insert_select(select_query: SelectQuery) -> Result<(), Box<dyn Error>> {
        // here we need to check the data model to see if we have that data on insert.

        println!("Select: {:?}", select_query.from);
        println!("Into: {:?}", select_query.into);
        println!("Projection: {:?}", select_query.projection);
        println!("Constraints?");

        Ok(())
    }
}
