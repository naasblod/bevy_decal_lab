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

fn parallaxMapping(V: vec3<f32>, uv: vec2<f32>, parallaxHeight: f32) -> vec2<f32> {
    let parallaxScale = 160.0;
    var initialHeight: f32 = parallaxHeight;

    // Calculate amount of offset for Parallax Mapping
    var texCoordOffset: vec2<f32> = parallaxScale * V.xz / V.z * initialHeight;

    // Calculate amount of offset for Parallax Mapping With Offset Limiting
    texCoordOffset = parallaxScale * V.xz * initialHeight;

    // Return modified texture coordinates
    return uv + texCoordOffset;
}

@fragment
fn fragment(in: VertexOutput,
    @builtin(front_facing) is_front: bool) -> @location(0) vec4<f32> {

    let sample_index = 0u;
    let depth = prepass_depth(in.position, sample_index);

    let diff_depth = abs(in.position.z - depth);

    let ray = normalize(view.world_position  - in.world_position.xyz) ;
    var new_in = in;
    //let center_pos_xz = custom_material.center_pos.xz;
    let local_space = in.world_position.xz - custom_material.center_pos.xz + vec2(0.5);
    new_in.uv = parallaxMapping(ray, local_space,  depth - in.position.z );

    var pbr_input = pbr_input_from_standard_material(new_in, is_front);
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);
    let ray2 =  normalize(in.world_position.xyz - view.world_position);
    pbr_input.world_position = vec4(in.world_position.xyz - (ray2 * diff_depth * 600.0) , depth);

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    

    //let alpha = smoothstep(0.0, 1.0, 1.0 - diff_depth * 400.0);
    var alpha = min(clamp(1.0 - diff_depth * 600.0 , 0.0, 1.0), out.color.a);
    let color =  out.color.rgb;
    if alpha < 0.1 {
      alpha = 0.0;
    }

    //return out.color;

    // depth mask
    //return vec4(vec3(alpha), 1.0);

    // just distortion
    //return vec4(color, 1.0);

    // regular
    return vec4(color, alpha);
}
