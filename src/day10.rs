struct Point {
    position: (isize, isize),
    velocity: (isize, isize)
}

impl Point {
    fn step(&mut self) {
        let ((x, y), (vx, vy)) = (self.position, self.velocity);
        self.position = (x + vx, y + vy);
    }
}

struct Points {
    points: Vec<Point>
}

impl Points {
    fn from_str(s: &str) -> Points {
        let r = regex::Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>").unwrap();
        let points = s.lines().map(|line| {
            let caps = r.captures(line).unwrap();
            Point {
                position: (caps.get(1).unwrap().as_str().parse().unwrap(), caps.get(2).unwrap().as_str().parse().unwrap()),
                velocity: (caps.get(3).unwrap().as_str().parse().unwrap(), caps.get(4).unwrap().as_str().parse().unwrap())
            }
        }).collect::<Vec<_>>();
        Points { points }
    }

    fn step(&mut self) {
        self.points.iter_mut().for_each(Point::step);
    }

    fn bounds(&self) -> ((isize, isize), (isize, isize)) {
        let x_min = self.points.iter().map(|p| p.position.0).min().unwrap();
        let y_min = self.points.iter().map(|p| p.position.1).min().unwrap();
        let x_max = self.points.iter().map(|p| p.position.0).max().unwrap();
        let y_max = self.points.iter().map(|p| p.position.1).max().unwrap();

        ((x_min, y_min), (x_max, y_max))
    }
    
    fn size(&self) -> (usize, usize) {
        let ((x_min, y_min), (x_max, y_max)) = self.bounds();
        ((x_max - x_min) as usize, (y_max - y_min) as usize)
    }
}

pub fn part1(input: &str) -> (String, usize) {
    let mut points = Points::from_str(input);
    let mut steps = 0;
    while points.size().1 > 10 {
        points.step();
        steps += 1;
    }

    let ((x_min, y_min), (x_max, y_max)) = points.bounds();
    let mut display = Vec::new();
    for _ in 0..(y_max - y_min + 2) as usize {
        display.push(vec![' '; (x_max - x_min + 1) as usize]);
    }
    for p in &points.points {
        let (x, y) = (p.position.0 - x_min, p.position.1 - y_min);
        display.get_mut(y as usize).unwrap()[x as usize] = '#';
    }
    display.insert(0, Vec::new());

    (display.join(&'\n').iter().collect(), steps)
}

pub fn part2(_: &str, steps: usize) -> String {
    steps.to_string()
}