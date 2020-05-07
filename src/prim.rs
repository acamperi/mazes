use crate::maze::{Coordinates, Maze, Wall, WallDirection};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::collections::HashSet;

pub fn generate(height: usize, width: usize) -> Maze {
    let mut maze = Maze::full(height, width);
    let mut rng = thread_rng();
    let start_cell = Coordinates {
        row: rng.gen_range(0, height),
        column: rng.gen_range(0, width),
    };
    let mut visited_cells: HashSet<Coordinates> = HashSet::new();
    visited_cells.insert(start_cell);
    let mut wall_list: Vec<Wall> = vec![
        Wall {
            row: start_cell.row,
            column: start_cell.column,
            direction: WallDirection::Right,
        },
        Wall {
            row: start_cell.row,
            column: start_cell.column,
            direction: WallDirection::Down,
        },
    ];
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
            wall_list.append(&mut maze.adjoining_walls(cell));
        }
        let wall_index = wall_list.iter().position(|w| *w == wall).unwrap();
        wall_list.remove(wall_index);
    }
    maze
}
