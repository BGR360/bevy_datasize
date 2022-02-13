use std::marker::PhantomData;

use crate::DataSize;

/// Indicates that a type can estimate the heap usage of values of type `T`.
///
/// All types which implement [`DataSize`] have this trait implemented on
/// themselves.
///
/// This trait exists in order to work around the fact that this crate cannot
/// add impls of [`DataSize`] to types from the Bevy crate(s).
pub trait DataSizeEstimator<T> {
    /// Estimates the size of heap memory taken up by the given value.
    ///
    /// Does not include data on the stack, which is usually determined using
    /// [`std::mem::size_of`].
    fn estimate_heap_size(&self, value: &T) -> usize;
}

impl<T: DataSize> DataSizeEstimator<T> for ForwardingEstimator<T> {
    fn estimate_heap_size(&self, value: &T) -> usize {
        <T as DataSize>::estimate_heap_size(value)
    }
}

/// A [`DataSizeEstimator`] that simply forwards to `T`'s implementation of
/// [`DataSize`].
#[derive(Debug)]
pub struct ForwardingEstimator<T>(PhantomData<T>);

impl<T> Default for ForwardingEstimator<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
