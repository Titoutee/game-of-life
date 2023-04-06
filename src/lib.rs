//! A simple Game of Life simulation written in Rust
//! Rules of the gaes (reminder):
//! ->
//! Any live cell with two or three live neighbours survives.
//! Any dead cell with three live neighbours becomes a live cell.
//! All other live cells die in the next generation. Similarly, all other dead cells stay dead.

use grid::Grid;
use std::{
    fmt::{Debug, Display},
    time::Duration,
};
#[derive(Default, Debug, PartialEq)]
/// A cell
pub enum Cell {
    //Binary state of a cell
    Live,
    #[default]
    Dead,
}
impl Cell {
    pub fn as_char(&self) -> char {
        match self {
            Cell::Live => 'X',
            Cell::Dead => ' ',
        }
    }
    /// Follow the rules and determines if the cell has changed or not
    fn has_to_change(&self, neighs: usize) -> bool {
        let substitute = match self {
            Cell::Live => {
                if neighs == 2 || neighs == 3 {
                    Cell::Live
                } else {
                    Cell::Dead
                }
            }
            Cell::Dead => {
                if neighs == 3 {
                    Cell::Live
                } else {
                    Cell::Dead
                }
            }
        };
        *self != substitute
    }
    /// Switch from on state to the other
    fn switched(&self) -> Cell {
        match *self {
            Cell::Dead => Cell::Live,
            Cell::Live => Cell::Dead,
        }
    }
}

pub struct LifeGrid {
    grid: Grid<Cell>,
}
impl LifeGrid {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            grid: Grid::new(row, col),
        } // Default is
    }
    pub fn grid(&self) -> &Grid<Cell> {
        &self.grid
    }
    pub fn grid_mut(&mut self) -> &mut Grid<Cell> {
        &mut self.grid
    }
    /// Helper for getting rows number
    fn rows(&self) -> usize {
        self.grid().rows()
    }
    /// Helper for getting cols number
    fn cols(&self) -> usize {
        self.grid().cols()
    }
    /// Mutates the given Cell (coords) of the LifeGrid
    pub fn muts(&mut self, row: usize, col: usize, pred: Cell) {
        // Might be more robust
        *self.grid_mut().get_mut(row, col).unwrap() = pred;
    }
    /// Counts the living neighbouring cells of a given cell
    fn live_neighs(&self, row: usize, col: usize) -> usize {
        self.ring(row, col)
            .into_iter()
            .filter(|cell| matches!(**cell, Cell::Live))
            .count()
    }
    /// Gets the surrouding cells of a given cell
    fn ring(&self, row: usize, col: usize) -> Vec<&Cell> {
        const OFFSET: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        OFFSET
            .iter()
            .filter_map(|(off_r, off_c)| {
                let r_get = row.checked_add_signed(*off_r)?; // Exclude if overflow
                let c_get = col.checked_add_signed(*off_c)?; // Exclude if overflow
                self.grid().get(r_get, c_get) // -> Should only include the "existing cells" around
            })
            .collect()
    }
    /// Returns a vector of coords, to be processed by an update procedure
    /// The coords are linked to the cell which will undergo a swap in the next timestep
    fn construct_changes(&mut self) -> Vec<(usize, usize)> {
        let mut changes = Vec::with_capacity(self.full_size()); //Maximum changes is the number of elts itself
        let rows_n = self.grid().rows(); // Avoid immutable-mutable borrowing
        for idx in 0..self.full_size() {
            let (row, col) = from_simple(idx, rows_n);
            let cell = self.grid().get(row, col).unwrap(); // This shoudn't panic
            if cell.has_to_change(self.live_neighs(row, col)) {
                changes.push((row, col));
            }
        }
        changes
    }
    /// Main updating process function
    pub fn update(&mut self) {
        for (row, col) in self.construct_changes() {
            // All changes to process
            self.muts(row, col, self.grid().get(row, col).unwrap().switched())
            //Switch internal Cell
        }
    }
    /// Full size (inner vec)
    fn full_size(&self) -> usize {
        self.rows() * self.cols()
    }
}
impl Display for LifeGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for row in 0..self.rows() {
            self.grid().iter_row(row).for_each(|cell| {
                output.push_str(&format!("|{}", cell.as_char()));
            });
            output.push_str("|\n");
        }
        writeln!(f, "{}", output)
    }
}
/// Converts an index for a flattened `Vec` to a doublet (row, col) in this order
fn from_simple(simple: usize, rows: usize) -> (usize, usize) {
    (simple / rows, simple % rows)
}

/// Parses time amount from the command line
pub fn parse_time(mut args: impl Iterator<Item = String>) -> Option<Duration> {
    args.next()?;
    Some(Duration::from_millis(args.next()?.parse().ok()?))
}

/// Parses a coords entry into a tuple of two coordinates
pub fn parse_to_tuple(input: &str) -> Option<(usize, usize)> {
    let mut coords = input.trim().split(',').filter_map(|coord| coord.parse::<usize>().ok());
    Some((coords.next()?, coords.next()?))
}

#[cfg(test)]
mod tests {
    use super::Cell;
    #[test]
    #[ignore = "reason"]
    fn has_changed() {
        //let mut grid: Grid<Cell> = Grid::new(30, 30);
        let cell = Cell::Dead;
        assert_eq!(cell.has_to_change(45), false);
    }
}
