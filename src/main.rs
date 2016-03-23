extern crate sdl2;
extern crate rand;

pub mod objparse;

use objparse::{Model, Face, Vertex};

use sdl2::event::{Event};
use sdl2::rect::{Point};
use sdl2::pixels::{Color};
use sdl2::render::{Renderer};

use std::time::{Instant, Duration};
use std::thread::{sleep};

static WIDTH  : u32 = 500;
static HEIGHT : u32 = 500;
static TITLE  : &'static str = "Pixels";
static FPS    : u8 = 15;

fn triangle() -> Model {
    let faces = vec![Face { verts: vec![1, 2, 3] }];
    let verts = vec![Vertex {x:0.01,  y:0.01,  z:0.},
    Vertex {x:2., y:1., z:0.},
    Vertex {x:0.,  y:2., z:0.}];

    let triangle : Model = Model {
        verts: verts,
        faces: faces,
    };
    triangle
}

fn draw_line(mut x0: i32, mut y0: i32,
             mut x1: i32, mut y1: i32,
             renderer: &mut Renderer, color: Color) {

    renderer.set_draw_color(color);

    let dx = (x1-x0).abs();
    let dy = (y1-y0).abs();
    let steep = if dx < dy { true } else { false };

    // TODO(Lito): Why is it so hard to write a swap function?
    if steep { // transpose the whole drawing across x = y
        let mut tmp_x = x0;
        x0 = y0;
        y0 = tmp_x;

        tmp_x = x1;
        x1 = y1;
        y1 = tmp_x;
    }

    if (x0 > x1) {
        let mut tmp = x0;
        x0 = x1;
        x1 = tmp;

        tmp = y0;
        y0 = y1;
        y1 = tmp;
    }

    // assert!(x1 > x0, "x0:{}, x1:{}", x0, x1);
    for x in x0..x1 {
        let t = (x-x0) as f32 / (x1-x0) as f32;
        let y = y0 as f32 * (1.-t) + y1 as f32 * t;
        let p = if steep {
            Point::new(y.round() as i32, x) // transposed
        } else {
            Point::new(x, y.round() as i32)
        };
        renderer.draw_point(p);
    }
}

fn draw(model: &Model, width: u32, height: u32, image: &mut Renderer) {
    let white = Color::RGB(255, 255, 255);
    let red = Color::RGB(255, 0, 0);
    draw_faces(model, image);
}

fn draw_xy_line_between_verts(v1: Vertex, v2: Vertex, r: &mut Renderer, c: Color) {
    let scale = 100.;
    let shift_x = 200.;
    let shift_y = 200.;
    draw_line((v1.x * scale + shift_x) as i32, (v1.y * scale + shift_y) as i32,
    (v2.x * scale + shift_x) as i32, (v2.y * scale + shift_y) as i32, r, c);
}

fn draw_wireframe_triangle(v0: Vertex, v1: Vertex, v2: Vertex, image: &mut Renderer, color: Color) {
    draw_xy_line_between_verts(v0, v1, image, color);
    draw_xy_line_between_verts(v1, v2, image, color);
    draw_xy_line_between_verts(v2, v0, image, color);
}

fn draw_filled_triangle(mut v0: Vertex, mut v1: Vertex, mut v2: Vertex,
                        image: &mut Renderer, color: Color) {
    // Bubble sort verts
    if v0.y > v1.y {
        let tmp = v0;
        v0 = v1;
        v1 = tmp;
    }
    if v0.y > v2.y {
        let tmp = v0;
        v0 = v2;
        v2 = tmp;
    }
    if v1.y > v2.y {
        let tmp = v1;
        v1 = v2;
        v2 = tmp;
    }
    let y = v0.y as u32;
    for 
        // draw_xy_line_between_verts(v0, v1, image, Color::RGB(0,255,0));
        // draw_xy_line_between_verts(v1, v2, image, Color::RGB(0,255,0));
        // draw_xy_line_between_verts(v2, v0, image, Color::RGB(255,0,0));

}

fn draw_faces(model: &Model, image: &mut Renderer) {
    for face in &model.faces {
        let color = Color::RGB(rand::random::<u8>(),
        rand::random::<u8>(),
        rand::random::<u8>());
        let v0 = model.verts[face.verts[0]-1];
        let v1 = model.verts[face.verts[1]-1];
        let v2 = model.verts[face.verts[2]-1];

        draw_filled_triangle(v0, v1, v2, image, color);
    }
}

fn main() {
    let FRAMETIME : Duration = Duration::from_millis(1000/FPS as u64);

    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let scale = 1;
    let window_width = WIDTH * scale;
    let window_height = HEIGHT * scale;

    let window = match video_ctx.window(TITLE, window_width, window_height)
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

    let starman : Model = objparse::load("./model.obj");
    let medamaude : Model = objparse::load("./medamaude.obj");
    let cube : Model = objparse::load("./cube.obj");
    let triangle = triangle();

    draw(&triangle, window_width, window_height, &mut renderer);

    renderer.present();

    let mut events = ctx.event_pump().unwrap();


    'main : loop {
        let start_time = Instant::now();

        for event in events.poll_iter() {
            match event {
                // Handle keys here
                Event::Quit{..} => break 'main,
                _               => continue
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
                println!("rendered slow! {:?} milliseconds", time_elapsed.subsec_nanos() / 1000000);
                Duration::new(0,0)
            } else {
                FRAMETIME - time_elapsed
            };

        // Don't max out the CPU!
        sleep(sleep_time);
    }
}
