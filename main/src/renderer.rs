use std::sync::Arc;
// wgpu needs this so the window keeps alive
use winit::window::Window;
use crate::pipeline::create_pipeline;
use crate::mesh::{CUBE_INDICES, create_cube_buffer, create_index_buffer};

// gpu struct use pub to make it usable to other files as well
pub struct WgpuState {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    depth_texture: wgpu::Texture,
    depth_view: wgpu::TextureView,
}


impl WgpuState {
    // start the connection to gpu with async because gpu's awakening takes a little time
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        // "Can i speak to the gpu pls pls"
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());

        // make a film on top of the window
        let surface = instance.create_surface(window.clone()).unwrap();

        // give some info about us to the computer and choose the best gpu
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                apply_limit_buckets: false, 
            })
            .await
            .unwrap();


        // open the device and queue to gpu
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device Descriptor"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::Performance,
                    experimental_features: Default::default(),
                    trace: wgpu::Trace::Off, // Explicitly pass the wgpu Trace Enum
                },
            )
            .await
            .unwrap();
        


        // ask the monitor what colors it supports and pass it to the adapter so it knows what colors it can use
        let surface_caps = surface.get_capabilities(&adapter);
        
        // Show sRGB colors if the monitors doesnt support it take the first one
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d { width: size.width.max(1), height: size.height.max(1), depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let depth_view = depth_texture.create_view(&wgpu::wgt::TextureViewDescriptor::default());

        //upload the cube vertices to gpu
        let vertex_buffer = create_cube_buffer(&device);

        // upload cube vertices to the gpu
        let index_buffer = create_index_buffer(&device);

        // load shader
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/shader.wgsl"));

        // describe how each vertex is stored in memory
        let render_pipeline = create_pipeline(&device, &shader, surface_format);



        // Picks the first alpha mode which is Auto which chooses either Opaque or Inherit 
        let alpha_mode = surface_caps.alpha_modes[0];

        // Make the canvas size its the same size as the window
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::Fifo, // Turn on vsync to prevent screen tearing
            alpha_mode, // Pass the alpha mode here
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: wgpu::SurfaceColorSpace::Auto,
        };
        surface.configure(&device, &config);


        Self {
            surface,
            device,
            queue,
            config,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            depth_texture,
            depth_view,
        }
    }

    // if the window is resized tell the new size to gpu and adjust the canvas size with it
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }


    // THIS IS USED FOR DRAWING *IMPORTANT*
    pub fn render(&mut self) {
        // ask the screen for empty "paper" for this frame
        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => texture,
            wgpu::CurrentSurfaceTexture::Outdated => {
                println!("Surface outdated, skipping frame"); // print this if the window is too small to draw anything or its minimized
                return;
            }
            _ => return,
        };
        
        // make a new view to the "paper" and create an encoder for GPU commands..
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            // Throw a space color paint bucket to the screen
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.05, g: 0.05, b: 0.1, a: 1.0 }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None, 
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { 
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations { 
                        load: wgpu::LoadOp::Clear(1.0), 
                        store: wgpu::StoreOp::Store 
                        }), 
                    stencil_ops: None 
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..CUBE_INDICES.len() as u32, 0, 0..1);
        }

        // Send the recorded commands trough a "conveyor belt" to the GPU
        self.queue.submit(std::iter::once(encoder.finish()));
        
        // Push the ready picture to the screen
        self.queue.present(output); 
    }
}