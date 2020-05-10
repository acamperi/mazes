use crate::algorithms::{dfs, kruskal, prim, wilson};
use crate::maze::Maze;
use clap::arg_enum;

arg_enum! {
    pub enum MazeGenerationAlgorithm {
        Dfs,
        Kruskal,
        Prim,
        Wilson,
    }
}

pub fn generate(algorithm: MazeGenerationAlgorithm, height: usize, width: usize) -> Maze {
    match algorithm {
        MazeGenerationAlgorithm::Dfs => dfs::generate(height, width),
        MazeGenerationAlgorithm::Kruskal => kruskal::generate(height, width),
        MazeGenerationAlgorithm::Prim => prim::generate(height, width),
        MazeGenerationAlgorithm::Wilson => wilson::generate(height, width),
    }
}
