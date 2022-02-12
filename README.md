# bevy_datasize

[![Tests](https://github.com/bgr360/bevy_datasize/actions/workflows/tests.yml/badge.svg)](https://github.com/bgr360/bevy_datasize/actions/workflows/tests.yml)

This is a library for tracking memory usage in [Bevy](https://lib.rs/bevy) apps.

It is based on the [`datasize`](https://lib.rs/datasize) crate.

## Example

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
        .add_system(print_datasize)
        .run();
}

fn print_datasize(memory_usage: Res<MemoryUsage>) {
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

## Docs

Rustdocs for the main branch can be found
[here](https://bgr360.github.io/bevy_datasize/bevy_datasize/)

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
