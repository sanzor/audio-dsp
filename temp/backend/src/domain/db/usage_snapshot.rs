use crate::domain::db::db_token_bucket_usage::DbTokenBucketUsage;
use crate::domain::db::db_token_window_usage::DbTokenWindowUsage;

pub enum UsageSnapshot {
    TokenBucket(DbTokenBucketUsage),
    TokenWindow(DbTokenWindowUsage),
}
