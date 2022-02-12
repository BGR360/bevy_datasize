use bevy::prelude::*;
use bevy_datasize::{DataSize, MemoryUsage, MemoryUsagePlugin, RegisterSizedTypes};

#[derive(Component, DataSize)]
struct MyComponent {
    pub vec: Vec<u8>,
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(MemoryUsagePlugin)
        .register_sized_component::<MyComponent>()
        .add_startup_system(spawn_entities)
        .add_system(print_size)
        .run();
}

fn spawn_entities(mut commands: Commands) {
    for _ in 0..8 {
        let component = MyComponent { vec: vec![0; 1024] };
        commands.spawn().insert(component);
    }
}

fn print_size(memory_usage: Res<MemoryUsage>) {
    let bytes = memory_usage.get_usage::<MyComponent>().unwrap();

    println!("Memory usage: {} bytes", bytes);
}
