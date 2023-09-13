use std::ops;

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct SizePoint {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn empty() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }

    pub fn update_new(&self, offset_x: i32, offset_y: i32) -> Self {
        Self {
            x: self.x + offset_x,
            y: self.y + offset_y
        }
    }

    pub fn to_size(&self, max_x: i32, max_y: i32) -> Option<SizePoint> {
        if self.x < 0 || self.y < 0 || self.x >= max_x || self.y >= max_y {
            return None;
        }

        Some(SizePoint {
            x: self.x as usize,
            y: self.y as usize,
        })
    }

    pub fn to_string(&self) -> String {
        let mut string = String::from("(");

        string += self.x.to_string().as_str();
        string += ",";
        string += self.y.to_string().as_str();

        string += ")";

        string
    }
}

impl ops::Add<i32> for Point {
    type Output = Point;

    fn add(self, rhs: i32) -> Self::Output {
        Point::new(self.x + rhs, self.y + rhs)
    }
}

impl ops::AddAssign<i32> for Point {
    fn add_assign(&mut self, rhs: i32) {
        self.x += rhs;
        self.y += rhs;
    }
}

