fn are_pairs(a: char, b: char) -> bool {
    if a.to_ascii_lowercase() != b.to_ascii_lowercase() { return false; }
    a.is_ascii_uppercase() != b.is_ascii_uppercase()
}

fn reacted_length(s: &str) -> usize {
    let mut stack = Vec::with_capacity(s.len());
    for c in s.chars().filter(|c| !c.is_ascii_whitespace()) {
        if stack.len() == 0 {
            stack.push(c);
        } else {
            if are_pairs(stack[stack.len() - 1], c) {
                stack.pop();
            } else {
                stack.push(c);
            }
        }
    }
    stack.len()
}

pub fn part1(input: &str) -> (String, ()) {
    (reacted_length(input).to_string(), ())
}

pub fn part2(input: &str, _: ()) -> String {
    "abcdefghijklmnopqrstuvwxyz".chars()
        .map(|to_remove| {
            let s = input.chars().filter(|c| c.to_ascii_lowercase() != to_remove).collect::<String>();
            (to_remove, reacted_length(&s))
        })
        .min_by_key(|(_, l)| *l)
        .unwrap().1.to_string()
}