use crate::DataSource;
use anyhow::{anyhow, Result};
use sqlparser::ast::{Select, SetExpr, Statement, TableFactor, TableWithJoins};
use sqlparser::dialect::Dialect;
use sqlparser::parser::Parser;

struct Source<'a>(&'a [TableWithJoins]);

#[derive(Debug, Default)]
pub struct MinervaDialect;

impl Dialect for MinervaDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        ch.is_alphabetic() || ch.is_ascii_digit() || ch == '_'
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        self.is_identifier_start(ch) || [':', '-', '_', '.', '$'].contains(&ch)
    }
}

pub fn normalize_sql(
    sql: &str,
    ds: &DataSource,
    allow_non_query: bool,
    format: Option<&'static str>,
    compression: Option<&'static str>,
) -> Result<String> {
    let sql = sql.trim();
    let dialect = MinervaDialect {};
    let mut ast = Parser::parse_sql(&dialect, sql)?;
    if ast.len() != 1 {
        return Err(anyhow!(
            "invalid query: {}. Must contain 1 and only 1 sql statement",
            sql
        ));
    }

    let stmt = ast.pop().expect("ast.len() == 1");

    // we only allow query statement
    let query = match &stmt {
        Statement::Query(query) => query,
        Statement::Explain { .. } => {
            if allow_non_query {
                let query = replace_data_source(sql, ds);
                return Ok(append_format(&query, format, compression));
            } else {
                return Err(anyhow!("invalid query: {}. Must be a query statement", sql));
            }
        }
        _ => {
            if allow_non_query {
                let first_keyword = sql.split_whitespace().next().unwrap_or("").to_lowercase();
                let query = replace_data_source(sql, ds);
                if ["describe", "analyze", "show"].contains(&first_keyword.as_str()) {
                    return Ok(append_format(&query, format, compression));
                }
                return Ok(query);
            } else {
                return Err(anyhow!("invalid query: {}. Must be a query statement", sql));
            }
        }
    };

    let has_limit = query.limit.is_some();
    let Select {
        from: table_with_joins,
        selection: _where_clause,
        projection: _projection,

        group_by: _,
        ..
    } = match query.body.as_ref() {
        SetExpr::Select(statement) => statement.as_ref(),
        _ => return Err(anyhow!("We only support Select Query at the moment")),
    };

    let source: &str = Source(table_with_joins).try_into()?;

    let table_name = ds.table_name();
    if source != table_name {
        return Err(anyhow!(
            "invalid query: {}. Must query from {}",
            sql,
            table_name
        ));
    }

    let mut query = replace_data_source(stmt.to_string(), ds);

    if !has_limit {
        query.push_str(" LIMIT 1000");
    }

    Ok(append_format(&query, format, compression))
}

fn append_format(
    query: &str,
    format: Option<&'static str>,
    compression: Option<&'static str>,
) -> String {
    let format = format.unwrap_or("Arrow");
    if format == "Arrow" {
        format!(
            "{} format {} settings output_format_arrow_compression_method='{}'",
            query,
            format,
            compression.unwrap_or("none")
        )
    } else {
        format!("{} format {}", query, format,)
    }
}
fn replace_data_source(query: impl AsRef<str>, ds: &DataSource) -> String {
    let query = query.as_ref();
    let table_name = ds.table_name();
    match ds {
        DataSource::S3(ref bucket) => {
            let s3_table = format!(
                "s3('https://{}.s3.{}.amazonaws.com/{}')",
                bucket.name, bucket.region, bucket.key
            );
            query.replace(&table_name, &s3_table)
        }
        DataSource::Local(ref local_file) => {
            let file_table = format!("file('{}')", local_file.path);
            query.replace(&table_name, &file_table)
        }
    }
}

impl<'a> TryFrom<Source<'a>> for &'a str {
    type Error = anyhow::Error;

    fn try_from(source: Source<'a>) -> Result<Self, Self::Error> {
        if source.0.len() != 1 {
            return Err(anyhow!("We only support single data source at the moment"));
        }

        let table = &source.0[0];
        if !table.joins.is_empty() {
            return Err(anyhow!("We do not support joint data source at the moment"));
        }

        match &table.relation {
            TableFactor::Table { name, .. } => Ok(&name.0.first().unwrap().value),
            _ => Err(anyhow!("We only support table")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::LocalFile;

    use super::*;

    #[test]
    fn normalize_query_should_just_work() {
        let sql = "SELECT * FROM test";
        let ds = LocalFile::new("crates/clickhouse/fixtures/test.arrow").into();

        let query = normalize_sql(sql, &ds, false, None, None).unwrap();
        assert_eq!(
            query,
            "SELECT * FROM file('crates/clickhouse/fixtures/test.arrow') LIMIT 1000 format Arrow settings output_format_arrow_compression_method='none'"
        );
    }

    #[test]
    fn normalize_query_with_limit_should_work() {
        let sql = "SELECT * FROM test LIMIT 10";
        let ds = LocalFile::new("crates/clickhouse/fixtures/test.arrow").into();

        let query = normalize_sql(sql, &ds, false, Some("JSONEachRow"), Some("zstd")).unwrap();
        assert_eq!(
            query,
            "SELECT * FROM file('crates/clickhouse/fixtures/test.arrow') LIMIT 10 format JSONEachRow"
        );
    }

    #[test]
    fn normalize_describe_should_work() {
        let sql = "DESCRIBE test";
        let ds = LocalFile::new("crates/clickhouse/fixtures/test.arrow").into();

        let query = normalize_sql(sql, &ds, true, None, None).unwrap();
        assert_eq!(
            query,
            "DESCRIBE file('crates/clickhouse/fixtures/test.arrow') format Arrow settings output_format_arrow_compression_method='none'"
        );
    }
}
