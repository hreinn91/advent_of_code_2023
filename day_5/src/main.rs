use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::str::Split;

struct SDMap {
    name: String,
    source_destination_map: BTreeMap<usize, (usize, usize)>,
}

impl SDMap {
    fn get_destination(&self, source: usize) -> usize {
        if self.source_destination_map.contains_key(&source) {
            return self.source_destination_map.get(&source).unwrap().0;
        }
        if source < *self.source_destination_map.keys().into_iter().next().unwrap() {
            return source;
        }
        for (map_source, value) in &self.source_destination_map {
            if source < map_source + value.1 && source > *map_source {
                return value.0 + source - map_source;
            }
        }
        return source;
    }
}


fn get_lowest_location_from_file(file: &str) -> (usize, usize) {
    let binding = fs::read_to_string(file)
        .expect("file missing");
    let mut input_split = binding
        .split("\n\n");

    let seeds_raw = input_split.next().unwrap();
    let seeds_list = get_seeds_list(seeds_raw);
    let seeds_ranges = get_seeds_range(&seeds_list);

    let maps = populate_maps_vector(&mut input_split);
    let part_1_location = get_lowest_location(&seeds_list, &maps);
    let mut part_2_location = get_location(&maps, *seeds_list.get(0).unwrap());
    for range in seeds_ranges {
        for i in range.0..range.0+range.1 {
            let location = get_location(&maps, i);
            if location < part_2_location {
                part_2_location = location;
            }
        }
        println!("Done");
    }
    return (part_1_location, part_2_location);
}

fn get_lowest_location(seeds: &Vec<usize>, maps: &Vec<SDMap>) -> usize {
    let mut location = get_location(&maps, seeds[0]);
    for seed in seeds {
        let tmp_location = get_location(&maps, *seed);
        if tmp_location < location {
            location = tmp_location;
        }
    }
    location
}

fn get_seeds_range(seeds_vector: &Vec<usize>) -> Vec<(usize, usize)> {
    let mut seeds_range = Vec::new();
    let mut i = 0;
    let seed_length = seeds_vector.len();
    while i < seed_length {
        let range_start = seeds_vector[i];
        let range_length = seeds_vector[i + 1];
        seeds_range.push((range_start, range_length));
        // if i % (seed_length / 10) == 0 {
        //     println!("progress: {i}/{seed_length}")
        // }
        i += 2;
    }
    return seeds_range;
}

fn get_seeds_list(seeds_raw: &str) -> Vec<usize> {
    let seeds: Vec<usize> = seeds_raw
        .split(" ")
        .into_iter()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();
    seeds
}

fn populate_maps_vector(input_split: &mut Split<&str>) -> Vec<SDMap> {
    let mut maps: Vec<SDMap> = Vec::new();
    for next in input_split {
        maps.push(get_sd_map(next));
    }
    return maps;
}

fn get_location(maps: &Vec<SDMap>, mut source: usize) -> usize {
    for map in maps {
        source = map.get_destination(source);
    }
    return source;
}

fn get_sd_map(raw: &str) -> SDMap {
    let mut split = raw.split("\n");
    let name = split.next().unwrap().to_string();
    let mut ranges: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    split.into_iter().for_each(|s| insert_range(&mut ranges, s));
    return SDMap { name, source_destination_map: ranges };
}

fn insert_range(map: &mut BTreeMap<usize, (usize, usize)>, raw_range: &str) {
    let mut split = raw_range.split(" ");
    let destination = split.next().unwrap().parse::<usize>().unwrap();
    let source = split.next().unwrap().parse::<usize>().unwrap();
    let length = split.next().unwrap().parse::<usize>().unwrap();
    map.insert(source, (destination, length));
}


fn main() {
    let (part_1, part_2) = get_lowest_location_from_file("input.txt");
    println!("lowest location part_1 {part_1}, part_2 {part_2}");
}
