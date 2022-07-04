use lfg_core::model::Integration;

use super::TestUser;

pub struct TestIntegration {
    integration: Integration,
}

impl TestIntegration {
    pub fn new() -> Self {
        Self {
            integration: Integration {
                id: 1,
                owner: TestUser::new().build(),
                webhook_url: "https://test-webhook.com".into(),
                success_url: "https://test-success.com".into(),
            },
        }
    }

    pub fn id(mut self, id: i32) -> Self {
        self.integration.id = id;
        self
    }

    pub fn build(self) -> Integration {
        self.integration
    }
}
