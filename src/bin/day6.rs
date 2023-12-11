fn main() {
    let content = std::fs::read_to_string("inputs/day6.txt").unwrap();
    part1(&content);
    part2(&content);
}

// let hold_down = t;
// let speed = hold_down = t;
// dist = speed * (race.time - hold_down);
// dist = t * (race.time - t) = race.time * t - t ^ 2;
// dist > race.rec_dist;
// race.time * t - t ^ 2 > race.rec_dist;
// race.time * t - t ^ 2 - race.rec_dist > 0;
// t ^ 2 - race.time * t + rec_dist < 0;
// -> true when t1 < t < t2
fn solve<'a>(races: impl Iterator<Item = &'a Race>) -> f64 {
    races
        .map(|race| {
            let a: f64 = 1.0;
            let b = -(race.time as f64);
            let c = race.rec_dist as f64;

            let disc = (b * b - 4.0 * a * c).sqrt();
            let t1 = (-b - disc) / (2.0 * a);
            let t2 = (-b + disc) / (2.0 * a);

            let t1r = t1.floor() + 1.0;
            let t2r = t2.ceil() - 1.0;
            t2r - t1r + 1.0
        })
        .product()
}

fn part2(content: &str) {
    let mut lines = content.lines();
    let time_line = lines.next().unwrap();
    let dist_line = lines.next().unwrap();

    let time = time_line
        .split_ascii_whitespace()
        .skip(1)
        .fold(String::new(), |mut acc, e| {
            acc.push_str(e);
            acc
        });
    let rec_dist = dist_line
        .split_ascii_whitespace()
        .skip(1)
        .fold(String::new(), |mut acc, e| {
            acc.push_str(e);
            acc
        });

    let race = Race {
        time: time.parse().unwrap(),
        rec_dist: rec_dist.parse().unwrap(),
    };

    let answer = solve(std::iter::once(&race));

    println!("[PART 2] answer = {}", answer);
}

fn part1(content: &str) {
    let mut lines = content.lines();
    let time_line = lines.next().unwrap();
    let dist_line = lines.next().unwrap();

    let races = std::iter::zip(
        time_line.split_ascii_whitespace().skip(1),
        dist_line.split_ascii_whitespace().skip(1),
    )
    .map(|(t, d)| Race {
        time: t.parse().unwrap(),
        rec_dist: d.parse().unwrap(),
    })
    .collect::<Vec<Race>>();

    let answer: f64 = solve(races.iter());

    println!("[PART 1] answer = {}", answer);
}

#[derive(Debug)]
struct Race {
    time: usize,
    rec_dist: usize,
}
