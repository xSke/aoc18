use std::collections::{HashSet, VecDeque};

struct Neighbors {
    x: usize,
    y: usize,
    i: usize
}

impl Iterator for Neighbors {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        self.i += 1;
        match self.i - 1 {
            0 => Some(Point { x: self.x + 1, y: self.y }),
            1 => Some(Point { x: self.x - 1, y: self.y }),
            2 => Some(Point { x: self.x, y: self.y + 1 }),
            3 => Some(Point { x: self.x, y: self.y - 1 }),
            _ => None
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
struct Point {
    y: usize,
    x: usize
}

impl Point {
    fn neighbors(&self) -> Neighbors {
        Neighbors { x: self.x, y: self.y, i: 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Space,
    Tile
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UnitType {
    Goblin,
    Elf
}

impl UnitType {
    fn enemy(&self) -> UnitType {
        match self {
            UnitType::Goblin => UnitType::Elf,
            UnitType::Elf => UnitType::Goblin
        }
    }
}

#[derive(Debug)]
struct Unit {
    position: Point,
    ap: usize,
    hp: isize,
    ty: UnitType
}

struct Map {
    cells: Vec<Vec<Cell>>,
    units: Vec<Unit>
}

impl Map {
    fn new(input: &str, goblin_ap: usize, elf_ap: usize) -> Map {
        let mut cells = vec![];
        let mut units = vec![];
        for (y, line) in input.lines().enumerate() {
            let mut row = vec![];
            for (x, c) in line.chars().enumerate() {
                row.push(if c == '#' { Cell::Tile } else { Cell::Space });
                if c == 'E' { units.push(Unit { position: Point { x, y }, ty: UnitType::Elf, hp: 200, ap: elf_ap }); }
                if c == 'G' { units.push(Unit { position: Point { x, y }, ty: UnitType::Goblin, hp: 200, ap: goblin_ap }); }
            }
            cells.push(row);
        }
        Map { cells, units }
    }

    fn cell_at(&self, pos: Point) -> Cell {
        if pos.y >= self.cells.len() { return Cell::Tile };
        if pos.x >= self.cells[pos.y].len() { return Cell::Tile };
        self.cells[pos.y][pos.x]
    }

    fn unit_at(&self, pos: Point) -> Option<usize> {
        (0..self.units.len())
            .filter(|id| self.units[*id].position == pos)
            .filter(|id| self.units[*id].hp > 0)
            .next()
    }

    fn distance_to(&mut self, from: Point, to: Point) -> Option<usize> {
        let mut node_queue = VecDeque::new();
        node_queue.push_back((from, 0));

        let mut seen_nodes = HashSet::new();
        seen_nodes.insert(from);
        for u in self.units.iter().filter(|u| u.hp > 0) {
            seen_nodes.insert(u.position);
        }

        while let Some((current_node, current_distance)) = node_queue.pop_front() {
            if current_node == to {
                return Some(current_distance);
            }

            for p in current_node.neighbors()
                .filter(|p| self.cell_at(*p) == Cell::Space) {
                    if seen_nodes.contains(&p) { continue; }

                    node_queue.push_back((p, current_distance + 1));
                    seen_nodes.insert(p);
            }
        }

        None
    }

    fn find_closest(&mut self, from: Point, targets: HashSet<Point>) -> Option<Vec<Point>> {
        let mut node_queue = VecDeque::new();
        node_queue.push_back((from, 0));

        let mut seen_nodes = HashSet::new();
        seen_nodes.insert(from);
        for u in self.units.iter().filter(|u| u.hp > 0) {
            seen_nodes.insert(u.position);
        }

        let mut shortest_hitting_amount = None;
        let mut shortest_hitting = vec![];

        while let Some((current_node, current_distance)) = node_queue.pop_front() {
            if shortest_hitting_amount.is_some() && Some(current_distance) > shortest_hitting_amount {
                return Some(shortest_hitting);
            } 
            
            if targets.contains(&current_node) {
                shortest_hitting_amount = Some(current_distance);
                shortest_hitting.push(current_node);
            }

            for p in current_node.neighbors()
                .filter(|p| self.cell_at(*p) == Cell::Space) {
                    if seen_nodes.contains(&p) { continue; }

                    node_queue.push_back((p, current_distance + 1));
                    seen_nodes.insert(p);
            }
        }
        None
    }

    fn enemies_of(&self, ty: UnitType) -> Vec<usize> {
        (0..self.units.len())
            .filter(|id| self.units[*id].ty == ty.enemy())
            .filter(|id| self.units[*id].hp > 0)
            .collect()
    }

    fn step(&mut self) -> bool {
        // For instance, the order in which units take their turns within a round
        // is the reading order of their starting positions in that round, regardless
        // of the type of unit or whether other units have moved after the round started.        
        let mut unit_ids_sorted = (0..self.units.len()).collect::<Vec<_>>();
        unit_ids_sorted.sort_unstable_by_key(|id| self.units[*id].position);

        for unit_id in unit_ids_sorted {
            if self.units[unit_id].hp <= 0 { continue; }

            // Each unit begins its turn by identifying all possible targets (enemy units). 
            let enemies = self.enemies_of(self.units[unit_id].ty);
            // If no targets remain, combat ends.
            if enemies.len() == 0 { return false; }

            // If the unit is already in range of a target, it does not move, but continues its turn with an attack.
            if self.do_attack(unit_id) { continue; }

            // Otherwise, since it is not in range of a target, it moves.
            
            // Then, the unit identifies all of the open squares (.) that are in range of each target;
            let target_squares = enemies
                .iter()
                // these are the squares which are adjacent (immediately up, down, left, or right) to any target
                .flat_map(|id| self.units[*id].position.neighbors())
                // and which aren't already occupied by a wall
                .filter(|p| self.cell_at(*p) == Cell::Space)
                // or another unit.
                .filter(|p| self.unit_at(*p).is_none())
                .collect::<HashSet<_>>();

            // If ... there are no open squares which are in range of a target, the unit ends its turn.
            if target_squares.len() == 0 { continue; }

            let closest_squares = self.find_closest(self.units[unit_id].position, target_squares);
            if closest_squares.is_none() { continue; };
            let chosen_square = closest_squares.unwrap().into_iter().min().unwrap();

            // The unit then takes a single step toward the chosen square along the shortest path to that square.
            let free_neighbors = self.units[unit_id].position.neighbors()
                .filter(|p| self.cell_at(*p) == Cell::Space)
                .filter(|p| self.unit_at(*p).is_none())
                .collect::<Vec<_>>();

            let move_to_square = free_neighbors.into_iter()
                .flat_map(|p| self.distance_to(p, chosen_square).map(|d| (p, d)))
                // If multiple steps would put the unit equally closer to its destination, the unit chooses the step which is first in reading order.
                .min_by(|(p1, d1), (p2, d2)| d1.cmp(d2).then(p1.cmp(p2)));

            self.units[unit_id].position = move_to_square.unwrap().0;

            // After moving ..., the unit attacks.
            self.do_attack(unit_id);
        }

        //self.units.retain(|u| u.hp > 0);

        true
    }

    fn do_attack(&mut self, unit_id: usize) -> bool {
        // To attack, the unit first determines all of the targets that are in range of it by being immediately adjacent to it.
        let adjacant_enemies = self.units[unit_id].position.neighbors()
            .flat_map(|p| self.unit_at(p))
            .filter(|id| self.units[*id].ty == self.units[unit_id].ty.enemy())
            .collect::<Vec<_>>();
        
        // If there are no such targets, the unit ends its turn.
        if adjacant_enemies.len() == 0 { return false; }

        // Otherwise, the adjacent target with the fewest hit points is selected
        let target = adjacant_enemies.into_iter()
            .min_by(|i1, i2| self.units[*i1].hp.cmp(&self.units[*i2].hp)
                // in a tie, the adjacent target with the fewest hit points which is first in reading order is selected.
                .then(self.units[*i1].position.cmp(&self.units[*i2].position)))
            .unwrap();

        // The unit deals damage equal to its attack power to the selected target, reducing its hit points by that amount.
        self.units[target].hp -= self.units[unit_id].ap as isize;
        true
    }

    fn total_hp(&self) -> usize {
        self.units.iter()
            .map(|u| u.hp)
            .filter(|hp| *hp > 0)
            .sum::<isize>() as usize
    }

    fn is_any_dead(&self, ty: UnitType) -> bool {
        self.units.iter().filter(|u| u.ty == ty).any(|u| u.hp <= 0)
    }
}

pub fn part1(input: &str) -> (String, ()) {
    let mut map = Map::new(input, 3, 3);

    let mut rounds = 0;
    while map.step() {
        rounds += 1;
    }

    let outcome = rounds * map.total_hp();
    (outcome.to_string(), ())
}

pub fn part2(input: &str, _: ()) -> String {
    let mut power_level = 4;

    loop {
        let mut map = Map::new(input, 3, power_level);
        let mut rounds = 0;
        while map.step() && !map.is_any_dead(UnitType::Elf) {
            rounds += 1;
        }

        if !map.is_any_dead(UnitType::Elf) {
            return (rounds * map.total_hp()).to_string();
        }

        power_level += 1;
    }
}