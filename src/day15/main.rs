use std::collections::HashMap;
use clap::{App, Arg};

const INPUT: [usize; 6] = [12, 1, 16, 3, 11, 0];

fn run(starting: &[usize], n: usize) -> usize {
    let mut last_spoken = HashMap::<usize, (usize, Option<usize>)>::new();
    for (i, n) in starting.iter().enumerate() {
        last_spoken.insert(*n, (i, None));
    }
    let mut last = starting.last().unwrap().clone();
    for i in starting.len()..n {
        match last_spoken.get(&last).unwrap() {
            (_, None) => {
                last_spoken.entry(0)
                    .and_modify(|v| {*v = (i, Some(v.0))})
                    .or_insert((i, None));
                last = 0;
            },
            (n, Some(m)) => {
                let x = (n - m) as usize;
                last_spoken.entry(x)
                    .and_modify(|v| {*v = (i, Some(v.0))})
                    .or_insert((i, None));
                last = x;
            }
        }
    }
    last
}

fn part1() -> usize {
    run(&INPUT, 2020)
}

fn part2() -> usize {
    run(&INPUT, 30000000)
}


fn main() {
    let matches = App::new("AOC2020 Day15")
        .arg(Arg::with_name("part")
             .long("part")
             .required(true)
             .takes_value(true))
        .get_matches();


    let part = matches.value_of("part").unwrap();
    if part == "1" {
        println!("{}", part1());
    } else {
        println!("{}", part2());
    }
}