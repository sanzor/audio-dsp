use async_trait::async_trait;
use sqlx::PgPool;
use tracing::error;

use crate::domain::data_error::DataError;
use crate::domain::db::db_invoice::{DbInvoice, InvoiceId};
use crate::domain::db::db_organization::OrganizationId;
use crate::invoices::create_invoice_params::CreateInvoiceParams;
use crate::invoices::data_provider::invoices_data_provider::InvoicesDataProvider;

const SELECT: &str = "SELECT id, org_id, stripe_invoice_id, amount, currency, status, hosted_url, created_at::text FROM invoices";

pub struct InvoicesDataProviderService {
    pool: PgPool,
}

impl InvoicesDataProviderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InvoicesDataProvider for InvoicesDataProviderService {
    async fn create_invoice(&self, params: CreateInvoiceParams) -> Result<DbInvoice, DataError> {
        sqlx::query_as::<_, DbInvoice>(
            "INSERT INTO invoices (org_id, stripe_invoice_id, amount, currency, status, hosted_url) \
             VALUES ($1, $2, $3, $4, $5, $6) \
             RETURNING id, org_id, stripe_invoice_id, amount, currency, status, hosted_url, created_at::text",
        )
        .bind(params.org_id)
        .bind(&params.stripe_invoice_id)
        .bind(params.amount)
        .bind(&params.currency)
        .bind(&params.status)
        .bind(&params.hosted_url)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!(stripe_invoice_id = %params.stripe_invoice_id, error = %e, "create invoice failed");
            DataError::from(e)
        })
    }

    async fn get_invoice(&self, id: InvoiceId) -> Result<Option<DbInvoice>, DataError> {
        sqlx::query_as::<_, DbInvoice>(&format!("{SELECT} WHERE id = $1"))
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!(id, error = %e, "get invoice failed");
                DataError::from(e)
            })
    }

    async fn delete_invoice(&self, id: InvoiceId) -> Result<bool, DataError> {
        sqlx::query("DELETE FROM invoices WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map(|r| r.rows_affected() > 0)
            .map_err(|e| {
                error!(id, error = %e, "delete invoice failed");
                DataError::from(e)
            })
    }

    async fn list_by_org_paginated(
        &self,
        org_id: OrganizationId,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<DbInvoice>, i64), DataError> {
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM invoices WHERE org_id = $1")
            .bind(org_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                error!(org_id, error = %e, "count invoices by org failed");
                DataError::from(e)
            })?;

        let records = sqlx::query_as::<_, DbInvoice>(&format!(
            "{SELECT} WHERE org_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        ))
        .bind(org_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!(org_id, error = %e, "list invoices by org failed");
            DataError::from(e)
        })?;

        Ok((records, total))
    }

    async fn list_all_paginated(
        &self,
        org_id: Option<OrganizationId>,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<DbInvoice>, i64), DataError> {
        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM invoices WHERE ($1::bigint IS NULL OR org_id = $1::bigint)",
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "count all invoices failed");
            DataError::from(e)
        })?;

        let records = sqlx::query_as::<_, DbInvoice>(&format!(
            "{SELECT} WHERE ($1::bigint IS NULL OR org_id = $1::bigint) \
             ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        ))
        .bind(org_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "list all invoices failed");
            DataError::from(e)
        })?;

        Ok((records, total))
    }
}
