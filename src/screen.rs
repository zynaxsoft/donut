#[derive(Copy, Clone, Debug)]
pub struct WindowSize {
    pub width: usize,
    pub height: usize,
}

impl Default for WindowSize {
    fn default() -> Self {
        Self {
            width: 40,
            height: 40,
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
        print!("\x1B[2J\x1B[1;1H"); // Clear the screen
        for (i, v) in frame.data.iter().enumerate() {
            if i.rem_euclid(self.window_size.width) == 0 {
                println!();
            }
            print!("{}", v.ascii_repr());
        }
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
}

impl Frame {
    pub fn new(window_size: WindowSize) -> Self {
        Self {
            window_size,
            data: vec![Pixel(0); window_size.width * window_size.height + 1],
        }
    }
}
