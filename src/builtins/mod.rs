//! Support for Bevy's built-in types.

use std::any::Any;

use bevy::{
    app::{App, PluginGroup, PluginGroupBuilder},
    asset::Asset,
    ecs::{component::Component, system::Resource},
};

use crate::{systems, DataSizeEstimator, MemoryUsagePlugin, RegisterSizedTypes};

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

pub(crate) trait RegisterTypesWithEstimator: RegisterSizedTypes {
    fn register_component_with_estimator<T, E>(&mut self) -> &mut Self
    where
        T: Any + Component,
        E: DataSizeEstimator<T> + Default + 'static,
    {
        self.register_sized_type::<T, _, _>(systems::update_stats_for_component::<T, E>)
    }

    fn register_resource_with_estimator<T, E>(&mut self) -> &mut Self
    where
        T: Any + Resource,
        E: DataSizeEstimator<T> + Default + 'static,
    {
        self.register_sized_type::<T, _, _>(systems::update_stats_for_resource::<T, E>)
    }

    fn register_asset_with_estimator<T, E>(&mut self) -> &mut Self
    where
        T: Any + Asset,
        E: DataSizeEstimator<T> + Default + 'static,
    {
        self.register_sized_type::<T, _, _>(systems::update_stats_for_asset::<T, E>)
    }
}

impl RegisterTypesWithEstimator for App {}
