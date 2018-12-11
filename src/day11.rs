pub struct SumGrid {
    grid: Vec<Vec<isize>>
}

impl SumGrid {
    fn new(grid: Vec<Vec<isize>>) -> SumGrid {
        let (w, h) = (grid[0].len(), grid.len());
        let mut new_grid = vec![vec![0; w]; h];
        for y in 0..h {
            for x in 0..w {
                let get = |dx: usize, dy: usize| {
                    if x == 0 && dx > 0 { return 0 }
                    if y == 0 && dy > 0 { return 0 }
                    new_grid[y-dy][x-dx]
                };
                new_grid[y][x] = grid[y][x] + get(0, 1) + get(1, 0) - get(1, 1);
            }
        }
        SumGrid { grid: new_grid }
    }

    fn value(&self, x: usize, y: usize, w: usize, h: usize) -> isize {
        let a = if x > 0 && y > 0 { self.grid[y-1][x-1] } else { 0 };
        let b = if y > 0 { self.grid[y-1][x+w-1] } else { 0 };
        let c = if x > 0 { self.grid[y+h-1][x-1] } else { 0 };
        let d = self.grid[y+h-1][x+w-1];
        d + a - b - c
    }
}

pub fn power(x: usize, y: usize, serial: usize) -> isize {
    let rack_id = (x+1) + 10;
    let mut power = rack_id * (y+1);
    power += serial;
    power *= rack_id;
    ((power / 100) % 10) as isize - 5
}

pub fn rect(w: usize, h: usize) -> impl Iterator<Item=(usize, usize)> {
    (0..h).flat_map(move |y| (0..w).map(move |x| (x, y)))
}

pub fn find_power_rect(sg: &SumGrid, grid_size: usize, window_size: usize) -> ((usize, usize), isize) {
    rect(grid_size-(window_size-1), grid_size-(window_size-1))
        .map(|(x, y)| ((x, y), sg.value(x, y, window_size, window_size)))
        .max_by_key(|(_, power)| *power)
        .unwrap()
}

pub fn part1(input: &str) -> (String, SumGrid) {
    let serial = input.trim().parse().unwrap();

    let size = (300, 300);
    let grid: Vec<Vec<isize>> = (0..size.1).map(|y| (0..size.0).map(|x| power(x, y, serial)).collect()).collect();
    let sg = SumGrid::new(grid);

    let ((x, y), _) = find_power_rect(&sg, 300, 3);

    (format!("{},{}", x+1, y+1), sg)
}

pub fn part2(_: &str, grid: SumGrid) -> String {
    let (size, ((x, y), _)) = (1..301)
        .map(|size| (size, find_power_rect(&grid, 300, size)))
        .max_by_key(|(_, (_, power))| *power)
        .unwrap();
    format!("{},{},{}", x+1, y+1, size).to_string()
}