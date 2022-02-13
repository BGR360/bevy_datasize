use bevy::app::Plugin;

use crate::{MemoryConfig, MemoryUsage};

/// Adds memory usage tracking to Bevy apps.
///
/// Types must be registered on the [`App`] using the [`RegisterSizedTypes`]
/// extension methods in order to track their memory usage.
///
/// You can query the memory usage for any registered type at runtime using the
/// [`MemoryUsage`] resource.
///
/// You can configure the runtime behavior of this plugin using the
/// [`MemoryConfig`] resource.
///
/// [`App`]: bevy::app::App
/// [`RegisterSizedTypes`]: crate::RegisterSizedTypes
pub struct MemoryUsagePlugin;

impl Plugin for MemoryUsagePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<MemoryConfig>();
        app.init_resource::<MemoryUsage>();
    }
}
