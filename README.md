# bevy_datasize

[![Tests](https://github.com/bgr360/bevy_datasize/actions/workflows/tests.yml/badge.svg)](https://github.com/bgr360/bevy_datasize/actions/workflows/tests.yml)

`bevy_datasize` is a library for tracking memory usage in
[Bevy](https://lib.rs/bevy) apps.

`bevy_datasize` uses the `DataSize` trait from the [`datasize`] crate to
estimate the runtime memory usage of any components, resources, or assets that
are registered with the `MemoryUsagePlugin`.

The `DataSize` trait can be derived for your own custom types, and you can
inject custom estimators for third party types that do not implement `DataSize`.
See the [`datasize`] docs for more info on that.

[`datasize`]: https://docs.rs/datasize

## Docs

Rustdocs for the main branch can be found
[here](https://bgr360.github.io/bevy_datasize/bevy_datasize/)

## Examples

### Basic Usage

The following example demonstrates how to show the memory usage of all loaded
`Image`s:

```rust,no_run
use bevy::prelude::*;
use bevy_datasize::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultMemoryUsagePlugins)
        .add_system(print_image_usage)
        .run();
}

fn print_image_usage(memory_usage: Res<MemoryUsage>) {
    let MemoryStats {
        count,
        total_stack_bytes,
        total_heap_bytes,
    } = memory_usage.get_stats::<Image>().unwrap();

    println!("Image count: {count}");
    println!("Total stack usage: {total_stack_bytes} bytes");
    println!("Total heap usage: {total_heap_bytes} bytes");
}
```

### Custom Data Types

The following example demonstrates how to track memory usage for a custom
`Component` type when using minimal plugins:

```rust,no_run
use bevy::prelude::*;
use bevy_datasize::prelude::*;

#[derive(Component, DataSize)]
struct MyComponent {
    data: Vec<u8>,
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(MemoryUsagePlugin)
        .register_sized_component::<MyComponent>()
        .add_system(print_custom_usage)
        .run();
}

fn print_custom_usage(memory_usage: Res<MemoryUsage>) {
    let MemoryStats {
        count,
        total_stack_bytes,
        total_heap_bytes,
    } = memory_usage.get_stats::<MyComponent>().unwrap();

    println!("MyComponent count: {count}");
    println!("MyComponent total stack usage: {total_stack_bytes} bytes");
    println!("MyComponent total heap usage: {total_heap_bytes} bytes");
}
```

### More

See the [`examples`](examples/) directory for more examples.

## Optional Features

`bevy_datasize` can be configured very granularly to only pull in the parts of
`bevy` that you need for your app.

### Default features

This gives you support for all of the Bevy-internal data types that
`bevy_datasize` supports:

```toml
[dependencies]
bevy_datasize = "0.0.1"
```

### Minimal features

This gives you support for only your own custom datatypes:

```toml
[dependencies]
bevy_datasize = { version = "0.0.1", default-features = false }
```

### Other configurations

This, for example, gives you support for all rendering resources:

```toml
[dependencies]
bevy_datasize = { version = "0.0.1", default-features = false, features = ["render"] }
```

Or just for `Image` assets:

```toml
[dependencies]
bevy_datasize = { version = "0.0.1", default-features = false, features = ["image"] }
```

See the [`Cargo.toml`](Cargo.toml) to see all the available features.

## Feature Checklist

This crate is still in development. Everybody loves checklists!

### Main functionality features

- [x] Tracking custom types
- [x] Retrieving memory usage statistics from a resource
- [ ] Throttling the statistics gathering
- [ ] Hooking memory usage statistics up to `Diagnostics`
- [ ] Categories / category hierarchy
- [ ] Visual debugging and/or integration with `bevy_inspector_egui`

### Supported Bevy types

So far, the following types have built-in support for memory tracking:

- [ ] Types in `bevy::audio`
  - [ ] `Audio`
  - [ ] `AudioSource`[^1]
- [ ] Types in `bevy::render`
  - [x] `Mesh`
  - [x] `GpuMesh`
  - [x] `Image`
  - [ ] `GpuImage`
  - [ ] `Shader`[^1]
  - [ ] `RenderGraph`
  - [ ] `TextureCache`[^1]
  - [ ] `ComponentUniforms`
  - [ ] `VisibleEntities`
  - [ ] `ComputedVisibility`
  - [ ] `Visibility`
  - [ ] `ExtractedView`
- [ ] Types in `bevy::gltf`
  - [ ] `Gltf`
  - [ ] `GltfMesh`
  - [ ] `GltfNode`
  - [ ] `GltfPrimitive`
- [ ] Types in `bevy::pbr`
  - [x] `StandardMaterial`
  - [ ] `GpuStandardMaterial`
  - [ ] `MeshUniform`
  - [ ] `MeshViewBindGroup`
  - [ ] `AmbientLight`
  - [ ] `Clusters`
  - [ ] `CubemapVisibleEntities`
  - [ ] `DirectionalLight`
  - [ ] `ExtractedAmbientLight`
  - [ ] `ExtractedClusterConfig`
  - [ ] `ExtractedClustersPointLights`
  - [ ] `ExtractedDirectionalLight`
  - [ ] `ExtractedPointLight`
  - [ ] `GpuDirectionalLight`
  - [ ] `GpuLights`
  - [ ] `GpuPointLight`
  - [ ] `GpuPointLights`
  - [ ] `PointLight`
  - [ ] `ShadowView`
  - [ ] `ViewClusterBindings`
  - [ ] `ViewLightEntities`
  - [ ] `ViewLightsUniformOffset`
  - [ ] `ViewShadowBindings`
  - [ ] `VisiblePointLights`
  - [ ] `Wireframe`
- [ ] Types in `bevy::scene`
  - [ ] `Scene`
  - [ ] `DynamicScene`
- [ ] Types in `bevy::sprite`
  - [ ] `Sprite`
  - [ ] `SpriteBatch`
  - [ ] `TextureAtlasSprite`
  - [ ] `TextureAtlas`
  - [ ] `ColorMaterial`
  - [ ] `GpuColorMaterial`
  - [ ] `ExtractedSprites`
  - [ ] `Mesh2dUniform`
  - [ ] `Mesh2dViewBindGroup`
- [x] Types in `bevy::transform`
  - [x] `Transform`
  - [x] `GlobalTransform`
  - [x] `Children`
  - [x] `Parent`
  - [x] `PreviousParent`
- [ ] Types in `bevy::text`
  - [ ] `Font`[^1]
  - [ ] `FontAtlasSet`
  - [ ] `Text`
  - [ ] `Text2dSize`
- [ ] Types in `bevy::ui`
  - [ ] `CalculatedClip`
  - [ ] `CalculatedSize`
  - [ ] `ExtractedUiNodes`
  - [ ] `Node`
  - [ ] `Style`
  - [ ] `UiBatch`
  - [ ] `UiImage`

[^1]: It may not be possible to estimate the size of this type.

## License

Licensed under either of

 * Apache License, Version 2.0
   (<http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   (<http://opensource.org/licenses/MIT>)

at your option.

Copyright Ben Reeves 2022

[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
