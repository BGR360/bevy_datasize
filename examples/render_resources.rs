//! This example shows demonstrates tracking memory usage for render resources
//! like meshes, images, and materials.
//!
//! Adapted from the official Bevy `many_cubes` example.

use bevy::{
    prelude::*,
    render::{mesh::GpuMesh, texture::GpuImage},
};
use bevy_datasize::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultMemoryUsagePlugins)
        .add_startup_system(setup)
        .add_system(print_sizes)
        .run();
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;
    let texture = asset_server.load("grass_0_0.png");
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            // cube
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(texture.clone()),
                    ..Default::default()
                }),
                transform: Transform::from_xyz((x as f32) * 2.0, (y as f32) * 2.0, 0.0),
                ..Default::default()
            });
        }
    }

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(80.0, 80.0, 300.0),
        ..Default::default()
    });
}

fn print_sizes(memory_usage: Res<MemoryUsage>) {
    let mesh_stats = memory_usage.get_stats::<Mesh>().unwrap();
    let gpu_mesh_stats = memory_usage.get_stats::<GpuMesh>().unwrap();
    let image_stats = memory_usage.get_stats::<Image>().unwrap();
    let material_stats = memory_usage.get_stats::<StandardMaterial>().unwrap();

    println!();
    println!("Memory usage:");
    println!("Meshes: {mesh_stats}");
    println!("GPU Meshes: {gpu_mesh_stats}");
    println!("Images: {image_stats}");
    println!("Materials: {material_stats}");
}
