//! Memory usage tracking for Bevy's [`Mesh`] type.
//!
//! The behavior can be customized using the [`MemoryConfig`].

use bevy::{
    app::Plugin,
    asset::Assets,
    ecs::system::Res,
    render::mesh::{Mesh, VertexAttributeValues},
    utils::HashSet,
};

use crate::{
    systems, DataSize, DataSizeEstimator, MemoryConfig, MemoryStats, MemoryUsage,
    RegisterSizedTypes,
};

/// Adds memory tracking for [`Mesh`] assets.
#[derive(Debug, Default)]
pub struct MeshMemoryUsagePlugin;

impl Plugin for MeshMemoryUsagePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_sized_type::<Mesh, _, _>(update_stats_for_meshes);
    }
}

#[derive(Debug)]
struct MeshSizeEstimator<'a> {
    additional_vertex_attributes: &'a [&'static str],
}

impl<'a> Default for MeshSizeEstimator<'a> {
    fn default() -> Self {
        static NO_ADDITIONAL_ATTRIBUTES: [&str; 0] = [];

        Self {
            additional_vertex_attributes: &NO_ADDITIONAL_ATTRIBUTES[..],
        }
    }
}

impl<'a> DataSizeEstimator<Mesh> for MeshSizeEstimator<'a> {
    /// Sums up the sizes of the mesh's vertex attribute lists.
    fn estimate_heap_size(&self, mesh: &Mesh) -> usize {
        const DEFAULT_ATTRIBUTES: [&str; 7] = [
            Mesh::ATTRIBUTE_COLOR,
            Mesh::ATTRIBUTE_NORMAL,
            Mesh::ATTRIBUTE_TANGENT,
            Mesh::ATTRIBUTE_POSITION,
            Mesh::ATTRIBUTE_UV_0,
            Mesh::ATTRIBUTE_JOINT_WEIGHT,
            Mesh::ATTRIBUTE_JOINT_INDEX,
        ];

        let attributes: HashSet<&str> = DEFAULT_ATTRIBUTES
            .into_iter()
            .chain(self.additional_vertex_attributes.iter().copied())
            .collect();

        let total_size_of_attributes: usize = attributes
            .into_iter()
            .filter_map(|attribute_name| mesh.attribute(attribute_name))
            .map(|attributes| MemoryStats::total_size_of_with_estimator(attributes, self))
            .sum();

        total_size_of_attributes
    }
}

impl<'a> DataSizeEstimator<VertexAttributeValues> for MeshSizeEstimator<'a> {
    #[inline]
    fn estimate_heap_size(&self, values: &VertexAttributeValues) -> usize {
        use VertexAttributeValues::*;
        match values {
            Float32(v) => v.estimate_heap_size(),
            Sint32(v) => v.estimate_heap_size(),
            Uint32(v) => v.estimate_heap_size(),
            Float32x2(v) => v.estimate_heap_size(),
            Sint32x2(v) => v.estimate_heap_size(),
            Uint32x2(v) => v.estimate_heap_size(),
            Float32x3(v) => v.estimate_heap_size(),
            Sint32x3(v) => v.estimate_heap_size(),
            Uint32x3(v) => v.estimate_heap_size(),
            Float32x4(v) => v.estimate_heap_size(),
            Sint32x4(v) => v.estimate_heap_size(),
            Uint32x4(v) => v.estimate_heap_size(),
            Sint16x2(v) => v.estimate_heap_size(),
            Snorm16x2(v) => v.estimate_heap_size(),
            Uint16x2(v) => v.estimate_heap_size(),
            Unorm16x2(v) => v.estimate_heap_size(),
            Sint16x4(v) => v.estimate_heap_size(),
            Snorm16x4(v) => v.estimate_heap_size(),
            Uint16x4(v) => v.estimate_heap_size(),
            Unorm16x4(v) => v.estimate_heap_size(),
            Sint8x2(v) => v.estimate_heap_size(),
            Snorm8x2(v) => v.estimate_heap_size(),
            Uint8x2(v) => v.estimate_heap_size(),
            Unorm8x2(v) => v.estimate_heap_size(),
            Sint8x4(v) => v.estimate_heap_size(),
            Snorm8x4(v) => v.estimate_heap_size(),
            Uint8x4(v) => v.estimate_heap_size(),
            Unorm8x4(v) => v.estimate_heap_size(),
        }
    }
}

/// This system updates the [`MemoryStats`] for the [`Mesh`] type.
pub fn update_stats_for_meshes(
    memory_config: Res<MemoryConfig>,
    memory_usage: Res<MemoryUsage>,
    meshes: Res<Assets<Mesh>>,
) {
    let additional_vertex_attributes = &memory_config.additional_mesh_vertex_attributes[..];
    let estimator = MeshSizeEstimator {
        additional_vertex_attributes,
    };

    systems::update_stats::<Mesh, _>(&*memory_config, &*memory_usage, || {
        MemoryStats::from_values_with_estimator(
            meshes.iter().map(|(_handle, mesh)| mesh),
            &estimator,
        )
    })
}

/***************************************************************************************************

                             dMMMMMMP dMMMMMP .dMMMb dMMMMMMP .dMMMb
                               dMP   dMP     dMP" VP   dMP   dMP" VP
                              dMP   dMMMP    VMMMb    dMP    VMMMb
                             dMP   dMP     dP .dMP   dMP   dP .dMP
                            dMP   dMMMMMP  VMMMP"   dMP    VMMMP"

***************************************************************************************************/

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    use bevy::render::render_resource::PrimitiveTopology;
    use maplit::hashmap;

    const ATTRIBUTE_STACK_SIZE: usize = std::mem::size_of::<VertexAttributeValues>();

    fn create_mesh(attributes: HashMap<&'static str, usize>) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        for (name, bytes) in attributes.iter() {
            if bytes % 2 != 0 {
                panic!("Must specify a multiple of 2 bytes");
            }

            let values = VertexAttributeValues::Uint8x2(vec![[0, 0]; *bytes / 2]);
            mesh.set_attribute(*name, values);
        }

        mesh
    }

    #[test]
    fn counts_default_attributes() {
        let mesh = create_mesh(hashmap! {
            Mesh::ATTRIBUTE_POSITION => 100,
            Mesh::ATTRIBUTE_NORMAL => 100,
        });

        let estimator = MeshSizeEstimator::default();

        let estimated_heap_size = MemoryStats::heap_size_of_with_estimator(&mesh, &estimator);
        assert_eq!(estimated_heap_size, 200 + ATTRIBUTE_STACK_SIZE * 2);
    }

    #[test]
    fn does_not_count_extra_attributes_if_not_configured() {
        let mesh = create_mesh(hashmap! {
            Mesh::ATTRIBUTE_POSITION => 100,
            Mesh::ATTRIBUTE_NORMAL => 100,
            "foo" => 100,
            "bar" => 100,
        });

        let estimator = MeshSizeEstimator::default();

        let estimated_heap_size = MemoryStats::heap_size_of_with_estimator(&mesh, &estimator);
        assert_eq!(estimated_heap_size, 200 + ATTRIBUTE_STACK_SIZE * 2);
    }

    #[test]
    fn counts_extra_attributes_if_configured() {
        let mesh = create_mesh(hashmap! {
            Mesh::ATTRIBUTE_POSITION => 100,
            Mesh::ATTRIBUTE_NORMAL => 100,
            "foo" => 100,
            "bar" => 100,
        });

        let estimator = MeshSizeEstimator {
            additional_vertex_attributes: &["foo", "bar"],
        };

        let estimated_heap_size = MemoryStats::heap_size_of_with_estimator(&mesh, &estimator);
        assert_eq!(estimated_heap_size, 400 + ATTRIBUTE_STACK_SIZE * 4);
    }

    #[test]
    fn does_not_count_attribute_twice() {
        let mesh = create_mesh(hashmap! {
            Mesh::ATTRIBUTE_POSITION => 1000,
            Mesh::ATTRIBUTE_NORMAL => 1000,
        });

        let estimator = MeshSizeEstimator {
            additional_vertex_attributes: &[Mesh::ATTRIBUTE_POSITION],
        };

        let estimated_size = MemoryStats::total_size_of_with_estimator(&mesh, &estimator);
        assert!(estimated_size < 3000);
    }
}
