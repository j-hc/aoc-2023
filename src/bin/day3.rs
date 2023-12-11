use std::collections::HashMap;

fn main() {
    let mut cnt = std::fs::read("inputs/day3.txt").unwrap();
    let cols = std::str::from_utf8(&cnt)
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .len();
    cnt.retain(|&b| b != b'\n' && b != b'\r');
    let rows = cnt.len() / cols;
    let map = Map { cnt, rows, cols };
    let numbers = parse(&map);
    part1(&numbers);
    part2(&numbers);
}

struct Map {
    cnt: Vec<u8>,
    rows: usize,
    cols: usize,
}

#[derive(PartialEq, Debug)]
struct Point {
    v: char,
    coords: (usize, usize),
}

#[derive(Debug)]
struct Number {
    bounding_box: Vec<Point>,
    number: usize,
}

fn parse(schem: &Map) -> Vec<Number> {
    let stride = schem.cols;
    let rows = schem.rows;
    let cnt = &schem.cnt;

    let mut i = 0;
    let mut total = 0;

    let mut numbers = Vec::new();
    loop {
        if i >= cnt.len() {
            break;
        }
        if cnt[i].is_ascii_digit() {
            let s = i;
            let mut number = String::new();
            number.push(cnt[i] as char);
            i += 1;
            while cnt[i].is_ascii_digit() && i % stride != 0 {
                number.push(cnt[i] as char);
                i += 1;
            }
            let number = number.parse::<usize>().unwrap();
            let e = i;

            let x = s % stride;
            let y = s / stride;
            let num_width = e - s;
            let mut bounding_box = Vec::new();
            for bounding_y in y.saturating_sub(1)..(y + 2).min(rows) {
                for bounding_x in x.saturating_sub(1)..(x + num_width + 1).min(stride) {
                    if bounding_y == y && bounding_x >= x && bounding_x < x + num_width {
                        continue;
                    }
                    let c = cnt[bounding_y * stride + bounding_x];
                    bounding_box.push(Point {
                        v: c as char,
                        coords: (bounding_x, bounding_y),
                    });
                }
            }
            numbers.push(Number {
                bounding_box,
                number,
            });
        }
        i += 1;
    }
    return numbers;
}

fn part1(numbers: &[Number]) {
    let mut total = 0;
    for n in numbers.iter() {
        for bounding_char in &n.bounding_box {
            if bounding_char.v != '.' && !bounding_char.v.is_ascii_digit() {
                total += n.number;
                // println!("adj -> {}", n.number);
            }
        }
    }
    println!("[PART1] total = {}", total);
}

fn part2(numbers: &[Number]) {
    let mut total = 0;
    let mut adj_gears: HashMap<(usize, usize), (usize, Vec<usize>)> = HashMap::new();
    for n in numbers.iter() {
        for bounding_char in &n.bounding_box {
            if bounding_char.v == '*' {
                adj_gears
                    .entry(bounding_char.coords)
                    .and_modify(|v| {
                        v.0 += 1;
                        v.1.push(n.number)
                    })
                    .or_insert((1, vec![n.number]));
            }
        }
    }
    let total: usize = adj_gears
        .iter()
        .filter(|(_, (v, _))| *v == 2)
        .map(|(_, v)| v.1.iter().product::<usize>())
        .sum();
    println!("PART2] total = {}", total);
}
