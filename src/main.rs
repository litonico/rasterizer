extern crate sdl2;
use sdl2::event::{Event};
use sdl2::rect::{Point};
use sdl2::pixels::{Color};
use std::time::{Instant, Duration};
use std::thread::{sleep};

static WIDTH  : u32 = 160;
static HEIGHT : u32 = 144;
static TITLE  : &'static str = "Pixels";
static FPS    : u8 = 15;

fn main() {
    let FRAMETIME : Duration = Duration::from_millis(1000/FPS as u64);

    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let mut timer = ctx.timer().unwrap();

    let scale = 1;
    let window_width = WIDTH * scale;
    let window_height = HEIGHT * scale;

    let mut window = match video_ctx.window(TITLE, window_width, window_height)
        .position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err)     => panic!("failed to create renderer: {}", err)
    };

    // Draw things
    for x in 0..window_width as u8 {
        for y in 0..window_height as u8 {
            renderer.set_draw_color(Color::RGB(x,y,0));
            let point = Point::new(x as i32, y as i32);
            renderer.draw_point(point);
        }
    }

    renderer.present();

    let mut events = ctx.event_pump().unwrap();

    'main : loop {
        let start_time = Instant::now();

        'event : loop {

            for event in events.poll_iter() {
                match event {
                    // Handle keys here
                    Event::Quit{..} => break 'main,
                    _               => break 'event
                }
            }
        };

        // TODO(Lito): Is there overhead in making an Instant::now twice?
        // How about making a bunch of Durations?
        let time_elapsed = Instant::now().duration_since(start_time);

        // Ok, this is absurd, but sure:
        let sleep_time : Duration =
            // TODO(Lito): This will break if drawing or framerate goes slower
            // than 1 fps.
            if time_elapsed.subsec_nanos() > FRAMETIME.subsec_nanos()  {
                Duration::new(0,0)
            } else {
                FRAMETIME - time_elapsed
            };

        // Don't max out the CPU!
        sleep(sleep_time);
    }
}
