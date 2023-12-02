use std::collections::HashMap;

pub fn get_calibration_value_part2(raw: &str) -> i32 {
    let first_digit = get_first_digit(raw).unwrap();
    let second_digit = get_second_digit(raw).unwrap();
    return 10 * first_digit + second_digit;
}

fn number_map() -> HashMap<&'static str, i32> {
    return HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
}

fn get_first_digit(s: &str) -> Result<i32, &'static str>{
    let number_map = number_map();
    let mut number = String::new();
    for c in s.chars() {
        if c.is_digit(10) {
            let x = c.to_digit(10).unwrap();
            return Ok(i32::try_from(x).unwrap())
        }
        number.push(c);
        for key in number_map.keys() {
            if number.contains(key) {
                let x = number_map.get(*key).unwrap();
                return Ok(*x)
            }
        }
    }
    return Err("N/A")
}

fn get_second_digit(s: &str) -> Result<i32, &'static str>{
    let number_map = number_map();
    let mut number = String::new();
    for c in s.chars().rev() {
        if c.is_digit(10) {
            let x = c.to_digit(10).unwrap();
            return Ok(i32::try_from(x).unwrap())
        }
        number.insert(0, c);
        for key in number_map.keys() {
            if number.contains(key) {
                let x = number_map.get(*key).unwrap();
                return Ok(*x)
            }
        }
    }
    return Err("N/A")
}