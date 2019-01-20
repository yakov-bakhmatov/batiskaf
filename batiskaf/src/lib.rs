use rusqlite::types::ToSql;
use rusqlite::{self, Connection, Row, Statement};

#[cfg(test)]
mod tests;

pub trait SqlParam {
    fn to_named_params(&self, stmt: &Statement) -> Vec<(&str, &dyn ToSql)>;
}

pub trait SqlResult: Sized {
    fn from_row(row: &Row) -> rusqlite::Result<Self>;
}

pub trait SqlInsert {
    fn insert_statement(table: &str) -> String;
}

pub trait SqlUpdate {
    fn update_statement(table: &str) -> String;
}

pub trait SqlDelete {
    fn delete_statement(table: &str) -> String;
}

pub trait BatiskafConnection {
    fn select_one<T: SqlResult>(
        &self,
        sql: &str,
        params: &[(&str, &dyn ToSql)],
    ) -> rusqlite::Result<T>;
    fn select_many<T: SqlResult>(
        &self,
        sql: &str,
        params: &[(&str, &dyn ToSql)],
    ) -> rusqlite::Result<Vec<T>>;
    fn insert<T: SqlInsert + SqlParam>(&self, table: &str, value: &T) -> rusqlite::Result<i64>;
    fn update<T: SqlUpdate + SqlParam>(&self, table: &str, value: &T) -> rusqlite::Result<usize>;
    fn delete<T: SqlDelete + SqlParam>(&self, table: &str, value: &T) -> rusqlite::Result<usize>;
}

impl BatiskafConnection for Connection {
    fn select_one<T: SqlResult>(
        &self,
        sql: &str,
        params: &[(&str, &dyn ToSql)],
    ) -> rusqlite::Result<T> {
        self.query_row_named(sql, params, T::from_row)?
    }

    fn select_many<T: SqlResult>(
        &self,
        sql: &str,
        params: &[(&str, &dyn ToSql)],
    ) -> rusqlite::Result<Vec<T>> {
        let mut stmt = self.prepare(sql)?;
        let mut rows = stmt.query_named(params)?;
        let mut result = Vec::new();
        while let Some(row) = rows.next() {
            result.push(T::from_row(&row?)?);
        }
        Ok(result)
    }

    fn insert<T: SqlInsert + SqlParam>(&self, table: &str, value: &T) -> rusqlite::Result<i64> {
        let sql = T::insert_statement(table);
        let mut stmt = self.prepare(&sql)?;
        let changes = stmt.execute_named(&value.to_named_params(&stmt))?;
        match changes {
            1 => Ok(self.last_insert_rowid()),
            _ => Err(rusqlite::Error::StatementChangedRows(changes)),
        }
    }

    fn update<T: SqlUpdate + SqlParam>(&self, table: &str, value: &T) -> rusqlite::Result<usize> {
        let sql = T::update_statement(table);
        let mut stmt = self.prepare(&sql)?;
        stmt.execute_named(&value.to_named_params(&stmt))
    }

    fn delete<T: SqlDelete + SqlParam>(&self, table: &str, value: &T) -> rusqlite::Result<usize> {
        let sql = T::delete_statement(table);
        let mut stmt = self.prepare(&sql)?;
        stmt.execute_named(&value.to_named_params(&stmt))
    }
}
