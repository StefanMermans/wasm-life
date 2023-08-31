use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

use self::world::World;

mod point;
mod world;
mod screen;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

#[wasm_bindgen]
pub struct RustLife {
   world: World,
}

#[wasm_bindgen]
impl RustLife {
    pub fn new() -> Self {
        Self {
            world: World::new(WIDTH, HEIGHT)
        }
    }

    pub fn on_frame(&mut self, ctx: &CanvasRenderingContext2d) {
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.world.screen.pixel_buffer),
            WIDTH,
            HEIGHT,
        )
        .expect("should create ImageData from array");

        ctx.put_image_data(&data, 0.0, 0.0)
            .expect("should write array to context");
    }

    pub fn width(&self) -> u32 {
        self.world.screen.width
    }

    pub fn height(&self) -> u32 {
        self.world.screen.height
    }

    pub fn click(&mut self, x: i32, y: i32) {

    }
}