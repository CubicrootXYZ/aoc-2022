use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let calories = parse_input("src/input.txt".to_owned());

    // Part 1
    println!(
        "Top raindeer calories: {}",
        get_max_calories_for_reindeer(&calories)
    );

    // Part 2
    println!(
        "Top 3 raindeer calories: {}",
        get_top3_calories_for_reindeers(&calories)
    );

    Ok(())
}

fn get_max_calories_for_reindeer(calories: &Vec<Vec<i32>>) -> i32 {
    let mut max_calories = 0;
    for cals in calories.iter() {
        let summed_calories = cals.iter().sum();

        if summed_calories > max_calories {
            max_calories = summed_calories;
        }
    }

    return max_calories;
}

fn get_top3_calories_for_reindeers(calories: &Vec<Vec<i32>>) -> i32 {
    let mut top3_calories: [i32; 3] = [0, 0, 0];

    for cals in calories.iter() {
        let summed_calories = cals.iter().sum();

        for i in [0, 1, 2] {
            if summed_calories > top3_calories[i] {
                // Move all 1 down
                for j in (i..2).step_by(1) {
                    top3_calories[2 - j] = top3_calories[1 - j]
                }
                top3_calories[i] = summed_calories;
                break;
            }
        }

        // Prints a nice matrix with the top3's evolving over time
        //println!(
        //    "{} | {} | {}",
        //    top3_calories[0], top3_calories[1], top3_calories[2]
        //);
    }

    return top3_calories.iter().sum();
}

fn parse_input(filename: String) -> Vec<Vec<i32>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut calories = vec![vec![0; 0]; 0];
    let mut cals = vec![0; 0];
    for line in reader.lines() {
        let cal = line.unwrap();

        if cal == "" && cals.len() > 0 {
            calories.push(cals);

            cals = vec![0; 0];
        } else {
            cals.push(cal.parse::<i32>().unwrap());
        }
    }
    if cals.len() > 0 {
        calories.push(cals)
    }

    return calories;
}
