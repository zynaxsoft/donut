use donut::render::{RenderParameter, Renderable};
use donut::screen::{Frame, Screen, WindowSize};
use donut::shape;

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut donut = shape::Donut {
        inner_radius: 1.2,
        outer_radius: 1.2,
        smoothness: 100,
    };
    let window_size = WindowSize::default();
    let screen = Screen::new(window_size);
    let mut a = 0.0;
    let mut b = 0.0;

    const FPS: usize = 30;
    const NS_PER_FRAME: usize = 1_000_000_000 / FPS;
    let mut origin = time_now();
    loop {
        let now = time_now();
        if now - origin > NS_PER_FRAME as u128 {
            let mut zframe = Frame::new(window_size);
            let param = RenderParameter {
                window_size,
                a,
                b,
                k: 1.0,
            };
            let frame = donut.render(param, &mut zframe);
            screen.draw(&frame);
            a += 0.04;
            b += 0.02;
            origin = now;
        }
    }
}

fn time_now() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
}
