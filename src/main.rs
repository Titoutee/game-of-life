use game_of_life::{Cell, Grid, GridExt};
use std::{thread,time::Duration};
fn main() {
    let mut grid: Grid<Cell> = Grid::new(30, 30);
    *grid.get_mut(15, 15).unwrap() = Cell::Live;
    *grid.get_mut(15, 16).unwrap() = Cell::Live;
    *grid.get_mut(15, 17).unwrap() = Cell::Live;
    
    loop {
        grid.display();
        thread::sleep(Duration::new(1, 0));
        grid.update();
    }
}