use std::rc::Rc;
use crate::{TableIndex, backend::SchemaBuilder, SchemaStatementBuilder, types::*, prepare::*};

/// Drop an index for an existing table
///
/// # Examples
///
/// ```
/// use sea_query::{*, tests_cfg::*};
///
/// let index = Index::drop()
///     .name("idx-glyph-aspect")
///     .table(Glyph::Table)
///     .to_owned();
///
/// assert_eq!(
///     index.to_string(MysqlQueryBuilder),
///     r#"DROP INDEX `idx-glyph-aspect` ON `glyph`"#
/// );
/// assert_eq!(
///     index.to_string(PostgresQueryBuilder),
///     r#"DROP INDEX "idx-glyph-aspect""#
/// );
/// assert_eq!(
///     index.to_string(SqliteQueryBuilder),
///     r#"DROP INDEX `idx-glyph-aspect` ON `glyph`"#
/// );
/// ```
#[derive(Debug, Clone)]
pub struct IndexDropStatement {
    pub(crate) table: Option<Rc<dyn Iden>>,
    pub(crate) index: TableIndex,
}

impl Default for IndexDropStatement {
    fn default() -> Self {
        Self::new()
    }
}

impl IndexDropStatement {
    /// Construct a new [`IndexDropStatement`]
    pub fn new() -> Self {
        Self {
            table: None,
            index: Default::default(),
        }
    }

    /// Set index name
    pub fn name(mut self, name: &str) -> Self {
        self.index.name(name);
        self
    }

    /// Set target table
    pub fn table<T: 'static>(mut self, table: T) -> Self
        where T: Iden {
        self.table = Some(Rc::new(table));
        self
    }
}

impl SchemaStatementBuilder for IndexDropStatement {
    fn build<T: SchemaBuilder>(&self, schema_builder: T) -> String {
        let mut sql = SqlWriter::new();
        schema_builder.prepare_index_drop_statement(self, &mut sql);
        sql.result()
    }

    fn build_any(&self, schema_builder: &dyn SchemaBuilder) -> String {
        let mut sql = SqlWriter::new();
        schema_builder.prepare_index_drop_statement(self, &mut sql);
        sql.result()
    }
}
