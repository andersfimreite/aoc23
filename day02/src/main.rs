fn main() {
    let input = std::fs::read_to_string("input.txt").expect("could not read file");
    let output1 = part1(input.clone());
    let output2 = part2(input);
    println!("Part1: {output1}");
    println!("Part2: {output2}");
}

fn part1(input: String) -> i32 {
    return input
        .lines()
        .filter_map(|line| {
            let (id_str, sets) = line.split_once(": ").unwrap();
            let id = id_str.split_once(" ").unwrap().1.parse::<i32>().unwrap();
            for set in sets.split("; ") {
                for item in set.split(", ") {
                    let (count_str, color) = item.split_once(" ").unwrap();
                    let count = count_str.parse::<i32>().unwrap();
                    let invalid = match color {
                        "red" => count > 12,
                        "green" => count > 13,
                        "blue" => count > 14,
                        _ => true,
                    };
                    if invalid {
                        return None;
                    }
                }
            }
            return Some(id);
        })
        .sum();
}

fn part2(input: String) -> i32 {
    return input
        .lines()
        .map(|line| {
            let mut r_max = 0;
            let mut g_max = 0;
            let mut b_max = 0;
            line.split_once(": ")
                .unwrap()
                .1
                .split("; ")
                .flat_map(|set| set.split(", "))
                .for_each(|item| {
                    let (count_str, color) = item.split_once(" ").unwrap();
                    let count = count_str.parse::<i32>().unwrap();
                    match color {
                        "red" => {
                            if count > r_max {
                                r_max = count;
                            }
                        }
                        "green" => {
                            if count > g_max {
                                g_max = count;
                            }
                        }
                        "blue" => {
                            if count > b_max {
                                b_max = count;
                            }
                        }
                        _ => {}
                    };
                });
            return r_max * g_max * b_max;
        })
        .sum();
}
