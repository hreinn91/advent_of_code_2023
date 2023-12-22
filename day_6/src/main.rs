/*
Function of distance s by holding down button for t seconds: s(t). Race is for T number of seconds.
s(t) = t*(T-t) => s'(t) = T-2*t
Setting the derivative to 0 yields global minima: 0 = T-2*t => t = T/2
We expect the boat to travel the furthest distance for t = T/2
*/

use std::fs;

fn get_distance(press_time: usize, race_time: usize) -> usize {
    return press_time * (race_time - press_time);
}


fn get_number_of_combinations(record_distance: usize, race_time: usize) -> usize {
    let mid_race_time = race_time / 2;
    let mut combinations = 0;
    let mut increment_combinations = |range: Vec<usize>| {
        for press_time in range {
            let distance = get_distance(press_time, race_time);
            if distance > record_distance {
                combinations += 1;
            } else { break; }
        }
    };
    increment_combinations((mid_race_time..race_time).into_iter().collect());
    increment_combinations((0..mid_race_time).rev().into_iter().collect());
    return combinations;
}

fn main() {
    let raw_input = fs::read_to_string("day_6/input.txt")
        .expect("file missing");
    let mut input_split = raw_input.split("\n");
    let mut parse = || -> Vec<usize> {
        input_split.next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse::<usize>().unwrap())
            .collect()
    };
    let race_times = parse();
    let race_distances = parse();
    let mut combinations = 1;
    for i in 0..race_times.len() {
        combinations = combinations * get_number_of_combinations(race_distances[i], race_times[i]);
    }
    println!("part 1: {combinations}");

    let long_race_time = race_times.into_iter().map(|s| s.to_string()).collect::<String>().parse::<usize>().unwrap();
    let long_race_distance = race_distances.into_iter().map(|s| s.to_string()).collect::<String>().parse::<usize>().unwrap();
    let part_2_combinations = get_number_of_combinations(long_race_distance, long_race_time);
    println!("part 2: {part_2_combinations}")

}
