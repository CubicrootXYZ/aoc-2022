use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut calories = vec![vec![0; 0]; 0];
    let mut cals = vec![0; 0];
    for line in reader.lines() {
        let cal = line?;

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

    let mut max_calories = 0;
    for cals in calories.iter() {
        let mut summed_calories = 0;

        for cal in cals.iter() {
            summed_calories += cal;
        }

        if summed_calories > max_calories {
            max_calories = summed_calories;
        }
    }

    println!("Max calories per raindeer: {}", max_calories);

    Ok(())
}
