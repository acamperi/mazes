use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinates {
    pub row: usize,
    pub column: usize,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum WallDirection {
    Right,
    Down,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Wall {
    pub row: usize,
    pub column: usize,
    pub direction: WallDirection,
}

impl Wall {
    pub fn adjoining_cells(&self) -> (Coordinates, Coordinates) {
        let current = Coordinates {
            row: self.row,
            column: self.column,
        };
        match &self.direction {
            WallDirection::Right => (
                current,
                Coordinates {
                    row: self.row,
                    column: self.column + 1,
                },
            ),
            WallDirection::Down => (
                current,
                Coordinates {
                    row: self.row + 1,
                    column: self.column,
                },
            ),
        }
    }
}

pub struct Maze {
    pub height: usize,
    pub width: usize,
    pub walls: HashSet<Wall>,
}

impl Maze {
    pub fn empty(height: usize, width: usize) -> Self {
        Maze {
            height,
            width,
            walls: HashSet::new(),
        }
    }

    pub fn full(height: usize, width: usize) -> Self {
        let mut walls = HashSet::new();
        for row in 0..height {
            for column in 0..width {
                if column != width - 1 {
                    walls.insert(Wall {
                        row,
                        column,
                        direction: WallDirection::Right,
                    });
                }
                if row != height - 1 {
                    walls.insert(Wall {
                        row,
                        column,
                        direction: WallDirection::Down,
                    });
                }
            }
        }
        Maze {
            width,
            height,
            walls,
        }
    }

    pub fn adjoining_walls(&self, cell: Coordinates) -> Vec<Wall> {
        let mut walls: Vec<Wall> = Vec::new();
        // up
        if cell.row > 0 {
            let up = Wall {
                row: cell.row - 1,
                column: cell.column,
                direction: WallDirection::Down,
            };
            if self.walls.contains(&up) {
                walls.push(up);
            }
        }
        // left
        if cell.column > 0 {
            let left = Wall {
                row: cell.row,
                column: cell.column - 1,
                direction: WallDirection::Right,
            };
            if self.walls.contains(&left) {
                walls.push(left);
            }
        }
        // right
        if cell.row < self.height - 2 {
            let right = Wall {
                row: cell.row,
                column: cell.column,
                direction: WallDirection::Right,
            };
            if self.walls.contains(&right) {
                walls.push(right);
            }
        }
        // down
        if cell.column < self.width - 2 {
            let down = Wall {
                row: cell.row,
                column: cell.column,
                direction: WallDirection::Down,
            };
            if self.walls.contains(&down) {
                walls.push(down);
            }
        }
        walls
    }
}
