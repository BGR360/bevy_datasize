#![doc = include_str!("../README.md")]

pub use datasize::DataSize;

mod app_ext;
mod config;
mod plugin;
mod resource;
mod systems;

pub use app_ext::RegisterSizedTypes;
pub use config::MemoryConfig;
pub use plugin::MemoryUsagePlugin;
pub use resource::MemoryUsage;

pub fn estimate_stack_and_heap_size<T>(value: &T) -> usize
where
    T: DataSize,
{
    let stack_size = std::mem::size_of::<T>();
    let heap_size = value.estimate_heap_size();

    stack_size + heap_size
}
