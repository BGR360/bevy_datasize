//! Support for Bevy render resources.

use bevy::app::{PluginGroup, PluginGroupBuilder};

#[cfg(feature = "image")]
mod image;
#[cfg(feature = "mesh")]
mod mesh;

#[cfg(feature = "image")]
pub use image::ImageMemoryUsagePlugin;
#[cfg(feature = "mesh")]
pub use mesh::MeshMemoryUsagePlugin;

/// Adds memory tracking for the components, resources, and assets that are part
/// of Bevy's [`RenderPlugin`][bevy::render::RenderPlugin].
pub struct RenderMemoryUsagePlugins;

impl PluginGroup for RenderMemoryUsagePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        #[cfg(feature = "image")]
        group.add(ImageMemoryUsagePlugin);

        #[cfg(feature = "mesh")]
        group.add(MeshMemoryUsagePlugin);
    }
}
