use clap::{App, Arg};
use std::{fs, str::FromStr};

#[derive(Debug, Clone)]
enum Degrees {
    D90,
    D180,
    D270,
}

#[derive(Debug, Clone)]
enum Rotation {
    Left(Degrees),
    Right(Degrees),
}
#[derive(Clone, Debug)]
enum Heading {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone)]
enum Action {
    Move {heading: Heading, amount: i32},
    Rotate(Rotation),
    Forward(i32),
}

#[derive(Debug)]
struct Boat {
    x: i32,
    y: i32,
    // heading is one of 0 (North), 1 (East), 2 (South), 3 (West)
    heading: u32
}

impl Degrees {
    fn from_int(i: i32) -> Option<Self> {
        match i {
            90 => Some(Degrees::D90),
            180 => Some(Degrees::D180),
            270 => Some(Degrees::D270),
            _ => None,
        }
    }
}

impl FromStr for Action {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err("invalid action");
        }
        let letter = s.chars().next().unwrap();
        let amount = s[1..].parse::<i32>().map_err(|_| "invalid action")?;
        let action = match letter {
            'N' => Some(Action::Move {heading: Heading::North, amount}),
            'E' => Some(Action::Move {heading: Heading::East, amount}),
            'S' => Some(Action::Move {heading: Heading::South, amount}),
            'W' => Some(Action::Move {heading: Heading::West, amount}),
            'L' => Degrees::from_int(amount).map(|d| Action::Rotate(Rotation::Left(d))),
            'R' => Degrees::from_int(amount).map(|d| Action::Rotate(Rotation::Right(d))),
            'F' => Some(Action::Forward(amount)),
            _ => None
        };
        action.ok_or("invalid action")
    }
}
#[derive(Debug)]
struct Waypoint {
    x: i32,
    y: i32,
}

impl Waypoint {
    fn new() -> Self {
        Waypoint{x: 10, y: 1}
    }

    fn move_direction(&mut self, h: Heading, amount: i32) {
        match h {
            Heading::North => self.y += amount,
            Heading::East => self.x += amount,
            Heading::South => self.y -= amount,
            Heading::West => self.x -= amount,
        }
    }

    fn rotate(&mut self, r: Rotation) {
        let (new_x, new_y) = match r {
            Rotation::Right(Degrees::D90) | Rotation::Left(Degrees::D270) => (self.y, -self.x),
            Rotation::Right(Degrees::D180) | Rotation::Left(Degrees::D180)  => (-self.x, -self.y),
            Rotation::Right(Degrees::D270) | Rotation::Left(Degrees::D90) => (-self.y, self.x),
        };
        self.x = new_x;
        self.y = new_y;
    }
}

impl Boat {
    fn new() -> Self {
        Boat {x: 0, y: 0, heading: 1}
    }

    fn degree_index(d: Degrees) -> i32 {
        match d {
            Degrees::D90 => 1,
            Degrees::D180 => 2,
            Degrees::D270 => 3,
        }
    }

    fn rotate_index(r: Rotation) -> i32 {
        match r {
            Rotation::Right(d) => Self::degree_index(d),
            Rotation::Left(d) => -Self::degree_index(d)
        }
    }

    fn change_heading(&mut self, rotate: Rotation) {
        let r = Self::rotate_index(rotate);
        let h = (self.heading as i32 + r) % 4;
        self.heading = if h < 0 {(4 + h) as u32} else {h as u32};
    }

    fn move_direction(&mut self, h: Heading, amount: i32) {
        match h {
            Heading::North => self.y += amount,
            Heading::East => self.x += amount,
            Heading::South => self.y -= amount,
            Heading::West => self.x -= amount,
        }
    }

    fn move_forward(&mut self, amount: i32) {
        match self.heading {
            0 => self.y += amount,
            1 => self.x += amount,
            2 => self.y -= amount,
            3 => self.x -= amount,
            _ => unreachable!(),
        }
    }

    fn forward_to_waypoint(&mut self, waypoint: &Waypoint, amount: i32) {
        self.x += amount * waypoint.x;
        self.y += amount * waypoint.y;
    }
}

fn parse_input(s: String) -> Option<Vec<Action>> {
    s.split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| Action::from_str(l).ok())
        .collect()
}

fn part1(actions: Vec<Action>) -> i32 {
    let mut boat = Boat::new();
    for action in actions {
        match action {
            Action::Move {heading, amount} => boat.move_direction(heading, amount),
            Action::Rotate(r) => boat.change_heading(r),
            Action::Forward(amount) => boat.move_forward(amount),
        }
    }
    boat.x.abs() + boat.y.abs()
}

fn part2(actions: Vec<Action>) -> i32 {
    let mut boat = Boat::new();
    let mut waypoint = Waypoint::new();
    for action in actions {
        match action {
            Action::Move {heading, amount} => waypoint.move_direction(heading, amount),
            Action::Rotate(r) => waypoint.rotate(r),
            Action::Forward(amount) => boat.forward_to_waypoint(&waypoint, amount),
        }
    }
    boat.x.abs() + boat.y.abs()
}


fn main() {
    let matches = App::new("AOC2020 Day12")
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

    let actions = parse_input(input).expect("parsing input");

    let part = matches.value_of("part").unwrap();
    if part == "1" {
        println!("{}", part1(actions));
    } else {
        println!("{}", part2(actions));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACTIONS: [Action; 5] = [
            Action::Forward(10),
            Action::Move {heading: Heading::North, amount: 3},
            Action::Forward(7),
            Action::Rotate(Rotation::Right(Degrees::D90)),
            Action::Forward(11),
    ];

    #[test]
    fn part1_test() {
        assert_eq!(part1(ACTIONS.to_vec()), 25);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(ACTIONS.to_vec()), 286);
    }
}

