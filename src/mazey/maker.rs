extern crate rand;

use rand::thread_rng;
use rand::Rng;
use rand::seq::SliceRandom;

use super::maze::{Maze, CellPos };
use super::direction::{Direction, ALL_DIRECTIONS};

pub fn make_maze(maze: &mut Maze) {
    let mut random = thread_rng();

    reset_maze(maze);
    let start_row = random.gen_range(0..maze.rows) as i32;
    let start_col = random.gen_range(0..maze.columns) as i32;

    open_doors(maze, &mut random, CellPos { row: start_row, col: start_col });

    let mut row = random.gen_range(0..maze.rows) as i32;
    maze.open_door(CellPos { row, col: 0}, &Direction::Left);

    row = random.gen_range(0..maze.rows) as i32;
    maze.open_door(CellPos { row, col: (maze.columns - 1) as i32 }, &Direction::Right);
}

fn reset_maze(maze: &mut Maze) {
    for r in 0..maze.rows {
        for c in 0..maze.columns {
            let pos = CellPos { row: r as i32, col: c as i32 };
            for d in ALL_DIRECTIONS {
                pos.close_door(maze, d);
            }
            pos.set_mark(maze, 0);
        }
    }
}

fn open_doors<R: Rng + ?Sized>(maze: &mut Maze, rng: &mut R, pos: CellPos) -> bool {
    if pos.get_mark(maze) == 0 {
        pos.set_mark(maze, 1);
        let mut directions = ALL_DIRECTIONS.to_vec();
        directions.shuffle(rng);

        for dir in directions {
            let new_pos = pos.go(dir);
            if new_pos.is_in_maze(maze) {
                if open_doors(maze, rng, new_pos) {
                    pos.open_door(maze, dir);
                }
            }
        }
        true
    } else {
        false
    }
}
