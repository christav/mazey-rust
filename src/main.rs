use std::env;
mod mazey;
use mazey::maze::Maze;
use mazey::printer::{ MazeCharset, AsciiCharSet, UnicodeCharSet, MazePrinter };


fn main() {
    let char_set: &dyn MazeCharset;
    let args: Vec<String> = env::args().collect();

    char_set = if args.len() < 2 || args[1] == "-u" {
        &UnicodeCharSet {}
    } else {
        &AsciiCharSet {}
    };

    let maze = Maze::new(30, 20);
    let printer = MazePrinter::new(char_set);

    printer.print(&maze);
}
