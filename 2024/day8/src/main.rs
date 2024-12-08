use std::collections::{HashMap, HashSet};
use std::fs;

struct Map {
    row_size: usize,
    column_size: usize,
    map: Vec<Vec<char>>,
    antennas: HashMap<char, Vec<(i32, i32)>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        let row_size = map.len();
        let column_size = map[0].len();

        let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        for i in 0..row_size {
            for j in 0..column_size {
                if map[i][j].is_ascii_alphanumeric() {
                    antennas
                        .entry(map[i][j])
                        .or_insert(Vec::new())
                        .push((i as i32, j as i32));
                }
            }
        }
        Self {
            map,
            row_size,
            column_size,
            antennas,
        }
    }

    fn print(&self) {
        for row in &self.map {
            for column_element in row {
                print!("{}", column_element);
            }
            println!();
        }
    }

    fn insert(&mut self, row: i32, col: i32, element: char) {
        if row >= 0 && row < self.row_size as i32 && col >= 0 && col < self.column_size as i32 {
            self.map[row as usize][col as usize] = element;
        }
    }

    fn get_antinodes(&self, p1: (i32, i32), p2: (i32, i32)) -> Vec<(i32, i32)> {
        let di: i32 = p1.0 - p2.0;
        let dj: i32 = p1.1 - p2.1;
        let f1 = (p1.0 + di, p1.1 + dj);
        let f2 = (p2.0 - di, p2.1 - dj);
        vec![f1, f2]
    }
}

fn main() {
    let input = include_str!("../test.txt");
    println!("Hello, world!");
}

// Unit tests
#[cfg(test)] // This ensures the tests are only compiled and run during testing
mod tests {
    use super::*; // Import everything from the outer scope

    #[test]
    fn simple_map() {
        let input = include_str!("../test.txt");
        let mut map = Map::new(input);
        assert_eq!(map.antennas[&'a'].len(), 2);
        let antinodes = map.get_antinodes((3, 4), (5, 5));
        assert_eq!(antinodes, vec![(1, 3), (7, 6)]);
        for antinode in antinodes {
            map.insert(antinode.0, antinode.1, '#');
        }
        map.print();
    }

    #[test]
    fn simple_map_2() {
        let input = include_str!("../test2.txt");
        let mut map = Map::new(input);
        let aantennas = map.antennas[&'a'].clone();
        assert_eq!(aantennas.len(), 3);
        for i in 0..aantennas.len() { // Here we go :)
            for j in i + 1..aantennas.len() {
                let aantenna_1 = aantennas[i];
                let aantenna_2 = aantennas[j];
                for aantinode in map.get_antinodes(aantenna_1, aantenna_2) {
                    map.insert(aantinode.0, aantinode.1, '#');
                }
            }
        }
        map.print();
    }
}

fn get_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to read file")
}
