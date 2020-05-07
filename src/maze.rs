use std::collections::HashSet;

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
    pub fn adjoining_cells(&self) -> ((usize, usize), (usize, usize)) {
        let current = (self.row, self.column);
        match &self.direction {
            WallDirection::Right => (current, (self.row, self.column + 1)),
            WallDirection::Down => (current, (self.row + 1, self.column)),
        }
    }
}

pub struct Maze {
    pub height: usize,
    pub width: usize,
    pub walls: HashSet<Wall>,
}

impl Maze {
    pub fn new(height: usize, width: usize) -> Self {
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
}
