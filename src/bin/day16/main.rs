use aoc_2024::coord::Coord;
use std::collections::{HashSet, HashMap, BinaryHeap};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct ElfState {
    location: Coord,
    direction: Coord
}

#[derive(Debug, Copy, Clone)]
struct ElfStateWithScore {
    state: ElfState,
    score: u64
}

impl Ord for ElfStateWithScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

impl PartialOrd for ElfStateWithScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ElfStateWithScore {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for ElfStateWithScore {}
    
fn best_path(start_coord: Coord, end_coord: Coord, tiles: HashSet<Coord>) -> u64 {
    let start_state = ElfState {
        location: start_coord,
        direction: Coord(1, 0)
    };
    let mut queue: BinaryHeap<ElfStateWithScore> = BinaryHeap::new();
    let mut score_map: HashMap<ElfState, u64> = HashMap::new();
    queue.push(ElfStateWithScore { state: start_state, score: 0 });

    let mut best_score = u64::MAX;

    while let Some(ElfStateWithScore { state, score }) = queue.pop() {
        if score_map.contains_key(&state) {
            continue;
        }
        let ElfState { location, direction } = state;
        let left_state = ElfState { location, direction: direction.rotate_left() };
        let right_state = ElfState { location, direction: direction.rotate_right() };
        if !score_map.contains_key(&left_state) {
            queue.push(ElfStateWithScore { score: score + 1000, state: left_state });
        }
        if !score_map.contains_key(&right_state) {
            queue.push(ElfStateWithScore { score: score + 1000, state: right_state });
        }
        let forward_location = location + direction;
        let forward_state = ElfState { location: forward_location, direction };
        if tiles.contains(&forward_location) && !score_map.contains_key(&forward_state) {
            queue.push(ElfStateWithScore { score: score + 1, state: forward_state });
        }
        score_map.insert(state, score);
        if location == end_coord {
            best_score = std::cmp::min(score, best_score);
        }
    }

    best_score
}

fn part_1(input: &str) -> u64 {
    let mut start_coord = Coord(0, 0);
    let mut end_coord = Coord(0, 0);
    let mut tiles: HashSet<Coord> = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                tiles.insert(Coord(x as i64, y as i64));
            } else if c == 'S' {
                let coord = Coord(x as i64, y as i64);
                start_coord = coord;
                tiles.insert(coord);
            } else if c == 'E' {
                let coord = Coord(x as i64, y as i64);
                end_coord = coord;
                tiles.insert(coord);
            }
        }
    }

    return best_path(start_coord, end_coord, tiles);
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("Part 1: {}", part_1(&input));
    //println!("Part 2: {}", part_2(&input));
}
