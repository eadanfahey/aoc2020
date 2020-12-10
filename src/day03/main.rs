use clap::{App, Arg};
use std::{fs};

fn parse_row(s: &str) -> Vec<bool> {
    s.chars().map(|c| if c == '#' {true} else {false}).collect()
}

#[derive(Debug)]
struct Grid {
    pos: (usize, usize),
    rows: Vec<Vec<bool>>,
    nrows: usize,
    ncols: usize,
}

impl Grid {
    fn new(rows: Vec<Vec<bool>>) -> Result<Grid, &'static str> {
        // Make sure all rows have the same number of columns
        let nrows = rows.len();
        if nrows == 0 {
            return Err("At least one row must be supplied")
        }
        let ncols = rows[0].len();
        for i in 2..nrows {
            if rows[i].len() != ncols {
                return Err("All rows must be the same length")
            }
        }
        Ok(Grid{pos: (1, 1), rows, nrows, ncols})
    }

    fn step(&mut self, x: usize, y: usize) -> Result<(), &'static str> {
        let new_y = self.pos.1 + y;
        if new_y > self.nrows {
            return Err("step outside of grid")
        }
        self.pos = (self.pos.0 + x, new_y) ;
        Ok(())
    }

    fn get(&self) -> bool {
        assert!(self.pos.1 <= self.nrows);
        let x = (self.pos.0 - 1) % self.ncols;
        let y = self.pos.1 - 1;
        self.rows[y][x]
    }

    fn reset(&mut self) {
        self.pos = (1, 1)
    }
}

fn traverse_grid(grid: &mut Grid, step: (usize, usize)) -> i64 {
    let mut ntrees = 0;
    loop {
        if grid.get() {
            ntrees += 1
        }
        if grid.pos.1 + step.1 > grid.nrows {
            break
        }
        grid.step(step.0, step.1).unwrap();
    }
    ntrees
}

fn part1(grid: &mut Grid) -> i64 {
    traverse_grid(grid, (3, 1))
}

fn part2(grid: &mut Grid) -> i64 {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut ntrees = Vec::with_capacity(slopes.len());
    for slope in slopes.into_iter() {
        grid.reset();
        ntrees.push(traverse_grid(grid, slope));
    }
    ntrees.iter().fold(1, |acc, n| acc * n)
}


fn main() {
    let matches = App::new("AOC2020 Day3")
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

    let rows: Vec<_> = contents.split('\n')
        .filter(|l| !l.is_empty())
        .map(|s| parse_row(&s))
        .collect();

    let mut grid = Grid::new(rows).unwrap();

    let part = matches.value_of("part").unwrap();
    if part == "1" {
        println!("Number of trees = {}", part1(&mut grid))
    } else {
        println!("Result = {}", part2(&mut grid))
    }
}
