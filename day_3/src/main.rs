use std::collections::{HashMap, HashSet};
use std::fs;

fn get_input(file: &str) -> String { return fs::read_to_string(file).expect("file 404"); }

fn main() {
    let string = get_input("day_3/input.txt");
    let x_size = string.split("\n").next().unwrap().len();
    let y_size: usize = string.split("\n").into_iter().map(|_s| 1).sum();
    let chars: Vec<char> = string.replace("\n", "").chars().collect();
    let mut part_numbers = HashMap::new();

    let mut sum = 0;
    let mut gear_ratio = 0;
    let mut y = 0;
    let mut x = 0;
    while y < y_size {
        while x < x_size {
            let c = chars[x + y * y_size];
            if c.is_digit(10) {
                let res = parse_number(&chars, x, y, x_size, y_size);
                let number = res.0;
                if is_valid(&number, &chars, x as i32, y as i32, x_size as i32, y_size as i32) {
                    sum += number.parse::<i32>().expect("Failed to parse number");
                    part_numbers.insert(x + y * y_size, number);
                }
                x = res.1;
            }
            x += 1;
        }
        x = 0;
        y += 1
    }

    for i in 0..x_size * y_size {
        let c = chars[i];
        if c == '*' {
            let y = i / y_size;
            let x = i % y_size;
            let mut number_indexes: HashSet<usize> = HashSet::new();
            for j in 0..9 {
                let y_j = y - 1 + j / 3;
                let x_j = x - 1 + j % 3;
                if y_j < y_size && x_j < x_size {
                    let mut x_k = x_j;
                    while chars[x_k + y_j * y_size].is_digit(10) {
                        number_indexes.insert(x_k + y_j * y_size);
                        if x_k == 0 { break; } else { x_k -= 1; }
                    }
                }
            }
            let gears : Vec<usize> = number_indexes.into_iter()
                .filter(|k| part_numbers.contains_key(k))
                .collect();
            if gears.len() == 2 {
                let mut prod = 1;
                for p in gears {
                    prod = prod * part_numbers.get(&p).unwrap().parse::<i32>().unwrap();
                }
                gear_ratio = gear_ratio + prod;
            }
        }
    }
    println!("sum {sum}");
    println!("gear ratio {gear_ratio}");
}

fn parse_number(chars: &Vec<char>, mut x: usize, y: usize, x_size: usize, y_size: usize) -> (String, usize) {
    let mut number = String::new();
    while x < x_size {
        let c = chars[x + y * y_size];
        if c.is_digit(10) {
            number.push(c);
        } else { break; }
        x = x + 1
    }
    return (number, x - 1);
}

fn is_valid(number: &String, chars: &Vec<char>, x: i32, y: i32, x_size: i32, y_size: i32) -> bool {
    let number_size = number.len() as i32;
    let left = x - 1;
    let right = x + number_size;
    let top = y - 1;
    let bottom = y + 1;
    let top_left_is_next_to_symbol = top >= 0 && left >= 0 && chars[(left + top * y_size) as usize] != '.';
    let bottom_left_is_next_to_symbol = bottom < y_size && left >= 0 && chars[(left + bottom * y_size) as usize] != '.';
    let top_right_is_next_to_symbol = top >= 0 && right < x_size && chars[(right + top * y_size) as usize] != '.';
    let bottom_right_is_next_to_symbol = bottom < y_size && right < x_size && chars[(right + bottom * y_size) as usize] != '.';
    let mut top_row_is_next_to_symbol = false;
    if top >= 0 {
        for i in x..right {
            if chars[(i + top * y_size) as usize] != '.' {
                top_row_is_next_to_symbol = true;
                break;
            }
        }
    }
    let mut bottom_row_is_next_to_symbol = false;
    if bottom < y_size {
        for i in x..right {
            if chars[(i + bottom * y_size) as usize] != '.' {
                bottom_row_is_next_to_symbol = true;
                break;
            }
        }
    }
    let left_is_next_to_symbol = left >= 0 && chars[(left + y * y_size) as usize] != '.';
    let right_is_next_to_symbol = right < x_size && chars[(right + y * y_size) as usize] != '.';
    let is_valid = top_left_is_next_to_symbol ||
        bottom_left_is_next_to_symbol ||
        top_right_is_next_to_symbol ||
        bottom_right_is_next_to_symbol ||
        top_row_is_next_to_symbol || bottom_row_is_next_to_symbol ||
        left_is_next_to_symbol || right_is_next_to_symbol;
    return is_valid;
}
