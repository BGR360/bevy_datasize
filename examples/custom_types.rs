//! This example shows how to register your own resource, component, and asset
//! types for memory usage tracking.

use bevy::{asset::AssetPlugin, prelude::*, reflect::TypeUuid};
use bevy_datasize::prelude::*;

#[derive(DataSize)]
struct MyResource {
    pub data: Vec<u8>,
}

#[derive(Component, DataSize)]
struct MyComponent {
    pub data: Vec<u8>,
}

#[derive(DataSize, TypeUuid)]
#[uuid = "0669d1a3-34b5-4bac-b9cd-26a2e2df79ff"]
pub struct MyAsset {
    pub data: Vec<u8>,
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(AssetPlugin)
        .add_asset::<MyAsset>()
        .insert_resource(MyResource {
            data: vec![42; 4096],
        })
        .add_plugin(MemoryUsagePlugin)
        .register_sized_resource::<MyResource>()
        .register_sized_component::<MyComponent>()
        .register_sized_asset::<MyAsset>()
        .add_startup_system(spawn_entities)
        .add_startup_system(add_asset)
        .add_system(print_sizes)
        .run();
}

fn spawn_entities(mut commands: Commands) {
    for _ in 0..8 {
        let component = MyComponent {
            data: vec![0; 1024],
        };
        commands.spawn().insert(component);
    }
}

fn add_asset(mut handle: Local<Handle<MyAsset>>, mut assets: ResMut<Assets<MyAsset>>) {
    *handle = assets.add(MyAsset {
        data: vec![68; 425],
    });
}

fn print_sizes(memory_usage: Res<MemoryUsage>) {
    let resource_stats = memory_usage.get_stats::<MyResource>().unwrap();
    let component_stats = memory_usage.get_stats::<MyComponent>().unwrap();
    let asset_stats = memory_usage.get_stats::<MyAsset>().unwrap();

    println!();
    println!("Memory usage:");
    println!("MyResource: {:#?}", resource_stats);
    println!("MyComponent: {:#?}", component_stats);
    println!("MyAsset: {:#?}", asset_stats);
}
