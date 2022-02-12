#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub use datasize::DataSize;

mod app_ext;
mod config;
mod plugin;
mod resource;
mod stats;
pub mod systems;

pub use app_ext::RegisterSizedTypes;
pub use config::MemoryConfig;
pub use plugin::MemoryUsagePlugin;
pub use resource::MemoryUsage;
pub use stats::MemoryStats;
