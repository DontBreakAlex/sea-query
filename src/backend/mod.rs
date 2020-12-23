//! Translating unified SQL representation into database specific SQL statement.
//! 
//! There traits are defined to model the CRUD (create, read, update, delete) behaviours
//! of each type of SQL statement, namely [`query`], [`table`], [`index`], [`foreign_key`].
//! 
//! NOTE: not all operations are support at the time, we will add more functionality in the future. :)

use crate::*;
use std::fmt::Write as FmtWrite;

mod mysql;
mod sqlite;
mod postgres;

pub use mysql::*;
pub use sqlite::*;
pub use postgres::*;

pub trait GenericBuilder: QueryBuilder + TableBuilder + IndexBuilder + ForeignKeyBuilder {
    fn query_builder(&self) -> Box<dyn QueryBuilder>;
    fn table_builder(&self) -> Box<dyn TableBuilder>;
    fn index_builder(&self) -> Box<dyn IndexBuilder>;
    fn foreign_key_builder(&self) -> Box<dyn ForeignKeyBuilder>;
}

pub trait QueryBuilder {
    /// Translate [`InsertStatement`] into database specific SQL statement.
    fn prepare_insert_statement(&self, insert: &InsertStatement, sql: &mut dyn FmtWrite, collector: &mut dyn FnMut(Value));

    /// Translate [`SelectStatement`] into database specific SQL statement.
    fn prepare_select_statement(&self, select: &SelectStatement, sql: &mut dyn FmtWrite, collector: &mut dyn FnMut(Value));

    /// Translate [`UpdateStatement`] into database specific SQL statement.
    fn prepare_update_statement(&self, update: &UpdateStatement, sql: &mut dyn FmtWrite, collector: &mut dyn FnMut(Value));

    /// Translate [`DeleteStatement`] into database specific SQL statement.
    fn prepare_delete_statement(&self, delete: &DeleteStatement, sql: &mut dyn FmtWrite, collector: &mut dyn FnMut(Value));

    /// Translate [`SimpleExpr`] into database specific SQL statement.
    fn prepare_simple_expr(&self, simple_expr: &SimpleExpr, sql: &mut dyn FmtWrite, collector: &mut dyn FnMut(Value));

    /// Translate [`SelectDistinct`] into database specific SQL statement.
    fn prepare_select_distinct(&self, select_distinct: &SelectDistinct, sql: &mut dyn FmtWrite, collector: &mut dyn FnMut(Value));

    /// Translate [`SelectExpr`] into database specific SQL statement.
    fn prepare_select_expr(&self, select_expr: &SelectExpr, sql: &mut dyn FmtWrite, collector: &mut dyn FnMut(Value));

    /// Translate [`JoinExpr`] into database specific SQL statement.
    fn prepare_join_expr(&self, join_expr: &JoinExpr, sql: &mut dyn FmtWrite, collector: &mut dyn FnMut(Value));

    /// Translate [`TableRef`] into database specific SQL statement.
    fn prepare_table_ref(&self, table_ref: &TableRef, sql: &mut dyn FmtWrite, collector: &mut dyn FnMut(Value));

    /// Translate [`UnOper`] into database specific SQL statement.
    fn prepare_un_oper(&self, un_oper: &UnOper, sql: &mut dyn FmtWrite, collector: &mut dyn FnMut(Value));

    /// Translate [`BinOper`] into database specific SQL statement.
    fn prepare_bin_oper(&self, bin_oper: &BinOper, sql: &mut dyn FmtWrite, collector: &mut dyn FnMut(Value));

    /// Translate [`LogicalChainOper`] into database specific SQL statement.
    fn prepare_logical_chain_oper(&self, log_chain_oper: &LogicalChainOper, i: usize, length: usize, sql: &mut dyn FmtWrite, collector: &mut dyn FnMut(Value));

    /// Translate [`Function`] into database specific SQL statement.
    fn prepare_function(&self, function: &Function, sql: &mut dyn FmtWrite, collector: &mut dyn FnMut(Value));

    /// Translate [`JoinType`] into database specific SQL statement.
    fn prepare_join_type(&self, join_type: &JoinType, sql: &mut dyn FmtWrite, collector: &mut dyn FnMut(Value));

    /// Translate [`OrderExpr`] into database specific SQL statement.
    fn prepare_order_expr(&self, order_expr: &OrderExpr, sql: &mut dyn FmtWrite, collector: &mut dyn FnMut(Value));

    /// Translate [`JoinOn`] into database specific SQL statement.
    fn prepare_join_on(&self, join_on: &JoinOn, sql: &mut dyn FmtWrite, collector: &mut dyn FnMut(Value));

    /// Translate [`Order`] into database specific SQL statement.
    fn prepare_order(&self, order: &Order, sql: &mut dyn FmtWrite, collector: &mut dyn FnMut(Value));

    /// Translate [`Value`] into database specific SQL statement.
    fn prepare_value(&self, value: &Value, sql: &mut dyn FmtWrite, collector: &mut dyn FnMut(Value));

    /// Translate [`Value`] into database specific SQL statement.
    fn prepare_value_param(&self, value: &Value, collector: &mut dyn FnMut(Value));

}

pub trait TableBuilder {
    /// Translate [`TableCreateStatement`] into database specific SQL statement.
    fn prepare_table_create_statement(&self, insert: &TableCreateStatement, sql: &mut dyn FmtWrite);

    /// Translate [`ColumnDef`] into database specific SQL statement.
    fn prepare_column_def(&self, column_def: &ColumnDef, sql: &mut dyn FmtWrite);

    /// Translate [`ColumnType`] into database specific SQL statement.
    fn prepare_column_type(&self, column_type: &ColumnType, sql: &mut dyn FmtWrite);

    /// Translate [`ColumnSpec`] into database specific SQL statement.
    fn prepare_column_spec(&self, column_spec: &ColumnSpec, sql: &mut dyn FmtWrite);

    /// Translate [`TableOpt`] into database specific SQL statement.
    fn prepare_table_opt(&self, table_opt: &TableOpt, sql: &mut dyn FmtWrite);

    /// Translate [`TablePartition`] into database specific SQL statement.
    fn prepare_table_partition(&self, table_partition: &TablePartition, sql: &mut dyn FmtWrite);

    /// Translate [`TableDropStatement`] into database specific SQL statement.
    fn prepare_table_drop_statement(&self, drop: &TableDropStatement, sql: &mut dyn FmtWrite);

    /// Translate [`TableDropOpt`] into database specific SQL statement.
    fn prepare_table_drop_opt(&self, drop_opt: &TableDropOpt, sql: &mut dyn FmtWrite);

    /// Translate [`TableTruncateStatement`] into database specific SQL statement.
    fn prepare_table_truncate_statement(&self, truncate: &TableTruncateStatement, sql: &mut dyn FmtWrite);

    /// Translate [`TableAlterStatement`] into database specific SQL statement.
    fn prepare_table_alter_statement(&self, alter: &TableAlterStatement, sql: &mut dyn FmtWrite);

    /// Translate [`TableRenameStatement`] into database specific SQL statement.
    fn prepare_table_rename_statement(&self, rename: &TableRenameStatement, sql: &mut dyn FmtWrite);

}

pub trait IndexBuilder {
    /// Translate [`IndexCreateStatement`] into database specific SQL statement.
    fn prepare_index_create_statement(&self, create: &IndexCreateStatement, sql: &mut dyn FmtWrite);

    /// Translate [`IndexDropStatement`] into database specific SQL statement.
    fn prepare_index_drop_statement(&self, drop: &IndexDropStatement, sql: &mut dyn FmtWrite);

}

pub trait ForeignKeyBuilder {
    /// Translate [`ForeignKeyCreateStatement`] into database specific SQL statement.
    fn prepare_foreign_key_create_statement(&self, create: &ForeignKeyCreateStatement, sql: &mut dyn FmtWrite);

    /// Translate [`ForeignKeyAction`] into database specific SQL statement.
    fn prepare_foreign_key_action(&self, foreign_key_action: &ForeignKeyAction, sql: &mut dyn FmtWrite);

    /// Translate [`ForeignKeyDropStatement`] into database specific SQL statement.
    fn prepare_foreign_key_drop_statement(&self, drop: &ForeignKeyDropStatement, sql: &mut dyn FmtWrite);

}