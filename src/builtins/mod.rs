//! Support for Bevy's built-in types.

use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::MemoryUsagePlugin;

pub mod render;
pub mod transform;

pub use render::RenderMemoryUsagePlugin;
pub use transform::TransformMemoryUsagePlugin;

/// Adds memory tracking for the components, resources, and assets that are part
/// of Bevy's [`DefaultPlugins`][bevy::prelude::DefaultPlugins].
///
/// This group registers the [`MemoryUsagePlugin`] as well as the following:
///
/// * [`RenderMemoryUsagePlugin`]
pub struct DefaultMemoryUsagePlugins;

impl PluginGroup for DefaultMemoryUsagePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(MemoryUsagePlugin);

        group.add(RenderMemoryUsagePlugin);
        group.add(TransformMemoryUsagePlugin);
    }
}
