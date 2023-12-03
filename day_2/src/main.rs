mod part_1;
mod part_2;

use std::fs;

fn get_input(file: &str) -> String { return fs::read_to_string(file).expect("file 404") }

fn calc_games_from_input(file: &str,
                         red_cubes: i32,
                         green_cubes: i32,
                         blue_cubes: i32) {
    let raw = get_input(file);
    let mut part_1_sum = 0;
    let mut part_2_sum = 0;
    for line in raw.split("\n") {
        part_1_sum += part_1::calc_game(line, red_cubes, green_cubes, blue_cubes);
        part_2_sum += part_2::calc_game(line);
    }

    println!("Part 1: {part_1_sum}");
    println!("Part 2: {part_2_sum}");
}

fn main() {
    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes= 14;
    calc_games_from_input("day_2/input.txt", red_cubes, green_cubes, blue_cubes);
}
