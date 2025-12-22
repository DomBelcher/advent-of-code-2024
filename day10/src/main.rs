use std::fs;
use std::collections::HashSet;

const FILENAME: &str = "./inputs/input";

const RADIX: u32 = 10;
const STEPS: [(i32, i32); 4] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1)
];

fn main() {
    let map = parse_input();
    let mut trail_count = 0;
    let mut trail_rating_count = 0;

    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == 0 {
                // println!("{}", find_trails(&map, x, y));
                let (trails, rating) = find_trails(&map, x, y);
                trail_count += trails;
                trail_rating_count += rating;
            }
        }
    }

    println!("{}", trail_count);
    println!("{}", trail_rating_count);
}

fn step(map: &Vec<Vec<u32>>, x: i32, y: i32, count: u32) -> (HashSet<(i32, i32)>, Vec<i32>) {
    let mut steps = HashSet::new();
    let mut rating_steps = vec![];

    if map[x as usize][y as usize] == 9 {
        steps.insert((x, y));
        rating_steps.push(1);
        return (steps, rating_steps)
    }
    for s in STEPS {
        let next = (x + s.0, y + s.1);
        if next.0 >= 0 && next.0 < map.len() as i32 && next.1 >= 0 && next.1 < map[0].len() as i32 {
            if map[next.0 as usize][next.1 as usize] == count + 1 {
                let (onward_steps, mut onward_rating_steps) = step(map, next.0, next.1, count + 1);
                steps.extend(onward_steps);
                rating_steps.append(&mut onward_rating_steps);
            }
        }
    }
    return (steps, rating_steps);
}

fn find_trails(map: &Vec<Vec<u32>>, x: usize, y: usize) -> (usize, usize) {
    let (trails, rating_trails) = step(map, x as i32, y as i32, 0);
    return (trails.len(), rating_trails.len());
}

fn parse_input () -> Vec<Vec<u32>> {
    let mut input = vec![];
    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        input.push(line.chars().map(|c| char::to_digit(c, RADIX).unwrap()).collect::<Vec<u32>>());
    }
    return input;
}