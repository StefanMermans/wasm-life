use crate::{point::Point};

use self::{world::World, screen::Screen};

mod world;
mod screen;

pub struct Life {
    world: World,
    screen: Screen,
}

impl Life {
    pub fn new(width: i32, height: i32, lively_edges: bool, seed: bool) -> Self {
        let mut life = Self { 
            world: World::new(width, height, lively_edges),
            screen: Screen::new(width, height),
        };

        if seed {
            life.set_state_at(true, &Point::new(20, 20));
            life.set_state_at(true, &Point::new(20, 21));
            life.set_state_at(true, &Point::new(20, 22));

            // Top
            life.set_state_at(true, &Point::new(100, 100));
            life.set_state_at(true, &Point::new(100, 101));
            life.set_state_at(true, &Point::new(100, 102));
        
            // Left
            life.set_state_at(true, &Point::new(98, 104));
            life.set_state_at(true, &Point::new(97, 104));
            life.set_state_at(true, &Point::new(96, 104));

            // Right
            life.set_state_at(true, &Point::new(102, 104));
            life.set_state_at(true, &Point::new(103, 102));
            life.set_state_at(true, &Point::new(104, 102));
        }

        life
    }

    pub fn screen(&self) -> &Screen {
        &self.screen
    }

    pub fn set_state_at(&mut self, state: bool, point: &Point) {
        if state {
            self.screen.set_color_at(&[0u8, 0u8, 0u8], point);
        } else {
            self.screen.set_color_at(&[0u8, 0u8, 0u8], point)
        }

        self.world.set_state_at(state, point);
    }

    pub fn invert_state_at(&mut self, point: &Point) {
        let state = !self.world.get_state_at(point);
        self.set_state_at(state, point);

        if state {
            self.screen.set_color_at(&[0u8, 0u8, 0u8], point);
        } else {
            self.screen.set_color_at(&[255u8, 255u8, 255u8], point);
        }
    }

    pub fn step(&mut self) {
        let mut new_world = World::new(self.world.width(), self.world.height(), self.world.lively_edges());
        let mut new_screen = Screen::new(self.world.width(), self.world.height());
        for x in 0..self.world.width() {
            for y in 0..self.world.height() {
                self.calculate_next_state(&Point::new(x as i32, y as i32), &mut new_world, &mut new_screen);
            }
        }

        self.world = new_world;
        self.screen = new_screen;
    }

    fn calculate_next_state(&self, point: &Point, new_world: &mut World, new_screen: &mut Screen) {
        let current_state = self.world.get_state_at(point);
        let mut live_neighbors = 0usize;

        // Row above
        if self.world.get_state_at(&point.update_new(-1, -1)) {
            live_neighbors += 1;
        }
        if self.world.get_state_at(&point.update_new(0, -1)) {
            live_neighbors += 1;
        }
        if self.world.get_state_at(&point.update_new(1, -1)) {
            live_neighbors += 1;
        }

        // Own row
        if self.world.get_state_at(&point.update_new(-1, 0)) {
            live_neighbors += 1;
        }
        if self.world.get_state_at(&point.update_new(1, 0)) {
            live_neighbors += 1;
        }

        // Row below
        if self.world.get_state_at(&point.update_new(-1, 1)) {
            live_neighbors += 1;
        }
        if self.world.get_state_at(&point.update_new(0, 1)) {
            live_neighbors += 1;
        }
        if self.world.get_state_at(&point.update_new(1, 1)) {
            live_neighbors += 1;
        }

        let state = self.next_state_for_cell(current_state, live_neighbors);
        new_world.set_state_at(state, point);
        
        if state {
            new_screen.set_color_at(&[0u8, 0u8, 0u8], point)
        } else {
            new_screen.set_color_at(&[255u8, 255u8, 255u8], point)
        }
    }

    fn next_state_for_cell(&self, is_alive: bool, live_neighbors: usize) -> bool {
        if is_alive {
            return match live_neighbors {
                2 | 3 => true,
                _ => false,
            }
        }

        live_neighbors == 3
    }

    pub fn to_string(&self) -> String {
        self.screen.to_string()
    }

    pub fn world(&self) -> &World {
        &self.world
    }

}
