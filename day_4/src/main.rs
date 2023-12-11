use std::collections::HashSet;
use std::fs;


struct Card {
    winning_numbers: HashSet<i32>,
    numbers: HashSet<i32>,
}

impl Card {
    fn count_wins(&self) -> i32 {
        let mut count: i32 = 0;
        for winning_number in &self.winning_numbers {
            if self.numbers.contains(&winning_number) {
                count += 1;
            }
        }
        return count;
    }
}

fn get_cards(file: &str) -> Vec<Card> {
    return fs::read_to_string(file)
        .expect("file missing")
        .split("\n")
        .into_iter()
        .map(|s| -> &str {
            let mut split = s.split(":");
            split.next();
            return split.next().unwrap();
        })
        .map(|s| {
            let mut split = s.split("|");
            let winning_numbers = str_to_set(split.next().unwrap());
            let numbers = str_to_set(split.next().unwrap());
            return Card { winning_numbers, numbers };
        }).collect();
}

fn two_pow(value: i32) -> i32 {
    return if value == 0 {
        0
    } else {
        2_i32.pow((value - 1) as u32)
    };
}

fn str_to_set(s: &str) -> HashSet<i32> {
    let mut numbers = HashSet::new();
    s.split(" ")
        .for_each(|tmp| {
            if tmp != "" {
                numbers.insert(tmp.parse::<i32>().unwrap());
            }
        });
    return numbers;
}

// OBS slow but it works...
// A better solution would be to cache the number of cards won by card starting with the last card
fn get_count(pile: &[Card], is_copy: bool) -> i32 {
    let size = pile.len();
    if size == 0 {
        return 0;
    } else if size == 1 {
        return 1;
    }
    let num_wins = pile[0].count_wins();
    let mut count = 1;
    for i in 1..=num_wins {
        count += get_count(&pile[i as usize..size], true);
    }
    if !is_copy {
        count += get_count(&pile[1..size], false)
    }
    return count;
}

fn main() {
    let pile = get_cards("day_4/input.txt");
    let mut score_part_1 = 0;
    pile.iter().clone().for_each(|c| { score_part_1 += two_pow(c.count_wins()) });
    let score_part_2 = get_count(&pile[..], false);
    println!("{score_part_1}");
    println!("{score_part_2}");
}
