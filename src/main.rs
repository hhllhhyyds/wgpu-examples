use winit::event_loop::EventLoop;

use wgpu_examples::app::App;

fn main() {
    env_logger::init();
    let mut app = App::default();
    let event_loop = EventLoop::new().unwrap();
    let _ = event_loop.run_app(&mut app);
}
