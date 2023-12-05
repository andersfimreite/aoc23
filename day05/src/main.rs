fn main() {
    let input = std::fs::read_to_string("input.txt").expect("could not read file");
    let output1 = part1(input.clone());
    let output2 = part2(input);
    println!("Part1: {output1}");
    println!("Part2: {output2}");
}

fn part1(input: String) -> i64 {
    let mut lines = input.lines().peekable();

    let mut seeds: Vec<i64> = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|v| v.parse::<i64>().unwrap())
        .collect();

    while let Some(_) = lines.peek() {
        lines.next();
        lines.next();

        let mut mappings: Vec<(i64, i64, i64)> = Vec::new();
        while let Some(mapping) = lines.next_if(|item| *item != "") {
            let mut nums = mapping.split(" ").map(|v| v.parse::<i64>().unwrap());
            let dst = nums.nth(0).unwrap();
            let src = nums.nth(0).unwrap();
            let len = nums.nth(0).unwrap();

            let d = dst - src;

            mappings.push((src, src + len, d));
        }

        seeds = seeds
            .iter()
            .map(|&v| {
                for (start, stop, delta) in &mappings {
                    if *start <= v && v < *stop {
                        return v + *delta;
                    }
                }
                return v;
            })
            .collect();
    }

    seeds.sort();
    return *seeds.get(0).unwrap();
}

fn part2(input: String) -> i64 {
    let mut lines = input.lines().peekable();

    let mut ranges: Vec<(i64, i64)> = Vec::new();

    let mut it = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|v| v.parse::<i64>().unwrap())
        .into_iter()
        .peekable();

    while let Some(_) = it.peek() {
        let src = it.next().unwrap();
        let len = it.next().unwrap();
        ranges.push((src, src + len));
    }

    while let Some(_) = lines.peek() {
        lines.next();
        lines.next();

        let mut mappings: Vec<(i64, i64, i64)> = Vec::new();
        while let Some(mapping) = lines.next_if(|item| *item != "") {
            let mut nums = mapping.split(" ").map(|v| v.parse::<i64>().unwrap());
            let dst = nums.nth(0).unwrap();
            let src = nums.nth(0).unwrap();
            let len = nums.nth(0).unwrap();

            let d = dst - src;

            mappings.push((src, src + len, d));
        }

        ranges = fix_ranges(&mappings, &ranges);
    }

    ranges.sort();
    return ranges.get(0).unwrap().0;
}

fn fix_ranges(mappings: &Vec<(i64, i64, i64)>, ranges: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut new_ranges: Vec<(i64, i64)> = Vec::new();
    let mut rest: Vec<(i64, i64)> = Vec::new();

    for (r_start, r_stop) in ranges {
        let mut added = false;
        for (m_start, m_stop, delta) in mappings {
            // range outside mapping
            if *m_start >= *r_stop || *m_stop <= *r_start {
                continue;
            }

            // range fully inside mapping
            if *m_start <= *r_start && *r_stop <= *m_stop {
                new_ranges.push((*r_start + *delta, *r_stop + *delta));
                added = true;
                break;
            }

            // start inside
            if *m_start <= *r_start {
                new_ranges.push((*r_start + *delta, *m_stop + *delta));
                rest.push((*m_stop, *r_stop));
                added = true;
                break;
            }

            // stop inside
            if *r_stop <= *m_stop {
                new_ranges.push((*m_start + *delta, *r_stop + *delta));
                rest.push((*r_start, *m_start));
                added = true;
                break;
            }
        }

        if !added {
            new_ranges.push((*r_start, *r_stop));
        }
    }

    if rest.len() > 0 {
        new_ranges.append(&mut fix_ranges(mappings, &rest));
    }

    return new_ranges;
}
