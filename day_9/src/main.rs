use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    // Part 1
    let moves = parse_moves();
    let mut rope = Rope {
        h_pos: [0; 2],
        t_pos: [0; 2],
        x_r: [0, 5],
        y_r: [0, 4],
    };
    println!("Found {} moves", moves.len());
    println!(
        "Unique fields: {}",
        apply_moves_to_rope_and_cnt(&mut rope, &moves)
    )
}

fn parse_moves() -> Vec<char> {
    let mut file = File::open("src/input.txt").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let mut moves: Vec<char> = Vec::new();
    for line in s.lines() {
        let mut splitted = line.split(" ");
        let dir = splitted.next().unwrap();
        let cnt = splitted.next().unwrap().parse::<i32>().unwrap();

        let c = dir.chars().nth(0).unwrap();
        for _ in 0..cnt {
            println!("Add {}", c);
            moves.push(c)
        }
    }

    return moves;
}

struct Rope {
    h_pos: [i32; 2],
    t_pos: [i32; 2],
    x_r: [i32; 2],
    y_r: [i32; 2],
}

impl Rope {
    fn move_t(&mut self) {
        self.h_pos[1] += 1;
        self.move_tail();
    }

    fn move_b(&mut self) {
        self.h_pos[1] -= 1;
        self.move_tail();
    }

    fn move_r(&mut self) {
        self.h_pos[0] += 1;
        self.move_tail();
    }

    fn move_l(&mut self) {
        self.h_pos[0] -= 1;
        self.move_tail();
    }

    fn move_tail(&mut self) {
        let dx = self.h_pos[0] - self.t_pos[0];
        let dy = self.h_pos[1] - self.t_pos[1];

        if dx.abs() > 1 {
            if dy == 0 {
                self.t_pos[0] += dx.signum();
                return;
            }
            // Diagonal move
            if dy.abs() == 1 {
                self.t_pos[0] += dx.signum();
                self.t_pos[1] += dy.signum();
                return;
            }
            panic!("dx and dy > 1")
        }
        if dy.abs() > 1 {
            if dx == 0 {
                self.t_pos[1] += dy.signum();
                return;
            }
            if dx.abs() == 1 {
                self.t_pos[0] += dx.signum();
                self.t_pos[1] += dy.signum();
                return;
            }
            panic!("dx and dy > 1")
        }
    }

    fn print(&self) {
        for y in self.y_r[0]..=self.y_r[1] {
            for x in self.x_r[0]..=self.x_r[1] {
                if x == self.t_pos[0] && y == self.t_pos[1] {
                    print!("T")
                } else if x == self.h_pos[0] && y == self.h_pos[1] {
                    print!("H")
                } else {
                    print!(".")
                }
            }
            println!();
        }
        println!("\n#####\n")
    }
}

fn apply_moves_to_rope_and_cnt(rope: &mut Rope, moves: &Vec<char>) -> i32 {
    let mut fields: HashMap<String, bool> = HashMap::new();
    let mut x_ext = [0; 2];
    let mut y_ext = [0; 2];

    for mv in moves.iter() {
        match mv {
            'U' => rope.move_t(),
            'D' => rope.move_b(),
            'R' => rope.move_r(),
            'L' => rope.move_l(),
            &_ => (),
        }

        fields.insert(format!("{}-{}", rope.t_pos[0], rope.t_pos[1]), true);
        if rope.h_pos[0] < x_ext[0] {
            x_ext[0] = rope.h_pos[0]
        }
        if rope.h_pos[0] > x_ext[1] {
            x_ext[1] = rope.h_pos[0]
        }
        if rope.h_pos[1] < y_ext[0] {
            y_ext[0] = rope.h_pos[1]
        }
        if rope.h_pos[1] > y_ext[1] {
            y_ext[1] = rope.h_pos[1]
        }
        //rope.print();
    }

    println!(
        "Extremas: X [{}, {}] Y [{}, {}]",
        x_ext[0], x_ext[1], y_ext[0], y_ext[1]
    );

    return fields.keys().len() as i32;
}
