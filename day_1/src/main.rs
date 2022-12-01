use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut calories = vec![vec![0; 0]; 0];
    for line in reader.lines() {
        let cal = line?;
        let mut cals = vec![0; 0];

        if cal == "" {
            calories.push(cals);

            cals = vec![0; 0];
        } else {
            cals.push(cal.parse::<i32>().unwrap());
        }

        println!("{}", calories.len());
    }

    Ok(())
}
