use crate::catalog::SchemaRef;
use crate::common::table_ref::TableReference;
use crate::planner::logical_plan::OrderByExpr;
use crate::{
    catalog::Schema,
    execution::{ExecutionContext, VolcanoExecutor},
    storage::Tuple,
    BustubxResult,
};
use std::sync::Arc;

#[derive(Debug, derive_new::new)]
pub struct PhysicalCreateIndex {
    pub name: String,
    pub table: TableReference,
    pub table_schema: SchemaRef,
    pub columns: Vec<OrderByExpr>,
}

impl VolcanoExecutor for PhysicalCreateIndex {
    fn next(&self, context: &mut ExecutionContext) -> BustubxResult<Option<Tuple>> {
        // TODO implement
        Ok(None)
    }
    fn output_schema(&self) -> SchemaRef {
        Arc::new(Schema::empty())
    }
}

impl std::fmt::Display for PhysicalCreateIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}