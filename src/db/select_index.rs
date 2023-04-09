// this file will hold all of the selects that have been made by other nodes
use std::collections::HashMap;
use std::error::Error;

use crate::models::{
    insert_query::{self, InsertQuery},
    select_query::SelectQuery,
};

use super::data::convert_row_to_hashmap;

pub struct SelectIndex {
    // get by relation -> projection -> constraints
    // get by relation -> contraints -> projection

    // data model is => HashMap<Relation, Vec<(SelectQuery, Addr)>>
    selects: HashMap<String, Vec<(SelectQuery, String)>>,
}

impl SelectIndex {
    // access pattern to get all the addresses subscribed to a
    pub fn get_addr_for_insert(
        &self,
        insert_query: InsertQuery,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let selects = self.selects.get(&insert_query.table_name);

        if let Some(select) = selects {
            // super naive algorithm yo...
            let result_vec = vec![];

            for row in insert_query.rows.iter() {
                let hash_row = convert_row_to_hashmap(&insert_query.columns, &row);

                for sel in select.iter() {
                    if sel.0.evaluate(hash_row) {
                        result_vec.push(sel.1);
                    }
                }
            }

            Ok(result_vec)
        } else {
            Ok(vec![])
        }
    }

    // insert a select statement, happens when either this node or another node asks to query a subset of data
    pub fn insert_select(
        &mut self,
        addr: &str,
        select_query: SelectQuery,
    ) -> Result<(), Box<dyn Error>> {
        let current_select = self.selects.get_mut(&select_query.from);

        if Some(select) = current_select {
            select.push((select_query, addr.to_string()));
        } else {
            self.selects.insert(
                select_query.from.to_string(),
                vec![(select_query, addr.to_string())],
            );
        }

        Ok(())
    }
}
