use std::fs;

fn get_input(file: &str) -> String { return fs::read_to_string(file).expect("file 404"); }

fn main() {
    let string = get_input("day_3/test01.txt");
    let x_size = string.split("\n").next().unwrap().len();
    let y_size: usize = string.split("\n").into_iter().map(|s| 1).sum();
    let chars: Vec<char> = string.replace("\n", "").chars().collect();

    let mut sum = 0;
    let mut y = 0;
    let mut x = 0;
    while y < y_size {
        while x < x_size {
            let c = chars[x + y * y_size];
            if c.is_digit(10) {
                let res = parse_number(&chars, x, y, x_size, y_size);
                let number = res.0;
                if is_valid(number, &chars, x, y, x_size, y_size) {
                    sum += number.parse::<i32>().expect("Failed to parse number")
                }
                x = res.1;
            }
            x += 1;
        }
        x = 0;
        y += 1
    }

    println!("Hello, world!");
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

fn is_valid(number: String, chars: &Vec<char>, x: usize, y: usize, x_size: usize, y_size: usize) -> bool {
    let number_size = number.len();
    for
}
