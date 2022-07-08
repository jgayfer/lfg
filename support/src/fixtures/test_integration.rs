use lfg_core::model::Integration;

pub struct TestIntegration {
    integration: Integration,
}

impl TestIntegration {
    pub fn new() -> Self {
        Self {
            integration: Integration {
                id: 1,
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

pub struct TestNewIntegration {
    new_integration: NewIntegration,
}

impl TestNewIntegration {
    pub fn new() -> Self {
        Self {
            new_integration: NewIntegration {
                webhook_url: "https://test-webhook.com".into(),
                success_url: "https://test-success.com".into(),
            },
        }
    }

    pub fn build(self) -> NewIntegration {
        self.new_integration
    }
}
