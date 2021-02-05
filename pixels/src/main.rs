use fltk::{app, prelude::*, window::Window};
use pixels::{Pixels, SurfaceTexture};
use std::{thread, time::Duration};

const WIDTH: u32 = 600;
const HEIGHT: u32 = 400;
const CIRCLE_RADIUS: i16 = 64;

struct World {
    circle_x: i16,
    circle_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    let mut win = Window::default()
        .with_size(WIDTH as i32, HEIGHT as i32)
        .with_label("Pixels");
    win.end();
    win.make_resizable(true);
    win.show();

    let mut pixels = {
        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &win);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut world = World::new();

    win.draw(move || {
        world.update();
        world.draw(pixels.get_frame());
        pixels.render().unwrap();
    });


    Ok(while app.wait() {
        win.redraw();
        thread::sleep(Duration::from_millis(16));
    })
}

impl World {
    fn new() -> Self {
        Self {
            circle_x: 300,
            circle_y: 200,
            velocity_x: 5,
            velocity_y: 5,
        }
    }

    fn update(&mut self) {
        if self.circle_x - CIRCLE_RADIUS <= 0 || self.circle_x + CIRCLE_RADIUS > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.circle_y - CIRCLE_RADIUS <= 0 || self.circle_y + CIRCLE_RADIUS > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.circle_x += self.velocity_x;
        self.circle_y += self.velocity_y;
    }

    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;
            let d = {
                let xd = x as i32 - self.circle_x as i32;
                let yd = y as i32 - self.circle_y as i32;
                ((xd.pow(2) + yd.pow(2)) as f64).sqrt().powi(2)
            };
            let inside_the_circle = d < (CIRCLE_RADIUS as f64).powi(2);

            let rgba = if inside_the_circle {
                [0xac, 0x00, 0xe6, 0xff]
            } else {
                [0x26, 0x00, 0x33, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
