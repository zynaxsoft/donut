#[derive(Copy, Clone, Debug)]
pub struct WindowSize {
    pub width: usize,
    pub height: usize,
}

impl Default for WindowSize {
    fn default() -> Self {
        Self {
            width: 80,
            height: 24,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Screen {
    window_size: WindowSize,
}

impl Screen {
    pub fn new(window_size: WindowSize) -> Self {
        Self { window_size }
    }

    pub fn draw(&self, frame: &Frame) {
        let mut result = String::new();
        result.push_str("\x1b[H");
        for (i, v) in frame.data.iter().enumerate() {
            if i.rem_euclid(self.window_size.width) == 0 {
                result.push('\n');
            }
            result.push(v.ascii_repr());
        }
        print!("{}", result);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Pixel(pub usize);

impl Pixel {
    fn ascii_repr(&self) -> char {
        const RENDER_CHAR: [char; 12] =
            ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];
        let value = self.0.max(0).min(11);
        if value == 0 {
            return ' ';
        }
        RENDER_CHAR[value]
    }
}

#[derive(Debug)]
pub struct Frame {
    pub window_size: WindowSize,
    pub data: Vec<Pixel>,
}

impl Frame {
    pub fn set(&mut self, x: usize, y: usize, value: Pixel) {
        self.data[y * self.window_size.width + x] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> Pixel {
        self.data[y * self.window_size.width + x]
    }
}

impl Frame {
    pub fn new(window_size: WindowSize) -> Self {
        Self {
            window_size,
            data: vec![Pixel(0); window_size.width * window_size.height + 1],
        }
    }
}
