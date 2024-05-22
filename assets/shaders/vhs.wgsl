#import bevy_pbr::utils
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct VHSPostProcessSettings {
    time: f32,
}
@group(0) @binding(2) var<uniform> settings: VHSPostProcessSettings;

#ifndef RANDOM_SCALE
#ifdef RANDOM_HIGHER_RANGE
#define RANDOM_SCALE vec4(.1031, .1030, .0973, .1099)
#else
#define RANDOM_SCALE vec4(443.897, 441.423, .0973, .1099)
#endif
#endif

fn random(st: vec2<f32>) -> f32 {
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

fn distort(uv: vec2<f32>, time: f32) -> vec3<f32> {
    let st = warp(uv);
    var color = textureSample(screen_texture, texture_sampler, st).xyz;

    let d2 = 0.0;
    let d3 = random(vec2(st * 5. + time)) * 0.1 - 0.05;
    color += mix(d2, d3, step(0.005, pow( 1.0*st.x*st.y*(1.0-st.y)*(1.0-st.x), 1.0 )));
    
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
