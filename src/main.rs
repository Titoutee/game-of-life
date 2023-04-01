use game_of_life::{Cell, Grid, GridExt};
use std::thread;
use std::time::Duration;
fn main() {
    let mut grid: Grid<Cell> = Grid::new(30, 30);
    loop {
        grid.display();
        thread::sleep(Duration::new(2, 0));
    }
}
