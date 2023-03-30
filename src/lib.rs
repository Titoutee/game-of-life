//! A simple Game of Life simulation written in Rust
//! Rules of the gaes (reminder):
//! ->
//! Any live cell with two or three live neighbours survives.
//! Any dead cell with three live neighbours becomes a live cell.
//! All other live cells die in the next generation. Similarly, all other dead cells stay dead.

pub use grid::Grid;
use std::{fmt::Debug};
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
}

pub trait GridExt {
    //Using the Grid advantages while adding behaviour (extension trait)
    fn display(&self);
    fn ring(&self, row: usize, col: usize) -> Vec<Cell>;
    fn live_neighs(&self, row: usize, col: usize) -> usize;
    fn get_row_checked(&self, idx: usize) -> Option<Vec<&Cell>>;
    fn get_col_checked(&self, idx: usize) -> Option<Vec<&Cell>>;
}

impl GridExt for Grid<Cell> {
    fn live_neighs(&self, row: usize, col: usize) -> usize {
        todo!();
    }
    fn ring(&self, row: usize, col: usize) -> Vec<Cell> {
        let res: Vec<Cell> = Vec::with_capacity(8 /*Max number of neighbours of a cell*/);
        res
    }
    fn get_row_checked(&self, idx: usize) -> Option<Vec<&Cell>> {
        if idx < self.rows() {
            return Some(self.iter_row(idx).collect())
        }
        None
    }
    fn get_col_checked(&self, idx: usize) -> Option<Vec<&Cell>> {
        if idx < self.cols() {
            return Some(self.iter_col(idx).collect())
        }
        None
    }
    fn display(&self) {
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
    use super::{Grid, Cell, GridExt};

    #[test]
    //#[should_panic]
    fn rounding() {
        let grid: Grid<Cell> = Grid::new(30, 30);
        let a = grid.get_row_checked(29).unwrap();
        println!("{:?}", a);
    }
}