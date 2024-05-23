#import bevy_pbr::utils
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct VHSPostProcessSettings {
    time: f32,
    _padding_a: vec3<f32>,
}
@group(0) @binding(2) var<uniform> settings: VHSPostProcessSettings;

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

fn warp(uv: vec2<f32>) -> vec2<f32> {
	let delta = uv - 0.5;
	let delta2 = dot(delta.xy, delta.xy);
	let delta4 = delta2 * delta2;
	let delta_offset = delta4 * -0.2;
	
	return uv + delta * delta_offset;
}

fn modulo(x: f32, y: f32) -> f32 {
    return x - y * floor(x / y);
}

fn distort(uv: vec2<f32>, time: f32) -> vec3<f32> {
    var color = textureSample(screen_texture, texture_sampler, uv).xyz;
    let st = warp(uv);

    let d2 = 1.0;
    let d3 = random_vec2(vec2(st * 5. + time)) * 1. - random(step((0.5 * modulo(-time/1.5 + st.y, 2.0)), .95));
    color += mix(d2, d3, step(0.005, pow( 1.0*st.x*st.y*(1.0-st.y)*(1.0-st.x), 1.0 )))/5.;
    
    return color;
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let vhs = distort(in.uv, settings.time);

    return vec4<f32>(
        vhs,
        1.0
    );
}
