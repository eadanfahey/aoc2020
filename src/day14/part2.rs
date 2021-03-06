use std::collections::HashMap;
use itertools::Itertools;

struct Memset {
    pos: usize,
    value: usize,    
}

#[derive(Clone)]
struct Mask {
    on: usize,
    floating: Vec<usize>,
}

struct Instruction {
    mask: Mask,
    memset: Memset,
}

impl Mask {
    fn from_str(s: &str) -> Mask {
        let mut floating = Vec::new();
        let mut on = 0;
        for (i, c) in s.chars().rev().enumerate() {
            match c {
                'X' => floating.push(i),
                '1' => on |= 1 << i, 
                '0' => {},
                _ => unreachable!()
            }
        }
        Mask{on, floating}
    }
}

fn parse_input(s: &str) -> Option<Vec<Instruction>> {
    let mut instructions = Vec::new();
    let mut cur_mask = Mask {on: 0, floating: vec![]};
    for line in s.split('\n').filter(|l| !l.is_empty()) {
        let mut sp = line.split(" = ");
        let ident = sp.next()?;
        if ident == "mask" {
            cur_mask = Mask::from_str(sp.next()?);
            continue
        }
        assert!(ident.starts_with("mem"));
        let pos = ident[4..(ident.len()-1)].parse::<usize>().ok()?;
        let value = sp.next()?.parse::<usize>().ok()?;
        instructions.push(Instruction{
            mask: cur_mask.clone(), 
            memset: Memset{pos, value},
        });
    }
    Some(instructions)
}

// set_bits returns an int which is bitwise 1 at the provided bit offsets and 0 otherwise.
fn set_bits(bits: &[usize]) -> usize {
    bits.iter().fold(0, |acc, k| acc | (1 << k))
}

pub fn part2(s: &str) -> usize {
    let instructions = parse_input(s).expect("parsing input");
    let mut memory = HashMap::new();
    for ins in instructions.iter() {
        // All combinations of the mask's floating bits
        let combinations = (0..ins.mask.floating.len()+1)
            .map(move |i| ins.mask.floating.iter().cloned().combinations(i))
            .flatten();

        // Calculate the base mask. Remember to turn off all floating bits.
        let m = (ins.memset.pos | ins.mask.on) & !set_bits(&ins.mask.floating);

        for pos in combinations.map(|bits| m | set_bits(&bits)) {
            memory.insert(pos, ins.memset.value);
        }
    }
    memory.values().sum()
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn set_bits_test() {
        assert_eq!(set_bits(&vec![5, 2, 0]), 0b100101);
    }

    #[test]
    fn mask_from_str_test() {
        let mask = Mask::from_str("000000000000000000000000000000X1001X");
        assert_eq!(mask.on, 0b10010);
        assert_eq!(mask.floating, vec![0, 5]);
    }
}