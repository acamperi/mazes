use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinates {
    pub row: usize,
    pub column: usize,
}

impl Coordinates {
    pub fn wall_with(&self, other: &Coordinates) -> Option<Wall> {
        let row_diff = if self.row > other.row {
            self.row - other.row
        } else {
            other.row - self.row
        };
        let column_diff = if self.column > other.column {
            self.column - other.column
        } else {
            other.column - self.column
        };
        if !((row_diff == 1 && column_diff == 0) || (row_diff == 0 && column_diff == 1)) {
            None
        } else {
            let first: &Coordinates;
            let second: &Coordinates;
            if self.row <= other.row && self.column <= other.column {
                first = self;
                second = other;
            } else {
                first = other;
                second = self;
            };
            Some(Wall {
                row: first.row,
                column: first.column,
                direction: if first.row == second.row {
                    WallDirection::Right
                } else {
                    WallDirection::Down
                },
            })
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum WallDirection {
    Right,
    Down,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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
            height,
            width,
            walls,
        }
    }

    pub fn random_cell<R>(&self, rng: &mut R) -> Coordinates
    where
        R: rand::Rng,
    {
        Coordinates {
            row: rng.gen_range(0, self.height),
            column: rng.gen_range(0, self.width),
        }
    }

    pub fn adjoining_cells(&self, cell: &Coordinates) -> Vec<Coordinates> {
        let mut cells: Vec<Coordinates> = Vec::new();
        // up
        if cell.row > 0 {
            cells.push(Coordinates {
                row: cell.row - 1,
                column: cell.column,
            });
        }
        // left
        if cell.column > 0 {
            cells.push(Coordinates {
                row: cell.row,
                column: cell.column - 1,
            })
        }
        // right
        if cell.column < self.width - 1 {
            cells.push(Coordinates {
                row: cell.row,
                column: cell.column + 1,
            })
        }
        // down
        if cell.row < self.height - 1 {
            cells.push(Coordinates {
                row: cell.row + 1,
                column: cell.column,
            })
        }
        cells
    }

    pub fn adjoining_walls(
        &self,
        cell: &Coordinates,
        connected_cells: Option<&HashSet<Coordinates>>,
    ) -> Vec<Wall> {
        let mut walls: Vec<Wall> = Vec::new();
        // up
        if cell.row > 0 {
            let up = Wall {
                row: cell.row - 1,
                column: cell.column,
                direction: WallDirection::Down,
            };
            let connected_to_up = connected_cells.map_or(false, |hs| {
                hs.contains(&Coordinates {
                    row: cell.row - 1,
                    column: cell.column,
                })
            });
            if !connected_to_up {
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
            let connected_to_left = connected_cells.map_or(false, |hs| {
                hs.contains(&Coordinates {
                    row: cell.row,
                    column: cell.column - 1,
                })
            });
            if !connected_to_left {
                walls.push(left);
            }
        }
        // right
        if cell.column < self.width - 1 {
            let right = Wall {
                row: cell.row,
                column: cell.column,
                direction: WallDirection::Right,
            };
            let connected_to_right = connected_cells.map_or(false, |hs| {
                hs.contains(&Coordinates {
                    row: cell.row,
                    column: cell.column + 1,
                })
            });
            if !connected_to_right {
                walls.push(right);
            }
        }
        // down
        if cell.row < self.height - 1 {
            let down = Wall {
                row: cell.row,
                column: cell.column,
                direction: WallDirection::Down,
            };
            let connected_to_down = connected_cells.map_or(false, |hs| {
                hs.contains(&Coordinates {
                    row: cell.row + 1,
                    column: cell.column,
                })
            });
            if !connected_to_down {
                walls.push(down);
            }
        }
        walls
    }
}
