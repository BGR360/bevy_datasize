//! This is a library for tracking memory usage in [Bevy](https://lib.rs/bevy)
//! apps.
//!
//! `bevy_datasize` uses the [`DataSize`] trait from the [`datasize`] crate to
//! estimate the runtime memory usage of any components, resources, or assets
//! that are registered with the [`MemoryUsagePlugin`].
//!
//! The [`DataSize`] trait can be derived for your own custom types, and you can
//! inject custom estimators for third party types that do not implement
//! `DataSize`. See the [`datasize`] docs for more info on that.
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
#![warn(missing_docs)]

pub use datasize;

pub use datasize::DataSize;

mod app_ext;
pub mod builtins;
mod config;
pub mod estimator;
mod plugin;
mod resource;
mod stats;
pub mod systems;

pub use app_ext::RegisterSizedTypes;
pub use config::MemoryConfig;
#[doc(inline)]
pub use estimator::DataSizeEstimator;
pub use plugin::MemoryUsagePlugin;
pub use resource::MemoryUsage;
pub use stats::MemoryStats;

#[allow(missing_docs)]
pub mod prelude {
    pub use crate::{
        builtins::DefaultMemoryUsagePlugins, DataSize, MemoryConfig, MemoryStats, MemoryUsage,
        MemoryUsagePlugin, RegisterSizedTypes,
    };
}

// Make sure the README example compiles
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
pub struct ReadmeTest;
