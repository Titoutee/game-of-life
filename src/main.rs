use game_of_life::{LifeGrid, parse_time, parse_to_tuple, Cell};
use std::{thread,time::Duration, env, io};
fn main() {
    let args = env::args();
    let time_step = match parse_time(args) {
        Some(t) => t,
        _ => Duration::from_secs(1),
    };
    let mut life_grid = LifeGrid::new(30, 30);
    let mut input_coords = String::new();
    loop {
        io::stdin().read_line(&mut input_coords).unwrap();
        let coords_tuple = match parse_to_tuple(&input_coords) {
            Some(tuple) => tuple,
            _ => break,
        };
        match life_grid.grid_mut().get_mut(coords_tuple.0, coords_tuple.1) {
            Some(cell) => *cell = Cell::Live,
            _ => break, 
        }
    }
    loop {
        println!("{}", life_grid);
        thread::sleep(time_step);
        life_grid.update();
    }
}