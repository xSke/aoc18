use std::collections::VecDeque;

struct Circle {
    // oh. Vecs are still faster. who knew?
    circle: VecDeque<usize>,
    players: Vec<usize>,
    current_player: usize,
    next_marble: usize
}

impl Circle {
    fn new(players: usize) -> Circle {
        Circle {
            circle: {let mut ll = VecDeque::new(); ll.push_back(0); ll},
            players: vec![0; players],
            current_player: 0,
            next_marble: 1
        }
    }

    fn step(&mut self) {
        if self.next_marble % 23 != 0 {
            let f = self.circle.pop_front().unwrap();
            self.circle.push_back(f);

            self.circle.push_back(self.next_marble);
        } else {
            for _ in 0..7 {
                let b = self.circle.pop_back().unwrap();
                self.circle.push_front(b);
            }
            let removed = self.circle.pop_back().unwrap();
            
            let f = self.circle.pop_front().unwrap();
            self.circle.push_back(f);

            self.players[self.current_player] += removed;
            self.players[self.current_player] += self.next_marble;
        }

        self.current_player = (self.current_player + 1) % self.players.len();
        self.next_marble += 1
    }

    fn winner_score(&self) -> usize {
        *self.players.iter().max().unwrap()
    }
}

pub fn part1(input: &str) -> (String, (usize, usize)) {
    let r = regex::Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    let caps = r.captures(input).unwrap();
    let (players, marbles) = (
        caps.get(1).unwrap().as_str().parse().unwrap(),
        caps.get(2).unwrap().as_str().parse().unwrap()
    );

    let mut circle = Circle::new(players);
    for _ in 0..marbles { circle.step() } 
    (circle.winner_score().to_string(), (players, marbles))
}

pub fn part2(_: &str, (players, marbles): (usize, usize)) -> String {
    let mut circle = Circle::new(players);
    for _ in 0..marbles*100 { circle.step() } 
    circle.winner_score().to_string()
}