//! A simple Game of Life simulation written in Rust
//! Rules of the gaes (reminder):
//! ->
//! Any live cell with two or three live neighbours survives.
//! Any dead cell with three live neighbours becomes a live cell.
//! All other live cells die in the next generation. Similarly, all other dead cells stay dead.

mod utils;
pub use grid::Grid;
use std::{fmt::Debug};
use utils::*;
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
        if *self == substitute {
            false
        } else {
            true
        }
    }
    /// Switch from on state to the other
    fn switch(&self) -> Cell {
        if matches!(*self, Cell::Live) {
            return Cell::Dead;
        }
        Cell::Live
    }
}

/// Extension trait for Grid data structure
pub trait GridExt {
    fn display(&self);
    fn ring(&self, row: usize, col: usize) -> Vec<&Cell>;
    fn live_neighs(&self, row: usize, col: usize) -> usize;
    /// Converts a simple index to a doublet
    fn construct_changes(&mut self) -> Vec<(usize, usize)>;
    fn update(&mut self);
    fn full_size(&self) -> usize;
}

impl GridExt for Grid<Cell> {
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
                self.get(r_get, c_get) // -> Should only include the "existing cells" around
            })
            .collect()
    }
    /// Returns a vector of coords, to be processed by an update procedure
    /// The coords are linked to the cell which will undergo a swap in the next timestep
    fn construct_changes(&mut self) -> Vec<(usize, usize)> {
        let mut changes = Vec::with_capacity(self.full_size()); //Maximum changes is the number of elts itself
        let rows_n = self.rows(); // Avoid immutable-mutable borrowing
        for idx in 0..self.full_size() {
            let (row, col) = from_simple(idx, rows_n);
            let cell = self.get(row, col).unwrap(); // This shoudn't panic
            if cell.has_to_change(self.live_neighs(row, col)) {
                changes.push((row, col));
            }
        }
        changes
    }
    /// Main updating process function
    fn update(&mut self) {
        for (row, col) in self.construct_changes() {
            // All changes to process
            *self.get_mut(row, col).unwrap() = self.get(row, col).unwrap().switch();
            //Switch internal Cell
        }
    }
    /// Full size (inner vec)
    fn full_size(&self) -> usize {
        self.size().0 * self.size().1
    }
    fn display(&self) {
        println!("{}", str_repeat("-", 20)); //Separate each update
        (0..self.rows()).for_each(|row| {
            for cell in self.iter_row(row) {
                print!("{}|", cell.as_char())
            }
            println!("");
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Cell, Grid, GridExt};
    use crate::utils::from_simple;
    #[test]
    #[ignore = "reason"]
    //#[should_panic]
    fn neighbours() {
        let mut grid: Grid<Cell> = Grid::new(30, 30);
        *grid.get_mut(1, 1).unwrap() = Cell::Live;
        *grid.get_mut(2, 1).unwrap() = Cell::Live;

        assert_eq!(grid.live_neighs(1, 2), 2);
    }
    #[test]
    #[ignore = "reason"]
    fn to_double() {
        //let grid: Grid<Cell> = Grid::new(30, 30);
        assert_eq!(from_simple(30, 30), (1, 0));
    }
    #[test]
    #[ignore = "reason"]
    fn construct_changes() {
        let mut grid: Grid<Cell> = Grid::new(30, 30);
        *grid.get_mut(3, 4).unwrap() = Cell::Live;
        *grid.get_mut(3, 5).unwrap() = Cell::Live;
        //*grid.get_mut(3, 6).unwrap()=Cell::Live;
        assert_eq!(grid.construct_changes(), vec![]);
    }
    #[test]
    #[ignore = "reason"]
    fn has_changed() {
        //let mut grid: Grid<Cell> = Grid::new(30, 30);
        let cell = Cell::Dead;
        assert_eq!(cell.has_to_change(45), false);
    }
    #[test]
    #[ignore = "reason"]
    fn ring() {
        let grid: Grid<Cell> = Grid::new(30, 30);
        println!("{:?}", grid.ring(0, 0));
    }
}
