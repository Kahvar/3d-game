
struct VertexPayload {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
) -> VertexPayload {
    var output: VertexPayload;

    output.position = vec4<f32>(position, 1.0);
    output.color = vec3<f32>(1.0, 0.4, 0.1);

    return output;
}

@fragment
fn fs_main(in: VertexPayload) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
