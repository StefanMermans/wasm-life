use crate::point::Point;

use self::color::Color;

const BYTES_PER_PIXEL: i32 = 4;

pub mod color;

pub struct Screen {
    pixel_buffer: Vec<u8>,
    width: i32,
    height: i32,
    buffer_width: i32,
}

impl Screen {
    pub fn new(width: i32, height: i32) -> Self {
        let pixel_count = width * height;
        let screen_size_in_bytes = pixel_count * BYTES_PER_PIXEL;
        Self {
            width,
            height,
            pixel_buffer: vec![255u8; screen_size_in_bytes as usize],
            buffer_width: width * BYTES_PER_PIXEL
        }
    }

    pub fn set_color_at(&mut self, color: &Color, point: &Point) {
        let i = self.get_buffer_index_for(point);
        self.pixel_buffer[i..i + 3].copy_from_slice(color.as_slice())
    }

    fn get_buffer_index_for(&self, point: &Point) -> usize {
        ((point.y * self.buffer_width) + (point.x * BYTES_PER_PIXEL)) as usize
    }

    pub fn pixel_buffer(&self) -> &Vec<u8> {
        &self.pixel_buffer
    }

    pub fn to_string(&self) -> String {
        let mut string =  String::new();
        let row_size = (self.width * BYTES_PER_PIXEL) as usize;

        for y in 0..self.height {
            for x in (0..row_size).step_by(BYTES_PER_PIXEL as usize) {
                string += "\x1b[38;2;";

                for depth in 0..3 {
                    let index = x as usize + (y as usize * row_size) + depth as usize;
                    string += self.pixel_buffer[index].to_string().as_str();

                    if depth != 2 {
                        string += ";";
                    }
                }

                string += "m";
                string += "O";
                string += "\x1b[0m"
            }

            string += "\n";
        }

        string
    }
}