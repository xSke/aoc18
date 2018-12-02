pub struct Day2;

fn contains_exactly(s: &str, amount: usize) -> bool {
    // O(n^2), sad
    for c in s.chars() {
        if s.chars().filter(|x| *x == c).count() == amount {
            return true;
        }
    }
    return false;
}

fn differing_characters(a: &str, b: &str) -> usize {
    // Assumes strings are the same length
    a.chars()
        .zip(b.chars())
        .map(|(ac, bc)| ac != bc)
        .map(|b| if b { 1 } else { 0 })
        .sum()
}

fn equal_chars(a: &str, b: &str) -> String {
    a.chars().zip(b.chars()).filter(|(ac, bc)| ac == bc).map(|(ac, _)| ac).collect()
}

impl super::Day for Day2 {
    fn name(&self) -> &str { "Inventory Management System" } 

    fn part_1(&mut self, input: &str) -> String {
        let contains_two = input.lines().filter(|s| contains_exactly(s, 2)).count();
        let contains_three = input.lines().filter(|s| contains_exactly(s, 3)).count();

        (contains_two * contains_three).to_string()
    }

    fn part_2(&mut self, input: &str) -> String {
        // Also O(n^2), sad
        for a in input.lines() {
            for b in input.lines() {
                if differing_characters(a, b) == 1 {
                    // Got it
                    return equal_chars(a, b);
                }
            }
        }
        unreachable!();
    }
}