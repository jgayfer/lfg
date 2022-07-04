use crate::model::User;

#[derive(Clone, Debug)]
pub struct Integration {
    pub id: i32,
    pub owner: User,
    pub webhook_url: String,
    pub success_url: String,
}

impl PartialEq for Integration {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
