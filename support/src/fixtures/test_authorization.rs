use lfg_core::model::{Authorization, Integration, User};

use super::{TestIntegration, TestUser};

pub struct TestAuthorization {
    authorization: Authorization,
}

impl TestAuthorization {
    pub fn new() -> Self {
        Self {
            authorization: Authorization {
                user: TestUser::new().build(),
                integration: TestIntegration::new().build(),
            },
        }
    }

    pub fn user(mut self, user: User) -> Self {
        self.authorization.user = user;
        self
    }

    pub fn integration(mut self, integration: Integration) -> Self {
        self.authorization.integration = integration;
        self
    }

    pub fn build(self) -> Authorization {
        self.authorization
    }
}
