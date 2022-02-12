//! Systems used by this library.

use std::any::Any;

use bevy::{
    asset::{Asset, Assets},
    ecs::{
        component::Component,
        system::{Query, Res, Resource},
    },
};
use datasize::DataSize;

use crate::{MemoryConfig, MemoryStats, MemoryUsage};

/// This system iterates through all components of type `T` and updates their
/// [`MemoryStats`].
pub fn update_stats_for_component<T>(
    query: Query<&T>,
    memory_config: Res<MemoryConfig>,
    memory_usage: Res<MemoryUsage>,
) where
    T: Any + Component + DataSize,
{
    update_stats::<T, _>(memory_config, memory_usage, || {
        MemoryStats::from_values(query.iter())
    });
}

/// This system updates the [`MemoryStats`] for the given resource type `T`.
pub fn update_stats_for_resource<T>(
    resource: Res<T>,
    memory_config: Res<MemoryConfig>,
    memory_usage: Res<MemoryUsage>,
) where
    T: Any + DataSize + Resource,
{
    update_stats::<T, _>(memory_config, memory_usage, || {
        MemoryStats::from_value(&*resource)
    });
}

/// This system updates the [`MemoryStats`] for the given asset type `T`.
pub fn update_stats_for_asset<T>(
    assets: Res<Assets<T>>,
    memory_config: Res<MemoryConfig>,
    memory_usage: Res<MemoryUsage>,
) where
    T: Any + DataSize + Asset,
{
    update_stats::<T, _>(memory_config, memory_usage, || {
        MemoryStats::from_values(assets.iter().map(|(_handle, asset)| asset))
    });
}

fn update_stats<T, F>(memory_config: Res<MemoryConfig>, memory_usage: Res<MemoryUsage>, op: F)
where
    T: Any + DataSize,
    F: FnOnce() -> MemoryStats,
{
    if !memory_config.global {
        return;
    }

    let stats = op();

    memory_usage.update_stats_fast::<T>(stats);
}
