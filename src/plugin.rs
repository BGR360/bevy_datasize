use bevy::app::Plugin;

use crate::{MemoryConfig, MemoryUsage};

/// Adds memory usage tracking to Apps.
///
/// You can configure this plugin using the [`MemoryConfig`] resource.
pub struct MemoryUsagePlugin;

impl Plugin for MemoryUsagePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<MemoryConfig>();
        app.init_resource::<MemoryUsage>();
    }
}
