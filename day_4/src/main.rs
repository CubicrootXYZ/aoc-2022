use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

fn main() {
    // Part 1
    let data = parse_input("src/input.txt".to_owned());
    println!("There are {} duplicates", count_duplicates(data.clone()));

    // Part 2
    println!("There are {} overlaps", count_overlaps(data));
}

fn parse_input(filename: String) -> Vec<Vec<RangeInclusive<i32>>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut data: Vec<Vec<RangeInclusive<i32>>> = Vec::new();
    for line in reader.lines() {
        let unwrapped = line.unwrap();
        let mut section_strs = unwrapped.split(",");
        let sec1 = section_strs.next().unwrap().to_owned();
        let sec2 = section_strs.next().unwrap().to_owned();

        data.push(vec![string_to_range(sec1), string_to_range(sec2)]);
    }

    return data;
}

fn string_to_range(range: String) -> RangeInclusive<i32> {
    let mut splitted = range.split("-");
    let lower_bound = splitted.next().unwrap().parse::<i32>().unwrap();
    let upper_bound = splitted.next().unwrap().parse::<i32>().unwrap();

    return lower_bound..=upper_bound;
}

fn count_duplicates(data: Vec<Vec<RangeInclusive<i32>>>) -> i32 {
    let mut cnt = 0;
    for pair in data.iter() {
        let min_max_values = get_max_min(pair);

        if min_max_values[0] >= min_max_values[2] && min_max_values[1] <= min_max_values[3] {
            cnt += 1;
        } else if min_max_values[2] >= min_max_values[0] && min_max_values[3] <= min_max_values[1] {
            cnt += 1;
        }
    }
    return cnt;
}

fn count_overlaps(data: Vec<Vec<RangeInclusive<i32>>>) -> i32 {
    let mut cnt = 0;
    for pair in data.iter() {
        let p2_clone = pair[1].clone();

        for i in pair[0].clone().step_by(1) {
            if p2_clone.contains(&i) {
                cnt += 1;
                break;
            }
        }
    }
    return cnt;
}

// I guess this is not how this is done but I debugged way to long ...
fn get_max_min(ranges: &Vec<RangeInclusive<i32>>) -> [i32; 4] {
    return [
        ranges[0].clone().min().unwrap(),
        ranges[0].clone().max().unwrap(),
        ranges[1].clone().min().unwrap(),
        ranges[1].clone().max().unwrap(),
    ];
}
