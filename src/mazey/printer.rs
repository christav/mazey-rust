// Helper function to print out a maze

use super::maze::{ Maze, CellPos };
use super::direction::Direction;

pub trait MazeCharset {
    fn corner_char(&self, index: usize) -> char;
    // fn solution_chars(&self, index: usize) -> String;
    fn horizontal_line(&self) -> String;
}

pub struct AsciiCharSet;

impl MazeCharset for AsciiCharSet {
    fn corner_char(&self, index: usize) -> char {
        ASCII_CORNER_CHARS[index]
    }

    fn horizontal_line(&self) -> String {
        format!("{}{}{}", self.corner_char(10), self.corner_char(10), self.corner_char(10))
    }
}

const ASCII_CORNER_CHARS: [char; 16] = [
    ' ',
    '+',
    '+',
    '+',
    '+',
    '|',
    '+',
    '+',
    '+',
    '+',
    '-',
    '+',
    '+',
    '+',
    '+',
    '+'
];

fn horizontal_line() -> String {
    format!("{}{}{}", ASCII_CORNER_CHARS[10], ASCII_CORNER_CHARS[10], ASCII_CORNER_CHARS[10])
}

// const ascii_solution_chars: &[&str] = &[
//     "   ",
//     "   ",
//     "   ",
//     " XX",
//     "   ",
//     " X ",
//     " XX",
//     "   ",
//     "   ",
//     "XX ",
//     "XXX",
//     "   ",
//     "XX ",
//     "   ",
//     "   ",
//     "   "
// ];

pub struct MazePrinter {
    char_set: &dyn MazeCharset
}

impl MazePrinter {
    pub fn new(char_set: &dyn MazeCharset) -> Self {
        Self {
            char_set
        }
    }
}

pub fn print_maze(m: &Maze) {
    for row in 0..m.rows as i32 {
        print_row_separator(m, row);
        print_row(m, row);
    }
    print_maze_bottom(m);
}

fn corner_char(maze: &Maze, pos: CellPos) -> char {
    let mut index: usize = 0;
    index |= if maze.can_go(pos, Direction::Up) && maze.can_go(pos.go(Direction::Up), Direction::Left) { 0 } else { 1 };
    index |= if maze.can_go(pos, Direction::Up) { 0 } else { 2 };
    index |= if maze.can_go(pos, Direction::Left) { 0 } else { 4 };
    index |= if maze.can_go(pos, Direction::Left) && maze.can_go(pos.go(Direction::Left), Direction::Up) { 0 } else { 8 };

    if pos.row == 0 {
        index &= 0xe;
    }
    if pos.col == 0 {
        index &= 0x7;
    }

    ASCII_CORNER_CHARS[index]
}

fn print_row_separator(maze: &Maze, row: i32) {
    for col in 0..maze.columns as i32 {
        let pos = CellPos { row, col };
        print!("{}", corner_char(maze, pos));
        if maze.can_go(pos, Direction::Up) {
            print!("   ");
        } else {
            print!("{}", horizontal_line());
        }
    }

    print_row_separator_end(maze, row);
    println!();
}

fn print_row_separator_end(maze: &Maze, row: i32) {
    let mut index: usize = 0;
    let last_cell_pos = CellPos { row, col: (maze.columns as i32) - 1 };
    index |= if maze.can_go(last_cell_pos, Direction::Up) && maze.can_go(last_cell_pos.go(Direction::Up), Direction::Right) { 0 } else { 1 };
    index |= if maze.can_go(last_cell_pos, Direction::Right) { 0 } else { 4 };
    index |= if maze.can_go(last_cell_pos, Direction::Up) { 0 } else { 8 };

    if row == 0 {
        index &= 0xe;
    }
    print!("{}", ASCII_CORNER_CHARS[index]);
}

fn print_row(maze: &Maze, row: i32) {
    for col in 0..maze.columns as i32 {
        let pos = CellPos { row, col };
        if maze.can_go(pos, Direction::Left) {
            print!(" ");
        } else {
            print!("{}", ASCII_CORNER_CHARS[5]);
        }
        print!("{}", cell_contents(maze, pos));
    }

    print!("{}", ASCII_CORNER_CHARS[5]);
    println!();
}

fn cell_contents(_: &Maze, _: CellPos) -> String {
    String::from("   ")
}

fn print_maze_bottom(maze: &Maze) {
    for col in 0..maze.columns as i32 {
        print_bottom_separator(maze, col);
    }
    print_bottom_right_char(maze);
    println!();
}

fn print_bottom_separator(maze: &Maze, col: i32) {
    let pos = CellPos { row: maze.rows as i32 - 1, col };
    let mut index = 0xa | (if maze.can_go(pos, Direction::Left) { 0 } else { 1 });
    if col == 0 {
        index &= 0x7;
    }
    print!("{}", ASCII_CORNER_CHARS[index]);
    print!("{}", horizontal_line());
}

fn print_bottom_right_char(maze: &Maze) {
    let bottom_right_pos = CellPos { row: maze.rows as i32 - 1, col: maze.columns as i32 - 1 };
    let index = 0x8 | (if maze.can_go(bottom_right_pos, Direction::Right) { 0 } else { 1 });
    print!("{}", ASCII_CORNER_CHARS[index]);
}
