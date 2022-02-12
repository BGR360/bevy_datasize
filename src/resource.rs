use std::any::{Any, TypeId};

use bevy::utils::HashMap;

#[derive(Debug, Default)]
pub struct MemoryUsage {
    pub(crate) datasizes: HashMap<TypeId, usize>,
}

impl MemoryUsage {
    pub fn register_type<T>(&mut self)
    where
        T: Any,
    {
        let type_id = TypeId::of::<T>();

        self.datasizes.insert(type_id, 0);
    }

    pub fn update_usage<T>(&mut self, total_bytes: usize)
    where
        T: Any,
    {
        let type_id = TypeId::of::<T>();

        let entry = self
            .datasizes
            .get_mut(&type_id)
            .expect("Memory usage not tracked for this type. Did you forget to register the type?");

        *entry = total_bytes;
    }

    pub fn get_usage<T>(&self) -> Option<usize>
    where
        T: Any,
    {
        let type_id = TypeId::of::<T>();

        self.datasizes.get(&type_id).copied()
    }
}
