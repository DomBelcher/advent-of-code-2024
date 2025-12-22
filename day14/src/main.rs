use std::{collections::{HashMap, HashSet}, fs};
use regex::Regex;

const FILENAME: &str = "./inputs/input";

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
const N_STEPS: usize = 10403;
const SYMMETRY_THRESHOLD: f64 = 0.15_f64;

const SURROUND: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1), (0, 1),
    (1, -1), (1, 0), (1, 1)
];


fn main() {
    let mut robots = parse_input();
    let n_robots = robots.len();
    let mut symmetric_total = 0;

    let mut max_isolation_score: f64 = 0_f64;

    draw_robots(&robots, 0);
   
    for i in 0..N_STEPS {
        for r in 0..n_robots {
            let robot = &robots[r];
            let next_coords = (
                (robot.coords.0 + robot.velocity.0 + WIDTH) % WIDTH,
                (robot.coords.1 + robot.velocity.1 + HEIGHT) % HEIGHT,
            );
            robots[r].coords = next_coords;
        }
        let isolation_score = isolation_score(&robots);
        if isolation_score > max_isolation_score {
            max_isolation_score = isolation_score;
        }
        if isolation_score > 3_f64 {
            draw_robots(&robots, i as i32);
        }
        if i % 1000 == 0 {
            println!("step {}", i)
        }
    }

    let mut quadrant_totals = [0,0,0,0,0];

    for robot in robots {
        let q = get_quadrant(robot.coords);
        quadrant_totals[q] += 1;
    }
    let mut score = 1;
    for i in 0..4 {
        score *= quadrant_totals[i];
    }
    println!("Safety score: {}", score);
    println!("Max isolation score: {}", max_isolation_score);
}

fn draw_robots(robots: &Vec<Robot>, step: i32) {
    println!("Drawing robots at step {}", step);
    let robot_coords = robots.iter().map(|r| r.coords).collect::<HashSet<_>>();
    let mut picture = vec![];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if robot_coords.contains(&(x, y)) {
                picture.push('0')
            } else {
                picture.push('.')
            }
        }
        // println!("{}", picture.iter().collect::<String>());
        picture.push('\n');
    }
    fs::write(format!("./target/out/{}.txt", step), picture.iter().collect::<String>()).expect("Unable to write file");
    // println!()
}

fn isolation_score (robots: &Vec<Robot>) -> f64 {
    let robot_coords = robots.iter().map(|r| r.coords).collect::<HashSet<_>>();

    let mut score = 0;
    for robot in robots {
        for s in SURROUND {
            let coords = (
                robot.coords.0 + s.0,
                robot.coords.1 + s.1
            );
            if robot_coords.contains(&coords) {
                score += 1;
            }
        }
    }

    return score as f64 / robots.len() as f64;
}

// find all positions robot can reach, with time to first reaching there, and cycle time
fn find_positons_for_robot (robot: &Robot) -> (HashMap<(i32, i32), i32>, i32) {
    let mut positions_map = HashMap::new();
    let mut cycle_time = 0;
    let mut next_position = robot.coords;
    let mut i = 0;

    loop {
        if positions_map.contains_key(&next_position) {
            break;
        }
        positions_map.insert(next_position, i);
        i += 1;
        next_position = (
            (next_position.0 + robot.velocity.0 + WIDTH) % WIDTH,
            (next_position.1 + robot.velocity.1 + HEIGHT) % HEIGHT
        );
    }

    return (positions_map, cycle_time);
}

fn calculate_quadrants (robots: &Vec<Robot>) -> [i32; 5] {
    let mut quadrant_totals = [0,0,0,0,0];

    for robot in robots {
        let q = get_quadrant(robot.coords);
        quadrant_totals[q] += 1;
    }
    return quadrant_totals;
}

fn is_symmetric_quadrants (quadrant_totals: [i32; 5]) -> bool {
    return quadrant_totals[0] == quadrant_totals[2] && quadrant_totals[1] == quadrant_totals[3];
}

fn is_symmetric (robots: &Vec<Robot>) -> bool {
    let robot_coords = robots.iter().map(|r| r.coords).collect::<HashSet<_>>();

    for coord in robot_coords.iter() {
        let mirror_coord = (WIDTH - coord.0 - 1, coord.1);
        if !robot_coords.contains(&mirror_coord) {
            return false;
        }
    }
    return true;
}

fn symmetry_score (robots: &Vec<Robot>) -> f64 {
    let robot_coords = robots.iter().map(|r| r.coords).collect::<HashSet<_>>();
    let mut symmetries = 0;

    for coord in robot_coords.iter() {
        let mirror_coord = (coord.0, HEIGHT - coord.1 - 1);
        // let mirror_coord = (WIDTH - coord.0 - 1, coord.1);
        if robot_coords.contains(&mirror_coord) {
            symmetries += 1
        }
    }
    return symmetries as f64 / robot_coords.len() as f64; 
}

fn get_quadrant (coords: (i32, i32)) -> usize {
    if coords.0 < WIDTH / 2 && coords.1 < HEIGHT / 2 {
        return 0
    }
    if coords.0 > WIDTH / 2  && coords.1 < HEIGHT / 2 {
        return 1
    }
    if coords.0 < WIDTH / 2 && coords.1 > HEIGHT / 2 {
        return 2
    }
    if coords.0 > WIDTH / 2  && coords.1 > HEIGHT / 2 {
        return 3
    }
    return 4
}

fn parse_input () -> Vec<Robot> {
    let re = Regex::new(r"p\=(?<px>\-?[0-9]+)\,(?<py>\-?[0-9]+)\sv\=(?<vx>\-?[0-9]+)\,(?<vy>\-?[0-9]+)").unwrap();
    let mut input = vec![];
    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        let capture = re.captures(line).unwrap();
        input.push(Robot {
            coords: (
                capture.name("px").unwrap().as_str().parse::<i32>().unwrap(),
                capture.name("py").unwrap().as_str().parse::<i32>().unwrap(),
            ),
            velocity: (
                capture.name("vx").unwrap().as_str().parse::<i32>().unwrap(),
                capture.name("vy").unwrap().as_str().parse::<i32>().unwrap(),
            )
        });
    }
    return input;
}

struct Robot {
    coords: (i32, i32),
    velocity: (i32, i32)
}