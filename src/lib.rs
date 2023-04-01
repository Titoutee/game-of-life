//! A simple Game of Life simulation written in Rust
//! Rules of the gaes (reminder):
//! ->
//! Any live cell with two or three live neighbours survives.
//! Any dead cell with three live neighbours becomes a live cell.
//! All other live cells die in the next generation. Similarly, all other dead cells stay dead.

mod utils;
use utils::*;
pub use grid::Grid;
use std::{fmt::Debug, collections::HashMap};
#[derive(Default, Debug)]
pub enum Cell {
    //Binary state of a cell
    Live,
    #[default]
    Dead,
}

impl Cell {
    pub fn as_char(&self) -> char {
        match self {
            Cell::Live => '1',
            Cell::Dead => '0',
        }
    }
    /// Follow the rules and conlude on the following state of the cell
    fn rules(&self, neighs: usize)  -> Self {
        match self {
            Cell::Live => if neighs==2 || neighs==3 {Cell::Live} else {Cell::Dead}
            Cell::Dead => if neighs==3 {Cell::Live} else {Cell::Dead}
        }
    }
    //fn switch(&mut self) {
    //    //Might be unused (change may be explicit)
    //    *self = if let Cell::Live = self {
    //        Cell::Dead
    //    } else {
    //        Cell::Live
    //    }
    //}
}

pub trait GridExt {
    /// Extension trait for Grid data structure
    fn display(&self);
    fn ring(&self, row: usize, col: usize) -> Vec<Option<&Cell>>;
    fn live_neighs(&self, row: usize, col: usize) -> usize;
    //fn get_row_checked(&self, idx: usize) -> Option<Vec<&Cell>>;
    //fn get_col_checked(&self, idx: usize) -> Option<Vec<&Cell>>;
    /// Converts a simple index to a doublet
    fn construct_changes(&mut self) -> HashMap<(usize, usize)/*Coords*/, Cell/*State*/>;
    fn update(&mut self);
}

impl GridExt for Grid<Cell> {
    /// Counts the living neighbouring cells of a given cell
    fn live_neighs(&self, row: usize, col: usize) -> usize {
        let mut living = 0;
        for opt in self.ring(row, col) {
            if let Some(cell) = opt {
                if let Cell::Live = cell {
                    living += 1
                }  
            }
        }
        living
    }
    
    /// Gets the surrouding cells of a given cell
    fn ring(&self, row: usize, col: usize) -> Vec<Option<&Cell>> {
        vec![
            //This may be cleaner than picking up each cell with index calculations
            self.get(row - 1, col - 1),
            self.get(row - 1, col),
            self.get(row - 1, col + 1),
            self.get(row, col - 1),
            self.get(row, col + 1),
            self.get(row + 1, col - 1),
            self.get(row + 1, col),
            self.get(row + 1, col + 1),
        ]
    }
    fn construct_changes(&mut self) -> HashMap<(usize, usize)/*Coords*/, Cell/*New State*/> {
        let mut map = HashMap::new();
        let rows_n = self.rows();
        for (idx, cell) in self.iter().enumerate() {
            let double = from_simple(idx, rows_n);
            map.insert(from_simple(idx, rows_n), cell.rules(self.live_neighs(double.0, double.1)));//
        }
        map
    }
    fn update(&mut self) {
        let changes = self.construct_changes();
        for (row, col) in changes.keys() {
        }
    }
    fn display(&self) {
        println!("{}", str_repeat("-", 20));
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

    #[test]
    //#[should_panic]
    fn neighbours() {
        let mut grid: Grid<Cell> = Grid::new(30, 30);
        *grid.get_mut(3, 2).unwrap() = Cell::Live;
        assert_eq!(grid.live_neighs(3, 3), 1);
    }
    fn to_double() {
        let mut grid: Grid<Cell> = Grid::new(30, 30);
        //assert_eq!(grid.from_simple(30), (1, 0));
    }
}
