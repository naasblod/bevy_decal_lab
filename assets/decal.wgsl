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

@fragment
fn fragment(in: VertexOutput,
    @builtin(front_facing) is_front: bool) -> @location(0) vec4<f32> {

    let sample_index = 0u;
    let depth = prepass_depth(in.position, sample_index);

    let diff_depth = in.position.z - depth;

    let ray = normalize(in.world_position.xyz - view.world_position);
    var new_in = in;
    let center_pos_xz = custom_material.center_pos.xz;
    let local_space = (in.world_position.xz + center_pos_xz + (ray.xz * diff_depth * 60.0  )) + vec2(0.5);
    new_in.uv = local_space;

    var pbr_input = pbr_input_from_standard_material(new_in, is_front);
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
#endif
    
    let color =  out.color.rgb;

    let alpha = clamp(diff_depth * 2000.0, 0.0, 1.0);
    return vec4(color, alpha);
}
