use winit::application::ApplicationHandler;         // why so many imports
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::KeyCode::Escape;
use winit::window::{Window, WindowId};
use winit::keyboard::{PhysicalKey};

#[derive(Default)]
struct App {        // add imports to this? idk
    window: Option<Window>,
}

impl ApplicationHandler for App {
    // call this when winit is ready to create a window
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    // handle the window events such as Close, Mouse click
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested | 
            // Or pressing escape
            WindowEvent::KeyboardInput {device_id: _, event: KeyEvent {physical_key: PhysicalKey::Code(Escape), state: ElementState::Pressed, repeat: false, ..}, .. }=> {
                println!("The close button was pressed; stopping");

                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                //What i understand from the docs i just draw frames here.

                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    // I chose ControlFlow::Poll over ControlFlow::Wait cause im making a game 
    event_loop.set_control_flow(ControlFlow::Poll);


    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}