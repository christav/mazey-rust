// Helper function to print out a maze

use super::maze::{ Maze, CellPos };
use super::direction::Direction;

// Charset objects encapsulate the set of characters used to
// render the maze on the console.

pub struct MazeCharset<'a> {
    corner_chars: &'a[char; 16]
}

impl<'a> MazeCharset<'a> {
    pub const fn new(corner_chars: &'a[char; 16]) -> Self {
        MazeCharset { corner_chars }
    }

    pub fn corner_char_at(&self, index: usize) -> char {
        self.corner_chars[index]
    }

    pub fn corner_char(&self, maze: &Maze, pos: CellPos) -> char {
        let mut index: usize = 0;
        index |= if maze.can_go(pos.go(&Direction::Up), Direction::Left) { 0 } else { 1 };
        index |= if maze.can_go(pos, Direction::Up) { 0 } else { 2 };
        index |= if maze.can_go(pos, Direction::Left) { 0 } else { 4 };
        index |= if maze.can_go(pos.go(&Direction::Left), Direction::Up) { 0 } else { 8 };

        if pos.row == 0 {
            index &= 0xe;
        }
        if pos.col == 0 {
            index &= 0x7;
        }

        self.corner_chars[index]
    }

    pub fn row_separator_end_char(&self, maze: &Maze, row: i32) -> char {
        let mut index: usize = 0;
        let last_cell_pos = CellPos { row, col: (maze.columns as i32) - 1 };
        index |= if maze.can_go(last_cell_pos, Direction::Up) && maze.can_go(last_cell_pos.go(&Direction::Up), Direction::Right) { 0 } else { 1 };
        index |= if maze.can_go(last_cell_pos, Direction::Right) { 0 } else { 4 };
        index |= if maze.can_go(last_cell_pos, Direction::Up) { 0 } else { 8 };

        if row == 0 {
            index &= 0xe;
        }

        self.corner_chars[index]
    }

    pub fn bottom_right_char(&self, maze: &Maze) -> char {
        let bottom_right_pos = CellPos { row: maze.rows as i32 - 1, col: maze.columns as i32 - 1 };
        let index = 0x8 | (if maze.can_go(bottom_right_pos, Direction::Right) { 0 } else { 1 });
        self.corner_chars[index]
    }

    pub fn bottom_separator_char(&self, maze: &Maze, col: i32) -> char {
        let pos = CellPos { row: maze.rows as i32 - 1, col };
        let mut index = 0xa | (if maze.can_go(pos, Direction::Left) { 0 } else { 1 });
        if col == 0 {
            index &= 0x7;
        }

        self.corner_chars[index]
    }

    pub fn horizontal_line(&self) -> String {
        let dash = self.corner_chars[10];
        format!("{}{}{}", dash, dash, dash)
    }
}

// Our two character sets - plain ASCII and Unicode line drawing

pub static ASCII_CHARSET: MazeCharset = MazeCharset::new(&ASCII_CORNER_CHARS);
pub static UNICODE_CHARSET: MazeCharset = MazeCharset::new(&UNICODE_CORNER_CHARS);

// Unicode charset tables

const UNICODE_CORNER_CHARS: [char; 16] = [
    ' ',
    '╹',
    '╺',
    '┗',
    '╻',
    '┃',
    '┏',
    '┣',
    '╸',
    '┛',
    '━',
    '┻',
    '┓',
    '┫',
    '┳',
    '╋'
];

// const UNICODE_SOLUTION_CHARS: &[&str] = &[
//     "   ",
//     "   ",
//     "   ",
//     " ╰┄",
//     "   ",
//     " ┆ ",
//     " ╭┄",
//     "   ",
//     "   ",
//     "┄╯ ",
//     "┄┄┄",
//     "   ",
//     "┄╮ ",
//     "   ",
//     "   ",
//     "   "
// ];

// ASCII charset tables

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

// const ASCII_SOLUTION_CHARS: &[&str] = &[
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

pub struct MazePrinter<'a> {
    char_set: &'a MazeCharset<'a>
}

impl<'a> MazePrinter<'a> {
    pub fn new(char_set: &'a MazeCharset) -> Self {
        Self {
            char_set
        }
    }

    pub fn print(&self, maze: &Maze) {
        for row in 0..maze.rows as i32 {
            self.print_row_separator(maze, row);
            self.print_row(maze, row);
        }
        self.print_maze_bottom(maze);
    }

    fn print_row_separator(&self, maze: &Maze, row: i32) {
        for col in 0..maze.columns as i32 {
            let pos = CellPos { row, col };
            print!("{}", self.char_set.corner_char(maze, pos));
            if maze.can_go(pos, Direction::Up) {
                print!("   ");
            } else {
                print!("{}", self.char_set.horizontal_line());
            }
        }

        print!("{}", self.char_set.row_separator_end_char(maze, row));
        println!();
    }

    fn print_row(&self, maze: &Maze, row: i32) {
        for col in 0..maze.columns as i32 {
            let pos = CellPos { row, col };
            if maze.can_go(pos, Direction::Left) {
                print!(" ");
            } else {
                print!("{}", self.char_set.corner_char_at(5));
            }
            print!("{}", self.cell_contents(maze, pos));
        }

        print!("{}", self.char_set.corner_char_at(5));
        println!();
    }

    fn cell_contents(&self, _: &Maze, _: CellPos) -> String {
        String::from("   ")
    }

    fn print_maze_bottom(&self, maze: &Maze) {
        for col in 0..maze.columns as i32 {
            self.print_bottom_separator(maze, col);
        }
        print!("{}", self.char_set.bottom_right_char(maze));
        println!();
    }

    fn print_bottom_separator(&self, maze: &Maze, col: i32) {
        print!("{}", self.char_set.bottom_separator_char(maze, col));
        print!("{}", self.char_set.horizontal_line());
    }
}
