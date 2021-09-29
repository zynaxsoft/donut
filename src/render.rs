use crate::screen::{Frame, WindowSize};

pub trait Renderable {
    fn render(&mut self, param:RenderParameter, zframe: &mut Frame) -> Frame;
}

pub struct RenderParameter {
    pub window_size: WindowSize,
    pub a: f32,
    pub b: f32,
    pub k: f32,
}
