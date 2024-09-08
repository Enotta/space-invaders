use bevy::prelude::*;

/// Alien position in the matrix
#[derive(Component, Hash, PartialEq, Eq)]
pub struct AlienPos {
    row: usize,
    col: usize
}

impl AlienPos {
    /// Create new position in alien 2d-matrix
    pub fn new(
        r: usize, 
        c: usize
    ) -> Self {
        AlienPos {
            row: r,
            col: c
        }
    }

    /// Get position row number
    pub fn get_row(&self) -> usize {
        self.row
    }

    /// Get position column number
    pub fn get_col(&self) -> usize {
        self.col
    }
}