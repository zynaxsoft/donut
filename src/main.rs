use donut::render::{RenderParameter, Renderable};
use donut::screen::{Screen, WindowSize};
use donut::shape;

fn main() {
    let mut donut = shape::Donut {
        inner_radius: 1.2,
        outer_radius: 1.2,
    };
    let window_size = WindowSize::default();
    let screen = Screen::new(window_size);
    let mut a = 0.0;
    loop {
        let param = RenderParameter {
            window_size,
            a,
            b: 1.0,
            k: 1.0,
        };
        let frame = donut.render(param);
        screen.draw(&frame);
        if a > 10.0 {
            a = 0.0;
        } else {
            a += 1.0;
        }
    }
}
