use deadpool_redis::redis::aio::MultiplexedConnection;
use sqlx::PgPool;

pub struct SyncWorkerProcessorParams {
    pub redis: MultiplexedConnection,
    pub db_conn: PgPool,
}
