use std::fs;

fn calc_part1(total: i64, left: i64, mut rest: Vec<i64>) -> bool {
    if total - left == 0 && rest.is_empty() {
        return true;
    }
    if rest.len() == 0 {
        return false;
    }
    let right = rest.remove(0);

    calc_part1(total, left + right, rest.clone())
    || calc_part1(total, left * right, rest.clone())
}

fn calc_part2(total: i64, left: i64, mut rest: Vec<i64>) -> bool {
    if total - left == 0 && rest.is_empty() {
        return true;
    }
    if rest.len() == 0 {
        return false;
    }
    let right = rest.remove(0);

    calc_part2(total, left + right, rest.clone())
    || calc_part2(total, left * right, rest.clone())
    || calc_part2(total, format!("{}{}", left, right).parse::<i64>().unwrap(),
                  rest.clone())
}

fn get_equation(line: &str) -> (i64, Vec<i64>) {
    let mut split = line.split(":");
    let total = split.next().unwrap().parse::<i64>().unwrap();
    let values = split.next().unwrap()
        .split_whitespace()
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    (total, values)
}

fn main() {
    let input = get_input("test.txt");
    let mut sum_part1 = 0;
    let mut sum_part2 = 0;
    input.lines().for_each(|line| {
        let equation = get_equation(line);
        let total = equation.0;
        let mut values = equation.1;
        let mut values_values = values.clone();
        if calc_part1(total, values.remove(0), values) {
            sum_part1 += equation.0
        }
        if calc_part2(total, values_values.remove(0), values_values) {
            sum_part2 += equation.0
        }
    });
    println!("Part 1: {}", sum_part1);
    println!("Part 2: {}", sum_part2);
}


fn get_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to read file")
}

