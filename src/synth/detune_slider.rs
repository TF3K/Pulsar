use std::io::{stdout, Write};

pub struct Slider {
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub width: usize,
}

impl Slider {
    pub fn new(min: f32, max: f32, width: usize) -> Self {
        Slider {
            min,
            max,
            value: (min + max) / 2.0,
            width,
        }
    }

    pub fn update_value(&mut self, position: usize) {
        let ratio = position as f32 / self.width as f32;
        self.value = self.min + ratio * (self.max - self.min);
    }

    pub fn draw(&self) {
        // Move cursor to the beginning of the line
        print!("\r");

        let filled_length = ((self.value - self.min) / (self.max - self.min) * self.width as f32) as usize;

        print!("[");
        for _ in 0..filled_length {
            print!("=");
        }
        for _ in filled_length..self.width {
            print!("-");
        }
        print!("] {:2}", self.value);

        // Flush to ensure immediate update
        stdout().flush().unwrap();
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }
}