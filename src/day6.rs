use std::collections::HashMap;

type Out = ((usize, usize), (usize, usize), Vec<(usize, usize)>);

pub fn manhattan(a: (usize, usize), b: (usize, usize)) -> usize {
    ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
}

pub fn closest_point(point: (usize, usize), points: &[(usize, usize)]) -> Option<usize> {
    let mut distances = points.iter().enumerate().map(|(i, p)| (i, manhattan(point, *p))).collect::<Vec<_>>();
    distances.sort_unstable_by_key(|(_, d)| *d);

    let (shortest_idx, shortest) = distances[0];
    let (_, next_shortest) = distances[1];

    if shortest == next_shortest {
        None
    } else {
        Some(shortest_idx)
    }
}

pub fn part1(input: &str) -> (String, Out) {
    let points = input
        .lines()
        .map(|line| {let mut split = line.split(", "); (split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap())})
        .collect::<Vec<(usize, usize)>>();

    let min_x = *points.iter().map(|(x, _)| x).min().unwrap() - 1;
    let max_x = *points.iter().map(|(x, _)| x).max().unwrap() + 2;
    let min_y = *points.iter().map(|(_, y)| y).min().unwrap() - 1;
    let max_y = *points.iter().map(|(_, y)| y).max().unwrap() + 1;

    let mut map = HashMap::new();
    for x in min_x..max_x {
        for y in min_y..max_y {
            if let Some(point) = closest_point((x, y), &points) {
                map.insert((x, y), point);
            }
        }
    }

    let total = {
        let mut areas = HashMap::new();
        for x in min_x..max_x {
            for y in min_y..max_y {
                if let Some(closest) = map.get(&(x, y)) {
                    areas.entry(closest)
                        .and_modify(|x| *x += 1)
                        .or_insert(1);
                }
            }
        }

        for y in min_y..max_y {
            if let Some(closest) = map.get(&(min_x, y)) {
                areas.remove(closest);
            }

            if let Some(closest) = map.get(&(max_x - 1, y)) {
                areas.remove(closest);
            }
        }

        for x in min_x..max_x {
            if let Some(closest) = map.get(&(x, min_y)) {
                areas.remove(closest);
            }

            if let Some(closest) = map.get(&(x, max_y - 1)) {
                areas.remove(closest);
            }
        }

        *areas.values().max().unwrap()
    };

    (total.to_string(), ((min_x, max_x), (min_y, max_y), points))
}

pub fn part2(_: &str, out: Out) -> String {
    let ((min_x, max_x), (min_y, max_y), points) = out;

    let mut size = 0;
    for x in min_x..max_x {
        for y in min_y..max_y {
            let total_dst: usize = points.iter().map(|p| manhattan((x, y), *p)).sum();
            if total_dst < 10000 {
                size += 1;
            }
        }
    }
    size.to_string()
}