//! Systems used by this library.

use std::any::Any;

use bevy::prelude::*;
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
    if !memory_config.global {
        return;
    }

    let stats = MemoryStats::from_values(query.iter());

    memory_usage.update_stats_fast::<T>(stats);
}
