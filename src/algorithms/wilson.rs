use crate::maze::{Coordinates, Maze, Wall};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::collections::HashSet;

pub fn generate(height: usize, width: usize) -> Maze {
    let mut maze = Maze::empty(height, width);
    let mut rng = thread_rng();
    let mut visited_cells: HashSet<Coordinates> = HashSet::new();
    let initial_cell = maze.random_cell(&mut rng);
    for wall in maze.adjoining_walls(&initial_cell, None) {
        maze.walls.insert(wall);
    }
    visited_cells.insert(initial_cell);
    while visited_cells.len() < height * width {
        let start_cell = choose_start_cell(&maze, &visited_cells, &mut rng);
        let new_path = joining_path(start_cell, &visited_cells, &maze, &mut rng);
        let new_walls = enclosing_walls(&new_path, &maze);
        let removed_wall = new_path
            .joining_cell
            .wall_with(new_path.cells.last().unwrap())
            .unwrap();
        for cell in new_path.cells {
            visited_cells.insert(cell);
        }
        for wall in new_walls {
            maze.walls.insert(wall);
        }
        maze.walls.remove(&removed_wall);
    }
    maze
}

fn choose_start_cell<R>(
    maze: &Maze,
    visited_cells: &HashSet<Coordinates>,
    rng: &mut R,
) -> Coordinates
where
    R: Rng,
{
    let mut start_cell = maze.random_cell(rng);
    while visited_cells.contains(&start_cell) {
        start_cell = maze.random_cell(rng);
    }
    start_cell
}

struct JoiningPath {
    cells: Vec<Coordinates>,
    joining_cell: Coordinates,
}

fn joining_path<R>(
    start_cell: Coordinates,
    visited_cells: &HashSet<Coordinates>,
    maze: &Maze,
    rng: &mut R,
) -> JoiningPath
where
    R: Rng,
{
    let mut cells = vec![start_cell];
    let mut last_cell_index: usize = 0;
    loop {
        let next = *maze
            .adjoining_cells(&cells[last_cell_index])
            .choose(rng)
            .unwrap();
        if visited_cells.contains(&next) {
            return JoiningPath {
                cells,
                joining_cell: next,
            };
        } else if let Some(index) = cells.iter().position(|cell| *cell == next) {
            cells.drain(index + 1..);
            last_cell_index = index;
        } else {
            cells.push(next);
            last_cell_index += 1;
        }
    }
}

fn enclosing_walls(joining_path: &JoiningPath, maze: &Maze) -> Vec<Wall> {
    let mut walls: Vec<Wall> = Vec::new();
    let path_length = joining_path.cells.len();
    for (i, cell) in joining_path.cells.iter().enumerate() {
        let mut connected_cells: HashSet<Coordinates> = HashSet::new();
        if i > 0 {
            connected_cells.insert(joining_path.cells[i - 1]);
        }
        if i < path_length - 1 {
            connected_cells.insert(joining_path.cells[i + 1]);
        }
        if i == path_length - 1 {
            connected_cells.insert(joining_path.joining_cell);
        }
        walls.append(&mut maze.adjoining_walls(cell, Some(&connected_cells)));
    }
    walls
}
