use crate::maze::{Maze, Wall, WallDirection};

pub fn print(maze: &Maze) {
    let column_width = 4;
    println!("+{}+", "-".repeat(maze.width * (column_width + 1) - 1));
    let with_right = format!("{}|", " ".repeat(column_width));
    let without_right = &(" ".repeat(column_width + 1));
    let with_down = format!("{}+", "-".repeat(column_width));
    let without_down = format!("{}+", " ".repeat(column_width));
    for row in 0..maze.height {
        let mut line1 = String::from("|");
        let mut line2 = String::from("+");
        for column in 0..maze.width {
            if column == maze.width - 1
                || maze.walls.contains(&Wall {
                    row,
                    column,
                    direction: WallDirection::Right,
                })
            {
                line1 += &with_right;
            } else {
                line1 += &without_right;
            }
            if maze.walls.contains(&Wall {
                row,
                column,
                direction: WallDirection::Down,
            }) {
                line2 += &with_down;
            } else {
                line2 += &without_down;
            }
        }
        println!("{}", line1);
        if row != maze.height - 1 {
            println!("{}", line2);
        }
    }
    println!("+{}+", "-".repeat(maze.width * (column_width + 1) - 1));
}
