use std::any::Any;

use bevy::prelude::*;
use datasize::DataSize;

use crate::{estimate_stack_and_heap_size, MemoryConfig, MemoryUsage};

pub fn update_datasize_for_component<T>(
    query: Query<&T>,
    memory_config: Res<MemoryConfig>,
    mut memory_usage: ResMut<MemoryUsage>,
) where
    T: Any + Component + DataSize,
{
    if !memory_config.tracking_enabled {
        return;
    }

    let total_bytes: usize = query.iter().map(estimate_stack_and_heap_size).sum();

    memory_usage.update_usage::<T>(total_bytes);
}
