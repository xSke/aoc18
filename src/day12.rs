use std::collections::HashSet;
use std::collections::HashMap;

pub fn parse_rules<'a>(lines: impl Iterator<Item=&'a str>) -> [bool; 32] {
    let mut arr = [false; 32];
    for line in lines {
        let mut chars = line.trim().chars();

        let bits = (0..5)
            .filter(|_| chars.next() == Some('#'))
            .fold(0, |a, i| a | (1 << i));

        for _ in 0..4 { chars.next(); }
        arr[bits] = chars.next() == Some('#');
    }

    arr
}

fn get_window_bits(state: &HashSet<isize>, index: isize) -> usize {
    (0..5)
        .filter(|i| state.contains(&(index-2+i)))
        .fold(0, |a, i| a | (1 << i))
}

fn step(state: HashSet<isize>, rules: &[bool; 32]) -> HashSet<isize> {
    let bounds = (state.iter().min().unwrap() - 2, state.iter().max().unwrap() + 2);
    (bounds.0..bounds.1)
        .filter(|i| rules[get_window_bits(&state, *i)])
        .collect()
}

pub fn part1(input: &str) -> (String, ([bool; 32], HashSet<isize>)) {
    let mut lines = input.lines();
    let initial_state = &lines.next().unwrap()[15..].trim();
    lines.next();
    let rules = parse_rules(lines.filter(|s| !s.is_empty()));

    let initial_state: HashSet<isize> = initial_state.chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .map(|(i, _)| i as isize)
        .collect(); 

    let mut current_state = initial_state.clone();
    for _ in 0..20 {
        current_state = step(current_state, &rules);        
    }

    (current_state.iter().sum::<isize>().to_string(), (rules, initial_state))
}

pub fn part2(_: &str, (rules, initial_state): ([bool; 32], HashSet<isize>)) -> String {
    let mut seen_states: HashMap<Vec<isize>, (usize, isize)> = HashMap::new();
    
    let mut current_state = initial_state.clone();
    let mut gen = 0;
    current_state = loop {
        gen += 1;
        current_state = step(current_state, &rules);

        let h_min = current_state.iter().min().unwrap();

        let mut aligned = current_state.iter().map(|x| x - h_min).collect::<Vec<isize>>();
        aligned.sort_unstable();

        if let Some((dupe_gen, dupe_h_min)) = seen_states.get(&aligned) {
            let gen_delta = gen - dupe_gen;
            let h_min_delta = h_min - dupe_h_min;

            let ffw_cycles = (50_000_000_000 - gen) / gen_delta;
            gen += ffw_cycles * gen_delta;
            let h_min_cycle_delta = h_min_delta * ffw_cycles as isize;

            break current_state.iter().map(|x| x + h_min_cycle_delta).collect();
        } else {
            seen_states.insert(aligned, (gen, *h_min));
        }
    };

    while gen < 50_000_000_000 {
        gen += 1;
        current_state = step(current_state, &rules);
    }

    current_state.iter().sum::<isize>().to_string()
}