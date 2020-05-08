use crate::maze::{Maze, Wall, WallDirection};

pub fn print(maze: &Maze) {
    let column_width = 3;
    let horizontal_wall = "\u{2500}".repeat(column_width);
    let empty_column = " ".repeat(column_width);

    // top border
    let mut top_border = String::from("\u{250c}");
    for column in 0..maze.width - 1 {
        top_border += &horizontal_wall;
        if maze.walls.contains(&Wall {
            row: 0,
            column,
            direction: WallDirection::Right,
        }) {
            top_border += "\u{252c}";
        } else {
            top_border += "\u{2500}";
        }
    }
    top_border += &format!("{}{}", horizontal_wall, "\u{2510}");
    println!("{}", top_border);

    // main section
    for row in 0..maze.height {
        // initialize verticals for row
        let mut verticals = String::from("\u{2502}");

        // initialize horizontals for row
        let mut horizontals = String::new();
        if maze.walls.contains(&Wall {
            row,
            column: 0,
            direction: WallDirection::Down,
        }) {
            horizontals += "\u{251c}";
        } else {
            horizontals += "\u{2502}";
        }

        // handle main section
        for column in 0..maze.width {
            // verticals
            let has_up = column == maze.width - 1
                || maze.walls.contains(&Wall {
                    row,
                    column,
                    direction: WallDirection::Right,
                });
            if has_up {
                verticals += &format!("{}{}", empty_column, "\u{2502}");
            } else {
                verticals += &format!("{} ", empty_column);
            }

            // horizontals
            let has_left = maze.walls.contains(&Wall {
                row,
                column,
                direction: WallDirection::Down,
            });
            let has_right = maze.walls.contains(&Wall {
                row,
                column: column + 1,
                direction: WallDirection::Down,
            });
            let has_down = column == maze.width - 1
                || maze.walls.contains(&Wall {
                    row: row + 1,
                    column,
                    direction: WallDirection::Right,
                });
            if has_left {
                horizontals += &horizontal_wall;
            } else {
                horizontals += &empty_column;
            }
            match (has_up, has_left, has_right, has_down) {
                (true, false, false, false) => horizontals += " ", // TODO partial up dash
                (true, true, false, false) => horizontals += "\u{2518}",
                (true, true, true, false) => horizontals += "\u{2534}",
                (true, true, true, true) => horizontals += "\u{253c}",
                (true, true, false, true) => horizontals += "\u{2524}",
                (true, false, true, true) => horizontals += "\u{251c}",
                (true, false, true, false) => horizontals += "\u{2514}",
                (true, false, false, true) => horizontals += "\u{2502}",
                (false, true, false, false) => horizontals += " ", // TODO partial left dash
                (false, true, true, false) => horizontals += "\u{2500}",
                (false, true, true, true) => horizontals += "\u{252c}",
                (false, true, false, true) => horizontals += "\u{2510}",
                (false, false, true, false) => horizontals += " ", // TODO partial right dash
                (false, false, true, true) => horizontals += "\u{250c}",
                (false, false, false, true) => horizontals += " ", // TODO partial down dash
                (false, false, false, false) => horizontals += " ",
            }
        }

        // print main section rows
        println!("{}", verticals);
        if row != maze.height - 1 {
            println!("{}", horizontals);
        }
    }

    // bottom border
    let mut bottom_border = String::from("\u{2514}");
    for column in 0..maze.width - 1 {
        bottom_border += &horizontal_wall;
        if maze.walls.contains(&Wall {
            row: maze.height - 1,
            column,
            direction: WallDirection::Right,
        }) {
            bottom_border += "\u{2534}";
        } else {
            bottom_border += "\u{2500}";
        }
    }
    bottom_border += &format!("{}{}", horizontal_wall, "\u{2518}");
    println!("{}", bottom_border);
}
