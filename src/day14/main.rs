mod part1;
mod part2;

use std::fs;
use clap::{App, Arg};

fn main() {
    let matches = App::new("AOC2020 Day14")
        .arg(Arg::with_name("input")
             .long("input")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("part")
             .long("part")
             .required(true)
             .takes_value(true))
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let path = fs::canonicalize(input_file).expect("file does not exist");
    let input = fs::read_to_string(path).expect("reading input");

    let part = matches.value_of("part").unwrap();
    if part == "1" {
        println!("{}", part1::part1(&input));
    } else {
        println!("{}", part2::part2(&input));
    }
}

