#import bevy_pbr::{
    mesh_view_bindings::{globals, view},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    prepass_utils::prepass_depth,
    view_transformations::position_world_to_clip
}

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
@group(1) @binding(200)
var<uniform> custom_material: CustomMaterial;

fn parallax_mapping(ray_to_view: vec3<f32>, uv: vec2<f32>, depth: f32) -> vec2<f32> {
    let parallaxScale = 160.0;

    // Calculate amount of offset for Parallax Mapping
    var texCoordOffset: vec2<f32> = parallaxScale * ray_to_view.xz / ray_to_view.z * depth;

    // Calculate amount of offset for Parallax Mapping With Offset Limiting
    texCoordOffset = parallaxScale * ray_to_view.xz * depth;

    // Return modified texture coordinates
    return uv + texCoordOffset;
}

@fragment
fn fragment(in: VertexOutput,
    @builtin(front_facing) is_front: bool) -> @location(0) vec4<f32> {

    let sample_index = 0u;
    let depth = prepass_depth(in.position, sample_index);

    let diff_depth= depth - in.position.z;
    let diff_depth_abs = abs(diff_depth);

    // todo: get smart and figure out why this is a good number.
    let depth_scale = 300.0;

    let ray = normalize(view.world_position  - in.world_position.xyz) ;
    var new_in = in;

    // this could be material field, but for size 1 quads this is it.
    let half_size = vec2(0.5);

    let local_space = in.world_position.xz - custom_material.center_pos.xz + half_size;
    new_in.uv = parallax_mapping(ray, local_space,  depth - in.position.z );

    var pbr_input = pbr_input_from_standard_material(new_in, is_front);
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

    pbr_input.world_position = vec4(in.world_position.xyz + (ray * diff_depth_abs * depth_scale) ,depth);

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    var alpha = min(clamp(1.0 - diff_depth_abs * depth_scale , 0.0, 1.0), out.color.a);

    // world_position_projections
    //return out.color;

    // depth mask
    //return vec4(vec3(alpha), 1.0);

    // uv distortion
    //return vec4(out.color.rgb, 1.0);

    // result
    return vec4(out.color.rgb, alpha);
}
