
pub fn calc_game(line: &str) -> i32 {
    let string = line.replace("Game ", "");
    let mut split = string.split(":");
    let x = split.next().unwrap();
    return get_product(split.next().unwrap());
}

struct Cubes {
    red: i32,
    green: i32,
    blue: i32,
}
fn get_product(games_line: &str) -> i32 {
    let games = games_line.split(";");
    let mut cubes = Cubes { red: 0, green: 0, blue: 0 };
    for game in games.into_iter() {
        cubes = update_cube_values(game, cubes);
    }
    return cubes.red * cubes.green * cubes.blue;
}

fn update_cube_values(game: &str, mut cubes: Cubes) -> Cubes {
    let binding = game.replace(" ", "");
    let revealed = binding.split(",").into_iter();
    for s in revealed {
        if s.contains("blue") {
            let revealed_blue = s.replace("blue", "").parse::<i32>().unwrap();
            if revealed_blue > cubes.blue {
                cubes.blue = revealed_blue;
            }
        }
        if s.contains("green") {
            let revealed_green = s.replace("green", "").parse::<i32>().unwrap();
            if revealed_green > cubes.green {
                cubes.green = revealed_green;
            }
        }
        if s.contains("red") {
            let revealed_red = s.replace("red", "").parse::<i32>().unwrap();
            if revealed_red > cubes.red {
                cubes.red = revealed_red;
            }
        }
    }
    return cubes;
}