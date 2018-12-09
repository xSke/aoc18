use std::collections::{BTreeSet, LinkedList};

struct Circle {
    // Some say linked lists should be banished for all eternity
    // *laughs in O(1) insertion/removal from both ends*
    circle: LinkedList<usize>,
    remaining_marbles: BTreeSet<usize>,
    players: Vec<usize>,
    current_player: usize
}

impl Circle {
    fn new(players: usize, marbles: usize) -> Circle {
        Circle {
            circle: {let mut ll = LinkedList::new(); ll.push_back(0); ll},
            remaining_marbles: (1..marbles+1).collect(),
            players: vec![0; players],
            current_player: 0
        }
    }

    fn step(&mut self) -> bool {
        let next_marble = *self.remaining_marbles.iter().next().unwrap();
        self.remaining_marbles.remove(&next_marble);

        if next_marble % 23 != 0 {
            let f = self.circle.pop_front().unwrap();
            self.circle.push_back(f);

            self.circle.push_back(next_marble);
        } else {
            for _ in 0..7 {
                let b = self.circle.pop_back().unwrap();
                self.circle.push_front(b);
            }
            let removed = self.circle.pop_back().unwrap();
            
            let f = self.circle.pop_front().unwrap();
            self.circle.push_back(f);

            self.players[self.current_player] += removed;
            self.players[self.current_player] += next_marble;
        }

        self.current_player = (self.current_player + 1) % self.players.len();
        self.remaining_marbles.len() > 0
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

    let mut circle = Circle::new(players, marbles);
    while circle.step() {
    }
    (circle.winner_score().to_string(), (players, marbles))
}

pub fn part2(_: &str, (players, marbles): (usize, usize)) -> String {
    let mut circle = Circle::new(players, marbles * 100);
    while circle.step() {}
    circle.winner_score().to_string()
}