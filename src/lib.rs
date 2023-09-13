use life::Life;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

use crate::point::Point;

mod life;
mod point;


const WIDTH: i32 = 500;
const HEIGHT: i32 = 500;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub struct RustLife {
   life: Life,
   width: i32,
   height: i32,
}

#[wasm_bindgen]
impl RustLife {
    pub fn new() -> Self {
        Self {
            life: Life::new(WIDTH, HEIGHT, false),
            width: WIDTH,
            height: HEIGHT,
        }

    }
    
    pub fn on_frame(&mut self, ctx: &CanvasRenderingContext2d) {
        self.life.step();
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.life.screen().pixel_buffer()),
            self.width as u32,
            self.height as u32,
        )
        .expect("should create ImageData from array");

        ctx.put_image_data(&data, 0.0, 0.0)
            .expect("should write array to context");

        let mut point = Point::empty();
        point += 2;
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn click(&mut self, x: i32, y: i32) {
        self.life.invert_state_at(&Point::new(x, y));
    }
}