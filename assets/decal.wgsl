#import bevy_pbr::{
    mesh_view_bindings::{globals, view},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    prepass_utils::prepass_depth,
    view_transformations::{
view_z_to_depth_ndc
    ,depth_ndc_to_view_z, position_world_to_clip, perspective_camera_near, perspective_camera_far, position_view_to_ndc },
    mesh_view_bindings as view_bindings,
    parallax_mapping::parallaxed_uv,
    pbr_bindings,
}
#import bevy_render::maths::affine2_to_square

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

struct CustomMaterial {
    center_pos: vec3<f32>,
    color: vec4<f32>,
}
@group(2) @binding(200)
var<uniform> custom_material: CustomMaterial;


@fragment
fn fragment(in: VertexOutput,
    @builtin(front_facing) is_front: bool) -> @location(0) vec4<f32> {
    let sample_index = 0u;

    let depth = prepass_depth(in.position, sample_index);
    let diff_depth = in.position.z - depth;
    let diff_depth_abs = abs(diff_depth);

    return vec4(vec3((1.0 - diff_depth_abs * 200.0 ) ),1.0);

}
