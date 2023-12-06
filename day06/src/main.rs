fn main() {
    let input = std::fs::read_to_string("input.txt").expect("could not read file");
    let output1 = part1(input.clone());
    let output2 = part2(input);
    println!("Part1: {output1}");
    println!("Part2: {output2}");
}

fn part1(input: String) -> i32 {
    let mut lines = input.lines();

    let times = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split(" ")
        .filter_map(|v| v.parse::<i32>().ok());

    let distances = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split(" ")
        .filter_map(|v| v.parse::<i32>().ok());

    return times
        .zip(distances)
        .map(|(t, d)| (1..t).into_iter().filter(|w| w * (t - w) > d).count() as i32)
        .fold(1, |t, s| t * s);
}

fn part2(input: String) -> i64 {
    let mut lines = input.lines();

    let time = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<i64>()
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<i64>()
        .unwrap();

    return (1..time)
        .into_iter()
        .filter(|w| w * (time - w) > distance)
        .count() as i64;
}
