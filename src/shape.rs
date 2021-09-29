use crate::render::{RenderParameter, Renderable};
use crate::screen::{Frame, Pixel};

pub struct Donut {
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub smoothness: u32,
}

impl Renderable for Donut {
    fn render(&mut self, param: RenderParameter, zframe: &mut Frame) -> Frame {
        #![allow(clippy::many_single_char_names)]
        let mut frame = Frame::new(param.window_size);
        let a = param.a;
        let b = param.b;
        let sin_a = a.sin();
        let sin_b = b.sin();
        let cos_a = a.cos();
        let cos_b = b.cos();
        for j in 0..self.smoothness {
            let real_j = j as f32 * (2.0 * std::f32::consts::PI / self.smoothness as f32);
            let sin_j = real_j.sin();
            let cos_j = real_j.cos();
            let cos_j2 = cos_j + 2.0;
            for i in 0..self.smoothness {
                let real_i = i as f32 * (2.0 * std::f32::consts::PI / self.smoothness as f32);
                let sin_i = real_i.sin();
                let cos_i = real_i.cos();

                let mess = 1.0 / (sin_i * cos_j2 * sin_a + sin_j + cos_a + 5.0);
                let t = sin_i * cos_j2 * cos_a - sin_j * sin_a;

                let x = (40.0 + 30.0 * mess * (cos_i * cos_j2 * cos_b - t * sin_b)) as usize;
                let y = (11.0 + 15.0 * mess * (cos_i * cos_j2 * sin_b + t * cos_b)) as usize;
                let n = (8.0
                    * ((sin_j * sin_a - sin_i * cos_j * cos_a) * cos_b
                        - sin_i * cos_j * sin_a
                        - sin_j * cos_a
                        - cos_i * cos_j * sin_b)) as usize;
                if 0 < y
                    && y < param.window_size.height
                    && 0 < x
                    && x < param.window_size.width
                    && (zframe.get(x, y).0) < mess
                {
                    zframe.set(x, y, Pixel(mess));
                    frame.set(x, y, Pixel(n as f32 + 1.0));
                }
            }
        }
        frame
    }
}
