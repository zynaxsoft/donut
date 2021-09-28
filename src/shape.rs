use crate::render::{RenderParameter, Renderable};
use crate::screen::{Frame, Pixel};

pub struct Donut {
    pub inner_radius: f32,
    pub outer_radius: f32,
}

impl Renderable for Donut {
    fn render(&mut self, param: RenderParameter) -> Frame {
        let mut frame = Frame::new(param.window_size);
        frame.set(20, 20, Pixel(2));
        frame.set(39, 39, Pixel(param.a.round() as usize));
        frame
    }
}
