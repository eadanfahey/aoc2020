use clap::{App, Arg};
use std::{fs};
use std::collections::{HashSet};


fn part1(groups: Vec<Vec<&str>>) -> usize {
    groups.iter()
        .map(|g| {
            g.iter()
                .map(|s| s.chars())
                .flatten()
                .collect::<HashSet<_>>()
                .len()
        })
    .fold(0, |acc, i| acc + i)
}

fn part2(groups: Vec<Vec<&str>>) -> usize {
    groups.iter()
        .map(|g| {
            let mut sets = g.iter().map(|s| s.chars().collect::<HashSet<_>>());
            let first = sets.next().unwrap();
            sets.fold(first, |acc, set| {
                acc.intersection(&set).cloned().collect::<HashSet<_>>()
            }).len()
        })
        .fold(0, |acc, i| acc + i)
}

fn main() {
    let matches = App::new("AOC2020 Day6")
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
    let contents = fs::read_to_string(path).expect("reading input");

    let mut groups = Vec::new();
    let mut g = Vec::new();
    for line in contents.split('\n') {
        if line.is_empty() {
            groups.push(g);
            g = Vec::new();
            continue;
        }
        g.push(line);
    }
    if !g.is_empty() {
        groups.push(g);
    }

    let part = matches.value_of("part").unwrap();
    if part == "1" {
        println!("{}", part1(groups));
    } else {
        println!("{}", part2(groups));
    }
}
