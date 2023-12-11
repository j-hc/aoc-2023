const RED: usize = 12;
const GREEN: usize = 13;
const BLUE: usize = 14;

fn main() {
    let content = std::fs::read("inputs/day2.txt").unwrap();
    let content = String::from_utf8(content).unwrap();
    let games = parse(&content);
    part1(&games);
    part2(&games);
}

enum Color {
    Red(usize),
    Green(usize),
    Blue(usize),
}

struct Game {
    id: usize,
    sets: Vec<Color>,
}

fn parse(content: &str) -> Vec<Game> {
    let mut games = Vec::new();
    for line in content.lines() {
        let (game, rest) = line.split_once(':').unwrap();
        let id = game.split(' ').nth(1).unwrap().parse::<usize>().unwrap();
        let rest = rest.trim();

        let mut sets = Vec::new();
        for set in rest.split(';') {
            for num_color in set.trim().split(',') {
                let mut s = num_color.trim().split(' ');
                let num = s.next().unwrap().parse::<usize>().unwrap();
                let color = s.next().unwrap();
                let color = match color {
                    "red" => Color::Red(num),
                    "green" => Color::Green(num),
                    "blue" => Color::Blue(num),
                    _ => unreachable!(),
                };
                sets.push(color);
            }
        }
        games.push(Game { id, sets });
    }
    return games;
}

fn part2(games: &[Game]) {
    let mut total = 0;
    for game in games {
        let mut min_r = 0;
        let mut min_g = 0;
        let mut min_b = 0;
        for set in &game.sets {
            match set {
                Color::Red(r) => min_r = min_r.max(*r),
                Color::Green(g) => min_g = min_g.max(*g),
                Color::Blue(b) => min_b = min_b.max(*b),
            }
        }
        total += min_r * min_g * min_b;
    }
	println!("total = {}", total);
}

fn part1(games: &[Game]) {
    let mut total = 0;
    for game in games {
        let mut possible = true;
        for set in &game.sets {
            match set {
                Color::Red(r) if *r > RED => possible = false,
                Color::Green(g) if *g > GREEN => possible = false,
                Color::Blue(b) if *b > BLUE => possible = false,
                _ => {}
            }
        }
        if possible {
            total += game.id;
        }
    }

    print!("total = {total}");
}
