use std::any::Any;

use bevy::app::App;
use bevy::ecs::component::Component;

use crate::{systems::update_stats_for_component, DataSize, MemoryUsage};

/// [`App`] extension methods to register types for memory usage tracking.
pub trait RegisterSizedTypes {
    /// Registers the given [`Component`] type with the
    /// [`MemoryUsagePlugin`][crate::MemoryUsagePlugin].
    ///
    /// The given type `T` will be available to query on the [`MemoryUsage`]
    /// resource.
    fn register_sized_component<T>(&mut self) -> &mut Self
    where
        T: Any + Component + DataSize;
}

impl RegisterSizedTypes for App {
    fn register_sized_component<T>(&mut self) -> &mut Self
    where
        T: Any + Component + DataSize,
    {
        let mut memory_usage = self.world.get_resource_mut::<MemoryUsage>().expect(
            "Cannot find resource `MemoryUsage`. Did you forget to add the `MemoryUsagePlugin`?",
        );

        memory_usage.register_type::<T>();

        self.add_system(update_stats_for_component::<T>);

        self
    }
}
