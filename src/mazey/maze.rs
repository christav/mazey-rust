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

#[derive(Clone, Copy, Debug)]
struct CellData {
    door_mask: u32,
    mark: u32
}

impl CellData {
    pub fn new() -> Self {
        CellData {
            door_mask: 0,
            mark: 0
        }
    }
}

#[derive(Debug)]
pub struct Maze {
    pub rows: usize,
    pub columns: usize,
    cells: Vec<CellData>
}

impl Maze {
    pub fn new(rows: usize, columns: usize) -> Self {
        let mut cells = Vec::with_capacity(rows * columns);
        for i in 0..rows * columns {
            cells.push(CellData::new());
        }

        // // Temporary fill to test printing
        // let mut mask = 7;
        // for i in 0..rows * columns {
        //     cells[i] = mask;
        //     mask = (mask + 1) % 16;
        // }
        cells[0].door_mask = 15;
        cells[9 * columns + 9].door_mask = 15;
        cells[9 * columns + 10].door_mask = Direction::Left.to_door_mask();
        cells[10 * columns + 9].door_mask = Direction::Up.to_door_mask();
        // Top row can't go up
        for i in 0..columns {
            cells[i].door_mask &= !Direction::Up.to_door_mask();
        }

        // Bottom row can't go down
        let last_row_start = (rows - 1) * columns;
        for i in 0..columns {
            cells[last_row_start + i].door_mask &= !Direction::Down.to_door_mask();
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
        let cell_mask = self.cells[index].door_mask;
        let direction_mask = direction.to_door_mask();
        cell_mask & direction_mask != 0
    }
}
