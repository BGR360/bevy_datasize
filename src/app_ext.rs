use std::any::Any;

use bevy::{
    app::App,
    asset::Asset,
    ecs::{component::Component, system::Resource},
};

use crate::{systems, DataSize, MemoryUsage};

/// [`App`] extension methods to register types for memory usage tracking.
pub trait RegisterSizedTypes {
    /// Registers the given [`Component`] type with the
    /// [`MemoryUsagePlugin`][crate::MemoryUsagePlugin].
    ///
    /// The given type `T` will be available to query on the [`MemoryUsage`]
    /// resource.
    fn register_sized_component<T>(&mut self) -> &mut Self
    where
        T: Any + DataSize + Component;

    /// Registers the given [`Resource`] type with the
    /// [`MemoryUsagePlugin`][crate::MemoryUsagePlugin].
    ///
    /// The given type `T` will be available to query on the [`MemoryUsage`]
    /// resource.
    fn register_sized_resource<T>(&mut self) -> &mut Self
    where
        T: Any + DataSize + Resource;

    /// Registers the given [`Asset`] type with the
    /// [`MemoryUsagePlugin`][crate::MemoryUsagePlugin].
    ///
    /// The given type `T` will be available to query on the [`MemoryUsage`]
    /// resource.
    fn register_sized_asset<T>(&mut self) -> &mut Self
    where
        T: Any + DataSize + Asset;
}

impl RegisterSizedTypes for App {
    fn register_sized_component<T>(&mut self) -> &mut Self
    where
        T: Any + Component + DataSize,
    {
        register_type::<T>(self);

        self.add_system(systems::update_stats_for_component::<T>);

        self
    }

    fn register_sized_resource<T>(&mut self) -> &mut Self
    where
        T: Any + DataSize + Resource,
    {
        register_type::<T>(self);

        self.add_system(systems::update_stats_for_resource::<T>);

        self
    }

    fn register_sized_asset<T>(&mut self) -> &mut Self
    where
        T: Any + DataSize + Asset,
    {
        register_type::<T>(self);

        self.add_system(systems::update_stats_for_asset::<T>);

        self
    }
}

fn register_type<T>(app: &mut App)
where
    T: Any + DataSize,
{
    let mut memory_usage = app.world.get_resource_mut::<MemoryUsage>().expect(
        "Cannot find resource `MemoryUsage`. Did you forget to add the `MemoryUsagePlugin`?",
    );

    memory_usage.register_type::<T>();
}
