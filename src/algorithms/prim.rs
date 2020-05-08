use crate::maze::{Coordinates, Maze, Wall};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

pub fn generate(height: usize, width: usize) -> Maze {
    let mut maze = Maze::full(height, width);
    let mut rng = thread_rng();
    let start_cell = maze.random_cell(&mut rng);
    let mut visited_cells: HashSet<Coordinates> = HashSet::new();
    visited_cells.insert(start_cell);
    let mut wall_list: Vec<Wall> = maze.adjoining_walls(&start_cell, None);
    while let Some(wall) = wall_list.choose(&mut rng).cloned() {
        let adjoining_cells = wall.adjoining_cells();
        let unvisited_cell = if !visited_cells.contains(&adjoining_cells.0) {
            Some(adjoining_cells.0)
        } else if !visited_cells.contains(&adjoining_cells.1) {
            Some(adjoining_cells.1)
        } else {
            None
        };
        if let Some(cell) = unvisited_cell {
            maze.walls.remove(&wall);
            visited_cells.insert(cell);
            wall_list.append(&mut maze.adjoining_walls(&cell, None));
        }
        let wall_index = wall_list.iter().position(|w| *w == wall).unwrap();
        wall_list.remove(wall_index);
    }
    maze
}
