use std::collections::HashMap;
use std::error::Error;

use async_trait::async_trait;

use lfg_core::{
    model::{Integration, NewIntegration},
    repo::IntegrationRepo,
};

pub struct InMemoryIntegrationRepo {
    integrations: HashMap<i32, Integration>,
}

impl InMemoryIntegrationRepo {
    pub fn new(integrations: HashMap<i32, Integration>) -> Self {
        Self { integrations }
    }
}

#[async_trait]
impl IntegrationRepo for InMemoryIntegrationRepo {
    async fn find(&self, integration_id: i32) -> Result<Option<Integration>, Box<dyn Error>> {
        Ok(self.integrations.get(&integration_id).cloned())
    }

    async fn create(
        &mut self,
        new_integration: NewIntegration,
    ) -> Result<Integration, Box<dyn Error>> {
        let integration = Integration {
            id: 1,
            success_url: new_integration.success_url,
            webhook_url: new_integration.webhook_url,
        };
        self.integrations.insert(1, integration.clone());
        Ok(integration)
    }
}

#[cfg(test)]
mod tests {
    use crate::fixtures::{TestIntegration, TestNewIntegration};

    use super::*;

    #[tokio::test]
    async fn find_empty() {
        let repo = InMemoryIntegrationRepo::new(HashMap::new());

        let integration = repo.find(1).await.unwrap();

        assert!(integration.is_none());
    }

    #[tokio::test]
    async fn find_some() {
        let integration = TestIntegration::new().id(1).build();
        let integrations = HashMap::from([(1, integration.clone())]);
        let repo = InMemoryIntegrationRepo::new(integrations);

        let option = repo.find(1).await.unwrap();

        assert_eq!(option, Some(integration));
    }

    #[tokio::test]
    async fn create() {
        let mut repo = InMemoryIntegrationRepo::new(HashMap::new());
        let new_integration = TestNewIntegration::new().build();

        let integration = repo.create(new_integration).await.unwrap();

        assert_eq!(repo.find(integration.id).await.unwrap(), Some(integration));
    }
}
