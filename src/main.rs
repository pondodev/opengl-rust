use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const WINDOW_WIDTH: u32 = 512;
const WINDOW_HEIGHT: u32 = 512;

fn main() {
    let event_loop = EventLoop::new();
    let builder = WindowBuilder::new()
        .with_title("OpenGL + winit in rust!")
        .with_resizable(false)
        .with_inner_size(winit::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));
    let window = builder.build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}