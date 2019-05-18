pub struct AI {}

impl AI {
    pub fn new() -> Self {
        AI {}
    }
    // demo app interaction
    pub fn be_smart(&self, m: &str) -> Option<String> {
        if m.len() < 4 {
            return None;
        }
        Some(format!("I am smart and can read long sentences"))
    }
}
