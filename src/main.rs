use mouse_rs::Mouse;
use rand::prelude::*;
use std::{thread, time};

const DELAY_MS: u64 = 100;

fn main() {
    let mouse = Mouse::new();
    let mut rng = rand::thread_rng();

    loop {
        let mut position = mouse.get_position()
            .expect("Coould not get mouse position");
        position.x += rng.gen_range(-2..2);
        position.y += rng.gen_range(-2..2);
        mouse.move_to(position.x, position.y)
            .expect("Could not move the mouse");
        thread::sleep(time::Duration::from_millis(DELAY_MS));
    }
}
