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

trait Pos where
    Self: Eq + std::hash::Hash + Sized + Clone
{
    fn neighbors(&self) -> Box<dyn Iterator<Item=Self> + '_>;
}


impl Pos for Pos3 {
    fn neighbors(&self) ->  Box<dyn Iterator<Item=Self> + '_> {
        let it = iproduct!(-1..2, -1..2, -1..2)
            .filter_map(move |delta| {
                match delta {
                    (0, 0, 0) => None,
                    (dx, dy, dz) => Some(Pos3{x: self.x + dx, y: self.y + dy, z: self.z + dz})
                }
            });
        Box::new(it)
    }
}

impl Pos for Pos4 {
    fn neighbors(&self) -> Box<dyn Iterator<Item=Self> + '_> {
        let it = iproduct!(-1..2, -1..2, -1..2, -1..2)
            .filter_map(move |delta| {
                match delta {
                    (0, 0, 0, 0) => None,
                    (dx, dy, dz, dw) => Some(Pos4{x: self.x + dx, y: self.y + dy, z: self.z + dz, w: self.w + dw})
                }
            });
        Box::new(it)
    }
}

struct Grid<P: Pos> {
    active_cubes: HashSet<P>
}

impl<P: Pos> Grid<P> {
    fn updates(&self) -> HashMap<P, bool> {
        let mut inactive_cubes = HashMap::new();
        let mut res = HashMap::new();

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
                res.insert(cube.clone(), false);
            }
        }

        for cube in inactive_cubes.into_iter().filter(|(_, v)| v == &3).map(|(k, _)| k) {
            res.insert(cube.clone(), true);
        }

        res
    }

    fn update_grid(&mut self) {
        let updates = self.updates();
        for (cube, &on) in updates.iter() {
            if on {
                self.active_cubes.insert(cube.clone());
            } else {
                self.active_cubes.remove(cube);
            }
        }
    }

}

fn parse_input3(s: &str) -> Grid<Pos3> {
    let mut active_cubes = HashSet::new();
    for (y, line) in s.split('\n').filter(|l| !l.is_empty()).enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active_cubes.insert(Pos3{x: x as i64, y: y as i64, z: 0});
            }
        }
    }
    
    Grid{active_cubes}
}

fn parse_input4(s: &str) -> Grid<Pos4> {
    let mut active_cubes = HashSet::new();
    for (y, line) in s.split('\n').filter(|l| !l.is_empty()).enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active_cubes.insert(Pos4{x: x as i64, y: y as i64, z: 0, w: 0});
            }
        }
    }
    
    Grid{active_cubes}
}

fn part1(mut grid: Grid<Pos3>) -> usize {
    for _ in 0..6 {
        grid.update_grid();
    }
    grid.active_cubes.len()
}

fn part2(mut grid: Grid<Pos4>) -> usize {
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
