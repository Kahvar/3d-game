// THIS IS SOME BOILERPLATE CODE AND I COPIED THIS
// I DON'T UNDERSTAND EVEN HALF OF THIS SO THE COMMENTS MAY BE WRONG..

use std::sync::Arc; // wgpu needs this so the window keeps alive
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode::Escape, PhysicalKey},
    window::{Window, WindowId},
};


// dump ugly gpu conf here so the main code looks clean
struct WgpuState {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}


impl WgpuState {
    // start the connection to gpu with async because gpu's awakening takes a little time
    async fn new(window: Arc<Window>) -> Self {
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
        }
    }

    // if the window is resized tell the new size to gpu and adjust the canvas size with it
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }


    // THIS IS USED FOR DRAWING *IMPORTANT*
    fn render(&mut self) {
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
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                depth_stencil_attachment: None, // 3D depth buffer comes here later
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
        }

        // Send the recorded commands trough a "conveyor belt" to the GPU
        self.queue.submit(std::iter::once(encoder.finish()));
        
        // Push the ready picture to the screen
        self.queue.present(output); 
    }
}



// This is the games core
#[derive(Default)]
struct App { // Both of these are none because no window is created yet
    window: Option<Arc<Window>>, // arc ensures the window stays in memory as long as some thread needs it
    gpu_state: Option<WgpuState>,
}

impl ApplicationHandler for App {

    // Winit calls this as soon as the OS is ready to open a window
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes().with_title("3D Engine Project");
            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            self.window = Some(window.clone());

            // init wgpu immediately after window creation
            // use pollster to stop the current thread and waits as long as it needs to init the gpu
            let gpu = pollster::block_on(WgpuState::new(window));
            self.gpu_state = Some(gpu);
        }
    }

    // dealing with the window events like resize and closing
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            // close it with X or esc
            WindowEvent::CloseRequested | 
            WindowEvent::KeyboardInput {
                event: KeyEvent { physical_key: PhysicalKey::Code(Escape), state: ElementState::Pressed, .. },
                ..
            } => {
                println!("Exiting application...");
                event_loop.exit();
            },

            // If the window is resized update the gpu config line 99
            WindowEvent::Resized(physical_size) => {
                if let Some(gpu) = &mut self.gpu_state {
                    gpu.resize(physical_size);
                }
            },

            // draw the new frame to screen
            WindowEvent::RedrawRequested => {
                if let Some(gpu) = &mut self.gpu_state {
                    gpu.render();
                }

                // ask winit for a new frame as soon as possible because im making a game
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    // using poll because this is going to be a game
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
