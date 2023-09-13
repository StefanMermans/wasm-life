use std::{thread, time::Duration};

use life::Life;

use crate::point::Point;

mod life;
mod point;

fn main() {
    let mut life = Life::new(20, 20, false, false);
    life.set_state_at(true, &Point::new(0, 1));
    life.set_state_at(true,&Point::new(1, 1));
    life.set_state_at(true,&Point::new(2, 1));
    life.set_state_at(true,&Point::new(3, 1));
    life.set_state_at(true,&Point::new(4, 1));
    print!("{}", life.world().to_string());
    println!("=======================================");
    print!("{}", life.screen().to_string());

    loop {
        life.step();
        println!("=======================================");
        print!("{}", life.world().to_string());

        thread::sleep(Duration::new(0, 50000000 * 2));
    }
}