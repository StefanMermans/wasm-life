use crate::point::Point;

pub struct World {
    pub state: Vec<bool>,
    width: i32,
    height: i32,
    lively_edges: bool,
}

pub struct WorldConfig {
    pub width: i32,
    pub height: i32,
    pub lively_edges: bool
}

impl World {
    pub fn new(width: i32, height: i32, lively_edges: bool) -> Self {
        let world_size = width * height;
        Self {
            state: vec![false; world_size as usize],
            width,
            height,
            lively_edges,
        }
    }

    pub fn set_state_at(&mut self, state: bool, point: &Point) {
        if let Some(index) = self.get_index_for(point) {
            self.state[index] = state;
        }
    }

    fn get_index_for(&self, point: &Point) -> Option<usize> {
        if let Some(size_point) = point.to_size(self.width as i32, self.height as i32) {
            return Some(size_point.y * self.width as usize + size_point.x)
        }

        None
    }

    pub fn get_state_at(&self, point: &Point) -> bool {
        if let Some(index) = self.get_index_for(point) {
            return self.state[index];
        }

        self.lively_edges
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();

        for y in 0..self.width {
            for x in 0..self.height {
                let index = (x + (y * self.width)) as usize;
                let state = self.state[index];
                if state {
                    string += "\x1b[38;2;0;255;0m";
                    string += "A";
                } else {
                    string += "\x1b[38;2;255;0;0m";
                    string += "O";
                }

                string += "\x1b[0m";
            }

            string += "\n";
        }

        string
    }

    pub fn lively_edges(&self) -> bool {
        self.lively_edges
    }
}