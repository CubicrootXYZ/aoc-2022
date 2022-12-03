use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // Part 1
    let matches = part1("src/input.txt".to_owned());
    let sum: i32 = matches.iter().sum();
    println!("Summed points: {}", sum);

    // Part 2
    let matches = part2("src/input.txt".to_owned());
    let sum: i32 = matches.iter().sum();
    println!("Summed points: {}", sum);
}

fn shape_to_points(shape: &str) -> i32 {
    match shape {
        "X" | "A" => return 1,
        "Y" | "B" => return 2,
        "Z" | "C" => return 3,
        &_ => return 0,
    };
}

fn part1(filename: String) -> Vec<i32> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut matches = vec![0; 0];
    for line in reader.lines() {
        let match_ = line.unwrap();

        let shapes: Vec<&str> = match_.split(" ").collect();

        matches.push(shapes_to_points(shapes[0], shapes[1]) + shape_to_points(shapes[1]));
    }

    return matches;
}

fn part2(filename: String) -> Vec<i32> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut matches = vec![0; 0];
    for line in reader.lines() {
        let match_ = line.unwrap();

        let match_splitted: Vec<&str> = match_.split(" ").collect();
        let shape2 = shape1_and_output_to_shape2(match_splitted[0], match_splitted[1]);

        matches.push(shape_to_points(&shape2) + output_to_points(match_splitted[1]));
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

fn shape1_and_output_to_shape2(shape1: &str, output: &str) -> String {
    if output == "Y" {
        return shape1.to_owned();
    }
    if shape1 == "A" {
        if output == "X" {
            return "C".to_owned();
        }
        if output == "Z" {
            return "B".to_owned();
        }
    }
    if shape1 == "B" {
        if output == "X" {
            return "A".to_owned();
        }
        if output == "Z" {
            return "C".to_owned();
        }
    }
    if output == "X" {
        return "B".to_owned();
    }

    return "A".to_owned();
}

fn output_to_points(output: &str) -> i32 {
    match output {
        "X" => return 0,
        "Y" => return 3,
        "Z" => return 6,
        &_ => return 0,
    }
}

//Rock
//Paper
//Cissor
