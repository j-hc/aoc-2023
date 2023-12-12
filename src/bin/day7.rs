use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{Debug, Display, Write},
};

fn parse(content: String) -> Vec<HandBid> {
    content
        .lines()
        .map(|l| {
            let mut iter = l.split_ascii_whitespace();
            let mut hand: Hand = Hand([Label::A; 5]);
            iter.next()
                .unwrap()
                .chars()
                .map(Label::from)
                .enumerate()
                .for_each(|(i, l)| hand.0[i] = l);
            HandBid {
                hand,
                bid: iter.next().unwrap().parse().unwrap(),
            }
        })
        .collect::<Vec<_>>()
}
fn main() {
    let content = std::fs::read_to_string("inputs/day7.txt").unwrap();
    let hands = parse(content);
    part1(&hands);
    part2(&hands);
}

fn part2(hands: &[HandBid]) {
    let mut rank_map = HashMap::<Rank, Vec<HandBid>>::new();
    hands.iter().for_each(|hand| {
        let rank = hand.hand.rank2();
        rank_map
            .entry(rank)
            .and_modify(|hds| hds.push(*hand))
            .or_insert(vec![*hand]);
    });

    let mut rank = 1;
    let mut total = 0;
    for e in [
        Rank::HighCard,
        Rank::OnePair,
        Rank::TwoPair,
        Rank::ThreeOfAKind,
        Rank::FullHouse,
        Rank::FourOfAKind,
        Rank::FiveOfAKind,
    ]
    .iter()
    {
        let Some(hbss) = rank_map.get_mut(e) else {
            continue;
        };
        hbss.sort_unstable_by(|a, b| {
            for (&a, &b) in std::iter::zip(a.hand.iter(), b.hand.iter()) {
                match (a, b) {
                    (Label::J, Label::J) => continue,
                    (Label::J, _) => return Ordering::Less,
                    (_, Label::J) => return Ordering::Greater,
                    _ => {}
                }
                match a.strength().cmp(&b.strength()) {
                    Ordering::Equal => continue,
                    c => return c,
                }
            }
            unreachable!()
        });

        for hbs in hbss {
            // println!("{:?} {}*{}={}\n", hbs.hand, rank, hbs.bid, rank * hbs.bid);
            total += rank * hbs.bid;
            rank += 1;
        }
    }
    dbg!(total);
}

fn part1(hands: &[HandBid]) {
    let mut rank_map = HashMap::<Rank, Vec<HandBid>>::new();
    hands.iter().for_each(|hand| {
        let rank = hand.hand.rank1();
        rank_map
            .entry(rank)
            .and_modify(|hds| hds.push(*hand))
            .or_insert(vec![*hand]);
    });

    let mut rank = 1;
    let mut total = 0;
    for e in [
        Rank::HighCard,
        Rank::OnePair,
        Rank::TwoPair,
        Rank::ThreeOfAKind,
        Rank::FullHouse,
        Rank::FourOfAKind,
        Rank::FiveOfAKind,
    ]
    .iter()
    {
        let Some(hbss) = rank_map.get_mut(e) else {
            continue;
        };

        hbss.sort_unstable_by(|a, b| {
            for (&a, &b) in std::iter::zip(a.hand.iter(), b.hand.iter()) {
                match a.strength().cmp(&b.strength()) {
                    std::cmp::Ordering::Equal => {}
                    c => return c,
                }
            }
            unreachable!()
        });

        for hbs in hbss {
            // println!("{:?} {}*{}={}\n", hbs.hand, rank, hbs.bid, rank * hbs.bid);
            total += rank * hbs.bid;
            rank += 1;
        }
    }
    dbg!(total);
}

impl Hand {
    fn count_labels(&self) -> [usize; 13] {
        let mut counts = [0_usize; 13];
        for label in self.iter() {
            let i = label.strength();
            counts[i] += 1;
        }
        counts
    }

    fn rank2(&self) -> Rank {
        let mut counts = self.count_labels();

        let ji: usize = Label::J.strength();
        let j_count = counts[ji];

        let index_of_max = counts
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != ji)
            .max_by(|(_, &a), (_, &b)| a.cmp(&b))
            .map(|(index, _)| index)
            .unwrap();
        counts[index_of_max] += j_count;
        counts[ji] = 0;

        assert_eq!(counts.iter().sum::<usize>(), 5);

        if counts.iter().any(|&c| c == 5) {
            return Rank::FiveOfAKind;
        }
        if counts.iter().any(|&c| c == 4) {
            return Rank::FourOfAKind;
        }
        if counts.iter().any(|&c| c == 3) {
            if counts.iter().any(|&c| c == 2) {
                return Rank::FullHouse;
            } else {
                return Rank::ThreeOfAKind;
            }
        }
        if let Some(label) = counts.iter().position(|&c| c == 2) {
            if let Some(label_r) = counts.iter().rposition(|&c| c == 2) {
                if label != label_r {
                    return Rank::TwoPair;
                }
            }
            return Rank::OnePair;
        }
        Rank::HighCard
    }

    fn rank1(&self) -> Rank {
        let counts = self.count_labels();
        assert_eq!(counts.iter().sum::<usize>(), 5);
        if counts.iter().any(|&c| c == 5) {
            return Rank::FiveOfAKind;
        }
        if counts.iter().any(|&c| c == 4) {
            return Rank::FourOfAKind;
        }
        if counts.iter().any(|&c| c == 3) {
            if counts.iter().any(|&c| c == 2) {
                return Rank::FullHouse;
            } else {
                return Rank::ThreeOfAKind;
            }
        }
        if let Some(label) = counts.iter().position(|&c| c == 2) {
            if let Some(label_r) = counts.iter().rposition(|&c| c == 2) {
                if label != label_r {
                    return Rank::TwoPair;
                }
            }
            return Rank::OnePair;
        }
        Rank::HighCard
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Hand([Label; 5]);
impl Hand {
    fn iter(&self) -> impl Iterator<Item = &Label> {
        self.0.iter()
    }
}
impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for label in self.iter() {
            let c: char = (*label).into();
            f.write_char(c)?;
        }
        Ok(())
    }
}
impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Rank {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct HandBid {
    hand: Hand,
    bid: usize,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Label {
    N2 = 0,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

impl Label {
    fn strength(&self) -> usize {
        use Label::*;
        match self {
            N2 => 0,
            N3 => 1,
            N4 => 2,
            N5 => 3,
            N6 => 4,
            N7 => 5,
            N8 => 6,
            N9 => 7,
            T => 8,
            J => 9,
            Q => 10,
            K => 11,
            A => 12,
        }
    }
}

impl From<char> for Label {
    fn from(value: char) -> Self {
        use Label::*;
        match value {
            'A' => A,
            'K' => K,
            'Q' => Q,
            'J' => J,
            'T' => T,
            '9' => N9,
            '8' => N8,
            '7' => N7,
            '6' => N6,
            '5' => N5,
            '4' => N4,
            '3' => N3,
            '2' => N2,
            c => panic!("not a label {c}"),
        }
    }
}

impl From<Label> for char {
    fn from(value: Label) -> Self {
        use Label::*;
        match value {
            N2 => '2',
            N3 => '3',
            N4 => '4',
            N5 => '5',
            N6 => '6',
            N7 => '7',
            N8 => '8',
            N9 => '9',
            T => 'T',
            J => 'J',
            Q => 'Q',
            K => 'K',
            A => 'A',
        }
    }
}
