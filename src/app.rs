use winit::{
    application::ApplicationHandler,
    event::*,
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use crate::state::State;

static mut GLOBAL_WINDOW: Option<Window> = None;

#[derive(Default)]
pub struct App<'a> {
    state: Option<State<'a>>,
}

impl<'a> AsRef<State<'a>> for App<'a> {
    fn as_ref(&self) -> &State<'a> {
        self.state.as_ref().unwrap()
    }
}

impl<'a> AsMut<State<'a>> for App<'a> {
    fn as_mut(&mut self) -> &mut State<'a> {
        self.state.as_mut().unwrap()
    }
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.state.is_none() {
            unsafe {
                GLOBAL_WINDOW = Some(
                    event_loop
                        .create_window(Window::default_attributes())
                        .unwrap(),
                );

                self.state = Some(pollster::block_on(State::new(
                    GLOBAL_WINDOW.as_ref().unwrap(),
                )));
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                self.as_mut().resize(physical_size);
            }
            WindowEvent::RedrawRequested => {
                self.as_ref().window.request_redraw();

                self.as_mut().update();
                match self.as_mut().render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = self.state.as_ref().unwrap().size;
                        self.as_mut().resize(size)
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        log::error!("OutOfMemory");
                        event_loop.exit();
                    }

                    // This happens when the a frame takes too long to present
                    Err(wgpu::SurfaceError::Timeout) => {
                        log::warn!("Surface timeout")
                    }
                }
            }
            _ => (),
        }
    }
}
