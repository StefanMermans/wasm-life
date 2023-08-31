use crate::point::Point;

const BYTES_PER_PIXEL: u32 = 4;

pub struct Screen {
    pub pixel_buffer: Vec<u8>,
    pub pixel_count: u32,
    pub width: u32,
    pub height: u32,

}

impl Screen {
    pub fn new(width: u32, height: u32) -> Self {
        let pixel_count = width * height;
        let screen_size_in_bytes = pixel_count * BYTES_PER_PIXEL;
        Self {
            pixel_count,
            width,
            height,
            pixel_buffer: vec![255u8; screen_size_in_bytes as usize],
        }
    }

    pub fn set_color_at(&mut self, point: &Point) {
        let i = self.get_buffer_index_for(point);
        self.pixel_buffer[i..i + 3].copy_from_slice([0, 0, 0].as_slice())
    }

    fn get_buffer_index_for(&self, point: &Point) -> usize {
        (point.y * self.width as usize + point.x) * BYTES_PER_PIXEL as usize
    }
}