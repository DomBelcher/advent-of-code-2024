use std::collections::{HashMap, HashSet};
use colored::Colorize;

const DIRECTIONS: [(i32, i32); 4] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1)
];

const SURROUNDS: [(i32, i32); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1, 0), (1, 0),
    (-1, 1), (0, 1), (1, 1)
];

const JUNCTION_COORDS: [[(i32, i32); 2]; 4] = [
    [(1, 0), (0, 1)], [(1, 0), (0, -1)], [(-1, 0), (0, 1)], [(-1, 0), (0, -1)]
];


const START_CHAR: char = 'S';
const EMPTY_CHAR: char = '.';
const WALL_CHAR: char = '#';
const END_CHAR: char = 'E';

pub fn junction_graph (maze: &HashMap<(i32, i32), char>, width: usize, height: usize) -> (HashMap<Node, HashMap<Node, i32>>, Node, Node) {
    let mut graph = HashMap::new();
    let mut junction_points = HashSet::new();
    for y in 0..height {
        for x in 0..width {
            let coords = (x as i32, y as i32);
            if !is_space(maze.get(&coords)) {
                continue;
            }

            for jnc in JUNCTION_COORDS {
                let jc1 = (coords.0 + jnc[0].0, coords.1 + jnc[0].1);
                let jc2 = (coords.0 + jnc[1].0, coords.1 + jnc[1].1);
                let char_1 = maze.get(&jc1);
                let char_2 = maze.get(&jc2);

                if is_space(char_1) && is_space(char_2) {
                    junction_points.insert(coords);
                    break
                }
            }
            if is_dead_end(maze, coords) {
                junction_points.insert(coords);
            }
        }
    }

    for point in junction_points.iter() {
        let mut edges = HashMap::new();
        let start_node = Node {
            x: point.0,
            y: point.1
        };
        for (d, dir) in DIRECTIONS.iter().enumerate() {

            let arc_end = find_arc_end(maze, point, dir, &junction_points);
            if arc_end.is_some() {
                let end_node = Node {
                    x: arc_end.unwrap().0,
                    y: arc_end.unwrap().1
                };
                let length = dist(&start_node, &end_node);
                edges.insert(end_node, length);
            }

        }
        graph.insert(start_node, edges);
    }


    let start_coords = find_start(maze, width, height);
    let terminal_coords = find_end(maze, width, height);
    let start_node = Node {
        x: start_coords.0,
        y: start_coords.1
    };
    let terminal_node = Node {
        x: terminal_coords.0,
        y: terminal_coords.1
    };

    return (graph, start_node, terminal_node);
}

pub fn build_graph (maze: &HashMap<(i32, i32), char>, width: usize, height: usize) -> (HashMap<Node, HashMap<Node, i32>>, Node, Node) {
    let mut graph = HashMap::new();

    let mut start_node = None;
    let mut end_node = None;

    for y in 0..height {
        for x in 0..width {
            let coords = (x as i32, y as i32);
            if !is_space(maze.get(&coords)) {
                continue;
            }
            let tile = maze.get(&coords).unwrap();

            let node = Node { x: coords.0, y: coords.1 };
            if tile == &START_CHAR {
                start_node = Some(node.clone())
            } else if tile == &END_CHAR {
                end_node = Some(node.clone())
            }

            let mut edges = HashMap::new();
            for dir in DIRECTIONS {
                let next_x = coords.0 + dir.0;
                let next_y = coords.1 + dir.1;
                // if next_x < 0 || next_y < 0 || next_x >= width as i32 || next_y >= height as i32 {
                //     continue
                // }
                if !is_space(maze.get(&(next_x, next_y))) {
                    continue;
                }
                let next_node = Node { x: next_x, y: next_y };
                edges.insert(next_node, 1);
            }
            graph.insert(node, edges);
        }
    }
    let sn = start_node.expect("No start node");
    let en = end_node.expect("No end node");

    return (graph, sn, en);
}

fn find_arc_end (maze: &HashMap<(i32, i32), char>, start_coords: &(i32, i32), dir: &(i32, i32), junction_points: &HashSet<(i32, i32)>) -> Option<(i32, i32)> {
    let next_coords = (start_coords.0 + dir.0, start_coords.1 + dir.1);
    let c = maze.get(&next_coords);
    if junction_points.contains(&next_coords) {
        return Some(next_coords);
    } else if is_space(c) {
        return find_arc_end(maze, &next_coords, dir, junction_points)
    } else {
        return None;
    }
}

fn find_start (maze: &HashMap<(i32, i32), char>, width: usize, height: usize) -> (i32, i32) {
    return _find(maze, width, height, &START_CHAR).expect("Start not found");
}

fn find_end (maze: &HashMap<(i32, i32), char>, width: usize, height: usize) -> (i32, i32) {
    return _find(maze, width, height, &END_CHAR).expect("End not found");
}

fn _find (maze: &HashMap<(i32, i32), char>, width: usize, height: usize, target_char: &char) -> Option<(i32, i32)> {
    for y in 0..height {
        for x in 0..width {
            let coords = (x as i32, y as i32);
            let c = maze.get(&coords);
            if c.is_some() && c.unwrap() == target_char {
                return Some(coords);
            }
        }
    }
    return None
}

fn is_dead_end (maze: &HashMap<(i32, i32), char>, coords: (i32, i32)) -> bool {
    let mut surround_count = 0;
    for s in SURROUNDS {
        let s_coords = (coords.0 + s.0, coords.1 + s.1);
        let s_char = maze.get(&s_coords);
        if is_space(s_char) {
            surround_count += 1;
        }
    }
    return surround_count == 1;
}

pub fn is_space (c: Option<&char>) -> bool {
    if c.is_none() {
        return false;
    }
    return match *c.unwrap() {
        EMPTY_CHAR => true,
        START_CHAR => true,
        END_CHAR => true,
        WALL_CHAR => false,
        _ => panic!("invalid char")
    }
}

pub fn is_char (c: Option<&char>, target: &char) -> bool {
    if c.is_none() {
        return false;
    }
    return match *c.unwrap() {
        target => true,
        _ => false
    }
}

pub fn is_end (c: Option<&char>) -> bool {
    return is_char(c, &END_CHAR)
}


fn dist (node1: &Node, node2: &Node) -> i32 {
    return (node1.x - node2.x).abs() + (node1.y - node2.y).abs()
}

pub fn grid_to_hashmap (maze: &Vec<Vec<char>>) -> (HashMap<(i32, i32), char>, usize, usize) {
    let height = maze.len();
    let width = maze[0].len();
    let mut maze_map = HashMap::new();

    for x in 0..width {
        for y in 0..height {
            let coords = (x as i32, y as i32);
            maze_map.insert(coords, maze[y][x]);
        }
    }
    return (maze_map, width, height);
}

fn draw_maze (maze: &HashMap<(i32, i32), char>, width: usize, height: usize, reds: &HashSet<(i32, i32)>, blues: &HashSet<(i32, i32)>) {
    let mut red_count = 0;
    let mut blue_count = 0;

    for y in 0..height {
        for x in 0..width {
            let coords = (x as i32, y as i32);
            let c = maze.get(&coords).expect("oh no");

            if reds.contains(&coords) {
                red_count += 1;
                print!("{}", c.to_string().red())
            } else if blues.contains(&coords) {
                blue_count += 1;
                print!("{}", c.to_string().blue())
            } else {
                print!("{}", c);
            }
        }
        println!();
    }

    println!("Red count: {} ({}) | Blue count: {} ({})", red_count, reds.len(), blue_count, blues.len());
}

#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
#[derive(Debug)]
pub struct Node {
    pub x: i32,
    pub y: i32
}

impl Node {
    pub fn from_coords (coords: (i32, i32)) -> Self {
        return Self {
            x: coords.0,
            y: coords.1
        }
    }
}

pub fn node (coords: (i32, i32)) -> Node {
    return Node {
        x: coords.0,
        y: coords.1
    }
}

pub trait Coords: {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
}

impl Coords for Node {
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y
    }
}