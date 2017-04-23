#[macro_use]
extern crate glium;

extern crate glium_sdl2;
extern crate sdl2;

use glium_sdl2::DisplayBuild;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let display = video_subsystem.window("My window", 800, 600)
        .resizable()
        .build_glium()
        .unwrap();

    let mut running = true;
    let mut event_pump = sdl_context.event_pump().unwrap();

    while running {
        let mut target = display.draw();
        // do drawing here...
        target.finish().unwrap();

        // Event loop: includes all windows

        for event in event_pump.poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit { .. } => {
                    running = false;
                }
                _ => (),
            }
        }
    }
}
