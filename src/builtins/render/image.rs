//! Memory usage tracking for Bevy's [`Image`] type.
use bevy::{app::Plugin, render::texture::Image};

use crate::{systems, DataSize, DataSizeEstimator, RegisterSizedTypes};

/// Adds memory tracking for [`Image`] assets.
#[derive(Debug, Default)]
pub struct ImageMemoryUsagePlugin;

impl Plugin for ImageMemoryUsagePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_sized_type::<Image, _, _>(
            systems::update_stats_for_asset::<Image, ImageSizeEstimator>,
        );
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
