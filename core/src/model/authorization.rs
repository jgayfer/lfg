use crate::model::{Integration, User};

pub struct Authorization {
    pub user: User,
    pub integration: Integration,
}
