#[derive(Clone, Debug)]
pub struct Ctx {
    pub user_id: Option<u64>,
}

impl Ctx {
    pub fn new() -> Self {
        Self { user_id: None }
    }

    pub fn set(&mut self, user_id: u64) {
        self.user_id = Some(user_id);
    }

    pub fn user_id(&self) -> u64 {
        self.user_id.unwrap()
    }
}

impl Default for Ctx {
    fn default() -> Self {
        Self::new()
    }
}
