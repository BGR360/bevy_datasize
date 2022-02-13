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
