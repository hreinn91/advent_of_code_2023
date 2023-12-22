use std::collections::HashMap;
use std::fs;

struct Documents {
    instructions: Instructions,
    network: HashMap<String, (String, String)>,
    state: String
}

impl Documents {
    fn walk_the_camel(&mut self) -> usize {
        let lr = self.instructions.next();
        let values = self.network.get(&self.state).unwrap().clone();
        let new_state = if lr == 'L' { values.0 } else { values.1 };
        return if new_state == "ZZZ" {
            self.instructions.state
        } else {
            self.state = new_state;
            self.walk_the_camel()
        }
    }
}

struct Instructions {
    instructions: Vec<char>,
    state: usize,
}

impl Instructions {
    fn next(&mut self) -> char {
        let length = self.instructions.len();
        let index = self.state % length;
        let character = self.instructions[index];
        self.state += 1;
        return character;
    }

    pub fn new(instructions: &str) -> Instructions {
        let characters: Vec<char> = instructions.chars().collect();
        return Instructions { instructions: characters, state: 0 };
    }
}

fn get_documents(file: &str) -> Documents {
    let raw = fs::read_to_string(file).expect("file not found");
    let mut split = raw.split("\n\n");
    let instructions = Instructions::new(split.next().unwrap());
    let mut network: HashMap<String, (String, String)> = HashMap::new();
    let network_raw = split.next().unwrap();
    let mut network_raw = network_raw.replace("= (", "");
    network_raw = network_raw.replace(",", "");
    network_raw = network_raw.replace(")", "");
    let network_split = network_raw.split("\n");
    for connection_raw in network_split {
        let mut whitespace = connection_raw.split_whitespace();
        let key = whitespace.next().unwrap().to_string();
        let left = whitespace.next().unwrap().to_string();
        let right = whitespace.next().unwrap().to_string();
        network.insert(key, (left, right));
    }
    return Documents { instructions, network, state: "AAA".to_string() };
}

fn main() {
    let mut documents = get_documents("day_8/input.txt");
    println!("{}", documents.walk_the_camel());
}
