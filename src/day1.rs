use std::collections::HashSet;

pub fn part1(input: &str) -> (String, ()) {
    return (input
        .lines()
        .fold(0, |acc, line| acc + line.parse::<isize>().unwrap())
        .to_string(), ());
}

pub fn part2(input: &str, _: ()) -> String {
    let mut seen = HashSet::new();

    let mut acc = 0;

    loop {
        for line in input.lines() {
            acc += line.parse::<isize>().unwrap();
            
            if seen.contains(&acc) {
                return acc.to_string();
            }
            seen.insert(acc);
        }
    }
}