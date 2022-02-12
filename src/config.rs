#[derive(Debug)]
pub struct MemoryConfig {
    pub tracking_enabled: bool,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            tracking_enabled: true,
        }
    }
}
