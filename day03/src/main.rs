fn main() {
    let input = std::fs::read_to_string("input.txt").expect("could not read file");
    let output1 = part1(input.clone());
    let output2 = part2(input);
    println!("Part1: {output1}");
    println!("Part2: {output2}");
}

fn part1(input: String) -> u32 {
    let symbols: Vec<Vec<Option<char>>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| {
                    if let Some(_) = char.to_digit(10) {
                        return None;
                    } else {
                        if char == '.' {
                            return None;
                        } else {
                            return Some(char);
                        }
                    }
                })
                .collect()
        })
        .collect();

    return input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            let mut numbers: Vec<u32> = vec![];
            let mut chars = line.chars().enumerate().peekable();
            let mut prev_was_symbol = false;
            while let Some((j, char)) = chars.next() {
                if let Some(digit) = char.to_digit(10) {
                    let mut add = prev_was_symbol;
                    let mut num = digit;
                    let start_idx = j;
                    let mut stop_idx = j;

                    while let Some((k, char)) = chars.peek() {
                        if let Some(digit) = char.to_digit(10) {
                            num *= 10;
                            num += digit;
                            stop_idx = *k;
                            chars.next();
                        } else {
                            if *char != '.' {
                                add = true;
                            }
                            break;
                        }
                    }

                    if add {
                        numbers.push(num);
                    } else {
                        if i != 0 {
                            let mut a = if start_idx > 0 {
                                start_idx - 1
                            } else {
                                start_idx
                            };
                            let b = if stop_idx < symbols[i].len() - 1 {
                                stop_idx + 1
                            } else {
                                stop_idx
                            };

                            while a <= b {
                                if let Some(_) = symbols[i - 1][a] {
                                    add = true;
                                }
                                a += 1;
                            }
                        }

                        if i != symbols.len() - 1 {
                            let mut a = if start_idx > 0 {
                                start_idx - 1
                            } else {
                                start_idx
                            };
                            let b = if stop_idx < symbols[i].len() - 1 {
                                stop_idx + 1
                            } else {
                                stop_idx
                            };

                            while a <= b {
                                if let Some(_) = symbols[i + 1][a] {
                                    add = true;
                                }
                                a += 1;
                            }
                        }

                        if add {
                            numbers.push(num);
                        }
                    }
                } else {
                    if char != '.' {
                        prev_was_symbol = true;
                    } else {
                        prev_was_symbol = false;
                    }
                }
            }
            return numbers;
        })
        .sum();
}

fn part2(input: String) -> u32 {
    let numbers: Vec<Vec<(u32, i32, i32)>> = input
        .lines()
        .map(|line| {
            let mut nums: Vec<(u32, i32, i32)> = vec![];
            let mut chars = line.chars().enumerate().peekable();
            while let Some((j, char)) = chars.next() {
                if let Some(digit) = char.to_digit(10) {
                    let mut num = digit;
                    let mut stop_idx = j;
                    while let Some((k, char)) = chars.peek() {
                        if let Some(digit) = char.to_digit(10) {
                            num = num * 10 + digit;
                            stop_idx = *k;
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    nums.push((num, j as i32, stop_idx as i32));
                }
            }
            return nums;
        })
        .collect();

    return input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(j, char)| match char {
                    '*' => Some((j as i32, i as i32)),
                    _ => None,
                })
        })
        .map(|star| {
            let mut num1 = 0;
            let mut num2 = 0;

            if star.1 > 0 {
                for (num, s, e) in &numbers[(star.1 - 1) as usize] {
                    if *s - 1 <= star.0 && *e + 1 >= star.0 {
                        if num1 == 0 {
                            num1 = *num;
                        } else if num2 == 0 {
                            num2 = *num;
                        }
                    }
                }
            }

            if star.1 < (numbers.len() - 1) as i32 {
                for (num, s, e) in &numbers[(star.1 + 1) as usize] {
                    if *s - 1 <= star.0 && *e + 1 >= star.0 {
                        if num1 == 0 {
                            num1 = *num;
                        } else if num2 == 0 {
                            num2 = *num;
                        }
                    }
                }
            }

            for (num, s, e) in &numbers[star.1 as usize] {
                if *s - 1 == star.0 || *e + 1 == star.0 {
                    if num1 == 0 {
                        num1 = *num;
                    } else if num2 == 0 {
                        num2 = *num;
                    }
                }
            }

            return num1 * num2;
        })
        .sum();
}
