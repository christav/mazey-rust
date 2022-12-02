use std::env;
mod mazey;
use mazey::maze::Maze;
use mazey::printer::{ MazePrinter, MazeCharset, ASCII_CHARSET, UNICODE_CHARSET };


fn main() {
    let char_set: &MazeCharset;
    let args: Vec<String> = env::args().collect();

    char_set = if args.len() < 2 || args[1] == "-u" {
        &UNICODE_CHARSET
    } else {
        &ASCII_CHARSET
    };

    let maze = Maze::new(30, 20);
    let printer = MazePrinter::new(char_set);

    printer.print(&maze);
}
