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

    pub fn go(self, direction: &Direction) -> Self {
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

        let mut m = Maze {
            rows,
            columns,
            cells
        };

        let test_cell = CellPos { row: 9, col: 9 };
        m.open_door(test_cell, &Direction::Up);
        m.open_door(test_cell, &Direction::Left);

        m.open_door(CellPos { row: 0, col: 0}, &Direction::Left);
        m
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

    fn open_door_at(&mut self, pos: CellPos, direction: &Direction) {
        if pos.is_in_maze(self) {
            let index = pos.to_index(self);
            self.cells[index].door_mask |= direction.to_door_mask();
        }
    }

    pub fn open_door(&mut self, pos: CellPos, direction: &Direction) {
        self.open_door_at(pos, direction);
        self.open_door_at(pos.go(direction), direction.to_opposite());
    }

}
