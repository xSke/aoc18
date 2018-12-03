use std::collections::HashMap;

fn parse(s: &str) -> (usize, (usize, usize), (usize, usize)) {
    let r = regex::Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    let caps = r.captures(s).unwrap();

    (caps.get(1).unwrap().as_str().parse().unwrap(),
        (caps.get(2).unwrap().as_str().parse().unwrap(), caps.get(3).unwrap().as_str().parse().unwrap()),
        (caps.get(4).unwrap().as_str().parse().unwrap(), caps.get(5).unwrap().as_str().parse().unwrap())
    )
}

fn has_other_claims(x: usize, y: usize, w: usize, h: usize, claims: &HashMap<(usize, usize), usize>) -> bool {
    for xx in x..(x+w) {
        for yy in y..(y+h) {
            let count = *claims.get(&(xx, yy)).unwrap_or(&0);
            if count != 1 {
                return true;
            }
        }
    }
    return false;
}

pub fn part1(input: &str) -> (String, HashMap<(usize, usize), usize>) {
    let mut claims = HashMap::new();

    let mut max_x = 0;
    let mut max_y = 0;

    for claim in input.lines() {
        let (_, (x, y), (w, h)) = parse(claim);

        for xx in x..(x+w) {
            for yy in y..(y+h) {
                claims.entry((xx, yy))
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
            }
        }

        if x + w > max_x { max_x = x + w }
        if y + h > max_y { max_y = y + h }
    }

    let mut total = 0;
    for xx in 0..max_x {
        for yy in 0..max_y {
            let count = *claims.get(&(xx, yy)).unwrap_or(&0);
            if count > 1 { total += 1 }
        }
    }

    (total.to_string(), claims)
}

pub fn part2(input: &str, claims: HashMap<(usize, usize), usize>) -> String {
    for claim in input.lines() {
        let (id, (x, y), (w, h)) = parse(claim);
        if !has_other_claims(x, y, w, h, &claims) {
            return id.to_string();
        }
    }
    unreachable!()
}