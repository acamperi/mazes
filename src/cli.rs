use crate::generator;
use crate::generator::MazeGenerationAlgorithm;
use crate::print;
use clap::{value_t, App, Arg};

pub fn parse_args_and_run() {
    let matches = App::new("Maze Generator")
        .version("0.1.0")
        .author("Ariel Camperi <ariel.camperi@gmail.com>")
        .arg(
            Arg::with_name("algorithm")
                .short("a")
                .long("algorithm")
                .takes_value(true)
                .required(true)
                .possible_values(&MazeGenerationAlgorithm::variants())
                .case_insensitive(true)
                .help("Which maze generation algorithm to use."),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .takes_value(true)
                .required(true)
                .help("The height of the maze to generate."),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .takes_value(true)
                .required(true)
                .help("The width of the maze to generate."),
        )
        .get_matches();

    let algorithm = value_t!(matches, "algorithm", MazeGenerationAlgorithm).unwrap();
    let height = value_t!(matches, "height", usize).unwrap();
    let width = value_t!(matches, "width", usize).unwrap();
    let maze = generator::generate(algorithm, height, width);
    print::print(&maze);
}
