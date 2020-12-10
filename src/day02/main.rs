use clap::{App, Arg};
use std::{fs, process};
use regex::Regex;


#[derive(Debug)]
struct PasswordPolicy {
    letter: char,
    min: usize,
    max: usize,
    password: String,
}

fn parse_password_policy(s: &str) -> Option<PasswordPolicy> {
    let re = Regex::new(r"(\d+)\-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let caps = re.captures(s)?;

    let g1 = caps.get(1).map(|g| g.as_str())?;
    let g2 = caps.get(2).map(|g| g.as_str())?;
    let g3 = caps.get(3).map(|g| g.as_str())?;
    let g4 = caps.get(4).map(|g| g.as_str())?;

    let min = g1.parse::<usize>().ok()?;
    let max = g2.parse::<usize>().ok()?;
    let letter = g3.chars().next()?;
    let password = g4.to_owned();

    Some(PasswordPolicy{letter, min, max, password})
}

fn xor(a: bool, b: bool) -> bool {
    return (a && !b) || (!a && b)
}

impl PasswordPolicy {
    fn is_valid_p1(&self) -> bool {
        let n = self.password.matches(self.letter).count();
        n >= self.min && n <= self.max
    }

    fn is_valid_p2(&self) -> bool {
        let c1 = self.password.chars().nth(self.min-1).unwrap(); 
        let c2 = self.password.chars().nth(self.max-1).unwrap();
        xor(self.letter == c1, self.letter == c2)
    }
}

fn part1(passwords: Vec<PasswordPolicy>) -> i64 {
    passwords.iter().map(|p| p.is_valid_p1())
        .fold(0, |acc, valid| acc + (if valid {1} else {0}))
}

fn part2(passwords: Vec<PasswordPolicy>) -> i64 {
    passwords.iter().map(|p| p.is_valid_p2())
        .fold(0, |acc, valid| acc + (if valid {1} else {0}))
}

fn main() {
    let matches = App::new("AOC2020 Day2")
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

    let (passwords, errors): (Vec<_>, Vec<_>) = contents.split('\n')
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(i, s)| parse_password_policy(&s)
             .ok_or(format!("parsing failed on line {}", i)))
        .partition(Result::is_ok);
    let passwords = passwords.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();

    if errors.len() > 0 {
        let err = errors.join("\n");
        eprintln!("Error parsing file:\n{}", err);
        process::exit(1);
    }


    let part = matches.value_of("part").unwrap();
    if part == "1" {
        println!("Num. valid = {}", part1(passwords))
    } else {
        println!("Num. valid = {}", part2(passwords))
    }
}
