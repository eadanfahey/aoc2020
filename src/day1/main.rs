use clap::{App, Arg};
use std::fs;

fn part1(numbers: Vec<i64>) -> i64 {
    let mut result = 0;
    for i in 1..numbers.len() {
        for j in i..numbers.len(){
            let n0 = numbers[i];
            let n1 = numbers[j];
            if n0 + n1 == 2020 {
                result = n0 * n1;
                break
            }
        }
        if result != 0 {
            break;
        }
    }
    return result;
}

fn part2(numbers: Vec<i64>) -> i64 {
    let mut result = 0;
    for i in 1..numbers.len() {
        for j in (i+1)..numbers.len(){
            for k in (j+1)..numbers.len() {
                let n0 = numbers[i];
                let n1 = numbers[j];
                let n2 = numbers[k];
                if n0 + n1 + n2 == 2020 {
                    result = n0 * n1 * n2;
                    break
                }
            }
            if result != 0 {
                break;
            }
        }
        if result != 0 {
            break;
        }
    }
    return result;
}


fn main() {
    let matches = App::new("AOC2020 Day1")
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
    let numbers: Vec<i64> = contents.split('\n')
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    let part = matches.value_of("part").unwrap();

    if part == "1" {
        let answer = part1(numbers);
        println!("{}", answer);
    } else {
        let answer = part2(numbers);
        println!("{}", answer);
    }
    
}
