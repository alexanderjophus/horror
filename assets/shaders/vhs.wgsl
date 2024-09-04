#import bevy_render::view::View
#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::globals
#import bevy_pbr::utils

@group(0) @binding(0) var<uniform> view: View;
@group(2) @binding(101) var texture: texture_2d<f32>;
@group(2) @binding(102) var texture_sampler: sampler;

#ifndef RANDOM_SCALE
#ifdef RANDOM_HIGHER_RANGE
#define RANDOM_SCALE vec4(.1031, .1030, .0973, .1099)
#else
#define RANDOM_SCALE vec4(443.897, 441.423, .0973, .1099)
#endif
#endif

fn random(x: f32) -> f32 {
#ifdef RANDOM_SINLESS
    x = fract(x * RANDOM_SCALE.x);
    x *= x + 33.33;
    x *= x + x;
    return fract(x);
#else
    return fract(sin(x) * 43758.5453);
#endif
}

fn random_vec2(st: vec2<f32>) -> f32 {
#ifdef RANDOM_SINLESS
    vec3 p3  = fract(vec3(st.xyx) * RANDOM_SCALE.xyz);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
#else
    return fract(sin(dot(st.xy, vec2(12.9898, 78.233))) * 43758.5453);
#endif
}

fn modulo(x: f32, y: f32) -> f32 {
    return x - y * floor(x / y);
}

fn rolling_distort(uv: vec2<f32>, time: f32) -> f32 {
    return random_vec2(vec2(uv * 5. + time)) * 1. - random(step((0.5 * modulo(-time/1.5 + uv.y, 2.0)), .95));
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    var colour = textureSample(texture, texture_sampler, uv).xyz;
    let vhs = rolling_distort(uv, -globals.time /2);
    let edges = step(0.005, pow( 1.0*uv.x*uv.y*(1.0-uv.y)*(1.0-uv.x), 1.0 ));
    colour += mix(1.0, vhs, 1.0)/3.;

    colour = mix(vec3<f32>(0.0, 0.0, 0.0), colour, edges);

    return vec4<f32>(
        colour,
        1.0
    );
}
