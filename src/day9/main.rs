use clap::{App, Arg};
use std::fs;


// Pairs implements an iterator over all distinct pairs of elements in a slice.
struct Pairs<'a, T> {
    items: &'a[T],
    i: usize,
    j: usize,
}

impl<'a, T> Pairs<'a, T> {
    fn new(items: &'a[T]) -> Pairs<'a, T> {
        Pairs{items, i: 0, j: 1}
    }
}

impl<'a, T> Iterator for Pairs<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.items.len();
        if n < 2 {
            return None;
        }
        if (self.i == n-2) && (self.j == n-1) {
            return None;
        }
        let new_next = (&self.items[self.i], &self.items[self.j]);
        if self.j == n-1 {
            self.i += 1;
            self.j = self.i + 1;
        } else {
            self.j += 1;
        }
        Some(new_next)
    }
}

fn parse_input(s: &str) -> Option<Vec<usize>> {
    s.split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<usize>().ok())
        .collect()
}

fn part1(numbers: &[usize]) -> usize {
    for (i, &n) in numbers.iter().enumerate().skip(25) {
        let prev = &numbers[i-25..i];
        let m = Pairs::new(prev)
            .map(|(a, b)| a + b)
            .find(|&x| n == x);
        if m.is_none() {
            return n;
        }
    }
    0
}

fn part2(numbers: &[usize], p1_ans: usize) -> usize {
    let len = numbers.len();
    for i in 0..(len-1) {
        let mut sum = numbers[i];
        for j in (i+1)..len {
            sum += numbers[j];
            if sum == p1_ans {
                let slice = &numbers[i..j];
                return slice.iter().min().unwrap() + slice.iter().max().unwrap();
            }
        }
    }
    0
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

    let numbers = parse_input(&input).expect("parsing input");

    let part = matches.value_of("part").unwrap();
    let part1_ans = part1(&numbers);
    if part == "1" {
        println!("{}", part1_ans);
    } else {
        println!("{}", part2(&numbers, part1_ans));
    }
}
