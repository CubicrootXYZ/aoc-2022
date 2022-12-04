use std::{
    fs::File,
    io::{BufRead, BufReader},
};

static PRIORITIES: [&str; 26] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z",
];

fn main() {
    // This all would be way more efficient if done with raw strings and some array/set magic, but I want to try out the OOP features of rust.

    // Part 1
    let mut rucksacks = file_to_rucksacks("src/input.txt".to_owned());
    let mut prio_sum: i32 = 0;
    for rucksack in rucksacks.iter_mut() {
        for item in rucksack.get_duplicates().iter() {
            prio_sum += item_to_priority(*item)
        }
    }
    println!("Summed prio of duplicate items: {}", prio_sum);

    // Part 2
    let mut rucksacks = file_to_rucksacks("src/input.txt".to_owned());

    let mut badges_sum = 0;
    for i in (0..rucksacks.len() - 1).step_by(3) {
        let group_rucksacks = &mut rucksacks[i..i + 3];

        let items1 = group_rucksacks[1].get_items();
        let items2 = group_rucksacks[2].get_items();

        for item in group_rucksacks[0].get_items().iter() {
            if items1.contains(item) && items2.contains(item) {
                badges_sum += item_to_priority(item);
                break;
            }
        }
    }
    println!("Summed badges: {}", badges_sum);
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
        let mut checked: Vec<String> = Vec::new();

        for item in self.compartment1.iter() {
            if checked.contains(item) {
                continue;
            }
            if self.compartment2.contains(item) {
                duplicates.push(item);
            }
            checked.push(item.to_owned());
        }

        return duplicates;
    }

    pub fn get_items(&mut self) -> Vec<String> {
        let mut items: Vec<String> = Vec::new();
        items.append(&mut self.compartment1);
        items.append(&mut self.compartment2);
        return items;
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
