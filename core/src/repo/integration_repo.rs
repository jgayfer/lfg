use std::error::Error;

use async_trait::async_trait;

use crate::model::{Integration, NewIntegration};

#[async_trait]
pub trait IntegrationRepo {
    async fn find(&self, integration_id: i32) -> Result<Option<Integration>, Box<dyn Error>>;
    async fn create(
        &mut self,
        new_integration: NewIntegration,
    ) -> Result<Integration, Box<dyn Error>>;
}
