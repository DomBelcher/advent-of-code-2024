use std::{collections::{HashMap, HashSet}, fs};

const FILENAME: &str = "./inputs/input";
const WIDTH: i32 = 71;
const HEIGHT: i32 = 71;
const CORRUPTIONS: usize = 1024;

const DIRECTIONS: [(i32, i32); 4] = [
    (1, 0), (0, 1), (-1, 0), (0, -1)
];

fn main() {
    let coords = parse_input();
    let corrupt_coords = coords[..CORRUPTIONS].iter().collect::<HashSet<_>>();
    println!("{}", corrupt_coords.len());
    let graph = build_graph(&corrupt_coords);
    println!("{}", graph.len());
    println!("{}", graph.get(&Node { x: 0, y: 0 }).unwrap().len());

    let shortest_path_len = a_star(&graph).unwrap();
    println!("Shortest path is {} steps long", shortest_path_len);

    let cutoff = find_cutoff(&coords, 0, coords.len());
    println!("Cutoff is at {}", cutoff);
    let cutoff_coords = coords[cutoff];
    println!("Cutoff coords: ({}, {})", cutoff_coords.0, cutoff_coords.1);
}

fn find_cutoff (coords: &Vec<(i32, i32)>, start: usize, end: usize) -> usize {
    println!("{}, {}", start, end);
    if end - start == 1 {
        return start
    }

    let cutoff = (end - start) / 2 + start;
    println!("{}", cutoff);
    let corrupt_coords = coords[..cutoff].iter().collect::<HashSet<_>>();

    let graph = build_graph(&corrupt_coords);
    let shortest_path_len = a_star(&graph);
    if shortest_path_len.is_some() {
        return find_cutoff(coords, cutoff, end)
    } else {
        return find_cutoff(coords, start, cutoff)
    }
}

fn a_star (graph: &HashMap<Node, HashMap<Node, i32>>) -> Option<i32> {
    let start_node = Node {
        x: 0, y: 0
    };
    let terminal_node = Node {
        x: WIDTH - 1, y: HEIGHT - 1
    };

    // perform A* search on the graph
    let mut open_set = HashSet::new();
    let mut closed_set = HashSet::new();
    let mut g_scores = HashMap::new();
    let mut f_scores: HashMap<Node, i32> = HashMap::new();
    let mut best_path = None;
    let mut parents: HashMap<&Node, HashSet<Node>> = HashMap::new();
    g_scores.insert(start_node.clone(), 0);
    f_scores.insert(start_node.clone(), dist(&start_node, &terminal_node));

    open_set.insert(start_node);
    parents.insert(&terminal_node, HashSet::new());

    while open_set.len() > 0 {
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
                f_scores.insert(next_node.clone(), tentative_g + dist(next_node, &terminal_node));

                if !open_set.contains(next_node) && !closed_set.contains(next_node) {
                    open_set.insert(next_node.clone());
                }
            }
        }
        closed_set.insert(node);
    }
    return f_scores.get(&terminal_node).map(|v| *v)
}

fn dist (node1: &Node, node2: &Node) -> i32 {
    return (node1.x - node2.x).abs() + (node1.y - node2.y).abs()
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

fn build_graph (corrupt_coords: &HashSet<&(i32, i32)>) -> HashMap<Node, HashMap<Node, i32>> {
    let mut graph = HashMap::new();

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if !corrupt_coords.contains(&(x, y)) {
                let node = Node {
                    x: x,
                    y: y
                };
                let mut edges = HashMap::new();
                for dir in DIRECTIONS {
                    let next_node = Node {
                        x: x + dir.0,
                        y: y + dir.1
                    };
                    if !corrupt_coords.contains(&(next_node.x, next_node.y))
                        && next_node.x >= 0 && next_node.x < WIDTH
                        && next_node.y >= 0 && next_node.y < HEIGHT {
                        edges.insert(next_node, 1);
                    }
                }
                graph.insert(node, edges);
            }
        }
    }
    return graph;
}

fn parse_input () -> Vec<(i32, i32)> {
    let mut coords = vec![];
    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        let nums = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
        coords.push((
            nums[0], nums[1]
        ));
    }
    return coords;
}

#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
struct Node {
    x: i32,
    y: i32
}