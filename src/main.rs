use std::env;
mod mazey;
use mazey::maze::Maze;
use mazey::printer::{ MazePrinter, ASCII_CHARSET, UNICODE_CHARSET };
use mazey::maker::make_maze;

fn main() {
    let char_set;
    let args: Vec<String> = env::args().collect();

    char_set = if args.len() < 2 || args[1] == "-u" {
        &UNICODE_CHARSET
    } else {
        &ASCII_CHARSET
    };

    let mut maze = Maze::new(30, 20);
    let printer = MazePrinter::new(char_set);

    make_maze(&mut maze);
    printer.print(&maze);
}
