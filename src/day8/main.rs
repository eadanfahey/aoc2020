use clap::{App, Arg};
use std::{fs};
use std::collections::{HashSet};
use std::convert::{TryFrom};

#[derive(Debug, Clone)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

#[derive(Debug)]
struct VM {
    instructions: Vec<Instruction>,
    acc:          i64,
    pos:          usize,
}

impl VM {
    fn new(instructions: Vec<Instruction>) -> VM {
        VM{instructions, acc: 0, pos: 0}
    }

    // execute runs the next instruction unless doing so would cause the VM to be in
    // an invalid state in which case the function returns None.
    fn execute(&mut self) -> Option<()> {
        let instruction = self.instructions.get(self.pos).unwrap();
        match instruction {
            Instruction::Acc(n) => {
                if self.pos >= self.instructions.len() - 1 {
                    return None;
                }
                self.acc += n; 
                self.pos += 1;
            },
            Instruction::Jmp(n) => {
                let npos = usize::try_from(i64::try_from(self.pos).unwrap() + n).unwrap(); 
                if npos >= self.instructions.len() {
                    return None;
                }
                self.pos = npos;
            },
            Instruction::Nop(_) => {
                if self.pos >= self.instructions.len() - 1 {
                    return None;
                }
                self.pos += 1;
            }
        }
        Some(())
    }
}

fn parse_op(line: &str) -> Option<Instruction> {
    let sp: Vec<&str> = line.split(' ').collect();
    let op = sp.get(0)?;
    let n = sp.get(1).map(|i| i.parse::<i64>().ok())??;
    match *op {
        "acc" => Some(Instruction::Acc(n)),
        "jmp" => Some(Instruction::Jmp(n)),
        "nop" => Some(Instruction::Nop(n)),
        _ => None,
    }
}

fn parse_input(s: &str) -> Option<Vec<Instruction>> {
    s.split('\n')
        .filter(|l| !l.is_empty())
        .map(parse_op)
        .collect()
}

fn part1(instructions: Vec<Instruction>) -> i64 {
    let mut vm = VM::new(instructions);
    let mut visited: HashSet<usize> = HashSet::new();
    loop {
        if !visited.insert(vm.pos) {
            return vm.acc;
        }
        vm.execute().unwrap();
    }
}

fn part2(instructions: Vec<Instruction>) -> i64 {
    let swaps = instructions.iter()
        .enumerate()
        .filter_map(|(i, instruction)| {
            match instruction {
                Instruction::Acc(_) => None,
                Instruction::Jmp(n) => Some((i, Instruction::Nop(*n))),
                Instruction::Nop(n) => Some((i, Instruction::Jmp(*n))),
            }
        });
    for (i, instruction) in swaps {
        let mut new_instructions = instructions.clone();
        new_instructions[i] = instruction;
        let mut vm = VM::new(new_instructions);
        let mut visited: HashSet<usize> = HashSet::new();
        loop {
            if !visited.insert(vm.pos) {
                break;
            }
            if vm.execute().is_none() {
                break;
            }
        }
        if vm.pos == vm.instructions.len() - 1 {
            return vm.acc;
        }
    }
    0
}

fn main() {
    let matches = App::new("AOC2020 Day6")
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

    let instructions = parse_input(&input).expect("parsing input");

    let part = matches.value_of("part").unwrap();
    if part == "1" {
        println!("{}", part1(instructions));
    } else {
        println!("{}", part2(instructions));
    }
}
