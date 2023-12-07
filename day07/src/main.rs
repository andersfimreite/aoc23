use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("could not read file");
    let output1 = part1(input.clone());
    let output2 = part2(input);
    println!("Part1: {output1}");
    println!("Part2: {output2}");
}

#[derive(Debug, Ord, Eq, PartialEq)]
enum Type {
    FiveOfKind(Vec<i32>),
    FourOfKind(Vec<i32>),
    FullHouse(Vec<i32>),
    ThreeOfKind(Vec<i32>),
    TwoPairs(Vec<i32>),
    OnePair(Vec<i32>),
    HighCard(Vec<i32>),
}

impl Type {
    fn value(&self) -> Vec<i32> {
        match self {
            Type::FiveOfKind(val) => val.to_vec(),
            Type::FourOfKind(val) => val.to_vec(),
            Type::FullHouse(val) => val.to_vec(),
            Type::ThreeOfKind(val) => val.to_vec(),
            Type::TwoPairs(val) => val.to_vec(),
            Type::OnePair(val) => val.to_vec(),
            Type::HighCard(val) => val.to_vec(),
        }
    }
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.sort_key() == other.sort_key() {
            let arr1 = self.value();
            let arr2 = other.value();
            for i in 0..arr1.len() {
                if arr1.get(i) == arr2.get(i) {
                    continue;
                }

                if arr1.get(i) < arr2.get(i) {
                    return Some(std::cmp::Ordering::Greater);
                }

                return Some(std::cmp::Ordering::Less);
            }
        }

        return Some(self.cmp(other));
    }
}

trait SortKey {
    fn sort_key(&self) -> i32;
}

impl SortKey for Type {
    fn sort_key(&self) -> i32 {
        return match self {
            Type::HighCard(_) => 7,
            Type::OnePair(_) => 6,
            Type::TwoPairs(_) => 5,
            Type::ThreeOfKind(_) => 4,
            Type::FullHouse(_) => 3,
            Type::FourOfKind(_) => 2,
            Type::FiveOfKind(_) => 1,
        };
    }
}

impl SortKey for char {
    fn sort_key(&self) -> i32 {
        return match self {
            'j' => 1,
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => self.to_digit(10).unwrap() as i32,
        };
    }
}

fn part1(input: String) -> i32 {
    let mut hands: Vec<(Type, i32)> = input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(hand, bid)| (hand, bid.parse::<i32>().unwrap()))
        .map(|(hand, bid)| {
            let mut counts = HashMap::new();
            for card in hand.chars() {
                if let Some(count) = counts.get_mut(&card) {
                    *count += 1;
                } else {
                    counts.insert(card, 1);
                }
            }
            let sort_keys: Vec<i32> = hand.chars().map(|c| c.sort_key()).collect();
            let typ: Type = match counts.keys().count() {
                5 => Type::HighCard(sort_keys),
                4 => Type::OnePair(sort_keys),
                3 => match counts.values().any(|v| *v == 3) {
                    true => Type::ThreeOfKind(sort_keys),
                    false => Type::TwoPairs(sort_keys),
                },
                2 => match counts.values().any(|v| *v == 3) {
                    true => Type::FullHouse(sort_keys),
                    false => Type::FourOfKind(sort_keys),
                },
                1 => Type::FiveOfKind(sort_keys),
                _ => panic!("Something is wrong"),
            };
            return (typ, bid);
        })
        .collect();

    hands.sort();
    hands.reverse();
    return hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as i32 * bid)
        .sum();
}

fn part2(input: String) -> i32 {
    let mut hands: Vec<(Type, i32)> = input
        .replace("J", "j")
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(hand, bid)| (hand, bid.parse::<i32>().unwrap()))
        .map(|(hand, bid)| {
            let mut counts = HashMap::new();
            for card in hand.chars() {
                if let Some(count) = counts.get_mut(&card) {
                    *count += 1;
                } else {
                    counts.insert(card, 1);
                }
            }
            let j_count = counts.get(&'j').unwrap_or(&0);
            let sort_keys: Vec<i32> = hand.chars().map(|c| c.sort_key()).collect();
            let typ: Type = match (counts.keys().count(), j_count) {
                (5, 0) => Type::HighCard(sort_keys),
                (5, _) => Type::OnePair(sort_keys),
                (4, 0) => Type::OnePair(sort_keys),
                (4, _) => Type::ThreeOfKind(sort_keys),
                (3, 0) => match counts.values().any(|v| *v == 3) {
                    true => Type::ThreeOfKind(sort_keys),
                    false => Type::TwoPairs(sort_keys),
                },
                (3, 1) => match counts.values().any(|v| *v == 3) {
                    true => Type::FourOfKind(sort_keys),
                    false => Type::FullHouse(sort_keys),
                },
                (3, _) => Type::FourOfKind(sort_keys),
                (2, 0) => match counts.values().any(|v| *v == 3) {
                    true => Type::FullHouse(sort_keys),
                    false => Type::FourOfKind(sort_keys),
                },
                (2, _) => Type::FiveOfKind(sort_keys),
                (1, _) => Type::FiveOfKind(sort_keys),
                _ => panic!("Something is wrong"),
            };
            return (typ, bid);
        })
        .collect();

    hands.sort();
    hands.reverse();
    return hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as i32 * bid)
        .sum();
}
