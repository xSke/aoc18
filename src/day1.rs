use std::collections::BTreeSet;

pub struct Day1;

impl super::Day for Day1 {
    fn name(&self) -> &str { "Chronal Calibration" } 

    fn part_1(&mut self, input: &str) -> String {
        input
            .lines()
            .fold(0, |acc, line| acc + line.parse::<isize>().unwrap())
            .to_string()
    }

    fn part_2(&mut self, input: &str) -> String {
        let mut seen = BTreeSet::new();

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
}