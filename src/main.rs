extern crate rand;

mod mazey;
use mazey::maze::Maze;
use mazey::printer::print_maze;
fn main() {
    let maze = Maze::new(30, 20);
    print_maze(&maze);
}
