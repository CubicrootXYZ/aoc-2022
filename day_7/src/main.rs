use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::Read,
};

fn main() {
    // Part 1
    parse();
}

fn parse() {
    let mut file = File::open("src/input.txt").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let mut tree = Dir {
        size: 0,
        subdirs: HashMap::new(),
        is_file: false,
    };
    let mut current_dir: VecDeque<String> = VecDeque::new();

    for line in s.lines() {
        let first4 = &line[0..=3];

        if first4 == "$ cd".to_owned() {
            let dir = &line[5..line.len()];
            if dir == "..".to_owned() {
                current_dir.pop_back();
            } else if dir == "/" {
                current_dir = VecDeque::new();
            } else {
                current_dir.push_back(dir.to_string())
            }

            for dir in current_dir.iter() {
                print!("{}/", dir);
            }
            println!("")
        } else if first4 == "$ ls".to_owned() {
            // ls command
        } else {
            // dir list
            if first4 == "dir " {
                add_new_dir(line[4..line.len()].to_owned(), &current_dir, &mut tree);
                continue;
            }

            let mut splitted = line.split(" ");
            let file_size = splitted.next().unwrap().parse::<i32>().unwrap();
            let file_name = splitted.next().unwrap();
            set_file_size(file_name.to_owned(), file_size, &current_dir, &mut tree)
        }
    }

    calculate_folder_sizes(&mut tree);

    println!("Sum of small folders is {}", sum_small_folders(&tree));
    println!("Needed space is {}", 30000000 - (70000000 - tree.size));
    println!(
        "Smalles dir to delete is {}",
        find_suitable_dir(30000000 - (70000000 - tree.size), &tree)
    )
}

fn add_new_dir(newDir: String, dirs: &VecDeque<String>, parentDir: &mut Dir) {
    let mut currentDir = parentDir;
    for dir in dirs {
        currentDir = currentDir.subdirs.get_mut(dir).unwrap()
    }

    currentDir.subdirs.insert(
        newDir,
        Dir {
            size: 0,
            subdirs: HashMap::new(),
            is_file: false,
        },
    );
}

fn set_file_size(file_name: String, file_size: i32, dirs: &VecDeque<String>, parentDir: &mut Dir) {
    let mut currentDir = parentDir;
    for dir in dirs {
        currentDir = currentDir.subdirs.get_mut(dir).unwrap()
    }

    currentDir.subdirs.insert(
        file_name,
        Dir {
            size: file_size,
            subdirs: HashMap::new(),
            is_file: true,
        },
    );
}

fn calculate_folder_sizes(parentDir: &mut Dir) -> i32 {
    let mut summed = 0;
    for entry in parentDir.subdirs.iter_mut() {
        if entry.1.is_file {
            summed += entry.1.size
        } else {
            summed += calculate_folder_sizes(entry.1)
        }
    }

    parentDir.size = summed;

    return summed;
}

fn sum_small_folders(parentDir: &Dir) -> i32 {
    let mut sum = 0;

    for entry in &parentDir.subdirs {
        if entry.1.is_file {
            continue;
        }

        if entry.1.size <= 100000 {
            sum += entry.1.size;
        }

        sum += sum_small_folders(entry.1)
    }

    return sum;
}

fn find_suitable_dir(min_size: i32, parentDir: &Dir) -> i32 {
    let mut smallest = 999999999;
    for entry in &parentDir.subdirs {
        if entry.1.is_file {
            continue;
        }

        if entry.1.size < smallest && entry.1.size > min_size {
            smallest = entry.1.size;
            for el in &entry.1.subdirs {
                let smallest_child = find_suitable_dir(min_size, el.1);

                if smallest_child < smallest {
                    smallest = smallest_child;
                }
            }
        }
    }

    return smallest;
}

struct Dir {
    size: i32,
    subdirs: HashMap<String, Dir>,
    is_file: bool,
}
