
fn main() {
    let content = std::fs::read("inputs/day4.txt").unwrap();
    let cards = parse(std::str::from_utf8(&content).unwrap());
    part1(&cards);
    part2(&cards);
}

#[derive(Debug)]
struct Card {
    id: usize,
    winning: Vec<usize>,
    have: Vec<usize>,
}
fn part2(cards: &[Card]) {
    let mut cards_count = vec![1usize; cards.len()];
    for card in cards.iter() {
        let win_count = card
            .have
            .iter()
            .fold(0usize, |acc, ch| acc + card.winning.contains(ch) as usize);
        let current_card_count = cards_count[card.id - 1];
        // println!("{}->{:?}", current_card_count, card);

        if win_count == 0 {
            continue;
        }
        for card_won in card.id + 1..=card.id + win_count {
            cards_count[card_won - 1] += 1 * current_card_count;
        }
    }
    let total: usize = cards_count.iter().sum();
    println!("[PART2] total = {}", total);
}

fn part1(cards: &[Card]) {
    let mut total = 0;
    for card in cards.iter() {
        let win_count = card
            .have
            .iter()
            .fold(0usize, |acc, ch| acc + card.winning.contains(ch) as usize);

        if win_count > 0 {
            total += 2i32.pow(win_count as u32 - 1);
        }
    }
    println!("[PART1] total = {total}");
}

fn parse(content: &str) -> Vec<Card> {
    let mut cards = Vec::new();
    for line in content.lines() {
        let (game, rest) = line.split_once(':').unwrap();
        let id = game
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let rest = rest.trim();
        let mut rest_iter = rest.split('|');
        let parse_cards = |s: &str| {
            s.trim()
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect()
        };
        let winning = parse_cards(rest_iter.next().unwrap());
        let have = parse_cards(rest_iter.next().unwrap());
        cards.push(Card { id, winning, have });
    }
    return cards;
}
