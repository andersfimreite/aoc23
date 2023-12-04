use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("could not read file");
    let output1 = part1(input.clone());
    let output2 = part2(input);
    println!("Part1: {output1}");
    println!("Part2: {output2}");
}

fn part1(input: String) -> i32 {
    input
        .lines()
        .map(|line| {
            let (winners, numbers) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
            let winners = winners
                .split(" ")
                .filter_map(|v| v.parse::<i32>().ok())
                .collect::<HashSet<i32>>();
            let numbers = numbers
                .split(" ")
                .filter_map(|v| v.parse::<i32>().ok())
                .collect::<HashSet<i32>>();
            let count = winners.intersection(&numbers).count();
            if count == 0 {
                return count as i32;
            }
            return 2i32.pow((count - 1) as u32);
        })
        .sum()
}

fn part2(input: String) -> i32 {
    let mut counts: HashMap<usize, i32> = HashMap::new();

    input
        .lines()
        .map(|line| {
            let (winners, numbers) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
            let winners = winners
                .split(" ")
                .filter_map(|v| v.parse::<i32>().ok())
                .collect::<HashSet<i32>>();
            let numbers = numbers
                .split(" ")
                .filter_map(|v| v.parse::<i32>().ok())
                .collect::<HashSet<i32>>();
            winners.intersection(&numbers).count() as i32
        })
        .enumerate()
        .for_each(|(idx, win_count)| {
            if let Some(count) = counts.get_mut(&idx) {
                *count += 1;
            } else {
                counts.insert(idx, 1);
            }

            let count = *counts.get_mut(&idx).unwrap();

            for i in idx..idx + win_count as usize {
                if let Some(cnt) = counts.get_mut(&(i + 1)) {
                    *cnt += count;
                } else {
                    counts.insert(i + 1, count);
                }
            }
        });

    return counts.values().into_iter().sum();
}
