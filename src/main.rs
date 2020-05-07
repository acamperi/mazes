use mazes::print::print;
use mazes::{kruskal, prim};

fn main() {
    let maze = prim::generate(20, 20);
    print(&maze);
}
