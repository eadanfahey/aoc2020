use clap::{App, Arg};

const EARLIEST_TIME: usize = 1007153;
const BUS_IDS: &str = "29,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,433,x,x,x,x,x,x,x,x,x,x,x,x,13,17,x,x,x,x,19,x,x,x,23,x,x,x,x,x,x,x,977,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,41";

fn ceil_div(x: usize, y: usize) -> usize {
    x / y + (if x % y == 0 {0} else {1})
}

fn part1(bus_ids: Vec<Option<usize>>) -> usize {
    let mut min_waiting_time = std::usize::MAX;
    let mut bus_id = 0;
    for id in bus_ids.iter().filter_map(|&id| id) {
        let wt = ceil_div(EARLIEST_TIME, id) * id - EARLIEST_TIME;
        if wt < min_waiting_time {
            min_waiting_time = wt;
            bus_id = id;
        }
    }
    min_waiting_time * bus_id
}


fn main() {
    let matches = App::new("AOC2020 Day13")
        .arg(Arg::with_name("part")
             .long("part")
             .required(true)
             .takes_value(true))
        .get_matches();

    let bus_ids: Vec<Option<usize>> = BUS_IDS.split(',')
        .filter(|s| !s.is_empty())
        .map(|s| match s {
            "x" => None,
            t => Some(t.parse::<usize>().unwrap())
        })
        .collect();

    let part = matches.value_of("part").unwrap();
    if part == "1" {
        println!("{}", part1(bus_ids));
    } else {
        // println!("{}", part2(actions));
    }
}