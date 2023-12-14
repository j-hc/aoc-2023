fn main() {
    let content = std::fs::read_to_string("inputs/day9.txt").unwrap();
    let histories = parse(content);
    part1(histories.clone());
    part2(histories.clone());
}

fn part2(histories: Vec<Vec<isize>>) {
    let mut total = 0;
    for mut history in histories {
        let mut nexts = vec![history];
        while !nexts[nexts.len() - 1].iter().all(|&e| e == 0) {
            nexts.push(
                nexts[nexts.len() - 1]
                    .windows(2)
                    .map(|w| w[1] - w[0])
                    .collect::<Vec<isize>>(),
            );
        }
        nexts.iter_mut().for_each(|n| n.reverse());

        let mut prev = 0;
        for next in nexts.iter_mut().rev().skip(1) {
            prev = -prev + next[next.len() - 1];
            // dbg!(&next, prev);
            next.push(prev);
        }
        total += prev;
    }
    println!("[PART2] total = {}", total);
}

fn part1(histories: Vec<Vec<isize>>) {
    let mut total = 0;
    for history in histories {
        let mut nexts = vec![history];
        while !nexts[nexts.len() - 1].iter().all(|&e| e == 0) {
            nexts.push(
                nexts[nexts.len() - 1]
                    .windows(2)
                    .map(|w| w[1] - w[0])
                    .collect::<Vec<isize>>(),
            );
        }

        let mut prev = 0;
        for next in nexts.iter_mut().rev().skip(1) {
            prev = prev + next[next.len() - 1];
            // dbg!(&next, prev);
            next.push(prev);
        }
        total += prev;
    }
    println!("[PART1] total = {}", total);
}

fn parse(content: String) -> Vec<Vec<isize>> {
    content
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|e| e.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
