use std::collections::HashMap;
use std::error::Error;

use async_trait::async_trait;

use lfg_core::{model::Integration, repo::IntegrationRepo};

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn find_empty() {
        let repo = InMemoryIntegrationRepo::new(HashMap::new());

        let integration = repo.find(1).await.unwrap();

        assert!(integration.is_none());
    }

    #[tokio::test]
    async fn find_some() {
        let integration = Integration { id: 1 };
        let integrations = HashMap::from([(1, integration.clone())]);
        let repo = InMemoryIntegrationRepo::new(integrations);

        let option = repo.find(1).await.unwrap();

        assert_eq!(option, Some(integration));
    }
}
