use crate::maze::{Maze, Wall};
use partitions::PartitionVec;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn generate(height: usize, width: usize) -> Maze {
    let mut maze = Maze::new(height, width);
    let mut sets: PartitionVec<(usize, usize)> = PartitionVec::new();
    for row in 0..height {
        for column in 0..width {
            sets.push((row, column));
        }
    }
    let mut random_walls: Vec<Wall> = maze.walls.iter().cloned().collect();
    random_walls.shuffle(&mut thread_rng());
    for wall in random_walls {
        let (first_cell, second_cell) = wall.adjoining_cells();
        let first_index = first_cell.0 * height + first_cell.1;
        let second_index = second_cell.0 * height + second_cell.1;
        if !sets.same_set(first_index, second_index) {
            maze.walls.remove(&wall);
            sets.union(first_index, second_index);
        }
    }
    maze
}
