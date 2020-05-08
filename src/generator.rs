use crate::algorithms::{kruskal, prim, wilson};
use crate::maze::Maze;
use clap::arg_enum;

arg_enum! {
    pub enum MazeGenerationAlgorithm {
        Kruskal,
        Prim,
        Wilson,
    }
}

pub fn generate(algorithm: MazeGenerationAlgorithm, height: usize, width: usize) -> Maze {
    match algorithm {
        MazeGenerationAlgorithm::Kruskal => kruskal::generate(height, width),
        MazeGenerationAlgorithm::Prim => prim::generate(height, width),
        MazeGenerationAlgorithm::Wilson => wilson::generate(height, width),
    }
}
