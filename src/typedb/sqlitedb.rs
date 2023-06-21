use super::TypeDb;
use crate::error::{Error, SqliteDbError};
use sqlitedb::{self, Query};
use std::fmt::Display;
use tonic::async_trait;

const SELECT_NAME: &'static str = "
    SELECT
        name
    FROM
        types
    WHERE
        type_id, language = (?, ?)
";

const SELECT_REPROCESS: &'static str = "
    SELECT
        material_id, quantity
    FROM
        reprocess
    WHERE
        type_id = ?
";

pub struct SqliteDb {
    inner: sqlitedb::SqliteDb,
}

impl SqliteDb {
    pub fn new(namespace: impl Display) -> Result<Self, Error> {
        Ok(Self {
            inner: sqlitedb::SqliteDb::new(namespace).map_err(|e| SqliteDbError::Initialize(e))?,
        })
    }
}

#[async_trait]
impl TypeDb for SqliteDb {
    async fn get_name(&self, type_id: u32, language: &str) -> Result<String, Error> {
        Ok(self
            .inner
            .select_one::<(String,)>(Query::new(SELECT_NAME).bind(type_id).bind(language))
            .await
            .map_err(|e| SqliteDbError::SelectName(e))?
            .ok_or(SqliteDbError::NameMissing(type_id))?
            .0)
    }

    async fn get_reprocess(&self, type_id: u32) -> Result<Vec<(u32, f64)>, Error> {
        Ok(self
            .inner
            .select_all::<(u32, f64)>(Query::new(SELECT_REPROCESS).bind(type_id))
            .await
            .map_err(|e| SqliteDbError::SelectReprocess(e))
            .and_then(|vec| match vec.is_empty() {
                true => Err(SqliteDbError::ReprocessMissing(type_id)),
                false => Ok(vec),
            })?)
    }
}
