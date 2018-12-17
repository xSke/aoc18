use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
struct Cart {
    id: usize,
    position: (usize, usize),
    direction: (isize, isize),
    step: usize
}

impl Cart {
    fn new(id: usize, x: usize, y: usize, direction: (isize, isize)) -> Cart {
        Cart {
            id,
            position: (x, y),
            direction,
            step: 0
        }
    }

    fn step(&mut self) {
        self.position.0 = (self.position.0 as isize + self.direction.0) as usize;
        self.position.1 = (self.position.1 as isize + self.direction.1) as usize;
    }

    fn flip_maybe(&mut self, track: char) {
        self.direction = match (track, self.direction) {
            ('/', (x, y)) => (-y, -x),
            ('\\', (x, y)) => (y, x),
            ('+', (x, y)) => {
                let d = match self.step % 3 {
                    0 => (y, -x),
                    1 => (x, y),
                    2 => (-y, x),
                    _ => unreachable!()
                };
                self.step += 1;
                d
            },
            (_, d) => d
        }
    }
}

struct Track {
    map: Vec<Vec<char>>,
    carts: Vec<Cart>
}

impl Track {
    fn new(input: &str) -> Track {
        let map: Vec<Vec<char>> = input.lines()
            .map(|l| l.chars().collect())
            .collect();

        let mut carts = Vec::new();
            map.iter().enumerate()
                .for_each(|(y, line)| line.iter().enumerate()
                    .for_each(|(x, c)| {
                        let cart = match *c {
                            'v' => Some(Cart::new(carts.len(), x, y, (0, 1))),
                            '^' => Some(Cart::new(carts.len(), x, y, (0, -1))),
                            '<' => Some(Cart::new(carts.len(), x, y, (-1, 0))),
                            '>' => Some(Cart::new(carts.len(), x, y, (1, 0))),
                            _ => None
                        };

                        carts.extend(cart.into_iter());
                    }));

        Track { map, carts }
    }

    fn step(&mut self) -> Vec<(usize, usize)> {
        self.carts.sort_unstable_by_key(|c| (c.position.1, c.position.0));

        let mut collisions = Vec::new();
        let mut dead_carts = HashSet::new();
        for i in 0..self.carts.len() {
            if dead_carts.contains(&self.carts[i].id) { continue; }

            let cart_mut = self.carts.get_mut(i).unwrap();
            cart_mut.step();
            cart_mut.flip_maybe(self.map[cart_mut.position.1][cart_mut.position.0]);

            if let Some(other) = self.carts.iter()
                .filter(|c| c.position == self.carts[i].position && c.id != self.carts[i].id && !dead_carts.contains(&c.id))
                .next() {
                    collisions.push(other.position);
                    dead_carts.insert(other.id);
                    dead_carts.insert(self.carts[i].id);
            }
        }

        self.carts.retain(|c| !dead_carts.contains(&c.id));

        collisions
    }
}

pub fn part1(input: &str) -> (String, ()) {
    let mut track = Track::new(input);
    
    loop {
        let collisions = track.step();
        if collisions.len() > 0 {
            return (format!("{},{}", collisions[0].0, collisions[0].1), ());
        }
    }
}

pub fn part2(input: &str, _: ()) -> String {    
    let mut track = Track::new(input);
    
    while track.carts.len() != 1 {
        track.step();
    }

    let last_cart = &track.carts[0];
    format!("{},{}", last_cart.position.0, last_cart.position.1)
}