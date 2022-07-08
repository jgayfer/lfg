#[derive(Clone, Debug)]
pub struct Integration {
    pub id: i32,
    pub webhook_url: String,
    pub success_url: String,
}

impl PartialEq for Integration {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct NewIntegration {
    pub webhook_url: String,
    pub success_url: String,
}
