//! Systems used by this library.

use std::any::Any;

use bevy::{
    asset::{Asset, Assets},
    ecs::{
        component::Component,
        system::{Query, Res, Resource},
    },
};

use crate::{DataSizeEstimator, MemoryConfig, MemoryStats, MemoryUsage};

/// This system updates the [`MemoryStats`] for the given component type `T`
/// using the given [`DataSizeEstimator`].
pub fn update_stats_for_component<T, E>(
    query: Query<&T>,
    memory_config: Res<MemoryConfig>,
    memory_usage: Res<MemoryUsage>,
) where
    T: Any + Component,
    E: DataSizeEstimator<T>,
{
    update_stats::<T, _>(memory_config, memory_usage, || {
        MemoryStats::from_values_with_estimator::<T, E, _>(query.iter())
    });
}

/// This system updates the [`MemoryStats`] for the given resource type `T`
/// using the given [`DataSizeEstimator`].
pub fn update_stats_for_resource<T, E>(
    resource: Res<T>,
    memory_config: Res<MemoryConfig>,
    memory_usage: Res<MemoryUsage>,
) where
    T: Any + Resource,
    E: DataSizeEstimator<T>,
{
    update_stats::<T, _>(memory_config, memory_usage, || {
        MemoryStats::from_value_with_estimator::<T, E>(&*resource)
    });
}

/// This system updates the [`MemoryStats`] for the given asset type `T`
/// using the given [`DataSizeEstimator`].
pub fn update_stats_for_asset<T, E>(
    assets: Res<Assets<T>>,
    memory_config: Res<MemoryConfig>,
    memory_usage: Res<MemoryUsage>,
) where
    T: Any + Asset,
    E: DataSizeEstimator<T>,
{
    update_stats::<T, _>(memory_config, memory_usage, || {
        MemoryStats::from_values_with_estimator::<T, E, _>(
            assets.iter().map(|(_handle, asset)| asset),
        )
    });
}

fn update_stats<T, F>(memory_config: Res<MemoryConfig>, memory_usage: Res<MemoryUsage>, op: F)
where
    T: Any,
    F: FnOnce() -> MemoryStats,
{
    if !memory_config.global {
        return;
    }

    let stats = op();

    memory_usage.update_stats_fast::<T>(stats);
}
