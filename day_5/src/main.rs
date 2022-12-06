use std::{
    collections::VecDeque,
    fs::File,
    io::Split,
    io::{BufRead, BufReader, Read},
};

fn main() {
    // Part 1
    let stack = load_stack();
    println!("Loaded {} stacks", stack.clone().len());
    let moves = load_moves();
    println!("Loaded {} moves", moves.clone().len());
    let mut moved_stack = apply_moves_to_stack(stack, moves);
    print!("Top elements are: ");
    for i in 0..9 {
        print!("{}", moved_stack[i as usize].pop_front().unwrap());
    }
}

fn load_stack() -> [VecDeque<char>; 9] {
    let file = File::open("src/stack.txt".to_owned()).unwrap();
    let reader = BufReader::new(file);

    let mut data: [VecDeque<char>; 9] = Default::default();
    for line in reader.lines() {
        let unwrapped = line.unwrap();
        let mut chars = unwrapped.chars();

        for i in 0..=9 {
            let mut step_size = 3;
            if i == 0 {
                step_size = 1;
            }

            let el = chars.nth(step_size);
            if el.is_none() {
                // Line already ended
                break;
            }

            let char = el.unwrap();
            if char == ' ' {
                // No entry here
                continue;
            } else if char == '1' {
                // Last line
                break;
            }

            data[i].push_back(char);
        }
    }

    return data;
}

fn load_moves() -> Vec<[i32; 3]> {
    // Let's try sth different
    let mut file = File::open("src/moves.txt").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let mut data: Vec<[i32; 3]> = Vec::new();
    for line in s.lines() {
        let mut splitted = line.split(" ");

        data.push([
            splitted.nth(1).unwrap().parse::<i32>().unwrap(),
            splitted.nth(1).unwrap().parse::<i32>().unwrap(),
            splitted.nth(1).unwrap().parse::<i32>().unwrap(),
        ])
    }

    return data;
}

fn apply_moves_to_stack(
    mut stack: [VecDeque<char>; 9],
    moves: Vec<[i32; 3]>,
) -> [VecDeque<char>; 9] {
    for mv in moves {
        for i in 0..mv[0] {
            let el = stack[(mv[1] - 1) as usize].pop_front().unwrap();
            stack[(mv[2] - 1) as usize].insert(0, el);
        }
    }

    return stack;
}
