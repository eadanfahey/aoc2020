use clap::{App, Arg};
use std::{fs, process};

#[derive(Debug)]
enum Row {
    Front,
    Back
}

#[derive(Debug)]
enum Column {
    Left,
    Right
}

#[derive(Debug)]
struct Partition {
    rows: Vec<Row>,
    columns: Vec<Column>,
}

fn find_seat_row(p: &[Row]) -> usize {
    assert!(p.len() == 7);
    let res = p.iter().fold((0, 127), |(min, max), r| {
        let mid = min + ((max - min) / 2);
        match r {
            Row::Front => (min, mid),
            Row::Back => (mid+1, max),
        }
    });
    assert!(res.0 == res.1);
    res.0
}

fn find_seat_column(p: &[Column]) -> usize {
    assert!(p.len() == 3);
    let res = p.iter().fold((0, 7), |(min, max), r| {
        let mid = min + ((max - min) / 2);
        match r {
            Column::Left => (min, mid),
            Column::Right => (mid+1, max),
        }
    });
    assert!(res.0 == res.1);
    res.0
}

fn find_seat(p: &Partition) -> (usize, usize) {
    (find_seat_row(&p.rows), find_seat_column(&p.columns))
}

fn seat_id((row, column): (usize, usize)) -> usize {
    row * 8 + column
}

fn parse_partition(s: &str) -> Option<Partition> {
    if s.len() != 10 {
        return None
    }
    let r: Option<Vec<_>> = s[..7].chars().map(|c| {
        match c {
            'F' => Some(Row::Front),
            'B' => Some(Row::Back),
            _ => None
        }
    }).collect();
    let rows = r?;

    let c: Option<Vec<_>> = s[7..].chars().map(|c| {
        match c {
            'L' => Some(Column::Left),
            'R' => Some(Column::Right),
            _ => None
        }
    }).collect();
    let columns = c?;

    Some(Partition{rows, columns})
}

fn part1(partitions: &[Partition]) -> usize {
    partitions.iter()
        .map(find_seat)
        .map(seat_id)
        .max()
        .unwrap()
}

fn part2(partitions: &[Partition]) -> usize {
    let mut seat_ids: Vec<_> = partitions.iter()
        .map(find_seat)
        .map(seat_id)
        .collect();
    seat_ids.sort();
    for i in 1..seat_ids.len() {
        if seat_ids[i] - seat_ids[i-1] == 2 {
            return seat_ids[i-1] + 1;
        }
    }
    0
}

fn main() {
    let matches = App::new("AOC2020 Day5")
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

    let _partitions: Result<Vec<_>, _> = contents.split('\n')
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(i, l)| parse_partition(l).
             ok_or(format!("invalid partition at input row {}", i)))
        .collect();
    if _partitions.is_err() {
        eprintln!("Error parsing input:\n{}", _partitions.unwrap_err());
        process::exit(1);
    }
    let partitions = _partitions.unwrap();


    let part = matches.value_of("part").unwrap();
    if part == "1" {
        println!("{}", part1(&partitions));
    } else {
        println!("{}", part2(&partitions));
    }
}
