/// Configuration for the [`MemoryUsagePlugin`][crate::MemoryUsagePlugin].
#[derive(Debug)]
pub struct MemoryConfig {
    /// Whether to track memory usage for all registered types.
    pub global: bool,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self { global: true }
    }
}
