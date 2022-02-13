use bevy::app::Plugin;

#[cfg(feature = "bevy_render")]
use bevy::{
    ecs::system::{Commands, Res},
    render::{RenderApp, RenderStage},
};

use crate::{MemoryConfig, MemoryUsage};

/// Adds memory usage tracking to Bevy apps.
///
/// Types can be registered for memory usage tracking on the [`App`] using the
/// [`RegisterSizedTypes`] extension methods.
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

        #[cfg(feature = "bevy_render")]
        {
            let memory_config = app.world.get_resource::<MemoryConfig>().unwrap().clone();
            let memory_usage = app.world.get_resource::<MemoryUsage>().unwrap().clone();

            let render_app = match app.get_sub_app_mut(RenderApp) {
                Ok(render_app) => render_app,
                _ => return,
            };

            render_app.insert_resource(memory_config);
            render_app.insert_resource(memory_usage);
            render_app.add_system_to_stage(RenderStage::Extract, synchronize_configs);
        }
    }
}

/// This system copies the [`MemoryConfig`] from the main world to the render
/// sub-world.
#[cfg(feature = "bevy_render")]
fn synchronize_configs(memory_config: Res<MemoryConfig>, mut render_commands: Commands) {
    let clone = memory_config.clone();

    render_commands.insert_resource(clone);
}
