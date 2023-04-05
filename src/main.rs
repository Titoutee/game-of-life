use game_of_life::{Cell, LifeGrid};
use std::{thread,time::Duration};
fn main() {
    let mut life_grid = LifeGrid::new(30, 30);
    life_grid.muts(3, 3, Cell::Live);
    life_grid.muts(3, 4, Cell::Live);
    life_grid.muts(2, 5, Cell::Live);
    life_grid.muts(2, 5, Cell::Live);
    life_grid.muts(1, 2, Cell::Live);
    life_grid.muts(1, 2, Cell::Live);
    loop {
        println!("{}", life_grid);
        thread::sleep(Duration::new(1, 0));
        life_grid.update();
    }
}