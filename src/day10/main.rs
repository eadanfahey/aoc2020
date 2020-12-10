use clap::{App, Arg};
use std::{collections::{HashMap, HashSet}, fs, iter};

fn parse_input(s: String) -> Option<Vec<usize>> {
    s.split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<usize>().ok())
        .collect()
}

fn part1(mut adapters: Vec<usize>) -> usize {
    adapters.sort_unstable();    
    let builtin = adapters.last().unwrap() + 3;

    let iter1 = iter::once(&0).chain(adapters.iter());
    let iter2 = adapters.iter().chain(iter::once(&builtin));

    let (ones, threes) = iter1.zip(iter2)
        .map(|(a, b)| b - a)
        .fold((0, 0), |(ones, threes), diff| {
            match diff {
                1 => (ones + 1, threes),
                3 => (ones, threes + 1),
                _ => (ones, threes),
            }
        });

    ones * threes
}

fn part2(mut adapters: Vec<usize>) -> usize {
    adapters.sort_unstable();
    let builtin = adapters.last().unwrap() + 3;

    let all_adapters: Vec<usize> = iter::once(0)
        .chain(adapters.into_iter())
        .chain(iter::once(builtin))
        .collect();
    
    // Construct a graph using a HashMap of parent -> children relationships. Each node
    // in the graph corresponds to an adapter, and an edge exists from node i to node
    // j if adapter j can connect to adapter i.
    let mut graph = HashMap::<usize, HashSet<usize>>::new();
    for (i, a) in all_adapters.iter().enumerate() {
        let children = all_adapters[(i+1)..].iter()
            .take(3)
            .filter(|&b| b - a <= 3)
            .cloned()
            .collect();
        graph.insert(*a, children);
    }

    // Count the total number of paths from the "outlet" to the "builtin" by walking
    // backwards over the graph.
    // counts stores the number of paths from each node to the "builtin" node
    // Note: we can iterate in reverse here because all_adapters is already a topological
    // ordering of the graph.
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for a in all_adapters.iter().rev() {
        let children = graph.get(a).unwrap();
        let count = if children.contains(&builtin) {
            1
        } else {
            children.iter().map(|c| counts.get(c).unwrap_or(&0)).sum()
        };
        counts.insert(*a, count);
    }

    counts[&0]
}

fn main() {
    let matches = App::new("AOC2020 Day9")
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

    let adapters = parse_input(input).expect("parsing input");

    let part = matches.value_of("part").unwrap();
    if part == "1" {
        println!("{}", part1(adapters));
    } else {
        println!("{}", part2(adapters));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_test() {
        let input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!(part1(input), 7 * 5);
    }
}
