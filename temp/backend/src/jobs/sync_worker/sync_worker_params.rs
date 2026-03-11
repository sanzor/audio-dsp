use deadpool_redis::redis::aio::MultiplexedConnection;
use sqlx::PgPool;

pub struct SyncWorkerParams {
    pub redis: MultiplexedConnection,
    pub db_conn: PgPool,
}
