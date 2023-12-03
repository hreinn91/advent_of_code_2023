pub fn calc_game(line: &str, red_cubes: i32, green_cubes: i32, blue_cubes: i32) -> i32 {
    let string = line.replace("Game ", "");
    let mut split = string.split(":");
    let x = split.next().unwrap();
    let game_number = x.parse::<i32>().unwrap();
    if are_games_possible(split.next().unwrap(), red_cubes, green_cubes, blue_cubes) {
        return game_number
    }
    return 0;
}

fn are_games_possible(games_line: &str,
                      red_cubes: i32,
                      green_cubes: i32,
                      blue_cubes: i32) -> bool {
    let games = games_line.split(";");
    for game in games.into_iter() {
        if !is_game_possible(game, red_cubes, green_cubes, blue_cubes){
            return false;
        }
    }
    return true;
}

fn is_game_possible(game: &str, red_cubes: i32, green_cubes: i32, blue_cubes: i32) -> bool {
    let binding = game.replace(" ", "");
    let revealed = binding.split(",").into_iter();
    for s in revealed {
        if s.contains("blue") {
            if s.replace("blue", "").parse::<i32>().unwrap() > blue_cubes {
                return false;
            }
        }
        if s.contains("green") {
            if s.replace("green", "").parse::<i32>().unwrap() > green_cubes {
                return false;
            }
        }
        if s.contains("red") {
            if s.replace("red", "").parse::<i32>().unwrap() > red_cubes {
                return false;
            }
        }
    }
    return true;
}