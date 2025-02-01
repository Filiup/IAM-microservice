use super::helper::get_env_variable;
use redis::Client;
use sqlx::{
    postgres::{PgConnectOptions, PgPool},
    Pool, Postgres,
};
use tokio::sync::OnceCell;

pub struct Database;
static POOL: OnceCell<Pool<Postgres>> = OnceCell::const_new();
static REDIS_CONNECTION: OnceCell<redis::aio::MultiplexedConnection> = OnceCell::const_new();

impl Database {
    pub async fn pg() -> Pool<Postgres> {
        let pool = POOL
            .get_or_try_init(|| async {
                let username = &get_env_variable("PG_USER");
                let password = &get_env_variable("PG_PASSWORD");
                let port = get_env_variable("PG_PORT")
                    .parse::<u16>()
                    .expect("PG_PORT must be an unsigned integer.");
                let database = &get_env_variable("PG_DB");
                let host = &get_env_variable("PG_HOST");

                let pg_connect_options = PgConnectOptions::new()
                    .username(username)
                    .password(password)
                    .port(port)
                    .database(database)
                    .host(host);

                PgPool::connect_with(pg_connect_options).await
            })
            .await
            .expect("Failed to initialize connection to PostgreSQL.");

        pool.clone()
    }

    pub async fn redis() -> redis::aio::MultiplexedConnection {
        let connection = REDIS_CONNECTION
            .get_or_try_init(|| async {
                let redis_host = get_env_variable("REDIS_HOST");
                let redis_port = get_env_variable("REDIS_PORT");
                let redis_db = get_env_variable("MS_IAM_REDIS_DB");

                let client = Client::open(format!(
                    "redis://{}:{}/{}",
                    redis_host, redis_port, redis_db
                ))
                .expect("Failed to open Redis client.");

                client.get_multiplexed_async_connection().await
            })
            .await
            .expect("Failed to initialize connection to Redis.");

        connection.clone()
    }
}
