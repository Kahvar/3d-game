// Moved code to different files so its more clean
mod app;
mod renderer;
mod mesh;

use app::App;
use winit::{
    event_loop::{ControlFlow, EventLoop},
};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    // using poll because this is going to be a game
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
