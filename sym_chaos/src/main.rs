extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;
use rand::Rng;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        sym_frac(&mut canvas);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 24));
    }
}

fn sym_icon(canvas: &mut sdl2::render::WindowCanvas) {
    let iterates = 500000;
    let mut count = 0;

    let height: f32 = 800.0;
    let width: f32 = 800.0;
    let (mut x, mut y, mut za, mut zb, mut zzbar, mut zreal, mut zimag, mut zn, mut p, mut xnew, mut ynew, mut scaled_x, mut scaled_y);
    x = 0.01;
    y = 0.01;
    let alpha = 1.0;
    let beta = -0.1;
    let gamma = 0.167;
    let omega = 0.0;
    let lambda = -2.08;
    let scale = 1.5;
    let n = 10;
    let scalex = width / (2.0 * scale);
    let scaley = height / (2.0 * scale);
    let mut colour: u8 = 0;

    loop {
        colour = (colour + 5u8) % 255;
        zzbar = (x * x) + (y * y);
        zreal = x;
        zimag = y;
        for _i in 0..n-2 {
            za = (zreal * x) - (zimag * y);
            zb = (zimag * x) + (zreal * y);
            zreal = za;
            zimag = zb;
            canvas.set_draw_color(Color::RGB(colour, 20, 255-colour));
        }
        zn = (x * zreal) - (y * zimag);
        p = lambda + (alpha * zzbar) + (beta * zn);
        xnew = (p * x) + (gamma * zreal) - (omega * y);
        ynew = (p * y) - (gamma * zimag) + (omega * x);

        count += 1;

        x = xnew;
        y = ynew;

        scaled_x = ((x * scalex) - (width / 2.0)).abs() as i32;
        scaled_y = ((y * scaley) - (height / 2.0)).abs() as i32;

        let point = Point::new(scaled_x, scaled_y);

        canvas.draw_point(point).unwrap();

        if count == iterates {
            break;
        }
    }
}

fn sym_frac(canvas: &mut sdl2::render::WindowCanvas) {
    let iterates = 50000;
    let mut count = 0;

    let height: f32 = 800.0;
    let width: f32 = 800.0;
    let (mut x, mut y, mut x1, mut y1, mut xnew, mut ynew, mut scaled_x, mut scaled_y, mut m);
    x = 0.01;
    y = 0.01;
    let a11 = 0.55;
    let a12 = 0.0;
    let a21 = 0.0;
    let a22 = -0.45;
    let b1 = 0.5;
    let b2 = -0.2;
    let conj = 0;
    let scale = 0.72;
    let n = 3;
    let c: Vec<f32> = (0..100).map(|i| (2.0 * std::f32::consts::PI * i as f32 / n as f32).cos()).collect();
    let s: Vec<f32> = (0..100).map(|i| (2.0 * std::f32::consts::PI * i as f32 / n as f32).sin()).collect();
    let scalex = width / 2.0 * scale;
    let scaley = height / 2.0 * scale;

    let mut colour: u8 = 0;
    loop {
        colour = (colour + 5u8) % 255;
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        count += 1;

        xnew = a11 * x + a12 * y + b1;
        ynew = a21 * x + a22 * y + b2;
        m = rand::thread_rng().gen_range(0..n) as usize;
        x1 = xnew;
        y1 = ynew;
        xnew = c[m] as f32 * x1 - s[m] as f32 * y1;
        ynew = s[m] as f32 * x1 + c[m] as f32 * y1;

        x = xnew;
        y = ynew;
        if conj == 0 {
            scaled_x = ((x * scalex) - (width / 2.0)).abs() as i32;
            scaled_y = ((y * scaley) - (height / 2.0)).abs() as i32;

            let point = Point::new(scaled_x, scaled_y);
            canvas.draw_point(point).unwrap();

            if count == iterates {
                break;
            }
            continue;
        }

        m = rand::thread_rng().gen_range(0..=1) as usize;
        if m == 1 {
            ynew = -ynew;
        }

        scaled_x = ((x * scalex) - (width / 2.0)).abs() as i32;
        scaled_y = ((y * scaley) - (height / 2.0)).abs() as i32;

        let point = Point::new(scaled_x, scaled_y);
        canvas.draw_point(point).unwrap();

        if count == iterates {
            break;
        }
    }
}
