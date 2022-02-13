//! Extension traits for [`App`].

use std::any::Any;

use bevy::{
    app::App,
    asset::Asset,
    ecs::{
        component::Component, schedule::IntoSystemDescriptor, schedule::StageLabel,
        system::Resource,
    },
    prelude::CoreStage,
};

#[cfg(feature = "bevy_render")]
use bevy::render::{render_asset::RenderAsset, RenderApp, RenderStage};

use crate::{
    estimator::{ForwardingEstimator, FromConfig},
    systems, DataSize, DataSizeEstimator, MemoryUsage,
};

/// [`App`] extension methods to register [`DataSize`] types for memory usage
/// tracking.
///
/// For types that do not implement [`DataSize`], you will need to implement a
/// [`DataSizeEstimator`] for them and register them using the methods in
/// [`RegisterTypesWithEstimator`].
pub trait RegisterSizedTypes: RegisterTypesWithEstimator {
    /// Registers the given [`Component`] type with the
    /// [`MemoryUsagePlugin`][crate::MemoryUsagePlugin].
    ///
    /// The given type `T` will be available to query on the [`MemoryUsage`]
    /// resource.
    fn register_sized_component<T>(&mut self) -> &mut Self
    where
        T: Any + DataSize + Component,
    {
        self.register_component_with_estimator::<T, ForwardingEstimator>()
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
        self.register_resource_with_estimator::<T, ForwardingEstimator>()
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
        self.register_asset_with_estimator::<T, ForwardingEstimator>()
    }

    /// Registers the given [`Asset`] type with the
    /// [`MemoryUsagePlugin`][crate::MemoryUsagePlugin].
    ///
    /// The following types will be available to query on the [`MemoryUsage`]
    /// resource:
    ///
    /// * `T`
    /// * `<T as RenderAsset>::ExtractedAsset`
    /// * `<T as RenderAsset>::PreparedAsset`
    #[cfg(feature = "bevy_render")]
    fn register_sized_render_asset<T>(&mut self) -> &mut Self
    where
        T: Any + DataSize + RenderAsset,
        <T as RenderAsset>::PreparedAsset: Any + DataSize,
    {
        self.register_render_asset_with_estimator::<T, ForwardingEstimator, ForwardingEstimator>()
    }
}

impl RegisterSizedTypes for App {}

/// [`App`] extension methods to register non-[`DataSize`] types for memory
/// usage tracking.
pub trait RegisterTypesWithEstimator: RegisterTypes {
    /// Like [`RegisterSizedTypes::register_sized_component`], but uses the
    /// given [`DataSizeEstimator`] type.
    fn register_component_with_estimator<T, E>(&mut self) -> &mut Self
    where
        T: Any + Component,
        E: DataSizeEstimator<T> + FromConfig + 'static,
    {
        self.register_type::<T, _, _, _>(
            systems::update_stats_for_component::<T, E>,
            CoreStage::Update,
        )
    }

    /// Like [`RegisterSizedTypes::register_sized_resource`], but uses the
    /// given [`DataSizeEstimator`] type.
    fn register_resource_with_estimator<T, E>(&mut self) -> &mut Self
    where
        T: Any + Resource,
        E: DataSizeEstimator<T> + FromConfig + 'static,
    {
        self.register_type::<T, _, _, _>(
            systems::update_stats_for_resource::<T, E>,
            CoreStage::Update,
        )
    }

    /// Like [`RegisterSizedTypes::register_sized_asset`], but uses the
    /// given [`DataSizeEstimator`] type.
    fn register_asset_with_estimator<T, E>(&mut self) -> &mut Self
    where
        T: Any + Asset,
        E: DataSizeEstimator<T> + FromConfig + 'static,
    {
        self.register_type::<T, _, _, _>(systems::update_stats_for_asset::<T, E>, CoreStage::Update)
    }

    /// Like [`RegisterSizedTypes::register_sized_asset`], but uses the given
    /// [`DataSizeEstimator`] types to estimate the size of the [`RenderAsset`]
    /// and its prepared format.
    #[cfg(feature = "bevy_render")]
    fn register_render_asset_with_estimator<T, E, F>(&mut self) -> &mut Self
    where
        T: Any + RenderAsset,
        E: DataSizeEstimator<T> + FromConfig + 'static,
        <T as RenderAsset>::PreparedAsset: Any,
        F: DataSizeEstimator<<T as RenderAsset>::PreparedAsset> + FromConfig + 'static;
}

impl RegisterTypesWithEstimator for App {
    #[cfg(feature = "bevy_render")]
    fn register_render_asset_with_estimator<T, E, F>(&mut self) -> &mut Self
    where
        T: Any + RenderAsset,
        E: DataSizeEstimator<T> + FromConfig + 'static,
        <T as RenderAsset>::PreparedAsset: Any,
        F: DataSizeEstimator<<T as RenderAsset>::PreparedAsset> + FromConfig + 'static,
    {
        RegisterTypes::register_type::<T, _, _, _>(
            self,
            systems::update_stats_for_asset::<T, E>,
            CoreStage::Update,
        );

        if let Ok(render_app) = self.get_sub_app_mut(RenderApp) {
            RegisterTypes::register_type::<<T as RenderAsset>::PreparedAsset, _, _, _>(
                render_app,
                systems::update_stats_for_render_asset::<T, F>,
                RenderStage::Queue,
            );

            // Also register the type on the main app so there are entries for
            // it in the hashmap.
            register_type_on_app::<T>(self);
        }

        self
    }
}

/// The lowest-level interface for registering types for memory usage tracking.
pub trait RegisterTypes {
    /// Registers a type whose [`MemoryStats`] will be updated with the given
    /// `system`, which will run in the given `stage`.
    ///
    /// The given type `T` will be available to query on the [`MemoryUsage`]
    /// resource.
    ///
    /// [`MemoryStats`]: crate::MemoryStats
    fn register_type<T, S, Params, L>(&mut self, system: S, stage: L) -> &mut Self
    where
        T: Any,
        S: IntoSystemDescriptor<Params>,
        L: StageLabel;
}

impl RegisterTypes for App {
    fn register_type<T, S, Params, L>(&mut self, system: S, stage: L) -> &mut Self
    where
        T: Any,
        S: IntoSystemDescriptor<Params>,
        L: StageLabel,
    {
        register_type_on_app::<T>(self);

        self.add_system_to_stage(stage, system);

        self
    }
}

fn register_type_on_app<T>(app: &mut App)
where
    T: Any,
{
    let mut memory_usage = app.world.get_resource_mut::<MemoryUsage>().expect(
        "Cannot find resource `MemoryUsage`. Did you forget to add the `MemoryUsagePlugin`?",
    );

    memory_usage.register_type::<T>();
}
