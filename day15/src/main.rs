use std::fs;

const ROBOT_CHAR: char = '@';
const WALL_CHAR: char = '#';
const EMPTY_CHAR: char = '.';
const BOX_CHAR: char = 'O';
const BOX_CHAR_LEFT: char = '[';
const BOX_CHAR_RIGHT: char = ']';

const FILENAME: &str = "./inputs/input";

const DIRECTION_VALS: [(char, usize); 4] = [
    ('^', 0),
    ('>', 1),
    ('v', 2),
    ('<', 3),
];

const DIRECTIONS: [(i32, i32); 4] = [
    (0, -1),
    (1, 0),
    (0, 1),
    (-1, 0)
];

fn main() {
    let (start_map, instructions) = parse_input();

    let width = start_map[0].len();
    let height = start_map.len();
    let mut robot_coords = find_robot(&start_map);


    let mut dbl_map = double_input(&start_map);
    let dbl_width = width * 2;
    let mut dbl_robot_coords = find_robot(&dbl_map);

    println!("Map dimensions: ({}, {})", width, height);
    println!("{} instructions", instructions.len());
    println!("Robot is at {:?}", robot_coords);


    println!("Double map dimensions: ({}, {})", dbl_width, height);
    println!("Double Robot is at {:?}", dbl_robot_coords);

    let mut map = start_map.clone();

    for (_idx, i) in instructions.iter().enumerate() {
        let dir = DIRECTIONS[*i];
        if valid_move(&map, robot_coords, dir) {
            robot_coords = do_move(&mut map, robot_coords, dir, EMPTY_CHAR, ROBOT_CHAR);
        }
        if valid_dbl_move(&dbl_map, dbl_robot_coords, dir) {
            dbl_robot_coords = do_dbl_move(&mut dbl_map, dbl_robot_coords, dir, EMPTY_CHAR, ROBOT_CHAR);
        }
        // draw_map(&dbl_map, _idx);
    }

    draw_map(&map, instructions.len());
    draw_map(&dbl_map, instructions.len());

    println!("GPS Sum: {}", sum_gps(&map, BOX_CHAR));
    println!("GPS Sum: {}", sum_gps(&dbl_map, BOX_CHAR_LEFT));
}

fn do_move (map: &mut Vec<Vec<char>>, coords: (i32, i32), dir: (i32, i32), moving_into: char, moving_object: char) -> (i32, i32) {
    map[coords.1 as usize][coords.0 as usize] = moving_into;

    let next_coords = (
        (coords.0 + dir.0),
        (coords.1 + dir.1)
    );
    let next_tile = map[next_coords.1 as usize][next_coords.0 as usize];

    match next_tile {
        EMPTY_CHAR => {
            map[next_coords.1 as usize][next_coords.0 as usize] = moving_object;
        },
        BOX_CHAR => {
            do_move(map, next_coords, dir, moving_object, BOX_CHAR);
        },
        _ => panic!("invalid space")
    };
    return next_coords;
}

fn do_dbl_move (map: &mut Vec<Vec<char>>, coords: (i32, i32), dir: (i32, i32), moving_into: char, moving_object: char) -> (i32, i32) {
    map[coords.1 as usize][coords.0 as usize] = moving_into;

    let next_coords = (
        (coords.0 + dir.0),
        (coords.1 + dir.1)
    );
    let next_tile = map[next_coords.1 as usize][next_coords.0 as usize];

    match next_tile {
        EMPTY_CHAR => {
            map[next_coords.1 as usize][next_coords.0 as usize] = moving_object;
        },
        BOX_CHAR_LEFT => {
            if dir == (1,0) || dir == (-1, 0) {
                do_dbl_move(map, next_coords, dir, moving_object, BOX_CHAR_LEFT);
            } else {
                do_dbl_move(map, next_coords, dir, moving_object, BOX_CHAR_LEFT);
                let right_next_coords = (next_coords.0 + 1, next_coords.1);
                do_dbl_move(map, right_next_coords, dir, EMPTY_CHAR, BOX_CHAR_RIGHT);
            }
        },
        BOX_CHAR_RIGHT => {
            if dir == (1,0) || dir == (-1, 0) {
                do_dbl_move(map, next_coords, dir, moving_object, BOX_CHAR_RIGHT);
            } else {
                let left_next_coords = (next_coords.0 - 1, next_coords.1);
                do_dbl_move(map, left_next_coords, dir, EMPTY_CHAR, BOX_CHAR_LEFT);
                do_dbl_move(map, next_coords, dir, moving_object, BOX_CHAR_RIGHT);
            }
        }
        _ => panic!("invalid space moving from {:?} to {:?} ({}), direction: {:?}", coords, next_coords, next_tile, dir)
    };
    return next_coords;
}

fn find_robot (map: &Vec<Vec<char>>) -> (i32, i32) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == ROBOT_CHAR {
                return (x as i32, y as i32);
            }
        }
    }
    panic!("Robot not found");
}

fn valid_move (map: &Vec<Vec<char>>, coords: (i32, i32), dir: (i32, i32)) -> bool {
    let next_coords = (
        (coords.0 + dir.0),
        (coords.1 + dir.1)
    );

    let next_tile = map[next_coords.1 as usize][next_coords.0 as usize];
    return match next_tile {
        EMPTY_CHAR => true,
        WALL_CHAR => false,
        BOX_CHAR => valid_move(map, next_coords, dir),
        _ => panic!("invalid space moving from {:?} to {:?} ({}), direction: {:?}", coords, next_coords, next_tile, dir)
    };
}

fn valid_dbl_move (map: &Vec<Vec<char>>, coords: (i32, i32), dir: (i32, i32)) -> bool {
    let next_coords = (
        (coords.0 + dir.0),
        (coords.1 + dir.1)
    );

    let next_tile = map[next_coords.1 as usize][next_coords.0 as usize];
    return match next_tile {
        EMPTY_CHAR => true,
        WALL_CHAR => false,
        BOX_CHAR_LEFT => {
            if dir == (1,0) || dir == (-1, 0) {
                valid_dbl_move(map, next_coords, dir)
            } else {
                valid_dbl_move(map, next_coords, dir) && valid_dbl_move(map, (
                    next_coords.0 + 1, next_coords.1
                ), dir)
            }
        },
        BOX_CHAR_RIGHT => {
            if dir == (1,0) || dir == (-1, 0) {
                valid_dbl_move(map, next_coords, dir)
            } else {
                valid_dbl_move(map, next_coords, dir) && valid_dbl_move(map, (
                    next_coords.0 - 1, next_coords.1
                ), dir)
            }
        },
        _ => panic!("invalid space moving from {:?} to {:?} ({}), direction: {:?}", coords, next_coords, next_tile, dir)
    };
}

fn sum_gps (map: &Vec<Vec<char>>, c: char) -> usize {
    let mut total = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == c {
                total += y * 100 + x;
            }
        }
    }

    return total;
}

fn doubler (c: char) -> [char; 2] {
    return match c {
        ROBOT_CHAR => [ROBOT_CHAR, EMPTY_CHAR],
        WALL_CHAR => [WALL_CHAR, WALL_CHAR],
        BOX_CHAR => [BOX_CHAR_LEFT, BOX_CHAR_RIGHT],
        EMPTY_CHAR => [EMPTY_CHAR, EMPTY_CHAR],
        _ => panic!("invalid char")
    }
}

fn double_input (input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let dbl = input.iter().map(
        |line| line.iter().map(|c| doubler(*c)).flatten().collect()
    ).collect();

    return dbl;
}

fn parse_input () -> (Vec<Vec<char>>, Vec<usize>) {
    let mut map = vec![];
    let mut instructions = vec![];
    let mut input_mode = "map";
    
    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        if line.len() == 0 {
            input_mode = "instructions";
        }

        if input_mode == "map" {
            map.push(line.chars().collect());
        } else if input_mode == "instructions" {
            instructions.append(
                &mut line.chars().map(arrow_to_dir).collect::<Vec<usize>>()
            );
        }
    }

    fn arrow_to_dir (c: char) -> usize {
        return match c {
            '^' => 0,
            '>' => 1,
            'v' => 2,
            '<' => 3,
            _ => panic!("oh no")
        }
    }

    return (map, instructions);
}

fn draw_map(map: &Vec<Vec<char>>, step: usize) {
    println!("Drawing map at step {}", step);

    for y in 0..map.len() {
        println!("{}", map[y].iter().collect::<String>());
    }
    println!()
}