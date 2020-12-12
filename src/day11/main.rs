use clap::{App, Arg};
use std::fs;

static HEADINGS: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

#[derive(Debug, Copy, Clone, PartialEq)]
enum Status {
    Floor,
    Empty,
    Occupied,
}

#[derive(Debug)]
struct Grid {
    layout: Vec<Vec<Status>>
}

struct Update {
    pos: (usize, usize),
    status: Status,
}

fn parse_input(s: String) -> Option<Grid> {
    let layout = s.split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| 
            l.chars()
            .map(|c| {
                match c {
                    '.' => Some(Status::Floor),
                    'L' => Some(Status::Empty),
                    _ => None,
                }
            })
            .collect::<Option<Vec<_>>>()
        )
        .collect::<Option<Vec<_>>>()?;
    
    Some(Grid{layout})
}

impl Grid {
    fn adjacent(&self, i: usize, j: usize) -> Vec<Status> {
        HEADINGS.iter()
            .map(|(di, dj)| (i as i32 + di, j as i32 + dj))
            .filter(|(x, y)| x >= &0 && y >= &0)
            .filter_map(|(x, y)| self.layout.get(x as usize)?.get(y as usize))
            .cloned()
            .collect()
    }

    fn find_seat_in_heading(&self, (i, j): (usize, usize), (di, dj): (i32, i32)) -> Option<Status> {
        let mut i = i as i32;
        let mut j = j as i32;
        loop {
            i += di;
            j += dj;
            if i < 0 || j < 0 {
                return None
            }
            match self.layout.get(i as usize).and_then(|v| v.get(j as usize)) {
                None => return None,
                Some(Status::Floor) => {},
                Some(&s) => return Some(s),
            }
        }
    }

    fn seats_in_sight(&self, i: usize, j: usize) -> Vec<Status> {
        HEADINGS.iter()
            .filter_map(|&heading| self.find_seat_in_heading((i, j), heading))
            .collect()
    }

    fn count_occupied(&self) -> usize {
        self.layout.iter()
            .flatten()
            .filter(|&s| s == &Status::Occupied)
            .count()
    }
}

fn update_part1(grid: &Grid, (i, j): (usize, usize)) -> Option<Update> {
    let status = grid.layout[i][j];
    match status {
        Status::Floor => None,
        Status::Empty | Status::Occupied => {
            let occ_adjacent = grid.adjacent(i, j).into_iter()
                .filter(|&s| s == Status::Occupied)
                .count();
            if status == Status::Empty && occ_adjacent == 0 {
                return Some(Update{pos: (i, j), status: Status::Occupied});
            } else if status == Status::Occupied && occ_adjacent >= 4 {
                return Some(Update{pos: (i, j), status: Status::Empty});
            }
            None
        },
    }
}

fn update_part2(grid: &Grid, (i, j): (usize, usize)) -> Option<Update> {
    let status = grid.layout[i][j];
    match status {
        Status::Floor => None,
        Status::Empty | Status::Occupied => {
            let occ_sight = grid.seats_in_sight(i, j).into_iter()
                .filter(|&s| s == Status::Occupied)
                .count();
            if status == Status::Empty && occ_sight == 0 {
                return Some(Update{pos: (i, j), status: Status::Occupied});
            } else if status == Status::Occupied && occ_sight >= 5 {
                return Some(Update{pos: (i, j), status: Status::Empty});
            }
            None
        },
    }
}

fn next_step<F>(grid: &Grid, update: F) -> Vec<Update> 
    where F: Fn(&Grid, (usize, usize)) -> Option<Update>
{
    (0..grid.layout.len())
        .map(move |i| (0..grid.layout[i].len()).map(move |j| (i, j)))
        .flatten()
        .filter_map(|pos| update(grid, pos))
        .collect()
}

fn simulate<F>(grid: &mut Grid, update: F)
    where F: Fn(&Grid, (usize, usize)) -> Option<Update> + Copy
{
    loop {
        let updates = next_step(&grid, update);
        if updates.len() == 0 {
            break;
        }
        for u in updates.iter() {
            let (i, j) = u.pos;
            grid.layout[i][j] = u.status;
        }
    }
}

fn part1(mut grid: Grid) -> usize{
    simulate(&mut grid, update_part1);
    grid.count_occupied()
}

fn part2(mut grid: Grid) -> usize{
    simulate(&mut grid, update_part2);
    grid.count_occupied()
}

fn main() {
    let matches = App::new("AOC2020 Day11")
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

    let grid = parse_input(input).expect("parsing input");

    let part = matches.value_of("part").unwrap();
    if part == "1" {
        println!("{}", part1(grid));
    } else {
        println!("{}", part2(grid));
    }
}
