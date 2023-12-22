use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Values;
use std::fs;

fn main() {
    let mut hands = get_hands("day_7/input.txt");
    hands.sort_by(comparator_part_1());
    let part_1 = get_total_winnings(&hands);
    hands.sort_by(comparator_part_2());
    let part_2 = get_total_winnings(&hands);
    for hand in hands { println!("{}", hand.0) }
    println!("part 1 {part_1}");
    println!("part 2 {part_2}");
}

fn comparator_part_1() -> fn(&(String, usize), &(String, usize)) -> Ordering {
    |a, b| get_hand_score(&a.0).cmp(&get_hand_score(&b.0))
}

fn comparator_part_2() -> fn(&(String, usize), &(String, usize)) -> Ordering {
    |a, b| get_hand_score_part_2(&a.0).cmp(&get_hand_score_part_2(&b.0))
}

fn get_total_winnings(hands: &Vec<(String, usize)>) -> usize {
    let mut sum = 0;
    for i in 1..hands.len() + 1 {
        sum += hands[i - 1].1 * i
    }
    return sum;
}

fn get_hand_score(hand: &String) -> usize {
    let set: HashSet<char> = hand.chars().into_iter().collect();
    let sum = get_high_card_score(hand, get_card_score);
    // One pair
    if set.len() == 4 {
        return sum + 10000000;
    }
    // five of a kind
    if set.len() == 1 {
        return sum + 60000000;
    }
    let chars: Vec<char> = hand.chars().collect();
    let first_card = chars[0];
    let first_card_count = chars.into_iter().filter(|c| c == &first_card).count();
    if set.len() == 2 {
        // four of a kind or full house
        return sum + if first_card_count == 1 || first_card_count == 4 {
            50000000
        } else {
            40000000
        };
    }
    if set.len() == 3 {
        let second_card = hand.chars().into_iter().find(|c| c != &first_card).unwrap();
        let second_card_count = hand.chars().into_iter().filter(|c| c == &second_card).count();
        // two pair or three of a kind
        return sum + if first_card_count == 2 || second_card_count == 2 {
            20000000
        } else {
            30000000
        };
    }
    return sum;
}

fn get_hand_score_part_2(hand: &String) -> usize {
    let mut card_map: HashMap<char, usize> = HashMap::new();
    let sum = get_high_card_score(hand, get_card_score_part_2);
    let number_of_jokers = hand.chars().into_iter().filter(|c| c == &'J').count();
    for c in hand.chars() {
        if c != 'J' {
            if card_map.contains_key(&c) {
                let value = card_map.get(&c).unwrap();
                card_map.insert(c, *value + 1);
            } else {
                card_map.insert(c, 1);
            }
        }
    }
    return sum + if card_map.values().any(|c| *c + number_of_jokers == 5) || number_of_jokers == 5 {
        600000000
    } else if card_map.values().any(|c| *c + number_of_jokers == 4) {
        500000000
    } else if has_full_house(card_map.values(), number_of_jokers) {
        400000000
    } else if card_map.values().any(|c| *c + number_of_jokers == 3) {
        300000000
    } else if card_map.values().any(|c| *c + number_of_jokers == 2) {
        if number_of_jokers == 0 && card_map.values()
            .filter(|v| **v == 2)
            .count() >= 2 {
            200000000
        } else {
            100000000
        }
    } else {
        0
    };
}

fn has_full_house(values: Values<char, usize>, joker_count: usize) -> bool {
    let mut two_count = 0;
    let mut three_count = 0;
    for count in values {
        if *count == 2 { two_count += 1; }
        if *count == 3 { three_count += 1; }
    }
    return (two_count == 1 && three_count == 1) || (two_count == 2 && joker_count > 0);
}
// JJAA2

fn get_high_card_score(hand: &String, card_score_function: fn(char) -> usize) -> usize {
    let mut exponent: i32 = 1;
    let mut sum: i32 = 0;
    for card in hand.chars().into_iter().rev() {
        let card_score = card_score_function(card) as i32;
        sum += card_score * exponent;
        exponent = exponent * 20;
    }
    sum as usize
}

// A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
fn get_card_score(card: char) -> usize {
    return match card {
        'T' => 9 + 1,
        'J' => 9 + 2,
        'Q' => 9 + 3,
        'K' => 9 + 4,
        'A' => 9 + 5,
        _ => card as usize - 49,
    };
}

fn get_card_score_part_2(card: char) -> usize {
    return match card {
        'T' => 10 + 1,
        'J' => 1,
        'Q' => 10 + 2,
        'K' => 10 + 3,
        'A' => 10 + 4,
        _ => card as usize - 48,
    };
}

fn get_hands(file: &str) -> Vec<(String, usize)> {
    return fs::read_to_string(file).expect("File not found")
        .split("\n")
        .map(|s| {
            let mut split = s.split_whitespace();
            return (split.next().unwrap().to_string(), split.next().unwrap().parse::<usize>().unwrap());
        })
        .collect();
}
