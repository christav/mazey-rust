extern crate rand;

mod mazey;
use mazey::maze::Maze;
use mazey::printer::{ AsciiCharSet, UnicodeCharSet, MazePrinter };

fn main() {
    let maze = Maze::new(30, 20);
    let printer1 = MazePrinter::new(&AsciiCharSet {});
    let printer2 = MazePrinter::new(&UnicodeCharSet {});

    printer1.print(&maze);

    println!();

    printer2.print(&maze);
}
