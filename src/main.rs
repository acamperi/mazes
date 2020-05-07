use mazes::kruskal;
use mazes::print::print;

fn main() {
    let maze = kruskal::generate(10, 10);
    print(&maze);
}
