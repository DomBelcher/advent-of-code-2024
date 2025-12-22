use std::fs;

const FILENAME: &str = "./inputs/input";

const DIRECTIONS: [(&'static i32, &'static i32); 8] = [
    (&1, &0),
    (&1, &1),
    (&0, &1),
    (&-1, &1),
    (&-1, &0),
    (&-1, &-1),
    (&0, &-1),
    (&1, &-1)
];

const DIAGONAL_DIRECTIONS: [(&'static i32, &'static i32); 4] = [
    (&1, &1),
    (&-1, &1),
    (&-1, &-1),
    (&1, &-1)
];

const XMAS_CHARS: [&'static char; 4] = [&'X', &'M', &'A', &'S']; 
const MA_CHARS: [&'static char; 2] = [&'A', &'M']; 
const AS_CHARS: [&'static char; 2] = [&'A', &'S'];

const XMAS_ARMS: [[&'static char; 2]; 4] = [
    MA_CHARS,
    MA_CHARS,
    AS_CHARS,
    AS_CHARS
];

fn main() {
    let grid = parse_input();
    let mut total = 0;
    let mut x_total = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            for dir in DIRECTIONS {
                if is_match(&grid, (i ,j), dir, 0, XMAS_CHARS.to_vec()) {
                    total += 1;
                }
            }
            
            for o in 0..4 {
                if is_x_mas(&grid, (i ,j), o) {
                    x_total += 1;
                }
            }
        }
    }
    println!("found {} matches", total);
    println!("found {} x-mas matches", x_total);
}


fn parse_input() -> Vec<Vec<char>> {
    let mut parsed_input: Vec<Vec<char>> = vec![];
    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        parsed_input.push(line.chars().collect());
    }
    return parsed_input;
}

fn is_match(grid: &Vec<Vec<char>>, coords: (usize, usize), dir: (&i32, &i32), char_ind: usize, char_list: Vec<&char>) -> bool {
    if &grid[coords.0][coords.1] != char_list[char_ind] {
        return false;
    }
    if char_ind == char_list.len() - 1 {
        return true
    }
    let next_x = coords.0 as i32 + dir.0;
    let next_y = coords.1 as i32 + dir.1;
    if next_x < 0 || next_x >= grid.len() as i32 || next_y < 0 || next_y >= grid[0].len() as i32  {
        return false;
    }

    return is_match(grid, (next_x as usize, next_y as usize), dir, char_ind + 1, char_list)
}

fn is_x_mas(grid: &Vec<Vec<char>>, coords: (usize, usize), orientation: usize) -> bool {
    for i in 0..4 {
        let ind = (i + orientation) % 4;
        let matches = is_match(grid, coords, DIAGONAL_DIRECTIONS[ind], 0, XMAS_ARMS[i].to_vec());
        if !matches {
            return false
        }
    }
    return true;
}