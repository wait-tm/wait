struct CameraUniform {
    view_projection: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec3f,
}

@vertex
fn vertex_main(
    @location(0) position: vec3f,
    @location(1) color: vec3f,
) -> VertexOutput {
    var output: VertexOutput;

    output.position =
        camera.view_projection *
        vec4f(position, 1.0);
    output.color = color;

    return output;
}

@fragment
fn fragment_main(
    @location(0) color: vec3f
) -> @location(0) vec4f {
    return vec4f(color, 1.0);
}