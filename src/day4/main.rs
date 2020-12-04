use clap::{App, Arg};
use std::{fs};
use std::collections::HashMap;

const EYE_COLOURS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

type Passport = HashMap<String, String>;

fn part1(passports: Vec<Passport>) -> i64 {
    passports.iter()
        .map(|p| p.len() == 8 || (p.len() == 7 && !p.contains_key("cid")))    
        .map(|b| if b {1} else {0})
        .fold(0, |acc, i| acc + i)
}

fn parse_byr(s: &str) -> Result<(), &'static str> {
    let byear = s.parse::<usize>().or(Err("could not parse birth year as int"))?;
    if byear < 1920 || byear > 2002 {
        return Err("birth year out of range")
    }
    Ok(())
}

fn parse_iyr(s: &str) -> Result<(), &'static str> {
    let iyear = s.parse::<usize>().or(Err("could not parse issue year as int"))?;
    if iyear < 2010 || iyear > 2020 {
        return Err("issue year out of range")
    }
    Ok(())
}

fn parse_eyr(s: &str) -> Result<(), &'static str> {
    let eyear = s.parse::<usize>().or(Err("could not parse expiration year as int"))?;
    if eyear < 2020 || eyear > 2030 {
        return Err("expiration year out of range")
    }
    Ok(())
}

fn parse_hgt(s: &str) -> Result<(), &'static str> {
    let l = s.len();
    let height = s[..l-2].parse::<usize>().or(Err("could not parse height as int"))?;
    if s.ends_with("cm") {
        if height < 150 || height > 193 {
            return Err("height cm out of range")
        }
    } else if s.ends_with("in") {
        if height < 59 || height > 76 {
            return Err("height inches out of range")
        }
    } else {
        return Err("invalid height units")
    }
    Ok(())
}

fn parse_hcl(s: &str) -> Result<(), &'static str> {
    if s.len() != 7 || !s.starts_with('#') {
        return Err("invalid hair color")
    }
    i64::from_str_radix(&s[1..], 16).or(Err("invalid hair color"))?;
    Ok(())
}

fn parse_ecl(s: &str) -> Result<(), &'static str> {
    if !EYE_COLOURS.iter().any(|&c| c == s) {
        return Err("invalid eye color")
    }
    Ok(())
}

fn parse_pid(s: &str) -> Result<(), &'static str> {
    if s.len() != 9 || !s.chars().all(|c| c.is_digit(10)) {
        return Err("invalid passport id")
    }
    Ok(())
}

fn passport_is_valid(p: &Passport) -> Result<(), &'static str> {
    p.get("byr").ok_or("byr missing").and_then(|s| parse_byr(s))?;
    p.get("iyr").ok_or("iyr missing").and_then(|s| parse_iyr(s))?;
    p.get("eyr").ok_or("eyr missing").and_then(|s| parse_eyr(s))?;
    p.get("hgt").ok_or("hgt missing").and_then(|s| parse_hgt(s))?;
    p.get("hcl").ok_or("hcl missing").and_then(|s| parse_hcl(s))?;
    p.get("ecl").ok_or("ecl missing").and_then(|s| parse_ecl(s))?;
    p.get("pid").ok_or("pid missing").and_then(|s| parse_pid(s))?;
    Ok(())
}

fn count(vals: impl Iterator<Item = bool>) -> i64 {
    vals.map(|b| if b {1} else {0}).fold(0, |acc, i| acc + i)
}

fn part2(passports: Vec<Passport>) -> i64 {
    let valid = passports.iter().map(|p| passport_is_valid(p).is_ok());
    count(valid)
}

fn main() {
    let matches = App::new("AOC2020 Day4")
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

    let mut passports: Vec<Passport> = Vec::new();
    let mut p = HashMap::new();
    for line in contents.split('\n') {
        if line.is_empty() {
            passports.push(p);
            p = HashMap::new();
            continue;
        }
        for kv in line.split(' ') {
            let sp: Vec<_> = kv.split(':').collect();
            p.insert(sp[0].to_owned(), sp[1].to_owned());
        }
    }
    // Don't forget the last one
    if p.len() > 0 {
        passports.push(p)
    }

    let part = matches.value_of("part").unwrap();
    if part == "1" {
        println!("Num. valid = {}", part1(passports));
    } else {
        println!("Num. valid = {}", part2(passports));
    }
}
