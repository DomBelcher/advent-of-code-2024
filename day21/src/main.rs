use std::{collections::{HashMap, HashSet}, fs, usize};

mod graph_tools;

const FILENAME: &str = "./inputs/input";

const BASE_COSTS: [(char, char, usize); 25] = [
    ('^', '>', 1), ('^', 'A', 1), ('^', '<', 1), ('^', 'v', 1), ('^', '^', 1),
    ('>', '^', 1), ('>', 'A', 1), ('>', '<', 1), ('>', 'v', 1), ('>', '>', 1),
    ('v', '^', 1), ('v', 'A', 1), ('v', '<', 1), ('v', '>', 1), ('v', 'v', 1),
    ('<', '^', 1), ('<', 'A', 1), ('<', '>', 1), ('<', 'v', 1), ('<', '<', 1),
    ('A', '^', 1), ('A', '>', 1), ('A', '<', 1), ('A', 'v', 1), ('A', 'A', 1),
    // ('S', )
];

const VALID_KEYS: [char; 15] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '^', '>', 'v', '<', 'A'];

const DIRECTIONS: [((i32, i32), char); 5] = [
    ((1, 0), '>'),
    ((0, 1), 'v'),
    ((-1, 0), '<'),
    ((0, -1), '^'),
    ((0, 0), 'A')
];

const N_ROBOTS: usize = 26;


fn main() {
    let codes = parse_input();

    let DOOR_FILE = read_file("./door_keypad");
    let DIRECTIONAL_FILE = read_file("./directional_keypad");

    let base_costs = base_graph();

    let door_graph = build_transition_graph(&DOOR_FILE);
    let directional_graph = build_transition_graph(&DIRECTIONAL_FILE);

    let costs_1 = cost_graph(&directional_graph, &base_costs);

    let mut cost_tower = cost_graph(&directional_graph, &base_costs);
    for i in 0..(N_ROBOTS - 2) {
        cost_tower = cost_graph(&directional_graph, &cost_tower);
    }

    // let costs_2 = cost_graph(&directional_graph, &costs_1);
    // let costs_3 = cost_graph(&directional_graph, &costs_2);
    // let costs_4 = cost_graph(&door_graph, &costs_2);
    let costs_end = cost_graph(&door_graph, &cost_tower);

    let mut total = 0;

    for (code, num) in codes {
        let mut cost = 0;
        let code_len = code.len();

        for i in 0..(code_len-1) {
            let c1 = code[i];
            let c2 = code[i+1];

            cost += costs_end.get(&(c1, c2)).unwrap();
        }

        println!("Code: {:?} | cost {} | total {}", code, cost, cost * num);
        total += (cost * num);
    }

    println!("{}", total);
}

fn cost_graph (transition_graph: &HashMap<char, Vec<GraphEdge>>, cost_map: &HashMap<(char, char), usize>) -> HashMap<(char, char), usize>{
    let mut graph: HashMap<(char, char), usize> = HashMap::new();

    // let mut chars = HashSet::new();

    // for char

    for char_1 in transition_graph.keys() {
        for char_2 in transition_graph.keys() {
            let cost = a_star(transition_graph, cost_map, *char_1, *char_2);
            // println!("({},{}): cost {}", char_1, char_2, cost.unwrap());
            graph.insert((*char_1, *char_2), cost.unwrap());
        }
    }

    // for (char_1, char_2, _) in BASE_COSTS {
    //     // find cost of moving between char_1 and char_2
    //     // using cost map of tier below & graph of transitions

    //     let cost = a_star(transition_graph, cost_map, char_1, char_2);
    //     graph.insert((char_1, char_2), cost);
    // }

    return graph;
}

fn a_star (transition_graph: &HashMap<char, Vec<GraphEdge>>, cost_map: &HashMap<(char, char), usize>, start_char: char, end_char: char) -> Option<usize> {
    let mut open_set: HashSet<GraphNode> = HashSet::new();
    let start_node = GraphNode {
        keypad_char: start_char,
        previous_transition: 'A'
    };
    let end_node = GraphNode {
        keypad_char: end_char,
        previous_transition: 'A'
    };

    if start_node == end_node {
        return Some(*cost_map.get(&('A', 'A')).unwrap());
    }

    open_set.insert(start_node);

    let mut g_scores = HashMap::new();
    g_scores.insert(start_node, 0);

    let mut f_scores = HashMap::new();
    // f_scores.insert(start_node, *cost_map.get(&(start_char, end_char)).unwrap_or(&0));
    f_scores.insert(start_node, 0);

    while open_set.len() != 0 {
        let current_node = find_current(&open_set, &f_scores);

        if current_node == end_node {
            return Some(*g_scores.get(&current_node).unwrap());
        }

        open_set.remove(&current_node);

        if !transition_graph.contains_key(&current_node.keypad_char) {
            println!("oh no: {}", current_node.keypad_char);
        }
        let edges = transition_graph.get(&current_node.keypad_char).unwrap();

        for edge in edges {
            let tentative_g = g_scores.get(&current_node).unwrap() + cost_map.get(&(current_node.previous_transition, edge.trans_char)).unwrap();
            let next_node = GraphNode::from_edge(&edge);
            
            if tentative_g < *g_scores.get(&next_node).unwrap_or(&usize::MAX) {
                g_scores.insert(next_node, tentative_g);

                let mut h = 0;
                // if cost_map.contains_key(&(next_node.keypad_char, end_char)) {
                //     h = *cost_map.get(&(next_node.keypad_char, end_char)).unwrap()
                // }

                f_scores.insert(next_node, tentative_g + h);

                if !open_set.contains(&next_node) {
                    open_set.insert(next_node);
                }
            }
        }
    }


    return None;
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct GraphNode {
    keypad_char: char,
    previous_transition: char
}

impl GraphNode {
    fn from_edge (edge: &GraphEdge) -> GraphNode {
        return GraphNode {
            keypad_char: edge.dest_char,
            previous_transition: edge.trans_char
        }
    }
}

fn find_current (open_set: &HashSet<GraphNode>, f_scores: &HashMap<GraphNode, usize>) -> GraphNode {
    let mut smallest_f = usize::MAX;
    let mut smallest_f_char = None;

    for c in open_set {
        let f = f_scores.get(c).unwrap();
        if *f < smallest_f {
            smallest_f = *f;
            smallest_f_char = Some(*c);
        }
    }

    return smallest_f_char.unwrap();
}

fn build_transition_graph (file: &HashMap<(i32, i32), char>) -> HashMap<char, Vec<GraphEdge>> {
    // let mut graph: HashMap<(char, char), char> = HashMap::new();
    let mut graph: HashMap<char, Vec<GraphEdge>> = HashMap::new();

    for (coords, char) in file {
        let mut edges = vec![];

        for (dir, transition_char) in DIRECTIONS {
            let adjacent = (coords.0 + dir.0, coords.1 + dir.1);

            if file.contains_key(&adjacent) {
                let adjacent_char = file.get(&adjacent).unwrap();
                edges.push(GraphEdge { src_char: *char, dest_char: *adjacent_char, trans_char: transition_char })
                // graph.insert((*char, *adjacent_char), transition_char);
            }
        }

        graph.insert(*char, edges);
    }

    return graph;
}

struct GraphEdge {
    src_char: char,
    dest_char: char,
    trans_char: char
}

fn read_file (fname: &str) -> HashMap<(i32, i32), char> {
    let mut valid_keys = HashSet::new();
    for vk in VALID_KEYS {
        valid_keys.insert(vk);
    }

    let mut input = HashMap::new();

    let mut width= 0;
    let mut height = 0;

    for (y, line) in fs::read_to_string(fname).unwrap().lines().enumerate() {
        width = line.len();
        height += 1;

        for (x, c) in line.chars().enumerate() {
            if (valid_keys.contains(&c)) {
                input.insert((x as i32, y as i32), c);
            }
        }
    }

    return input;
}

// fn build_graph (keypad: &HashMap<(i32, i32), char>, width: usize, height: usize) -> HashMap<char, HashMap<char, (i32, char)>> {
//     let mut graph = HashMap::new();

//     for y in 0..height {
//         for x in 0..width {
//             let coords: (i32, i32) = (x as i32, y as i32);
//             if !is_key(keypad.get(&coords)) {
//                 println!("{:?}", coords);
//                 println!("{:?}", keypad.get(&coords));
//                 continue;
//             }
//             let key = keypad.get(&coords).unwrap();

//             let mut edges = HashMap::new();
//             for (dir, dir_char) in DIRECTIONS {
//                 let next_x = coords.0 + dir.0;
//                 let next_y = coords.1 + dir.1;
//                 if !is_key(keypad.get(&(next_x, next_y))) {
//                     continue;
//                 }
//                 let next_node = keypad.get(&(next_x, next_y)).unwrap();
//                 edges.insert(*next_node, (1, dir_char));
//             }
//             graph.insert(*key, edges);
//         }
//     }

//     return graph;
// }

fn base_graph () -> HashMap<(char, char), usize> {
    let mut base_cost_map = HashMap::new();
    for cost in BASE_COSTS {
        base_cost_map.insert((cost.0, cost.1), cost.2);
    }
    return base_cost_map;
}

fn parse_input () -> Vec<(Vec<char>, usize)> {
    let mut input = vec![];

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        let mut chars = line.chars().collect::<Vec<char>>();
        chars.insert(0, 'A');
        let num = line[..3].parse::<usize>().unwrap();
        
        input.push((chars, num));
    }

    return input;
}