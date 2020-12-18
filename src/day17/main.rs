#[macro_use] extern crate itertools;
use clap::{App, Arg};
use std::{collections::{HashMap, HashSet}, fs};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Pos3 {
    x: i64,
    y: i64,
    z: i64,
}
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Pos4 {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Pos3 {
    fn neighbors(&self) -> impl Iterator<Item=Self> + '_ {
        iproduct!(-1..2, -1..2, -1..2)
            .filter_map(move |delta| {
                match delta {
                    (0, 0, 0) => None,
                    (dx, dy, dz) => Some(Pos3{x: self.x + dx, y: self.y + dy, z: self.z + dz})
                }
            })
    }
}

impl Pos4 {
    fn neighbors(&self) -> impl Iterator<Item=Self> + '_ {
        iproduct!(-1..2, -1..2, -1..2, -1..2)
            .filter_map(move |delta| {
                match delta {
                    (0, 0, 0, 0) => None,
                    (dx, dy, dz, dw) => Some(Pos4{x: self.x + dx, y: self.y + dy, z: self.z + dz, w: self.w + dw})
                }
            })
    }
}

struct Grid3 {
    active_cubes: HashSet<Pos3>
}

struct Grid4 {
    active_cubes: HashSet<Pos4>
}

impl Grid3 {
    fn update_grid(&mut self) {
        let mut inactive_cubes = HashMap::new();
        let mut deactivate_cubes = HashSet::new();

        for cube in self.active_cubes.iter() {
            let mut n_active_neighbors = 0;
            for pos in cube.neighbors() {
                if self.active_cubes.contains(&pos) {
                    n_active_neighbors += 1;
                } else {
                    let n = inactive_cubes.entry(pos).or_insert(0);
                    *n += 1;
                }
            }
            if !(n_active_neighbors == 2 || n_active_neighbors == 3) {
                deactivate_cubes.insert(cube.clone());
            }
        }

        for cube in inactive_cubes.into_iter().filter(|(_, v)| v == &3).map(|(k, _)| k) {
            self.active_cubes.insert(cube);
        }

        for cube in deactivate_cubes.iter() {
            self.active_cubes.remove(cube);
        }
    }

}

impl Grid4 {
    fn update_grid(&mut self) {
        let mut inactive_cubes = HashMap::new();
        let mut deactivate_cubes = HashSet::new();

        for cube in self.active_cubes.iter() {
            let mut n_active_neighbors = 0;
            for pos in cube.neighbors() {
                if self.active_cubes.contains(&pos) {
                    n_active_neighbors += 1;
                } else {
                    let n = inactive_cubes.entry(pos).or_insert(0);
                    *n += 1;
                }
            }
            if !(n_active_neighbors == 2 || n_active_neighbors == 3) {
                deactivate_cubes.insert(cube.clone());
            }
        }

        for cube in inactive_cubes.into_iter().filter(|(_, v)| v == &3).map(|(k, _)| k) {
            self.active_cubes.insert(cube);
        }

        for cube in deactivate_cubes.iter() {
            self.active_cubes.remove(cube);
        }
    }
}

fn parse_input3(s: &str) -> Grid3 {
    let mut active_cubes = HashSet::new();
    for (y, line) in s.split('\n').filter(|l| !l.is_empty()).enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active_cubes.insert(Pos3{x: x as i64, y: y as i64, z: 0});
            }
        }
    }
    
    Grid3{active_cubes}
}

fn parse_input4(s: &str) -> Grid4 {
    let mut active_cubes = HashSet::new();
    for (y, line) in s.split('\n').filter(|l| !l.is_empty()).enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active_cubes.insert(Pos4{x: x as i64, y: y as i64, z: 0, w: 0});
            }
        }
    }
    
    Grid4{active_cubes}
}


fn part1(mut grid: Grid3) -> usize {
    for _ in 0..6 {
        grid.update_grid();
    }
    grid.active_cubes.len()
}

fn part2(mut grid: Grid4) -> usize {
    for _ in 0..6 {
        grid.update_grid();
    }
    grid.active_cubes.len()
}

fn main() {
    let matches = App::new("AOC2020 Day17")
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
        let grid = parse_input3(&input);
        println!("{}", part1(grid));
    } else {
        let grid = parse_input4(&input);
        println!("{}", part2(grid));
    }
}
