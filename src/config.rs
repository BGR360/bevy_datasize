/// Configuration for the [`MemoryUsagePlugin`][crate::MemoryUsagePlugin].
#[derive(Debug)]
pub struct MemoryConfig {
    /// Whether to track memory usage for all registered types.
    pub global: bool,

    /// A list of additional [`Mesh`] vertex attributes to track.
    ///
    /// If the plugin is tracking meshes, then by default, it will only account
    /// for data stored in [the "built-in" vertex attributes of `Mesh`].
    ///
    /// If your meshes have any custom attributes, you'll need to specify them
    /// here in order for their memory usage to be tracked.
    ///
    /// This is due to the fact that, currently, [`Mesh`] has no method to
    /// iterate through its attributes.
    ///
    /// [`Mesh`]: bevy::render::mesh::Mesh
    /// [the "built-in" vertex attributes of `Mesh`]:
    ///     bevy::render::mesh::Mesh#associatedconstant.ATTRIBUTE_COLOR
    pub additional_mesh_vertex_attributes: Vec<&'static str>,
}

impl MemoryConfig {
    /// Returns a new [`MemoryConfig`] in its default state, except that
    /// `global` is `false`.
    pub fn disabled_at_start() -> Self {
        Self {
            global: false,
            ..Default::default()
        }
    }
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            global: true,
            additional_mesh_vertex_attributes: Default::default(),
        }
    }
}
