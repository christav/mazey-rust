extern crate rand;

mod mazey;
use mazey::maze::Maze;
use mazey::printer::{ AsciiCharSet, MazePrinter };

fn main() {
    let maze = Maze::new(30, 20);
    let printer = MazePrinter::new(&AsciiCharSet {});

    printer.print(&maze);
}
