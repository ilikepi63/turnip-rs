// this file will hold all of the selects that have been made by other nodes
use std::error::Error;

pub struct SelectIndex{

    // some data structure to hold all of the current selects
}

impl SelectIndex{

    // access pattern to get all the addresses subscribed to a 
    pub fn get_addr_for_insert(/*insert data model */) -> Result<Vec<String>, Box<dyn Error>> {
        Ok(vec![])
    }
    
    // insert a select statement, happens when either this node or another node asks to query a subset of data
    pub fn insert_select(/*select data model */) -> Result<(), Box<dyn Error>>{
        // here we need to check the data model to see if we have that data on insert.
        Ok(())
    }

    
}