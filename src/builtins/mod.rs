//! Support for Bevy's built-in types.

use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::MemoryUsagePlugin;

pub mod render;

pub use render::RenderMemoryUsagePlugins;

/// Adds memory tracking for the components, resources, and assets that are part
/// of Bevy's [`DefaultPlugins`][bevy::prelude::DefaultPlugins].
///
/// This group registers the [`MemoryUsagePlugin`] as well as the following:
///
/// * [`RenderMemoryUsagePlugins`]
pub struct DefaultMemoryUsagePlugins;

impl PluginGroup for DefaultMemoryUsagePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(MemoryUsagePlugin);

        RenderMemoryUsagePlugins.build(group);
    }
}
