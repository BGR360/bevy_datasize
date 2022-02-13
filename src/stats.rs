use std::{
    any::Any,
    ops::Add,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::{estimator::ForwardingEstimator, DataSize, DataSizeEstimator};

/// Memory usage statistics for a single data type.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemoryStats {
    /// The total number of instances of this data type.
    pub count: usize,

    /// The total number of "stack" bytes used by instances of this data type.
    ///
    /// See [`stack_size_of`] for details on the meaning of this quantity.
    ///
    /// [`stack_size_of`]: Self::stack_size_of
    pub total_stack_bytes: usize,

    /// The estimated total number of bytes used by instances of this data type.
    ///
    /// See [`heap_size_of`] for details on the meaning of this quantity.
    ///
    /// [`heap_size_of`]: Self::heap_size_of
    pub total_heap_bytes: usize,
}

impl MemoryStats {
    /// Returns the sum of `total_stack_bytes` and `total_heap_bytes` for
    /// `self`.
    #[inline]
    pub fn total_bytes(&self) -> usize {
        self.total_stack_bytes + self.total_heap_bytes
    }

    /// Returns the computed memory statistics for a single value.
    #[inline]
    pub fn from_value<T>(value: &T) -> Self
    where
        T: Any + DataSize,
    {
        Self::from_value_with_estimator(value, &ForwardingEstimator::default())
    }

    /// Returns the computed memory statistics for a single value using a
    /// specific [`DataSizeEstimator`].
    #[inline]
    pub fn from_value_with_estimator<T, E>(value: &T, estimator: &E) -> Self
    where
        T: Any,
        E: DataSizeEstimator<T>,
    {
        Self {
            count: 1,
            total_stack_bytes: Self::stack_size_of(value),
            total_heap_bytes: Self::heap_size_of_with_estimator(value, estimator),
        }
    }

    /// Returns the computed memory statistics for a collection of values.
    #[inline]
    pub fn from_values<'a, T, I>(values: I) -> Self
    where
        T: Any + DataSize,
        I: IntoIterator<Item = &'a T>,
    {
        Self::from_values_with_estimator(values, &ForwardingEstimator::default())
    }

    /// Returns the computed memory statistics for a collection of values.
    #[inline]
    pub fn from_values_with_estimator<'a, T, E, I>(values: I, estimator: &E) -> Self
    where
        T: Any,
        E: DataSizeEstimator<T>,
        I: IntoIterator<Item = &'a T>,
    {
        let mut stats = MemoryStats::default();

        for value in values.into_iter() {
            stats = stats + Self::from_value_with_estimator(value, estimator);
        }

        stats
    }

    /// Returns the "stack" size of the given value.
    ///
    /// This quantity represents how many bytes the type *would* take up on the
    /// stack if were allocated there.
    ///
    /// This quantity is **exact** and is computed using [`std::mem::size_of`].
    #[inline]
    pub fn stack_size_of<T>(value: &T) -> usize
    where
        T: Any,
    {
        std::mem::size_of_val(value)
    }

    /// Returns the estimated heap size of the given value.
    ///
    /// This quantity represents how many bytes the type occupies apart from the
    /// immediate bytes of its fields.
    #[inline]
    pub fn heap_size_of<T>(value: &T) -> usize
    where
        T: DataSize,
    {
        Self::heap_size_of_with_estimator(value, &ForwardingEstimator::default())
    }

    /// Returns the estimated heap size of the given value using a specific
    /// [`DataSizeEstimator`].
    #[inline]
    pub fn heap_size_of_with_estimator<T, E>(value: &T, estimator: &E) -> usize
    where
        E: DataSizeEstimator<T>,
    {
        estimator.estimate_heap_size(value)
    }

    /// Returns the estimated total size of the given value.
    ///
    /// This quantity is the sum of [`stack_size_of`] and [`heap_size_of`].
    ///
    /// [`stack_size_of`]: Self::stack_size_of
    /// [`heap_size_of`]: Self::heap_size_of
    #[inline]
    pub fn total_size_of<T>(value: &T) -> usize
    where
        T: Any + DataSize,
    {
        Self::total_size_of_with_estimator(value, &ForwardingEstimator::default())
    }

    /// Returns the estimated total size of the given value using a specific
    /// [`DataSizeEstimator`].
    pub fn total_size_of_with_estimator<T, E>(value: &T, estimator: &E) -> usize
    where
        T: Any,
        E: DataSizeEstimator<T>,
    {
        Self::stack_size_of(value) + Self::heap_size_of_with_estimator(value, estimator)
    }
}

impl Add for MemoryStats {
    type Output = MemoryStats;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            count: self.count + rhs.count,
            total_stack_bytes: self.total_stack_bytes + rhs.total_stack_bytes,
            total_heap_bytes: self.total_heap_bytes + rhs.total_heap_bytes,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct MemoryStatsInternal {
    count: AtomicUsize,
    total_stack_bytes: AtomicUsize,
    total_heap_bytes: AtomicUsize,
}

impl MemoryStatsInternal {
    pub(crate) fn get(&self) -> MemoryStats {
        MemoryStats {
            count: self.count.load(Ordering::Relaxed),
            total_stack_bytes: self.total_stack_bytes.load(Ordering::Relaxed),
            total_heap_bytes: self.total_heap_bytes.load(Ordering::Relaxed),
        }
    }

    pub(crate) fn set(&self, stats: MemoryStats) {
        self.count.store(stats.count, Ordering::Relaxed);
        self.total_stack_bytes
            .store(stats.total_stack_bytes, Ordering::Relaxed);
        self.total_heap_bytes
            .store(stats.total_heap_bytes, Ordering::Relaxed);
    }
}

impl From<MemoryStats> for MemoryStatsInternal {
    fn from(stats: MemoryStats) -> Self {
        Self {
            count: AtomicUsize::new(stats.count),
            total_stack_bytes: AtomicUsize::new(stats.total_stack_bytes),
            total_heap_bytes: AtomicUsize::new(stats.total_heap_bytes),
        }
    }
}
