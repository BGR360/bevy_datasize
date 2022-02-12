use std::any::Any;

use bevy::{
    app::App,
    asset::Asset,
    ecs::{component::Component, schedule::IntoSystemDescriptor, system::Resource},
};

use crate::{systems, DataSize, DataSizeEstimator, MemoryUsage};

/// [`App`] extension methods to register types for memory usage tracking.
pub trait RegisterSizedTypes {
    /// Registers the given [`Component`] type with the
    /// [`MemoryUsagePlugin`][crate::MemoryUsagePlugin].
    ///
    /// The given type `T` will be available to query on the [`MemoryUsage`]
    /// resource.
    fn register_sized_component<T>(&mut self) -> &mut Self
    where
        T: Any + DataSize + Component,
    {
        self.register_sized_type::<T, T, _, _>(systems::update_stats_for_component::<T, T>)
    }

    /// Registers the given [`Resource`] type with the
    /// [`MemoryUsagePlugin`][crate::MemoryUsagePlugin].
    ///
    /// The given type `T` will be available to query on the [`MemoryUsage`]
    /// resource.
    fn register_sized_resource<T>(&mut self) -> &mut Self
    where
        T: Any + DataSize + Resource,
    {
        self.register_sized_type::<T, T, _, _>(systems::update_stats_for_resource::<T, T>)
    }

    /// Registers the given [`Asset`] type with the
    /// [`MemoryUsagePlugin`][crate::MemoryUsagePlugin].
    ///
    /// The given type `T` will be available to query on the [`MemoryUsage`]
    /// resource.
    fn register_sized_asset<T>(&mut self) -> &mut Self
    where
        T: Any + DataSize + Asset,
    {
        self.register_sized_type::<T, T, _, _>(systems::update_stats_for_asset::<T, T>)
    }

    /// Registers a type that can be sized with the given [`DataSizeEstimator`]
    /// type and updated using the provided `system`.
    fn register_sized_type<T, E, S, Params>(&mut self, system: S) -> &mut Self
    where
        T: Any,
        E: DataSizeEstimator<T>,
        S: IntoSystemDescriptor<Params>;
}

impl RegisterSizedTypes for App {
    fn register_sized_type<T, E, S, Params>(&mut self, system: S) -> &mut Self
    where
        T: Any,
        E: DataSizeEstimator<T>,
        S: IntoSystemDescriptor<Params>,
    {
        register_type::<T>(self);

        self.add_system(system);

        self
    }
}

fn register_type<T>(app: &mut App)
where
    T: Any,
{
    let mut memory_usage = app.world.get_resource_mut::<MemoryUsage>().expect(
        "Cannot find resource `MemoryUsage`. Did you forget to add the `MemoryUsagePlugin`?",
    );

    memory_usage.register_type::<T>();
}
