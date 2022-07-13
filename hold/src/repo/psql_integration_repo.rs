use async_trait::async_trait;
use std::error::Error;

use sqlx::{Pool, Postgres};

use lfg_core::{
    model::{Integration, NewIntegration},
    repo::IntegrationRepo,
};

pub struct PsqlIntegrationRepo {
    pool: Pool<Postgres>,
}

impl PsqlIntegrationRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IntegrationRepo for PsqlIntegrationRepo {
    async fn find(&self, integration_id: i32) -> Result<Option<Integration>, Box<dyn Error>> {
        let integration = sqlx::query_as!(
            Integration,
            "SELECT * FROM integrations WHERE id = $1",
            integration_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(integration)
    }
    async fn create(
        &mut self,
        new_integration: NewIntegration,
    ) -> Result<Integration, Box<dyn Error>> {
        let integration = sqlx::query_as!(
            Integration,
            r#"
            INSERT INTO integrations(success_url, webhook_url)
            VALUES ($1, $2)
            RETURNING *
            "#,
            new_integration.success_url,
            new_integration.webhook_url
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(integration)
    }
}
