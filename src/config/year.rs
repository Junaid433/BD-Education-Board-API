#[derive(Debug, Clone)]
pub struct Year(pub u16);

impl Year {
    pub fn as_string(&self) -> String {
        self.0.to_string()
    }
}
