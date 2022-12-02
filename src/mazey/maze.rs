use std::vec;
use super::direction::Direction;

// Represents the coordinate of a position in the maze
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct CellPos {
    pub row: i32,
    pub col: i32,
}

impl CellPos {
    pub fn to_index(&self, maze: &Maze) -> usize {
        let cols_as_int = maze.columns as i32;
        (self.row * cols_as_int + self.col) as usize
    }

    pub fn is_in_maze(&self, maze: &Maze) -> bool {
        let row_in_range = self.row >= 0 && self.row < maze.rows.try_into().unwrap();
        let col_in_range = self.col >= 0 && self.col < maze.columns.try_into().unwrap();
        row_in_range && col_in_range
    }

    pub fn go(self, direction: Direction) -> Self {
        let (delta_row, delta_col) = direction.to_delta();
        Self {
            row: self.row + delta_row,
            col: self.col + delta_col
        }
    }
}

#[derive(Debug)]
pub struct Maze {
    pub rows: usize,
    pub columns: usize,
    pub cells: Vec<u32>
}

impl Maze {
    pub fn new(rows: usize, columns: usize) -> Self {
        let mut cells = vec![0u32; rows * columns];

        // // Temporary fill to test printing
        // let mut mask = 7;
        // for i in 0..rows * columns {
        //     cells[i] = mask;
        //     mask = (mask + 1) % 16;
        // }
        cells[0] = 15;
        cells[9 * columns + 9] = 15;
        cells[9 * columns + 10] = Direction::Left.to_door_mask() as u32;
        cells[10 * columns + 9] = Direction::Up.to_door_mask() as u32;
        // Top row can't go up
        for i in 0..columns {
            cells[i] = cells[i] & (!Direction::Up.to_door_mask() as u32);
        }

        // Bottom row can't go down
        let last_row_start = (rows - 1) * columns;
        for i in 0..columns {
            cells[last_row_start + i] = cells[last_row_start + i] & (!Direction::Down.to_door_mask() as u32);
        }

        Maze {
            rows,
            columns,
            cells
        }
    }

    pub fn can_go(&self, pos: CellPos, direction: Direction) -> bool {
        if !pos.is_in_maze(self) {
            return false;
        }

        let index = pos.to_index(self);
        let cell_mask = self.cells[index];
        let direction_mask = direction.to_door_mask() as u32;
        cell_mask & direction_mask != 0
    }
}
