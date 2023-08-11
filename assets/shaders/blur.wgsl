// This shader computes the chromatic aberration effect

#import bevy_pbr::utils

// Since post processing is a fullscreen effect, we use the fullscreen vertex shader provided by bevy.
// This will import a vertex shader that renders a single fullscreen triangle.
//
// A fullscreen triangle is a single triangle that covers the entire screen.
// The box in the top left in that diagram is the screen. The 4 x are the corner of the screen
//
// Y axis
//  1 |  x-----x......
//  0 |  |  s  |  . ´
// -1 |  x_____x´
// -2 |  :  .´
// -3 |  :´
//    +---------------  X axis
//      -1  0  1  2  3
//
// As you can see, the triangle ends up bigger than the screen.
//
// You don't need to worry about this too much since bevy will compute the correct UVs for you.
#import bevy_core_pipeline::fullscreen_vertex_shader FullscreenVertexOutput

@group(0) @binding(0)
var screen_texture: texture_2d<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;
struct BlurPostProcessSettings {
    intensity: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec3<f32>
#endif
}
@group(0) @binding(2)
var<uniform> settings: BlurPostProcessSettings;

fn s(uv: vec2<f32>) -> vec3<f32> {
    return textureSample(screen_texture, texture_sampler, uv).rgb;
}

fn p(x: f32, y: f32) -> vec2<f32> {
    return vec2<f32>(x, y) * 0.02;
}

fn s_blurred(uv: vec2<f32>) -> vec3<f32> {
    let r = p(1.0, 0.0);
    let tr = p(1.0, 1.0);
    let t = p(0.0, 1.0);
    let tl = p(-1.0, 1.0);
    let l = p(-1.0, 0.0);
    let bl = p(-1.0, -1.0);
    let b = p(0.0, -1.0);
    let br = p(1.0, -1.0);

    return 
        (s(uv) + s(uv + r) + s(uv + tr) + s(uv + t) + s(uv + tl) + s(uv + l) + s(uv + bl) + s(uv + b) + s(uv + br)) / 9.
        ;
}


@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let original = s(in.uv);
    let blurred = s_blurred(in.uv);

    let output = mix(original, blurred, settings.intensity);

    return vec4<f32>(output, 1.0);
}
