use derive_with::With;
use std::sync::Arc;

use crate::catalog::DataType;
use crate::common::TableReference;

pub type ColumnRef = Arc<Column>;

#[derive(Debug, Clone, With)]
pub struct Column {
    pub relation: Option<TableReference>,
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
}

impl PartialEq for Column {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.data_type == other.data_type
    }
}

impl Eq for Column {}

impl Column {
    pub fn new(name: String, data_type: DataType, nullable: bool) -> Self {
        Self {
            relation: None,
            name,
            data_type,
            nullable,
        }
    }
}
