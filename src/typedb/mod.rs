mod sqlitedb;

use crate::error::Error;
use std::fmt::Display;
use tonic::async_trait;

#[async_trait]
pub trait TypeDb: Send + Sync + 'static {
    async fn get_name(&self, type_id: u32, language: &str) -> Result<String, Error>;
    async fn get_reprocess(&self, type_id: u32) -> Result<Vec<(u32, f64)>, Error>;
}

pub fn new_type_db(namespace: impl Display) -> Result<impl TypeDb, Error> {
    sqlitedb::SqliteDb::new(namespace)
}
