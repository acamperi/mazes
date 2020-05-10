use crate::maze::{Coordinates, Maze};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

pub fn generate(height: usize, width: usize) -> Maze {
    let mut maze = Maze::full(height, width);
    let mut rng = thread_rng();
    let mut stack: Vec<Coordinates> = Vec::new();
    let mut visited_cells: HashSet<Coordinates> = HashSet::new();
    let initial_cell = maze.random_cell(&mut rng);
    stack.push(initial_cell);
    while let Some(cell) = stack.pop() {
        let adjoining_cells: Vec<Coordinates> = maze
            .adjoining_cells(&cell)
            .into_iter()
            .filter(|c| !visited_cells.contains(c))
            .collect();
        if let Some(&next) = adjoining_cells.choose(&mut rng) {
            maze.walls.remove(&cell.wall_with(&next).unwrap());
            stack.push(cell);
            visited_cells.insert(next);
            stack.push(next);
        }
    }
    maze
}
