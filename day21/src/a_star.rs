use std::{collections::{hash_set, HashMap, HashSet}, hash::Hash, fmt::Debug};


pub fn a_star<T: Eq + Hash + Clone + Debug> (start_node: &T, terminal_node: &T, graph: &HashMap<T, HashMap<T, (i32, char)>>, dist: fn(&T, &T) -> i32) -> Vec<char> {
    let mut open_set = HashSet::new();
    let mut closed_set = HashSet::new();
    let mut g_scores: HashMap<&T, i32> = HashMap::new();
    let mut f_scores: HashMap<&T, i32> = HashMap::new();
    let mut parents: HashMap<T, (T, char)> = HashMap::new();

    g_scores.insert(start_node, 0);
    f_scores.insert(start_node, dist(&start_node, &terminal_node));

    open_set.insert(start_node);
    // parents.insert(terminal_node, HashSet::new());

    while open_set.len() > 0 {
        // println!("{:?}", open_set);
        let node: T = find_smallest_f(&open_set, &f_scores);
        let q_g = *g_scores.get(&node).unwrap();
        open_set.remove(&node);

        if &node == terminal_node {
            closed_set.insert(node);
            break
        }
        // println!("{:?}", node);
        let next_nodes = graph.get(&node).unwrap();
        // println!("{:?}", next_nodes);

        for (next_node, (weight, arrow)) in next_nodes {
            let tentative_g = q_g + weight;
            let current_g = *g_scores.get(next_node).unwrap_or(&i32::MAX);

            if tentative_g < current_g {
                g_scores.insert(next_node, tentative_g);
                f_scores.insert(next_node, tentative_g + dist(next_node, terminal_node));
                parents.insert(next_node.clone(), (node.clone(), *arrow));

                if !open_set.contains(&next_node) && !closed_set.contains(&next_node) {
                    open_set.insert(next_node);
                }
            }
        }
        closed_set.insert(node);
    }

    let mut path: Vec<char> = vec![];

    // let mut path = HashMap::new();
    let mut curr_node = terminal_node;

    // println!("{}", *g_scores.get(&terminal_node).unwrap());

    loop {
        let parent = parents.get(curr_node);
        if parent.is_none() {
            break;
        }
        path.push(parent.unwrap().1);
        // *path.entry(parent.unwrap().1).or_insert(0) += 1;
        // path.push(parent.unwrap().1);
        curr_node = &parent.unwrap().0;
    }
    // path.reverse();
    // println!("{:?}", path);
    path.reverse();
    path = path.into_iter().filter(|c| c != &'E').collect();

    return path;
}


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
            min_node = Some(*node);
        }
    }

    return min_node.unwrap().clone();
}