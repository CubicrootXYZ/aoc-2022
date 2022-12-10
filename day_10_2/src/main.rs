use std::{fs::File, io::Read};

fn main() {
    // Part 1
    let cmds = parse_input();
    print_cmds(&cmds);
}

fn parse_input() -> Vec<i32> {
    let mut file = File::open("src/input.txt").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let mut cmds: Vec<i32> = Vec::new();
    // cmds contains the state of the END of the cycle, but the print process depends on the state during the cycle, fix
    // this by shifting everything by 1
    cmds.push(1);
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

fn print_cmds(cmds: &Vec<i32>) {
    // cmds is the center of the sprite so cmd +-1 is the sprite
    for cycle in 0..240 as i32 {
        if cycle % 40 == 0 {
            // line break every 40 cycles
            println!()
        }

        let curs = cycle % 40;
        let sprite_center = get_n(cmds, cycle as usize);
        if curs <= sprite_center + 1 && curs >= sprite_center - 1 {
            print!("#")
        } else {
            print!(".")
        }
    }
}
