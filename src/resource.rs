use std::any::{Any, TypeId};

use bevy::utils::HashMap;

use crate::MemoryStats;

/// Stores memory usage statistics for registered data types.
#[derive(Debug, Default)]
pub struct MemoryUsage {
    pub(crate) datasizes: HashMap<TypeId, MemoryStats>,
}

impl MemoryUsage {
    /// Returns the most recent [`MemoryStats`] for the given type.
    ///
    /// Returns `None` if the type has not been registered.
    pub fn get_stats<T>(&self) -> Option<MemoryStats>
    where
        T: Any,
    {
        let type_id = TypeId::of::<T>();

        self.datasizes.get(&type_id).copied()
    }

    /// Updates the [`MemoryStats`] for the given type.
    pub fn update_stats<T>(&mut self, stats: MemoryStats)
    where
        T: Any,
    {
        let type_id = TypeId::of::<T>();

        let entry = self
            .datasizes
            .get_mut(&type_id)
            .expect("Memory usage not tracked for this type. Did you forget to register the type?");

        *entry = stats;
    }

    /// Registers the given type with the usage tracker.
    pub fn register_type<T>(&mut self)
    where
        T: Any,
    {
        let type_id = TypeId::of::<T>();

        self.datasizes.insert(type_id, Default::default());
    }
}
