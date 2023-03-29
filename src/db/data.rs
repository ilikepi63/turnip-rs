// this is the in-memory(for now) DB for holding local data in the node
use std::error::Error;

use crate::models::select_query::SelectQuery;
use crate::models::insert_query::InsertQuery;
use super::errors::DatabaseError;


enum TypeValue{
    String,
    Number,
}

pub struct Db{
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

    data: HashMap<String, Vec<HashMap<TypeValue>>>
}

impl Db{

    pub fn query_data_by_select(query: SelectQuery) -> Vec<HashMap<TypeValue>>{
        // this shoud return a result from the DB depending on whether or not the query resulted in a data structure
    }

    pub fn insert_remote(query: InsertQuery) -> Result<(), DatabaseError>{
        // insert data into this database from a remote location
    }

}