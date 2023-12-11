fn main() {
    let content = std::fs::read("inputs/day1.txt").unwrap();
    part1(&content);
    part2(&content);
}

fn part2(content: &[u8]) {
    let mut total = 0;
    const NUMBERS: &[&str; 9] = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    fn number_from_idx(idx: usize, line: &[u8]) -> Option<u8> {
        if line[idx].is_ascii_digit() {
            return Some((line[idx] as char).to_digit(10).unwrap() as u8);
        }
        for (ni, number) in NUMBERS.iter().enumerate() {
            if line
                .get(idx..idx + number.len())
                .is_some_and(|w| w == number.as_bytes())
            {
                return Some(ni as u8 + 1);
            }
        }
        None
    }
    for line in std::str::from_utf8(content).unwrap().split_whitespace() {
        let line = line.as_bytes();
        let mut n_iter = (0..line.len())
            .into_iter()
            .enumerate()
            .filter_map(|(i, _)| number_from_idx(i, line));
        let first = n_iter.next().unwrap();
        let second = n_iter.last().unwrap_or(first);

        let s: u32 = format!("{}{}", first, second).parse().unwrap();
        total += s;
    }
    println!("PART2] {total}");
}

fn part1(content: &[u8]) {
    let mut total = 0;

    fn number_from_idx(c: u8) -> Option<u8> {
        c.is_ascii_digit()
            .then(|| (c as char).to_digit(10).unwrap() as u8)
    }

    for line in std::str::from_utf8(content).unwrap().split_whitespace() {
        let line = line.as_bytes();
        let mut n_iter = line.iter().filter_map(|&c| number_from_idx(c));
        let first = n_iter.next().unwrap();
        let second = n_iter.last().unwrap_or(first);

        let s: u32 = format!("{}{}", first, second).parse().unwrap();
        total += s;
    }

    println!("PART1] {total}");
}
