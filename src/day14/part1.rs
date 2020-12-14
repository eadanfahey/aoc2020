use std::collections::HashMap;

struct Memset {
    pos: usize,
    value: usize,
}

#[derive(Clone)]
struct Mask {
    on: usize,
    off: usize,
}

struct Instruction {
    mask: Mask,
    memset: Memset,
}

impl Mask {
    fn from_str(s: &str) -> Mask {
        let (on, off) = s.chars()
            .rev()
            .enumerate()
            .fold((0, 0), |(on, off), (i, c)| {
                match c {
                    'X' => (on, off),
                    '1' => (on | (1 << i), off),
                    '0' => (on, off | (1 << i)),
                    _ => unreachable!(),
                }
            });
        Mask{on, off}
    }

    fn apply(&self, i: usize) -> usize {
        (i | self.on) & !self.off
    }
}


fn parse_input(s: &str) -> Option<Vec<Instruction>> {
    let mut instructions = Vec::new();
    let mut cur_mask = Mask {on: 0, off: 0};
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

fn run(instructions: Vec<Instruction>) -> usize{
    let memory: HashMap<usize, usize> = instructions.iter()
        .map(|i| (i.memset.pos, i.mask.apply(i.memset.value)))
        .collect();
    memory.values().sum()
}

pub fn part1(input: &str) -> usize {
    let instructions = parse_input(input).expect("parsing input");
    run(instructions)
}


#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn mask_from_str_test() {
        let mask = Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(mask.off, !0b10);
        assert_eq!(mask.on, 0b1000000);
    }

    #[test]
    fn mask_apply_test() {
        let mask = Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(mask.apply(11), 73);
        assert_eq!(mask.apply(101), 101);
        assert_eq!(mask.apply(0), 64);
    }

    #[test]
    fn part1_test() {
        let mask = Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        let instructions = vec![
            Instruction{mask: mask.clone(), memset: Memset{pos: 8, value: 11}},
            Instruction{mask: mask.clone(), memset: Memset{pos: 7, value: 101}},
            Instruction{mask: mask.clone(), memset: Memset{pos: 8, value: 0}},
        ];
        assert_eq!(run(instructions), 165);
    }
}
