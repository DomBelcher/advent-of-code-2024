use std::{hash::Hash, collections::{HashMap, HashSet}};

pub fn a_star<T: Eq + Hash + Clone> (start_node: &T, terminal_node: &T, graph: &HashMap<T, HashMap<T, i32>>, dist: fn(&T, &T) -> i32) -> Option<i32> {
    let mut open_set = HashSet::new();
    let mut closed_set = HashSet::new();
    let mut g_scores = HashMap::new();
    let mut f_scores: HashMap<&T, i32> = HashMap::new();
    let mut best_path = None;
    // let mut parents: HashMap<&T, HashSet<T>> = HashMap::new();

    g_scores.insert(start_node, 0);
    f_scores.insert(start_node, dist(&start_node, &terminal_node));

    open_set.insert(start_node);
    // parents.insert(terminal_node, HashSet::new());

    while open_set.len() > 0 {
        let node: T = find_smallest_f(&open_set, &f_scores);
        open_set.remove(&node);
        let q_g = *g_scores.get(&node).unwrap();

        if node == *terminal_node {
            // println!("Distance to end: {}", q_g);
            if best_path.is_none() {
                best_path = Some(q_g);
            }
            closed_set.insert(node);
            break
        }
        let next_nodes = graph.get(&node).unwrap();

        for (next_node, weight) in next_nodes {
            let tentative_g = q_g + weight;
            let current_g = *g_scores.get(next_node).unwrap_or(&i32::MAX);

            if tentative_g < current_g {
                g_scores.insert(next_node, tentative_g);
                f_scores.insert(next_node, tentative_g + dist(next_node, &terminal_node));

                if !open_set.contains(next_node) && !closed_set.contains(next_node) {
                    open_set.insert(next_node);
                }
            }
        }
        closed_set.insert(node);
    }
    // return f_scores.get(&terminal_node).map(|v| *v)
    return best_path;
}

// fn dist<T> (node1: &T, node2: &T) -> i32 {
//     return (node1.x - node2.x).abs() + (node1.y - node2.y).abs()
// }

fn find_smallest_f<T: Eq + Hash + Clone> (open_set: &HashSet<&T>, f_scores: &HashMap<&T, i32>) -> T {
    let mut min = i32::MAX;
    let mut min_node = None;

    if open_set.len() == 0 {
        panic!("empty set")
    }

    for node in open_set {
        let f = *f_scores.get(node).unwrap_or(&i32::MAX);
        if f < min {
            min = f;
            min_node = Some(node);
        }
    }

    return min_node.unwrap().clone().clone();
}

pub fn walk_graph <'a, T: Eq + Hash + Clone> (start_node: &'a T, graph: &'a HashMap<T, HashMap<T, i32>>) -> HashMap<&'a T, i32> {
    let mut open_set = HashSet::new();
    let mut closed_set = HashSet::new();
    let mut g_scores: HashMap<&'a T, i32> = HashMap::new();

    g_scores.insert(start_node, 0);

    open_set.insert(start_node);

    while open_set.len() > 0 {
        let node: T = find_smallest_f(&open_set, &g_scores);
        open_set.remove(&node);
        let q_g = *g_scores.get(&node).unwrap();
        let next_nodes = graph.get(&node).unwrap();

        for (next_node, weight) in next_nodes {
            let tentative_g = q_g + weight;
            let current_g = *g_scores.get(next_node).unwrap_or(&i32::MAX);

            if tentative_g < current_g {
                g_scores.insert(next_node, tentative_g);

                if !open_set.contains(next_node) && !closed_set.contains(next_node) {
                    open_set.insert(next_node);
                }
            }
        }
        closed_set.insert(node);
    }
    // return f_scores.get(&terminal_node).map(|v| *v)
    return g_scores;
}



#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
#[derive(Debug)]
pub struct CheatNode {
    pub x: i32,
    pub y: i32,
    pub cheat: bool,
    pub cheated: bool,
    pub cheat_level: i32,
}
