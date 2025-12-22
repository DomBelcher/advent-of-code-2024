use std::{collections::HashSet, fs};

const FILENAME: &str = "./inputs/input";

const DIRECTIONS: [(i32, i32); 4] = [
        (-1, 0),
        (0, 1),
        (1, 0),
        (0, -1)
];

fn main() {
    let ((width, height), obstacles, mut guard_pos, mut guard_dir) = parse_input();
    println!("Does loop? {}", is_loop(width, height, &obstacles, guard_pos, guard_dir));

    let visited_locs = get_path(width, height, &obstacles, guard_pos, guard_dir);

    let mut looping_count = 0;

    for location in &visited_locs {
        let mut new_obstacles = obstacles.clone();
        new_obstacles.insert(*location);

        if is_loop(width, height, &new_obstacles, guard_pos, guard_dir) {
            looping_count += 1;
            // println!("{:?}", location);
        }
    }

    println!("visited {} locations", visited_locs.len());
    println!("looping paths: {}", looping_count);
}

fn get_path(width: i32, height: i32, obstacles: &HashSet<(i32, i32)>, start_guard_pos: (i32, i32), start_guard_dir: usize) -> HashSet<(i32, i32)> {
    let mut guard_pos = start_guard_pos;
    let mut guard_dir = start_guard_dir;

    let mut visited_locs = HashSet::new();

    while guard_pos.0 >= 0 && guard_pos.0 < width && guard_pos.1 >= 0 && guard_pos.1 < height {
        visited_locs.insert(guard_pos);

        let dir: (i32, i32) = DIRECTIONS[guard_dir];
        let next_guard_pos = (guard_pos.0 + dir.0, guard_pos.1 + dir.1);

        if obstacles.contains(&next_guard_pos) {
            guard_dir = (guard_dir + 1) % 4;
        } else {
            guard_pos = next_guard_pos;
        }
    }
    return visited_locs;
}

fn is_loop(width: i32, height: i32, obstacles: &HashSet<(i32, i32)>, start_guard_pos: (i32, i32), start_guard_dir: usize) -> bool {
    let mut guard_pos = start_guard_pos;
    let mut guard_dir = start_guard_dir;

    let mut visited_locs = HashSet::new();

    while guard_pos.0 >= 0 && guard_pos.0 < width && guard_pos.1 >= 0 && guard_pos.1 < height {
        if visited_locs.contains(&(guard_pos, guard_dir)) {
            return true
        }

        visited_locs.insert((guard_pos, guard_dir));

        let dir: (i32, i32) = DIRECTIONS[guard_dir];
        let next_guard_pos = (guard_pos.0 + dir.0, guard_pos.1 + dir.1);

        if obstacles.contains(&next_guard_pos) {
            guard_dir = (guard_dir + 1) % 4;
        } else {
            guard_pos = next_guard_pos;
        }
    }
    return false;
}

fn parse_input() -> ((i32, i32), HashSet<(i32, i32)>, (i32, i32), usize) {
    // let mut map = vec![];
    let mut obstacles: HashSet<(i32, i32)> = HashSet::new();
    let mut guard_pos: (i32, i32) = (0, 0);
    let mut guard_dir: usize = 0;
    let mut width = 0;
    let mut height = 0;

    let mut guard_exists = false;

    for (x, line) in fs::read_to_string(FILENAME).unwrap().lines().into_iter().enumerate() {
        // map.push(line.chars().collect());
        width += 1;
        height = line.len() as i32;
        let xi = x as i32;

        for (y, c) in line.chars().into_iter().enumerate() {
            let yi = y as i32;

            if c == '#' {
                obstacles.insert((xi, yi));
            } else if c == '^' {
                guard_pos = (xi, yi);
                guard_dir = 0;
                guard_exists = true;
            } else if c == '>' {
                guard_pos = (xi, yi);
                guard_dir = 1;
                guard_exists = true;
            } else if c == 'v' {
                guard_pos = (xi, yi);
                guard_dir = 2;
                guard_exists = true;
            } else if c == '<' {
                guard_pos = (xi, yi);
                guard_dir = 3;
                guard_exists = true;
            }
        }
    }
    if !guard_exists {
        panic!("Oh no");
    }

    return ((width, height), obstacles, guard_pos, guard_dir)
    // return map
}

