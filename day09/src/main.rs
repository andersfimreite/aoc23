fn main() {
    let input = std::fs::read_to_string("input.txt").expect("could not read file");
    let output1 = part1(input.clone());
    let output2 = part2(input);
    println!("Part1: {output1}");
    println!("Part2: {output2}");
}

fn next_in_seq(seq: Vec<i32>) -> i32 {
    if seq.iter().all(|&e| e == 0) {
        return 0;
    }

    let mut diff_seq = Vec::new();
    let mut i = 0;
    let mut j = 1;
    while j < seq.len() {
        let a = seq.get(i).unwrap();
        let b = seq.get(j).unwrap();
        diff_seq.push(b - a);
        i += 1;
        j += 1;
    }

    let last = seq.last().unwrap();
    return next_in_seq(diff_seq) + last;
}

fn part1(input: String) -> i32 {
    return input
        .lines()
        .map(|line| {
            let seq = line
                .split(" ")
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            return next_in_seq(seq);
        })
        .sum();
}

fn prev_in_seq(seq: Vec<i32>) -> i32 {
    if seq.iter().all(|&e| e == 0) {
        return 0;
    }

    let mut diff_seq = Vec::new();
    let mut i = 0;
    let mut j = 1;
    while j < seq.len() {
        let a = seq.get(i).unwrap();
        let b = seq.get(j).unwrap();
        diff_seq.push(b - a);
        i += 1;
        j += 1;
    }

    let first = seq.first().unwrap();
    return first - prev_in_seq(diff_seq);
}

fn part2(input: String) -> i32 {
    return input
        .lines()
        .map(|line| {
            let seq = line
                .split(" ")
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            return prev_in_seq(seq);
        })
        .sum();
}
