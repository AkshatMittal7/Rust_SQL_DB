use crate::{DataType, Table};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct TableJSON {
    pub table_name: String,
    pub columns: HashMap<String, Vec<DataType>>,
    pub select_columns: Vec<String>,
}

impl TableJSON {
    pub fn from_table(table: Box<dyn Table>) -> TableJSON {
        let table_name = table.get_table_name();
        let select_columns = table.get_select_columns().clone();
        let columns = table.get_columns().clone();
        TableJSON {
            table_name,
            select_columns,
            columns,
        }
    }
}
