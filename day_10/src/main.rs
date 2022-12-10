use std::{fs::File, io::Read};

fn main() {
    // Part 1
    let cmds = parse_input();

    println!(
        "Combined signal strength: {}",
        20 * cmds[18]
            + 60 * cmds[58]
            + 100 * cmds[98]
            + 140 * cmds[138]
            + 180 * cmds[178]
            + 220 * cmds[218]
    );
}

fn parse_input() -> Vec<i32> {
    let mut file = File::open("src/input.txt").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let mut cmds: Vec<i32> = Vec::new();
    for line in s.lines() {
        let first4 = &line[0..4];
        if first4 == "noop" {
            if cmds.len() == 0 {
                cmds.push(1);
            } else {
                cmds.push(get_n(&cmds, cmds.len() - 1))
            }
        } else if first4 == "addx" {
            let mut last = get_n(&cmds, cmds.len() - 1);
            if cmds.len() == 0 {
                last = 1;
            }
            cmds.push(last);
            cmds.push(line[5..line.len()].parse::<i32>().unwrap() + last)
        }
    }

    return cmds;
}

fn get_n(v: &Vec<i32>, n: usize) -> i32 {
    return *v.iter().nth(n).unwrap();
}
