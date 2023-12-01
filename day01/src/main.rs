fn main() {
    let input = std::fs::read_to_string("input.txt").expect("could not read file");
    let output1 = part1(input.clone());
    let output2 = part2(input);
    println!("Part1: {output1}");
    println!("Part2: {output2}");
}

fn part1(input: String) -> i32 {
    let mut total = 0;
    for line in input.trim_end().split("\n") {
        let mut num1: i32 = -1;
        let mut num2: i32 = -1;
        for char in line.chars() {
            if let Some(num) = char.to_digit(10) {
                if num1 == -1 {
                    num1 = num as i32;
                    num2 = num as i32;
                } else {
                    num2 = num as i32;
                }
            }
        }
        total += num1 * 10 + num2;
    }
    return total;
}

fn part2(input: String) -> i32 {
    let strings = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut total = 0;
    for line in input.trim_end().split("\n") {
        let mut num1: i32 = -1;
        let mut num2: i32 = -1;
        for (idx, char) in line.chars().enumerate() {
            match char.to_digit(10) {
                Some(num) => {
                    if num1 == -1 {
                        num1 = num as i32;
                        num2 = num as i32;
                    } else {
                        num2 = num as i32;
                    }
                }
                None => {
                    for s in &strings {
                        if line[idx..].starts_with(s) {
                            let num = strings.iter().position(|e| e == s).unwrap() + 1;
                            if num1 == -1 {
                                num1 = num as i32;
                                num2 = num as i32;
                            } else {
                                num2 = num as i32;
                            }
                            break;
                        }
                    }
                }
            };
        }
        total += num1 * 10 + num2;
    }
    return total;
}
