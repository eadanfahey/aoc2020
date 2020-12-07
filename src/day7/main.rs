#[macro_use] extern crate lazy_static;
use clap::{App, Arg};
use std::{fs};
use std::collections::{HashMap};
use regex::Regex;

#[derive(Debug)]
struct Bag {
    colour: String,
    contents: HashMap<String, usize>
}

#[derive(Debug)]
struct Rules {
    bags: Vec<Bag>,
    lookup: HashMap<String, usize>
}

impl Rules {
    fn new() -> Rules {
        Rules{bags: Vec::new(), lookup: HashMap::new()}
    }

    fn get_bag(&self, colour: &str) -> Option<&Bag> {
        let i = self.lookup.get(colour)?;
        self.bags.get(i.to_owned())
    }

    fn insert_bag(&mut self, bag: Bag) {
        match self.lookup.get(&bag.colour) {
            None => {
                self.lookup.insert(bag.colour.clone(), self.bags.len());
                self.bags.push(bag);
            }
            Some(&i) => {
                self.bags[i] = bag;
            }
        }
    }
}

fn parse_bag(s: &str) -> Option<Bag> {
    lazy_static!{
        static ref RE1: Regex = Regex::new(r"([a-z ]+) bags contain (.+)").unwrap();
    }
    let caps1 = RE1.captures(s)?;

    let colour = caps1.get(1).map(|g| g.as_str().to_owned())?;
    let contents_s = caps1.get(2).map(|g| g.as_str())?;

    if contents_s.starts_with("no other bags") {
        return Some(Bag{colour: colour, contents: HashMap::new()})
    }

    lazy_static!{
        static ref RE2: Regex = Regex::new(r"(\d+) ([a-z ]+) bags?").unwrap();
    }
    let mut contents = HashMap::new();
    for cap in RE2.captures_iter(contents_s) {
        let n = cap.get(1).map(|g| g.as_str())?;
        let c = cap.get(2).map(|g| g.as_str())?;
        let nn = n.parse::<usize>().ok()?;
        contents.insert(c.to_owned(), nn);
    }

    Some(Bag{colour, contents})
}

fn parse_rules(input: &str) -> Option<Rules> {
    let mut rules = Rules::new();
    let bags = input.split('\n')
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(parse_bag)
        .collect::<Option<Vec<_>>>()?;
    for bag in bags.into_iter() {
        rules.insert_bag(bag);
    }
    Some(rules)
}

fn part1(rules: &Rules) -> usize {
    // Depth-first search to find paths leading to the shiny gold bag.
    // We maintain a cache so as not to re-traverse the same paths.
    // If cache[colour] == true, then there is at least one path starting at that
    // bag to the shiny gold bag.
    let mut cache: HashMap<String, bool> = HashMap::new();
    for bag in rules.bags.iter().filter(|b| b.colour != "shiny gold") {
        let mut stack = vec![&bag.colour];
        while !stack.is_empty() {
            let parent = rules.get_bag(stack.last().unwrap()).unwrap();
            let child_colour = parent.contents
                .keys()
                .filter(|k| !cache.contains_key(k.as_str()))
                .next();
            match child_colour {
                Some(child_colour) => {
                    let child = rules.get_bag(child_colour).unwrap();
                    if child.colour == "shiny gold" {
                        cache.insert(parent.colour.clone(), true);
                        stack.pop();
                    } else {
                        stack.push(child_colour);
                    }
                }
                None => {
                    // We've visited all of this bag's children (or it has no children).
                    // Check if any child leads to shiny gold
                    let c = parent.contents.keys().filter(|k| cache[k.as_str()]).count();
                    cache.insert(parent.colour.clone(), c > 0);
                    stack.pop();
                }
            }
        }
    }

    cache.values().map(|b| if *b {1} else {0}).sum()
}

fn part2(rules: &Rules) -> usize {
    // Breadth-first search starting at the shiny gold bag
    let mut stack = vec![(1, rules.get_bag("shiny gold").unwrap())];
    let mut count = 0;
    while !stack.is_empty() {
        let (n, bag) = stack.pop().unwrap();
        count += bag.contents.values().map(|v| v * n).sum::<usize>();
        for (k, v) in bag.contents.iter() {
            stack.push((v*n, rules.get_bag(k).unwrap()));
        }
    }
    count
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
    let rules = parse_rules(&contents).expect("could not parse input");

    let part = matches.value_of("part").unwrap();
    if part == "1" {
        println!("{}", part1(&rules));
    } else {
        println!("{}", part2(&rules));
    }
}
