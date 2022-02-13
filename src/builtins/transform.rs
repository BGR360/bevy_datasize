//! Support for types in [`bevy::transform`].

use bevy::{
    app::{App, Plugin},
    transform::components::{Children, GlobalTransform, Parent, PreviousParent, Transform},
};

use crate::{
    builtins::RegisterTypesWithEstimator,
    estimator::{SliceEstimator, ZeroEstimator},
    DataSizeEstimator,
};

/// Adds memory tracking for the components that are part of Bevy's
/// [`TransformPlugin`][bevy::transform::TransformPlugin].
pub struct TransformMemoryUsagePlugin;

impl Plugin for TransformMemoryUsagePlugin {
    fn build(&self, app: &mut App) {
        app.register_component_with_estimator::<Children, ChildrenSizeEstimator>();

        // These types have no heap usage.
        app.register_component_with_estimator::<GlobalTransform, ZeroEstimator>();
        app.register_component_with_estimator::<Parent, ZeroEstimator>();
        app.register_component_with_estimator::<PreviousParent, ZeroEstimator>();
        app.register_component_with_estimator::<Transform, ZeroEstimator>();
    }
}

#[derive(Debug, Default)]
struct ChildrenSizeEstimator;

impl DataSizeEstimator<Children> for ChildrenSizeEstimator {
    const IS_DYNAMIC: bool = true;

    fn estimate_heap_size(&self, value: &Children) -> usize {
        SliceEstimator.estimate_heap_size(&*value)
    }
}
