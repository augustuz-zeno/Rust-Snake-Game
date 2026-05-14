use minifb::{Key, Window, WindowOptions};

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;

pub struct Engine {
    window: Window,
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl Engine {
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        let mut window = Window::new(
            title,
            width,
            height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Self {
            window,
            buffer: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn clear(&mut self, color: u32) {
        for pixel in self.buffer.iter_mut() {
            *pixel = color;
        }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        for i in 0..w {
            for j in 0..h {
                self.draw_pixel(x + i, y + j, color);
            }
        }
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }

    pub fn get_mouse_pos(&self) -> Option<(f32, f32)> {
        self.window.get_mouse_pos(minifb::MouseMode::Pass).map(|(x, y)| (x as f32, y as f32))
    }

    pub fn draw_vignette(&mut self) {
        let center_x = self.width as f32 / 2.0;
        let center_y = self.height as f32 / 2.0;
        let max_dist = (center_x * center_x + center_y * center_y).sqrt();

        for y in 0..self.height {
            for x in 0..self.width {
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                let dist = (dx * dx + dy * dy).sqrt();
                // Sharper vignette falloff
                let factor = 1.0 - (dist / max_dist).powf(3.0) * 0.8;
                
                let idx = y * self.width + x;
                let color = self.buffer[idx];
                let r = (((color >> 16) & 0xFF) as f32 * factor) as u32;
                let g = (((color >> 8) & 0xFF) as f32 * factor) as u32;
                let b = ((color & 0xFF) as f32 * factor) as u32;
                self.buffer[idx] = (r << 16) | (g << 8) | b;
            }
        }
    }

    pub fn get_string_width(&self, s: &str, scale: usize) -> usize {
        if s.is_empty() {
            0
        } else {
            (s.len() * 4 * scale) - scale
        }
    }

    pub fn draw_char(&mut self, x: usize, y: usize, c: char, color: u32, scale: usize) {
        let pixels: [u8; 5] = match c.to_ascii_uppercase() {
            '0' => [0b111, 0b101, 0b101, 0b101, 0b111],
            '1' => [0b010, 0b110, 0b010, 0b010, 0b111],
            '2' => [0b111, 0b001, 0b111, 0b100, 0b111],
            '3' => [0b111, 0b001, 0b111, 0b001, 0b111],
            '4' => [0b101, 0b101, 0b111, 0b001, 0b001],
            '5' => [0b111, 0b100, 0b111, 0b001, 0b111],
            '6' => [0b111, 0b100, 0b111, 0b101, 0b111],
            '7' => [0b111, 0b001, 0b001, 0b001, 0b001],
            '8' => [0b111, 0b101, 0b111, 0b101, 0b111],
            '9' => [0b111, 0b101, 0b111, 0b001, 0b111],
            'A' => [0b111, 0b101, 0b111, 0b101, 0b101],
            'B' => [0b110, 0b101, 0b110, 0b101, 0b110],
            'C' => [0b111, 0b100, 0b100, 0b100, 0b111],
            'D' => [0b110, 0b101, 0b101, 0b101, 0b110],
            'E' => [0b111, 0b100, 0b111, 0b100, 0b111],
            'F' => [0b111, 0b100, 0b111, 0b100, 0b100],
            'G' => [0b111, 0b100, 0b101, 0b101, 0b111],
            'H' => [0b101, 0b101, 0b111, 0b101, 0b101],
            'I' => [0b111, 0b010, 0b010, 0b010, 0b111],
            'J' => [0b001, 0b001, 0b001, 0b101, 0b111],
            'K' => [0b101, 0b101, 0b110, 0b101, 0b101],
            'L' => [0b100, 0b100, 0b100, 0b100, 0b111],
            'M' => [0b101, 0b111, 0b101, 0b101, 0b101],
            'N' => [0b111, 0b101, 0b101, 0b101, 0b101],
            'O' => [0b111, 0b101, 0b101, 0b101, 0b111],
            'P' => [0b111, 0b101, 0b111, 0b100, 0b100],
            'Q' => [0b111, 0b101, 0b101, 0b111, 0b001],
            'R' => [0b111, 0b101, 0b111, 0b101, 0b101],
            'S' => [0b111, 0b100, 0b111, 0b001, 0b111],
            'T' => [0b111, 0b010, 0b010, 0b010, 0b010],
            'U' => [0b101, 0b101, 0b101, 0b101, 0b111],
            'V' => [0b101, 0b101, 0b101, 0b101, 0b010],
            'W' => [0b101, 0b101, 0b101, 0b111, 0b101],
            'X' => [0b101, 0b101, 0b010, 0b101, 0b101],
            'Y' => [0b101, 0b101, 0b010, 0b010, 0b010],
            'Z' => [0b111, 0b001, 0b010, 0b100, 0b111],
            _ => [0b000, 0b000, 0b000, 0b000, 0b000],
        };

        for row in 0..5 {
            for col in 0..3 {
                if (pixels[row] >> (2 - col)) & 1 == 1 {
                    self.draw_rect(x + col * scale, y + row * scale, scale, scale, color);
                }
            }
        }
    }

    pub fn draw_string(&mut self, x: usize, y: usize, s: &str, color: u32, scale: usize) {
        let mut curr_x = x;
        for c in s.chars() {
            self.draw_char(curr_x, y, c, color, scale);
            curr_x += 4 * scale;
        }
    }

    pub fn draw_string_centered(&mut self, y: usize, s: &str, color: u32, scale: usize) {
        let width = self.get_string_width(s, scale);
        let x = if self.width > width { (self.width - width) / 2 } else { 0 };
        self.draw_string(x, y, s, color, scale);
    }
}
