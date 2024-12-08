use std::collections::HashSet;
use std::fs;

struct Map {
    row_size: i32,
    column_size: i32,
    map: Vec<Vec<char>>,
}

impl Map {
    fn clone(&self) -> Map {
        Map {
            row_size: self.row_size,
            column_size: self.column_size,
            map: self
                .map
                .iter()
                .map(|row| row.iter().cloned().collect())
                .collect(),
        }
    }
    fn new(input: &str) -> Self {
        let map: Vec<Vec<char>> = input
            .lines()
            .map(|line| {
                let vec: Vec<char> = line.chars().collect();
                vec
            })
            .collect();
        let row_size = map.len() as i32;
        let column_size = map[0].len() as i32;
        Self {
            map,
            row_size,
            column_size,
        }
    }

    fn get(&self, position: (i32, i32)) -> Option<char> {
        if position.0 < 0
            || position.0 >= self.row_size
            || position.1 < 0
            || position.1 >= self.column_size
        {
            return None;
        }
        Some(self.map[position.0 as usize][position.1 as usize])
    }

    fn find_guard(&self) -> (i32, i32) {
        for i in 0..self.row_size {
            for j in 0..self.column_size {
                if self.get((i, j)).unwrap() == '^' {
                    return (i, j);
                }
            }
        }
        unreachable!()
    }

    fn insert_block(&mut self, block_position: (i32, i32)) {
        self.map[block_position.0 as usize][block_position.1 as usize] = '#';
    }
}

struct Guard {
    position: (i32, i32),
    direction: i8,
    history: HashSet<(i32, i32, i8)>,
    is_looping: bool,
}

impl Guard {
    fn new(position: (i32, i32)) -> Self {
        Self {
            position,
            direction: 0,
            history: HashSet::new(),
            is_looping: false,
        }
    }

    fn turn(&mut self) {
        self.direction += 1;
        self.direction %= 4;
    }

    fn get_next_position(&self) -> (i32, i32) {
        match self.direction {
            0 => (self.position.0 - 1, self.position.1),
            1 => (self.position.0, self.position.1 + 1),
            2 => (self.position.0 + 1, self.position.1),
            3 => (self.position.0, self.position.1 - 1),
            _ => unreachable!(),
        }
    }

    fn step(&mut self, map: &Map) -> (i32, i32) {
        self.history
            .insert((self.position.0, self.position.1, self.direction));
        let next_position = self.get_next_position();
        let next_spot = map.get(next_position);
        if next_spot.is_some() {
            if next_spot.unwrap() == '#' {
                self.turn();
            } else {
                self.position = next_position;
            }
        }
        next_position
    }

    fn step_one(&mut self, map: &Map) -> bool {
        let next_position = self.step(map);
        map.get(next_position).is_some()
    }

    fn step_two(&mut self, map: &Map) -> bool {
        let position = self.step(map);
        self.is_looping = self
            .history
            .contains(&(position.0, position.1, self.direction));
        self.is_looping
    }

    fn get_distinct_positions(&self) -> HashSet<(i32, i32)> {
        let mut positions: HashSet<(i32, i32)> = HashSet::new();
        for record in &self.history {
            positions.insert((record.0, record.1));
        }
        positions
    }

    fn is_inside(&self, map: &Map) -> bool {
        map.get(self.get_next_position()).is_some()
    }
}

fn part1(map: &Map) -> Guard {
    let mut guard = Guard::new(map.find_guard());
    let mut is_inside_map = true;
    while is_inside_map {
        is_inside_map = guard.step_one(&map);
    }
    guard
}

fn part2(map: &Map, mut distinct_positions: HashSet<(i32, i32)>) -> i32 {
    let mut count = 0;
    let guard_start_position = map.find_guard();
    distinct_positions.remove(&guard_start_position);
    let total = distinct_positions.len();
    let mut i = 0;
    for block_position in distinct_positions {
        i += 1;
        if i % 400 == 0 {
            println!("Progress: {}/{}", i * 100 / total, total * 100 / total);
        }
        let mut cloned_map = map.clone();
        cloned_map.insert_block(block_position);
        let mut guard = Guard::new(guard_start_position);
        while !guard.is_looping && guard.is_inside(&cloned_map) {
            guard.step_two(&cloned_map);
        }
        if guard.is_looping {
            count += 1;
        }
    }
    count
}

fn main() {
    let input = get_input("test.txt");
    let map = Map::new(&input);
    let guard = part1(&map);
    let distinct_positions = guard.get_distinct_positions();
    println!("Part1: {}", distinct_positions.len());
    let part2 = part2(&map, distinct_positions);
    println!("Part2: {}", part2);
}

fn get_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to read file")
}
