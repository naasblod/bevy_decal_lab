use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline},
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, CompareFunction, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};

pub struct DecalPlugin;
impl Plugin for DecalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, DecalMaterial>> {
                prepass_enabled: false,
                ..default()
            },
        )
        .add_systems(Update, update_center_position);
    }
}

fn update_center_position(
    query: Query<(
        &GlobalTransform,
        &Handle<ExtendedMaterial<StandardMaterial, DecalMaterial>>,
    )>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, DecalMaterial>>>,
) {
    for (transform, handle) in &query {
        if let Some(material) = materials.get_mut(handle) {
            material.extension.center_pos = transform.translation();
        }
    }
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl MaterialExtension for DecalMaterial {
    fn fragment_shader() -> ShaderRef {
        "decal.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // descriptor.primitive.cull_mode = None;
        if let Some(label) = &mut descriptor.label {
            *label = format!("decal_{}", *label).into();
        }
        if let Some(ref mut depth) = &mut descriptor.depth_stencil {
            depth.depth_compare = CompareFunction::Always;
        }

        Ok(())
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct DecalMaterial {
    #[uniform(200)]
    pub center_pos: Vec3,
    #[uniform(200)]
    pub color: Color,
}
