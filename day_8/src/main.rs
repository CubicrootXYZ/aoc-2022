use std::{collections::VecDeque, fs::File, io::Read};

fn main() {
    let forest = parse_forest();
    println!("Forest is {} x {}", forest.size_x, forest.size_y);

    // Part 1
    println!("Visible trees {}", cnt_visible(&forest));

    // Part 2
    println!("Max scenic score is {}", max_scenic_score(&forest))
}

fn parse_forest() -> Forest {
    let mut file = File::open("src/input.txt").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let mut f = Forest {
        trees: Vec::new(),
        size_x: 0,
        size_y: 0,
    };
    for line in s.lines() {
        f.size_y += 1;
        f.size_x = line.len() as i32;

        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            row.push(c.to_string().parse::<i32>().unwrap())
        }
        f.trees.push(row);
    }

    return f;
}

struct Forest {
    trees: Vec<Vec<i32>>,
    size_x: i32,
    size_y: i32,
}

impl Forest {
    fn get_element(&self, x: i32, y: i32) -> i32 {
        if x >= self.size_x {
            return -1;
        }
        if y >= self.size_y {
            return -1;
        }

        let row = self.trees.get(x as usize).unwrap();

        return *row.get(y as usize).unwrap();
    }
}

fn cnt_visible(forest: &Forest) -> i32 {
    let mut cnt = 0;
    for x in 0..forest.size_x {
        for y in 0..forest.size_y {
            // Edges are always visible
            if x == 0 || y == 0 || x == forest.size_x || y == forest.size_y {
                cnt += 1;
                continue;
            }

            let current_height = forest.get_element(x, y);

            let mut is_hidden_xl = false;
            let mut is_hidden_xr = false;
            let mut is_hidden_yb = false;
            let mut is_hidden_yt = false;
            for xn in 0..x {
                let height = forest.get_element(xn, y);
                if height >= current_height {
                    is_hidden_xr = true;
                    break;
                }
            }
            for xn in (x + 1)..forest.size_x {
                let height = forest.get_element(xn, y);
                if height >= current_height {
                    is_hidden_xl = true;
                    break;
                }
            }
            for yn in 0..y {
                let height = forest.get_element(x, yn);
                if height >= current_height {
                    is_hidden_yt = true;
                    break;
                }
            }
            for yn in (y + 1)..forest.size_y {
                let height = forest.get_element(x, yn);
                if height >= current_height {
                    is_hidden_yb = true;
                    break;
                }
            }

            if is_hidden_xr && is_hidden_xl && is_hidden_yb && is_hidden_yt {
                continue;
            }

            cnt += 1
        }
    }

    return cnt;
}

fn max_scenic_score(forest: &Forest) -> i32 {
    let mut score = 0;

    for x in 0..forest.size_x {
        for y in 0..forest.size_y {
            let cur_h = forest.get_element(x, y);
            let mut dist = [0; 4];
            for xn in 0..x {
                dist[0] += 1;
                let height = forest.get_element(x - xn - 1, y);
                if height >= cur_h {
                    break;
                }
            }
            for xn in (x + 1)..forest.size_x {
                dist[1] += 1;
                let height = forest.get_element(xn, y);
                if height >= cur_h {
                    break;
                }
            }
            for yn in 0..y {
                dist[2] += 1;
                let height = forest.get_element(x, y - yn - 1);
                if height >= cur_h {
                    break;
                }
            }
            for yn in (y + 1)..forest.size_y {
                dist[3] += 1;
                let height = forest.get_element(x, yn);
                if height >= cur_h {
                    break;
                }
            }

            // Correct for edges
            if x == 0 {
                dist[0] = 0;
            } else if x == forest.size_x {
                dist[1] = 0;
            }
            if y == 0 {
                dist[2] = 0;
            } else if y == forest.size_y {
                dist[3] = 0;
            }

            let mult = dist[0] * dist[1] * dist[2] * dist[3];
            if mult > score {
                score = mult;
            }
        }
    }

    return score;
}
