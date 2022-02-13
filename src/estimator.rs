//! Heap size estimators.

use crate::{DataSize, MemoryConfig};

/// Indicates that a type can estimate the heap usage of values of type `T`.
///
/// This trait exists in order to work around the fact that this crate cannot
/// add impls of [`DataSize`] to types from the Bevy crate(s).
pub trait DataSizeEstimator<T: ?Sized> {
    /// If `true`, the type `T` has a heap size that can vary at runtime,
    /// depending on the actual value.
    const IS_DYNAMIC: bool;

    /// Estimates the size of heap memory taken up by the given value.
    ///
    /// Does not include data on the stack, which is usually determined using
    /// [`std::mem::size_of`].
    fn estimate_heap_size(&self, value: &T) -> usize;
}

/// A [`DataSizeEstimator`] that simply forwards to a type's implementation of
/// [`DataSize`].
#[derive(Default)]
pub struct ForwardingEstimator;

impl<T: DataSize> DataSizeEstimator<T> for ForwardingEstimator {
    const IS_DYNAMIC: bool = <T as DataSize>::IS_DYNAMIC;

    #[inline]
    fn estimate_heap_size(&self, value: &T) -> usize {
        <T as DataSize>::estimate_heap_size(value)
    }
}

/// A [`DataSizeEstimator`] that simply returns `0`.
#[derive(Default)]
pub struct ZeroEstimator;

impl<T> DataSizeEstimator<T> for ZeroEstimator {
    const IS_DYNAMIC: bool = false;

    #[inline(always)]
    fn estimate_heap_size(&self, _value: &T) -> usize {
        0
    }
}

/// A [`DataSizeEstimator`] that multiplies a type's stack size by the length of
/// a slice.
#[derive(Default)]
pub struct SliceEstimator;

impl<T> DataSizeEstimator<[T]> for SliceEstimator {
    const IS_DYNAMIC: bool = true;

    #[inline]
    fn estimate_heap_size(&self, value: &[T]) -> usize {
        std::mem::size_of::<T>() * value.len()
    }
}

/// Creates `Self` using data from the given [`MemoryConfig`].
pub trait FromConfig {
    /// Creates `Self` using data from the given [`MemoryConfig`].
    fn from_config(config: &MemoryConfig) -> Self;
}

impl<T: Default> FromConfig for T {
    fn from_config(_config: &MemoryConfig) -> T {
        Default::default()
    }
}
