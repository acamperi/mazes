use crate::maze::{Coordinates, Maze, Wall};
use partitions::PartitionVec;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn generate(height: usize, width: usize) -> Maze {
    let mut maze = Maze::full(height, width);
    let mut sets: PartitionVec<Coordinates> = PartitionVec::new();
    for row in 0..height {
        for column in 0..width {
            sets.push(Coordinates { row, column });
        }
    }
    let mut random_walls: Vec<Wall> = maze.walls.iter().cloned().collect();
    random_walls.shuffle(&mut thread_rng());
    for wall in random_walls {
        let (first_cell, second_cell) = wall.adjoining_cells();
        let first_index = first_cell.row * width + first_cell.column;
        let second_index = second_cell.row * width + second_cell.column;
        if !sets.same_set(first_index, second_index) {
            maze.walls.remove(&wall);
            sets.union(first_index, second_index);
        }
    }
    maze
}
