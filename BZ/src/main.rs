extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;
use rand::Rng;
use ndarray::{array, Array3};


const WIDTH: usize = 400;
const HEIGHT: usize = 400;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let alpha = 1.2;
    let beta = 1.0;
    let gamma = 1.0;

    let mut a: Array3<f32> = Array3::zeros((WIDTH, HEIGHT, 2));
    let mut b: Array3<f32> = Array3::zeros((WIDTH, HEIGHT, 2));
    let mut c: Array3<f32> = Array3::zeros((WIDTH, HEIGHT, 2));

    let sqrt2inv = 1.0 / 1.41421356237;
    let weights = array![sqrt2inv, 1.0, sqrt2inv, 1.0, 1.0, 1.0, sqrt2inv, 1.0, sqrt2inv];
    let weightsum = 7.8284271247523802;

    let mut p = 0;
    let mut q = 1;

    let mut rnd = rand::thread_rng();

    for i in 0..WIDTH as usize {
        for j in 0..HEIGHT as usize {
            a[[i, j, p]] = rnd.gen::<f32>();
            b[[i, j, p]] = rnd.gen::<f32>();
            c[[i, j, p]] = rnd.gen::<f32>();
        }
    }

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        for i in 1..WIDTH {
            for j in 1..HEIGHT {
                let (mut c_a, mut c_b, mut c_c) = (0.0, 0.0, 0.0);
                let mut weight_idx = 0;
                for x in i-1..=i+1 {
                    for y in j-1..=j+1 {
                        let weight = weights[weight_idx];
                        let ii = (x + WIDTH) % WIDTH;
                        let jj = (y + HEIGHT) % HEIGHT;
                        c_a += weight * a[[ii, jj, p]];
                        c_b += weight * b[[ii, jj, p]];
                        c_c += weight * c[[ii, jj, p]];
                        weight_idx += 1;
                    }
                }
                c_a /= weightsum;
                c_b /= weightsum;
                c_c /= weightsum;
                a[[i, j, q]] = constrain(c_a + c_a * (alpha * c_b - gamma * c_c), 0.0, 1.0);
                b[[i, j, q]] = constrain(c_b + c_b * (beta * c_c - alpha * c_a), 0.0, 1.0);
                c[[i, j, q]] = constrain(c_c + c_c * (gamma * c_a - beta * c_b), 0.0, 1.0);
                canvas.set_draw_color(Color::RGB((0.5 * 255.0) as u8, (0.7 * 255.0) as u8, (a[[i, j, q]] * 255.0) as u8));
                canvas.draw_point(Point::new(i as i32, j as i32)).expect("WHAT");
            }
        }

        if p == 0 {
            p = 1; q = 0;
        } else {
            p = 0; q = 1;
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
    }
}

fn constrain(val: f32, min_val: f32, max_val: f32) -> f32 {
    f32::min(max_val, f32::max(val, min_val))
}
