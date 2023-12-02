mod part1;
mod part2;

use std::fs;

fn get_document(file: &str) {
    let document_string = fs::read_to_string(file).expect("N/A");
    let mut calibration_sum_part1 = 0;
    let mut calibration_sum_part2 = 0;
    document_string.split("\n").for_each(|s| {
        calibration_sum_part1 += part1::get_calibration_value_part1(s);
        calibration_sum_part2 += part2::get_calibration_value_part2(s)
    });

    println!("Calibration sum part 1: {calibration_sum_part1}");
    println!("Calibration sum part 2: {calibration_sum_part2}");
}


fn main() {
    get_document("input.txt")
}
