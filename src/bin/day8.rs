use std::{
    collections::HashMap,
    fmt::{Debug, Display, Write},
};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Sym([u8; 3]);
impl Display for Sym {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.0.iter() {
            f.write_char(*c as char)?;
        }
        Ok(())
    }
}
impl Debug for Sym {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl From<&[u8]> for Sym {
    fn from(value: &[u8]) -> Self {
        Self(value.try_into().unwrap())
    }
}

impl From<&str> for Sym {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().try_into().unwrap())
    }
}

fn parse(content: &str) -> (Vec<Instr>, HashMap<Sym, [Sym; 2]>) {
    let mut lines = content.lines();
    let instrs = lines
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .map(|&b| match b {
            b'L' => Instr::L,
            b'R' => Instr::R,
            c => panic!("not an instruction {c}"),
        })
        .collect::<Vec<_>>();
    lines.next().unwrap();

    let mut map = HashMap::<Sym, [Sym; 2]>::new();
    for line in lines {
        let mut iter = line.split_ascii_whitespace();
        let s: Sym = iter.next().unwrap().into();
        iter.next().unwrap();

        let l = iter.next().unwrap();
        let l: Sym = l[1..l.len() - 1].as_bytes().into();

        let r = iter.next().unwrap();
        let r: Sym = r[..r.len() - 1].as_bytes().into();

        let res = map.insert(s, [l, r]);
        assert!(res.is_none());
    }
    (instrs, map)
}

fn part1(instrs: &[Instr], map: &HashMap<Sym, [Sym; 2]>) {
    let mut cur: Sym = "AAA".into();
    let mut steps = 0;
    'outer: loop {
        for instr in instrs {
            let d = map.get(&cur).unwrap();
            // dbg!(cur, d, instr);
            match instr {
                Instr::L => cur = d[0],
                Instr::R => cur = d[1],
            }
            steps += 1;
            if cur == "ZZZ".into() {
                break 'outer;
            }
        }
    }
    dbg!(steps);
}

fn part2(instrs: &[Instr], map: &HashMap<Sym, [Sym; 2]>) {
    let mut curs = map.keys().filter(|s| s.0[2] == b'A').collect::<Vec<_>>();
    let mut ss = vec![0usize; curs.len()];
    let mut steps = 0usize;

    println!("curs len = {}", curs.len());
    let mut i = 0;
    for cur in &mut curs {
        'outer: loop {
            print!("{steps}\r");
            for instr in instrs {
                let d = map.get(cur).unwrap();
                match instr {
                    Instr::L => *cur = &d[0],
                    Instr::R => *cur = &d[1],
                }
                steps += 1;
                if cur.0[2] == b'Z' {
                    ss[i] = steps;
                    i += 1;
                    steps = 0;
                    break 'outer;
                }
            }
        }
    }
    dbg!(lcm(&ss));
}

fn gcd_two(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_two(b, a % b)
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_two(a, b)
}

fn main() {
    let content = std::fs::read_to_string("inputs/day8.txt").unwrap();
    let (instrs, map) = parse(&content);

    part1(&instrs, &map);
    part2(&instrs, &map);
}

#[derive(Debug)]
enum Instr {
    L,
    R,
}
