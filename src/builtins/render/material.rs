//! Memory usage tracking for Bevy's [`Image`] type.
use bevy::{app::Plugin, pbr::StandardMaterial};

use crate::{app_ext::RegisterTypesWithEstimator, estimator::ZeroEstimator};

/// Adds memory tracking for [`StandardMaterial`] assets.
#[derive(Debug, Default)]
pub struct MaterialMemoryUsagePlugin;

impl Plugin for MaterialMemoryUsagePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // StandardMaterial has no heap usage.
        app.register_asset_with_estimator::<StandardMaterial, ZeroEstimator>();
    }
}
