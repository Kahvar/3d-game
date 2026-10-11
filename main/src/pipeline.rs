// render pipeline is instructions to the gpu to turn vertex data into pixels




pub fn create_pipeline(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    pixel_format: wgpu::TextureFormat,) -> wgpu::RenderPipeline {

    
    // describe the vertex data
    const ATTRIBUTES: [wgpu::VertexAttribute; 1] =
        wgpu::vertex_attr_array![0 => Float32x3];

    // how wgpu reads the vertex buffer bytes
    let vertex_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<crate::mesh::Vertex>()
            as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &ATTRIBUTES,
    };

    // creating the render pipeline
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Triangle Pipeline"),
        layout: None,

        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[Some(vertex_buffer_layout)],
        },

        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: pixel_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),

        primitive: Default::default(),
        depth_stencil: None,
        multisample: Default::default(),
        multiview_mask: None,
        cache: None,
    })
}