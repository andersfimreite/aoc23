use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("could not read file");
    let output1 = part1(input.clone());
    let output2 = part2(input);
    println!("Part1: {output1}");
    println!("Part2: {output2}");
}

fn part1(input: String) -> i32 {
    let input = input.replace("(", "").replace(")", "");
    let mut lines = input.lines();

    let inst: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();

    let mut nodes = HashMap::new();

    for line in lines {
        let (node, left_and_right) = line.split_once(" = ").unwrap();
        nodes.insert(node, left_and_right.split_once(", ").unwrap());
    }

    let mut steps = 0;
    let mut node = "AAA";
    while node != "ZZZ" {
        let options = nodes.get(&node).unwrap();
        node = match inst.get(steps % inst.len()).unwrap() {
            'L' => options.0,
            'R' => options.1,
            _ => panic!("unknown instruction"),
        };
        steps += 1;
    }

    return steps as i32;
}

fn part2(input: String) -> u64 {
    let input = input.replace("(", "").replace(")", "");
    let mut lines = input.lines();

    let inst: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();

    let mut map = HashMap::new();

    for line in lines {
        let (node, left_and_right) = line.split_once(" = ").unwrap();
        map.insert(node, left_and_right.split_once(", ").unwrap());
    }

    let steps = map
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|&node| {
            let mut s = 0;
            let mut n = node;
            while !n.ends_with("Z") {
                let options = map.get(n).unwrap();
                n = match inst.get(s % inst.len()).unwrap() {
                    'L' => &options.0,
                    'R' => &options.1,
                    _ => panic!("unknown instruction"),
                };
                s += 1;
            }
            return s as u64;
        })
        .fold(1, |acc, s| lcm(acc, s));

    return steps;
}

fn lcm(n: u64, m: u64) -> u64 {
    return n * m / gcd(n, m);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    return n;
}
