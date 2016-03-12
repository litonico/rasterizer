extern crate sdl2;
use sdl2::event::{Event};
use sdl2::rect::{Point};
use sdl2::pixels::{Color};
use sdl2::render::{Renderer};
use std::time::{Instant, Duration};
use std::thread::{sleep};

static WIDTH  : u32 = 100;
static HEIGHT : u32 = 100;
static TITLE  : &'static str = "Pixels";
static FPS    : u8 = 15;

fn line(x0: u32, y0: u32,
        x1: u32, y1: u32,
        renderer: &mut Renderer, color: Color) {

    renderer.set_draw_color(color);

    let mut t = 0.0;
    while t < 1. {
        t += 0.1;
        let x = x0 as f32 * (1.-t) + x1 as f32 * t;
        let y = y0 as f32 * (1.-t) + y1 as f32 * t;
        let p = Point::new(x as i32, y as i32);
        renderer.draw_point(p);
    }
}

fn draw(width: u32, height: u32, image: &mut Renderer) {
    let white = Color::RGB(255, 255, 255);
    let red = Color::RGB(255, 0, 0);
    line(13, 20, 80, 40, image, white);
    line(20, 13, 40, 80, image, red);
}

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


    renderer.set_draw_color(Color::RGB(0,0,0));
    renderer.clear();

    draw(window_width, window_height, &mut renderer);

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
                println!("rendered slow!");
                Duration::new(0,0)
            } else {
                FRAMETIME - time_elapsed
            };

        // Don't max out the CPU!
        sleep(sleep_time);
    }
}
