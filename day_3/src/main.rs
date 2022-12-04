use std::{
    fs::File,
    io::{BufRead, BufReader},
};

static PRIORITIES: [&str; 26] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z",
];

fn main() {
    // Part 1
    // Somehow the result is wrong and I do not understand why, with the example data it works fine
    let mut rucksacks = file_to_rucksacks("src/input.txt".to_owned());
    let mut types: Vec<&str> = Vec::new();
    for rucksack in rucksacks.iter_mut() {
        for item in rucksack.get_duplicates().iter() {
            if !types.contains(item) {
                types.push(*item);
            }
        }
    }
    let mut prio_sum: i32 = 0;
    for t in types.iter() {
        println!("{} - {}", *t, item_to_priority(*t));
        prio_sum += item_to_priority(*t)
    }

    println!("Summed prio of duplicate items: {}", prio_sum);
}

fn file_to_rucksacks(filename: String) -> Vec<Rucksack> {
    let mut rucksacks: Vec<Rucksack> = Vec::new();

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut rucksack: Rucksack = Rucksack {
            compartment1: Vec::new(),
            compartment2: Vec::new(),
        };
        rucksack.set_items(line.unwrap());

        rucksacks.push(rucksack);
    }

    return rucksacks;
}

pub struct Rucksack {
    compartment1: Vec<String>,
    compartment2: Vec<String>,
}

impl Rucksack {
    pub fn set_items(&mut self, items: String) {
        self.compartment1 = Vec::new();
        self.compartment2 = Vec::new();

        let items_cnt = items.len();
        for (i, c) in items.chars().enumerate() {
            if i < items_cnt / 2 {
                self.compartment1.push(c.to_string())
            } else {
                self.compartment2.push(c.to_string())
            }
        }
    }

    pub fn get_duplicates(&mut self) -> Vec<&str> {
        let mut duplicates: Vec<&str> = Vec::new();

        for item in self.compartment1.iter() {
            if self.compartment2.contains(item) {
                duplicates.push(item);
            }
        }

        return duplicates;
    }
}

fn item_to_priority(item: &str) -> i32 {
    let mut prio = PRIORITIES
        .iter()
        .position(|&x| x == item.to_lowercase())
        .unwrap();

    if item.to_lowercase() != item {
        prio += 26
    }

    return (prio + 1).try_into().unwrap();
}
