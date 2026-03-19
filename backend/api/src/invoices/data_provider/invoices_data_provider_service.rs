use async_trait::async_trait;
use sqlx::PgPool;
use tracing::error;
use crate::domain::data_error::DataError;
use crate::domain::db::db_invoice::{DbInvoice, InvoiceId};
use crate::invoices::create_invoice_params::CreateInvoiceParams;
use crate::invoices::data_provider::invoices_data_provider::InvoicesDataProvider;

const SELECT: &str = "SELECT id, user_id, stripe_invoice_id, amount, currency, status, hosted_url, created_at::text FROM invoices";

pub struct InvoicesDataProviderService {
    pool: PgPool,
}

impl InvoicesDataProviderService {
    pub fn new(pool: PgPool) -> Self { Self { pool } }
}

#[async_trait]
impl InvoicesDataProvider for InvoicesDataProviderService {
    async fn create_invoice(&self, params: CreateInvoiceParams) -> Result<DbInvoice, DataError> {
        sqlx::query_as::<_, DbInvoice>(
            "INSERT INTO invoices (user_id, stripe_invoice_id, amount, currency, status, hosted_url) \
             VALUES ($1, $2, $3, $4, $5, $6) \
             RETURNING id, user_id, stripe_invoice_id, amount, currency, status, hosted_url, created_at::text",
        )
        .bind(params.user_id)
        .bind(&params.stripe_invoice_id)
        .bind(params.amount)
        .bind(&params.currency)
        .bind(&params.status)
        .bind(&params.hosted_url)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| { error!(error = %e, "create invoice failed"); DataError::from(e) })
    }

    async fn get_invoice(&self, id: InvoiceId) -> Result<Option<DbInvoice>, DataError> {
        sqlx::query_as::<_, DbInvoice>(&format!("{SELECT} WHERE id = $1"))
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| { error!(id, error = %e, "get invoice failed"); DataError::from(e) })
    }

    async fn delete_invoice(&self, id: InvoiceId) -> Result<bool, DataError> {
        sqlx::query("DELETE FROM invoices WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map(|r| r.rows_affected() > 0)
            .map_err(|e| { error!(id, error = %e, "delete invoice failed"); DataError::from(e) })
    }

    async fn list_by_user_paginated(&self, user_id: i64, offset: i64, limit: i64) -> Result<(Vec<DbInvoice>, i64), DataError> {
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM invoices WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| { error!(error = %e, "count invoices by user failed"); DataError::from(e) })?;
        let records = sqlx::query_as::<_, DbInvoice>(&format!(
            "{SELECT} WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        ))
        .bind(user_id).bind(limit).bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| { error!(error = %e, "list invoices by user failed"); DataError::from(e) })?;
        Ok((records, total))
    }

    async fn list_all_paginated(&self, user_id: Option<i64>, offset: i64, limit: i64) -> Result<(Vec<DbInvoice>, i64), DataError> {
        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM invoices WHERE ($1::bigint IS NULL OR user_id = $1)"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| { error!(error = %e, "count all invoices failed"); DataError::from(e) })?;
        let records = sqlx::query_as::<_, DbInvoice>(&format!(
            "{SELECT} WHERE ($1::bigint IS NULL OR user_id = $1) ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        ))
        .bind(user_id).bind(limit).bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| { error!(error = %e, "list all invoices failed"); DataError::from(e) })?;
        Ok((records, total))
    }
}
