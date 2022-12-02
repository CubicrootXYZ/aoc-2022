use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let matches = parse_input("src/input.txt".to_owned());
    println!("Summed points: {}", matches.iter().sum())
}

fn shape_to_points(shape: &str) -> i32 {
    match shape {
        "X" => return 1,
        "Y" => return 2,
        "Z" => return 3,
        &_ => return 0,
    };
}

fn parse_input(filename: String) -> Vec<i32> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut matches = vec![0; 0];
    for line in reader.lines() {
        let match_ = line.unwrap();

        let shapes: Vec<&str> = match_.split(" ").collect();
        println!("{} | {}", shapes[0], shapes[1]);

        matches.push(shapes_to_points(shapes[0], shapes[1]) + shape_to_points(shapes[1]))
    }

    return matches;
}

fn shapes_to_points(shape1: &str, shape2: &str) -> i32 {
    if shape1 == "A" {
        if shape2 == "X" {
            return 3;
        }
        if shape2 == "Y" {
            return 6;
        }
        if shape2 == "Z" {
            return 0;
        }
    } else if shape1 == "B" {
        if shape2 == "X" {
            return 0;
        }
        if shape2 == "Y" {
            return 3;
        }
        if shape2 == "Z" {
            return 6;
        }
    } else if shape1 == "C" {
        if shape2 == "X" {
            return 6;
        }
        if shape2 == "Y" {
            return 0;
        }
        if shape2 == "Z" {
            return 3;
        }
    }

    return 0;
}
