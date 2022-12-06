use std::{fs::File, io::Read};

fn main() {
    println!("Packet marker is {}", part1());
    println!("Message marker is {}", part2())
}

fn part1() -> i32 {
    let mut file = File::open("src/input.txt").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let mut last4: [char; 4] = ['a', 'a', 'a', 'a'];

    let mut i = 0;
    for c in s.chars() {
        i += 1;

        for i in 0..3 {
            last4[i] = last4[i + 1]
        }
        last4[3] = c;

        println!("{}-{}-{}-{}", last4[0], last4[1], last4[2], last4[3]);

        if !has_duplicate(last4) && i > 4 {
            return i;
        }
    }

    return -1;
}

fn has_duplicate(arr: [char; 4]) -> bool {
    for i in 0..4 {
        for j in 0..4 {
            if i == j {
                continue;
            }

            if arr[i] == arr[j] {
                return true;
            }
        }
    }

    return false;
}

fn part2() -> i32 {
    let mut file = File::open("src/input.txt").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let mut last14: [char; 14] = ['a'; 14];

    let mut i = 0;
    for c in s.chars() {
        i += 1;

        for i in 0..13 {
            last14[i] = last14[i + 1]
        }
        last14[13] = c;

        for c in last14 {
            print!("{}", c);
        }
        println!();

        if !has_duplicate_14(last14) && i > 14 {
            return i;
        }
    }

    return -1;
}

fn has_duplicate_14(arr: [char; 14]) -> bool {
    for i in 0..14 {
        for j in 0..14 {
            if i == j {
                continue;
            }

            if arr[i] == arr[j] {
                return true;
            }
        }
    }

    return false;
}
