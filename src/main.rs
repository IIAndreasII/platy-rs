use winit::{event_loop::EventLoop, window::WindowBuilder, event::{Event, WindowEvent}};

extern crate glium;

fn main() {

    let event_loop = EventLoop::new().unwrap();

    let wb = WindowBuilder::new();

    let _window = wb.build(&event_loop).unwrap();

    // TODO: Create and pass opengl context

    event_loop.run(move | event, window_target | {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(),
                // TODO: Handle more window events
                _ => (),
            },
            // TODO: Handle more events
            _ => (),

        }
    }).unwrap();
}