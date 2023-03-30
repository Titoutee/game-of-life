use game_of_life::{Cell, Grid, GridExt};
fn main() {
    let mut grid: Grid<Cell> = Grid::new(30, 30);
    grid.display();
}
