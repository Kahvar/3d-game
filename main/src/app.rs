use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode::Escape, PhysicalKey},
    window::{Window, WindowId},
};

use crate::renderer::WgpuState;

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
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