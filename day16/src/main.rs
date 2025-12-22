use std::{collections::{HashMap, HashSet}, fs, i32};
use colored::Colorize;

const FILENAME: &str = "./inputs/input";

const START_CHAR: char = 'S';
const EMPTY_CHAR: char = '.';
const WALL_CHAR: char = '#';
const END_CHAR: char = 'E';

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

fn main() {
    let maze = parse_input();
    let start = find_start(&maze);
    let terminal = find_end(&maze);
    let graph = build_graph(&maze);

    println!("Number of nodes: {}", graph.len());
    let mut total_arcs = 0;
    for (_, arcs) in &graph {
        total_arcs += arcs.len();
    }
    println!("Number of arcs: {}", total_arcs);

    let start_node = Node {
        x: start.0,
        y: start.1,
        dir: 0
    };
    let terminal_node = Node {
        x: terminal.0,
        y: terminal.1,
        dir: 4
    };

    println!("Start node: ({}, {}, {})", start_node.x, start_node.y, start_node.dir);
    println!("End node: ({}, {}, {})", terminal_node.x, terminal_node.y, terminal_node.dir);

    // perform A* search on the graph
    let mut open_set = HashSet::new();
    let mut closed_set = HashSet::new();
    let mut g_scores = HashMap::new();
    let mut f_scores: HashMap<Node, i32> = HashMap::new();
    let mut best_path = None;
    let mut parents: HashMap<&Node, HashSet<Node>> = HashMap::new();
    g_scores.insert(start_node.clone(), 0);
    f_scores.insert(start_node.clone(), heuristic(&start_node, &terminal_node));

    open_set.insert(start_node);
    parents.insert(&terminal_node, HashSet::new());

    let mut considered_nodes = 0;

    while open_set.len() > 0 {
        considered_nodes += 1;
        let node: Node = find_smallest_f(&open_set, &f_scores);
        open_set.remove(&node);
        let q_g = *g_scores.get(&node).unwrap();

        if node == terminal_node {
            println!("Distance to end: {}", q_g);
            if best_path.is_none() {
                best_path = Some(q_g);
            }
            closed_set.insert(node);
            continue
        }
        let next_nodes = graph.get(&node).unwrap();

        for (next_node, weight) in next_nodes {
            let tentative_g = q_g + weight;
            let current_g = *g_scores.get(next_node).unwrap_or(&i32::MAX);

            if tentative_g < current_g {
                g_scores.insert(next_node.clone(), tentative_g);
                f_scores.insert(next_node.clone(), tentative_g + heuristic(next_node, &terminal_node));

                let mut parent_set = HashSet::new();
                parent_set.insert(node.clone());
                parents.insert(next_node, parent_set);

                if !open_set.contains(next_node) && !closed_set.contains(next_node) {
                    open_set.insert(next_node.clone());
                }
            } else if tentative_g == current_g {
                parents.get_mut(next_node).unwrap().insert(node.clone());
                if !open_set.contains(next_node) && !closed_set.contains(next_node) {
                    open_set.insert(next_node.clone());
                }
            }
        }
        closed_set.insert(node);
    }
    println!("Considered {} nodes", considered_nodes);

    println!("Parent nodes: {}", parents.len());

    let mut visited_nodes = HashSet::new();
    let visited_coords = backtrack(&terminal_node, &parents, &mut visited_nodes);

    let visited_node_coords = visited_nodes.iter().map(|n| (n.x, n.y)).collect::<HashSet<_>>();
    println!("{}", visited_nodes.len());
    println!("Visited {} tiles", visited_coords.len());

    // debugging
    draw_maze(&maze, &visited_node_coords, &visited_coords);
    
}

fn draw_maze (maze: &Vec<Vec<char>>, reds: &HashSet<(i32, i32)>, blues: &HashSet<(i32, i32)>) {
    let mut red_count = 0;
    let mut blue_count = 0;

    for y in 0..maze.len() {
        for x in 0..maze[0].len() {
            let coords = (x as i32, y as i32);
            let c = maze[y][x];

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

fn backtrack (node: &Node, parents: &HashMap<&Node, HashSet<Node>>, visited_nodes: &mut HashSet<Node>) -> HashSet<(i32, i32)> {
    let mut visited_coords = HashSet::new();
    _backtrack(node, parents, visited_nodes, &mut visited_coords);
    return visited_coords;
}

fn _backtrack (node: &Node, parents: &HashMap<&Node, HashSet<Node>>, visited_nodes: &mut HashSet<Node>, visited_coords: &mut HashSet<(i32, i32)>) {
    if !parents.contains_key(node) {
        return;
    }
    let parent_nodes = parents.get(&node).unwrap();

    for p in parent_nodes {
        visited_coords.extend(coords_between(node, p));

        if !visited_nodes.contains(p) {
            visited_nodes.insert(p.clone());
            _backtrack(p, parents, visited_nodes, visited_coords);
        }
    }
}

/**
 * Finds all coordinates between
 * two junction points
 */
fn coords_between (node1: &Node, node2: &Node) -> HashSet<(i32, i32)> {
    let mut coords = HashSet::new();
    if node1.x != node2.x && node1.y != node2.y {
        panic!("Invalid arc");
    }
    if node1.x == node2.x && node1.y == node2.y {
        return coords;
    }
    let c1 = (node1.x, node1.y);
    let c2 = (node2.x, node2.y);

    let dir;
    if node1.x < node2.x {
        dir = (1, 0)
    } else if node1.x > node2.x {
        dir = (-1, 0)
    } else if node1.y < node2.y {
        dir = (0, 1)
    } else {
        dir = (0, -1)
    }
    let mut c = c1;
    coords.insert(c);
    while c != c2 {
        c = (c.0 + dir.0, c.1 + dir.1);
        coords.insert(c);
    }

    return coords;
}

fn min_f (list: &Vec<(Node, i32, i32)>) -> i32 {
    return list.iter().map(|item| item.2).min().unwrap_or(i32::MAX);
}

fn find_smallest_f (open_set: &HashSet<Node>, f_scores: &HashMap<Node, i32>) -> Node {
    let mut min = i32::MAX;
    let mut min_node = None;

    if open_set.len() == 0 {
        panic!("empty set")
    }

    for node in open_set {
        let f = *f_scores.get(node).unwrap_or(&i32::MAX);
        if f < min {
            min = f;
            min_node = Some(node.clone());
        }
    }

    return min_node.unwrap();
}

/**
 * Heuristic is Manhattan distance
 * + 1000 if on a diagonal - i.e.
 * a corner must be turned
 */
fn heuristic(node1: &Node, node2: &Node) -> i32 {
    let mut h = 0;
    let dx = (node1.x - node2.x).abs();
    let dy = (node1.x - node2.y).abs();
    h += dx;
    h += dy;

    if dx != 0 && dy != 0 {
        h += 1000;
    }
    return h;
}

/**
 * Build graph where each junction is a node
 * with separate nodes for each orientation of
 * the reindeer
 */
fn build_graph (maze: &Vec<Vec<char>>) -> HashMap<Node, HashMap<Node, i32>> {
    let mut graph = HashMap::new();
    let mut junction_points = HashSet::new();

    let terminal_coords = find_end(maze);

    for y in 0..maze.len() {
        for x in 0..maze[0].len() {
            if !is_space(maze[y][x]) {
                continue;
            }

            let coords = (x as i32, y as i32);

            for jnc in JUNCTION_COORDS {
                let jc1 = (coords.0 + jnc[0].0, coords.1 + jnc[0].1);
                let jc2 = (coords.0 + jnc[1].0, coords.1 + jnc[1].1);
                let char_1 = maze[jc1.1 as usize][jc1.0 as usize];
                let char_2 = maze[jc2.1 as usize][jc2.0 as usize];

                if is_space(char_1) && is_space(char_2) {
                    junction_points.insert(coords);
                    continue
                }
            }
            if is_terminal(maze, coords) {
                junction_points.insert(coords);
            }
        }
    }

    let terminal_node = Node {
        x: terminal_coords.0,
        y: terminal_coords.1,
        dir: 4
    };
    let mut terminal_edges = HashMap::new();

    for point in junction_points.iter() {
        for (d, dir) in DIRECTIONS.iter().enumerate() {
            let start_node = Node {
                x: point.0,
                y: point.1,
                dir: d as i32
            };
            let mut edges = HashMap::new();
            edges.insert(left_turn(&start_node), 1000);
            edges.insert(right_turn(&start_node), 1000);

            let arc_end = find_arc_end(maze, *point, *dir, &junction_points);
            if arc_end.is_some() {
                let end_node = Node {
                    x: arc_end.unwrap().0,
                    y: arc_end.unwrap().1,
                    dir: d as i32,
                };
                let length = dist(&start_node, &end_node);
                edges.insert(end_node, length);
            }

            if *point == terminal_coords {
                edges.insert(terminal_node.clone(), 0);
                terminal_edges.insert(start_node.clone(), 0);
            }

            graph.insert(start_node, edges);
        }
    }


    return graph
}

fn find_arc_end (maze: &Vec<Vec<char>>, start_coords: (i32, i32), dir: (i32, i32), junction_points: &HashSet<(i32, i32)>) -> Option<(i32, i32)> {
    let next_coords = (start_coords.0 + dir.0, start_coords.1 + dir.1);
    let c = maze[next_coords.1 as usize][next_coords.0 as usize];
    if junction_points.contains(&next_coords) {
        return Some(next_coords);
    } else if is_space(c) {
        return find_arc_end(maze, next_coords, dir, junction_points)
    } else {
        return None;
    }
}

fn left_turn (node: &Node) -> Node {
    return Node {
        x: node.x,
        y: node.y,
        dir: (node.dir - 1 + 4) % 4 
    }
}

fn right_turn (node: &Node) -> Node {
    return Node {
        x: node.x,
        y: node.y,
        dir: (node.dir + 1 + 4) % 4 
    }
}

fn is_terminal (maze: &Vec<Vec<char>>, coords: (i32, i32)) -> bool {
    let mut surround_count = 0;
    for s in SURROUNDS {
        let s_coords = (coords.0 + s.0, coords.1 + s.1);
        let s_char = maze[s_coords.1 as usize][s_coords.0 as usize];
        if is_space(s_char) {
            surround_count += 1;
        }
    }
    return surround_count == 1;
}

fn is_space (c: char) -> bool {
    return match c {
        EMPTY_CHAR => true,
        START_CHAR => true,
        END_CHAR => true,
        WALL_CHAR => false,
        _ => panic!("invalid char")
    }
}

fn parse_input () -> Vec<Vec<char>> {
    let mut input = vec![];
    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        input.push(line.chars().collect());
    }

    return input;
}

fn find_start (maze: &Vec<Vec<char>>) -> (i32, i32) {
    for y in 0..maze.len() {
        for x in 0..maze[0].len() {
            if maze[y][x] == START_CHAR {
                return (x as i32, y as i32);
            }
        }
    }
    panic!("Robot not found");
}

fn find_end (maze: &Vec<Vec<char>>) -> (i32, i32) {
    for y in 0..maze.len() {
        for x in 0..maze[0].len() {
            if maze[y][x] == END_CHAR {
                return (x as i32, y as i32);
            }
        }
    }
    panic!("End not found");
}

#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
struct Node {
    x: i32,
    y: i32,
    dir: i32,
}

/**
 * Manhattan distance
 */
fn dist (node1: &Node, node2: &Node) -> i32 {
    return (node1.x - node2.x).abs() + (node1.y - node2.y).abs()
}