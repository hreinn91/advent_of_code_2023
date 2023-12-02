
pub fn get_calibration_value_part1(raw: &str) -> u32 {
    let first_digit = get_first_digit(raw).unwrap();
    let second_digit = get_second_digit(raw).unwrap();
    return 10 * first_digit + second_digit;
}

fn get_first_digit(s: &str) -> Result<u32, &'static str>{
    for c in s.chars() {
        if c.is_digit(10) {
            let i = c.to_digit(10).unwrap();
            return Ok(i);
        }
    }
    return Err("N/A")
}

fn get_second_digit(s: &str) -> Result<u32, &'static str>{
    for c in s.chars().rev() {
        if c.is_digit(10) {
            let i = c.to_digit(10).unwrap();
            return Ok(i);
        }
    }
    return Err("N/A")
}