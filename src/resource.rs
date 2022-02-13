use std::{
    any::{Any, TypeId},
    sync::Arc,
};

use bevy::utils::HashMap;
use parking_lot::RwLock;

use crate::stats::{MemoryStats, MemoryStatsInternal};

/// Stores memory usage statistics for registered data types.
#[derive(Debug, Default, Clone)]
pub struct MemoryUsage {
    inner: Arc<RwLock<MemoryUsageInner>>,
}

impl MemoryUsage {
    /// Registers the given type with the usage tracker.
    pub fn register_type<T>(&mut self)
    where
        T: Any,
    {
        let type_id = TypeId::of::<T>();

        self.inner
            .write()
            .datasizes
            .insert(type_id, Default::default());
    }

    /// Returns the most recent [`MemoryStats`] for the given type.
    ///
    /// Returns `None` if the type has not been registered.
    pub fn get_stats<T>(&self) -> Option<MemoryStats>
    where
        T: Any,
    {
        let type_id = TypeId::of::<T>();

        self.inner
            .read()
            .datasizes
            .get(&type_id)
            .map(MemoryStatsInternal::get)
    }

    /// Updates the [`MemoryStats`] for the given type.
    pub fn update_stats<T>(&mut self, stats: MemoryStats)
    where
        T: Any,
    {
        let type_id = TypeId::of::<T>();

        let mut inner = self.inner.write();

        let entry = inner
            .datasizes
            .get_mut(&type_id)
            .expect("Memory usage not tracked for this type. Did you forget to register the type?");

        *entry = MemoryStatsInternal::from(stats);
    }

    /// Like [`update_stats`][Self::update_stats] but operates on a shared reference.
    ///
    /// In exchange for the slight possibility that individual fields in
    /// [`MemoryStats`] will be inconsistent with each other, this allows making
    /// multiple updates to the resource concurrently.
    pub fn update_stats_fast<T>(&self, stats: MemoryStats)
    where
        T: Any,
    {
        let type_id = TypeId::of::<T>();

        let inner = self.inner.read();

        let entry = inner
            .datasizes
            .get(&type_id)
            .expect("Memory usage not tracked for this type. Did you forget to register the type?");

        entry.set(stats);
    }
}

#[derive(Debug, Default)]
struct MemoryUsageInner {
    datasizes: HashMap<TypeId, MemoryStatsInternal>,
}
