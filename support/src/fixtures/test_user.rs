use lfg_core::model::User;

pub struct TestUser {
    user: User,
}

impl TestUser {
    pub fn new() -> Self {
        Self {
            user: User {
                id: 1,
                name: "Jim Raynor".into(),
            },
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.user.name = name.into();
        self
    }

    pub fn build(self) -> User {
        self.user
    }
}
