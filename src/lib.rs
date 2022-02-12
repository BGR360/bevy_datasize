//! This is a library for tracking memory usage in [Bevy](https://lib.rs/bevy) apps.
//!
//! It is based on the [`datasize`] crate.
//!
//! # Example
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_datasize::prelude::*;
//!
//! #[derive(Component, DataSize)]
//! struct MyComponent {
//!     data: Vec<u8>,
//! }
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(MinimalPlugins)
//!         .add_plugin(MemoryUsagePlugin)
//!         .register_sized_component::<MyComponent>()
//!         .add_system(print_datasize)
//!         .run();
//! }
//!
//! fn print_datasize(memory_usage: Res<MemoryUsage>) {
//!     let MemoryStats {
//!         count,
//!         total_stack_bytes,
//!         total_heap_bytes,
//!     } = memory_usage.get_stats::<MyComponent>().unwrap();
//!
//!     println!("MyComponent count: {count}");
//!     println!("MyComponent total stack usage: {total_stack_bytes} bytes");
//!     println!("MyComponent total heap usage: {total_heap_bytes} bytes");
//! }
//! ```
//!
//! See the [`datasize`] docs for more information on the [`DataSize`] trait.

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

#[allow(missing_docs)]
pub mod prelude {
    pub use crate::{
        DataSize, MemoryConfig, MemoryStats, MemoryUsage, MemoryUsagePlugin, RegisterSizedTypes,
    };
}

// Make sure the README example compiles
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
pub struct ReadmeTest;
