use bevy::prelude::*;
use bevy_datasize::DataSize;

#[derive(Component, DataSize)]
struct MyComponent {
    pub vec: Vec<u8>,
}

fn main() {
    let component = MyComponent { vec: vec![0; 4096] };

    println!("{}", component.estimate_heap_size());
}
