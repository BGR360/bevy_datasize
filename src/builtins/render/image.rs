//! Memory usage tracking for Bevy's [`Image`] type.
use bevy::{app::Plugin, render::texture::Image};

use crate::{builtins::RegisterTypesWithEstimator, DataSize, DataSizeEstimator};

/// Adds memory tracking for [`Image`] assets.
#[derive(Debug, Default)]
pub struct ImageMemoryUsagePlugin;

impl Plugin for ImageMemoryUsagePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_asset_with_estimator::<Image, ImageSizeEstimator>();
    }
}

#[derive(Debug, Default)]
struct ImageSizeEstimator;

impl DataSizeEstimator<Image> for ImageSizeEstimator {
    /// Returns the size of the image's data array.
    #[inline]
    fn estimate_heap_size(&self, image: &Image) -> usize {
        image.data.estimate_heap_size()
    }
}
