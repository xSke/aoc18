fn step(elf1: &mut usize, elf2: &mut usize, scores: &mut Vec<u8>) {
    let new_score = scores[*elf1] + scores[*elf2];
    if new_score > 9 {
        scores.push(new_score / 10);
        scores.push(new_score % 10);
    } else {
        scores.push(new_score);
    }

    *elf1 = (*elf1 + 1 + scores[*elf1] as usize) % scores.len();
    *elf2 = (*elf2 + 1 + scores[*elf2] as usize) % scores.len();
}

pub fn part1(input: &str) -> (String, ()) {
    let mut elf1 = 0;
    let mut elf2 = 1;

    let mut scores = vec![3, 7];

    let count = input.trim().parse::<usize>().unwrap();
    while scores.len() < count + 10 {
        step(&mut elf1, &mut elf2, &mut scores);
    }
    (scores[count..count+10].iter().map(|x| x.to_string()).collect(), ())
}

pub fn part2(input: &str, _: ()) -> String {
    let mut elf1 = 0;
    let mut elf2 = 1;
    
    let mut scores = vec![3, 7];

    let input_chars = input.trim()
        .chars()
        .map(|x| x.to_string().parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    loop {
        step(&mut elf1, &mut elf2, &mut scores);

        if scores.len() >= input_chars.len() {
            if &scores[scores.len() - input_chars.len()..] == &input_chars[..] {
                break;
            }
        }

        if scores.len() >= input_chars.len() + 1 {
            if &scores[scores.len() - input_chars.len() - 1..scores.len() - 1] == &input_chars[..] {
                scores.pop();
                break;
            }
        }
    }

    (scores.len() - input_chars.len()).to_string()
}