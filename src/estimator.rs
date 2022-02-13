//! Heap size estimators.

use crate::DataSize;

/// Indicates that a type can estimate the heap usage of values of type `T`.
///
/// This trait exists in order to work around the fact that this crate cannot
/// add impls of [`DataSize`] to types from the Bevy crate(s).
pub trait DataSizeEstimator<T: ?Sized> {
    /// Estimates the size of heap memory taken up by the given value.
    ///
    /// Does not include data on the stack, which is usually determined using
    /// [`std::mem::size_of`].
    fn estimate_heap_size(&self, value: &T) -> usize;
}

/// A [`DataSizeEstimator`] that simply forwards to a type's implementation of
/// [`DataSize`].
#[derive(Debug, Default)]
pub struct ForwardingEstimator;

impl<T: DataSize> DataSizeEstimator<T> for ForwardingEstimator {
    fn estimate_heap_size(&self, value: &T) -> usize {
        <T as DataSize>::estimate_heap_size(value)
    }
}

/// A [`DataSizeEstimator`] that simply returns `0`.
#[derive(Debug, Default)]
pub struct ZeroEstimator;

impl<T> DataSizeEstimator<T> for ZeroEstimator {
    fn estimate_heap_size(&self, _value: &T) -> usize {
        0
    }
}

/// A [`DataSizeEstimator`] that multiplies a type's stack size by the length of
/// a slice.
#[derive(Debug, Default)]
pub struct SliceEstimator;

impl<T> DataSizeEstimator<[T]> for SliceEstimator {
    fn estimate_heap_size(&self, value: &[T]) -> usize {
        std::mem::size_of::<T>() * value.len()
    }
}
