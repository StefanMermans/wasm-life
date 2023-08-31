use crate::point::{Point, self};

use super::screen::Screen;

pub struct World {
    pub screen: Screen,
    world_state: Vec<bool>
}

impl World {
    pub fn new(width: u32, height: u32) -> Self {
        let world_size = width * height;
        let mut world = Self {
            screen: Screen::new(width, height),
            world_state: vec![false; world_size as usize]
        };

        world.set_state_at(true, &Point { x: 10, y: 10 });
        world.set_state_at(true, &Point { x: 11, y: 11 });
        world.set_state_at(true, &Point { x: 12, y: 12 });
        world.set_state_at(true, &Point { x: 13, y: 13 });
        world.set_state_at(true, &Point { x: 14, y: 14 });
        world.set_state_at(true, &Point { x: 15, y: 15 });
        world.set_state_at(true, &Point { x: 16, y: 16 });
        world.set_state_at(true, &Point { x: 17, y: 17 });

        world
    }

    pub fn set_state_at(&mut self, state: bool, point: &Point) {
        self.screen.set_color_at(point);
        let index = self.get_index_for(point);
        self.world_state[index] = state;
    }

    fn get_index_for(&self, point: &Point) -> usize {
        point.y * self.screen.width as usize + point.x
    }
}