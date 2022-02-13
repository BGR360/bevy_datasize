//! Support for Bevy render resources.

use bevy::app::{App, Plugin};

#[cfg(feature = "image")]
mod image;
#[cfg(feature = "material")]
mod material;
#[cfg(feature = "mesh")]
mod mesh;

#[cfg(feature = "image")]
pub use image::ImageMemoryUsagePlugin;
#[cfg(feature = "material")]
pub use material::MaterialMemoryUsagePlugin;
#[cfg(feature = "mesh")]
pub use mesh::MeshMemoryUsagePlugin;

/// Adds memory tracking for the components, resources, and assets that are part
/// of Bevy's [`RenderPlugin`][bevy::render::RenderPlugin].
pub struct RenderMemoryUsagePlugin;

impl Plugin for RenderMemoryUsagePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "image")]
        app.add_plugin(ImageMemoryUsagePlugin);

        #[cfg(feature = "material")]
        app.add_plugin(MaterialMemoryUsagePlugin);

        #[cfg(feature = "mesh")]
        app.add_plugin(MeshMemoryUsagePlugin);
    }
}
