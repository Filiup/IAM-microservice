use crate::common::database::Database;
use redis::AsyncCommands;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Clone)]
pub struct CashingService {}
 
impl CashingService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> anyhow::Result<Option<T>> {
        let mut redis_conn = Database::redis().await;
        let cached: Option<Vec<u8>> = redis_conn.get(key).await?;

        cached
            .map(|value| rmp_serde::from_slice(value.as_ref()))
            .transpose()
            .map_err(anyhow::Error::new)
    }

    pub async fn set<T: Serialize>(&self, key: String, value: &T) -> anyhow::Result<()> {
        let mut redis_conn = Database::redis().await;
        let value_string = rmp_serde::to_vec(value)?;

        redis_conn
            .set(key, value_string)
            .await
            .map_err(anyhow::Error::new)
    }
}
