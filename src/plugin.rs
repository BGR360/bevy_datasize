use bevy::app::Plugin;

use crate::{MemoryConfig, MemoryUsage};

pub struct MemoryUsagePlugin;

impl Plugin for MemoryUsagePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<MemoryConfig>();
        app.init_resource::<MemoryUsage>();
    }
}
