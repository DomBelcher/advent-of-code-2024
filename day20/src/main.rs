use std::{cmp::{max, min}, collections::{HashMap, HashSet}, fs, hash::DefaultHasher, i32, slice::Windows, io::prelude::*};
mod a_star;
mod graph_tools;
use a_star::CheatNode;
use graph_tools::{Node, Coords, grid_to_hashmap, junction_graph, is_space, is_char, build_graph};

const FILENAME: &str = "./inputs/input";
const DIRECTIONS: [(i32, i32); 4] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1)
];
const START_CHAR: char = 'S';
const EMPTY_CHAR: char = '.';
const WALL_CHAR: char = '#';
const END_CHAR: char = 'E';

const MIN_SAVING: i32 = 100;
const CHEAT_LENGTH: i32 = 20;

fn main() {
    println!("Hello, world!");
    let map = parse_input();
    let (maze, width, height) = grid_to_hashmap(&map);
    println!("Maze size: {}", maze.len());
    println!("{:?}", maze.get(&(3, 1)));
    // let (graph, start_node, end_node) = junction_graph(&maze, width, height);
    let (graph, start_node, end_node) = build_graph(&maze, width, height);
    println!("Graph size: {}", graph.len());
    // println!("{:?}", graph.values().map(|edges| edges.len()).max());
    println!("{:?}", start_node);
    println!("{:?}", end_node);
    let shortest_path = a_star::a_star(&start_node, &end_node, &graph, dist).unwrap();

    let dists_from_start = a_star::walk_graph(&start_node, &graph);
    // println!("{}", dists_from_start.values().collect::<HashSet<_>>().len());
    let dists_from_end = a_star::walk_graph(&end_node, &graph);

    println!("{:?}", dists_from_start.get(&Node::from_coords((9, 7))));
    println!("{:?}", dists_from_start.get(&Node::from_coords((11, 7))));

    println!("{}, {}", dists_from_start.len(), dists_from_end.len());

    let mut count = 0;
    for cheat_start in dists_from_start.iter() {
        for cheat_end in dists_from_end.iter() {
            let cheat_dist= dist(*cheat_start.0, *cheat_end.0);
            if cheat_dist > CHEAT_LENGTH {
                continue;
            }
            if (cheat_start.1 + cheat_end.1 + cheat_dist > shortest_path - MIN_SAVING) {
                continue;
            }

            // work out # of possible paths from start to end in at most N steps
            count += 1;
        } 
    }
    println!("Is this right? {}", count);

    /*
     * Everything after here massively 
     * overcomplicates the solution
     * and was a huge waste of time
     * ¯\_(ツ)_/¯ 
     */



    // println!("{}, {}", dists_from_start.get(&end_node).unwrap(), dists_from_end.get(&start_node).unwrap());

    // let (graph, start_node, end_node) = build_graph(&map);
    // let no_cheat_graph = no_cheat_graph(&graph);

    // let shortest_path = a_star::a_star(&start_node, &end_node, &no_cheat_graph, dist).unwrap();
    println!("Shortest path: {}", shortest_path);
    let mut count = 0;
    let mut total_cheats = 0;

    let f_id = 7;
    let mut file = fs::OpenOptions::new().write(true).create(true).append(true).open(format!("./target/out/{}.txt", f_id)).unwrap();

    let mut savings: HashMap<i32, i32> = HashMap::new();

    for x in 0..width {
        for y in 0..height {
            let coords_0 = (x as i32, y as i32);
            if maze.get(&coords_0).is_none() || is_space(maze.get(&coords_0)) {
                continue;
            }
            let cheats = get_cheats(&maze, coords_0, CHEAT_LENGTH);
            // writeln!(file, "({}, {}) | {}", x, y, cheats.len()).expect("Error writing to file");
            // fs::write(format!("./target/out/{}.txt", 0), ).expect("Unable to write file");

            for cheat in cheats {
                if cheat.len() == 0 {
                    continue;
                }
                total_cheats += 1;
                let mut best_path = i32::MAX;
                let cheat_start = cheat.first().unwrap();
                let cheat_end = cheat.last().unwrap();

                for dir_0 in DIRECTIONS {
                    let before_coords= (cheat_start.0 + dir_0.0, cheat_start.1 + dir_0.1);
                    if !is_space(maze.get(&before_coords)) {
                        continue;
                    }
                    
                    if is_space(maze.get(cheat_end)) {
                        let _best_path = dists_from_start.get(&Node::from_coords(before_coords)).unwrap() + cheat.len() as i32 + dists_from_end.get(&Node::from_coords(*cheat_end)).unwrap();
                        if _best_path < best_path {
                            best_path = _best_path;
                        }
                        // println!("Best on cheat end: {}", best_path);
                        // continue
                    }

                    for dir_1 in DIRECTIONS {
                        let after_coords= (cheat_end.0 + dir_1.0, cheat_end.1 + dir_1.1);
                        if !is_space(maze.get(&after_coords)) {
                            continue;
                        }
                        // println!("{:?}", after_coords);

                        let _best_path = dists_from_start.get(&Node::from_coords(before_coords)).unwrap() + cheat.len() as i32 + 1 + dists_from_end.get(&Node::from_coords(after_coords)).unwrap();
                        // println!("Best on next tile: {}", _bests_path);
                        if _best_path < best_path {
                            best_path = _best_path;
                        }
                    }
                }

                writeln!(file, "{:?} | {}", cheat, best_path).expect("Error writing to file");

                if best_path < shortest_path {
                    let saving = shortest_path - best_path;
                    savings.get_mut(&saving).map(|n| *n += 1).or_else(|| {
                        savings.insert(saving, 1);
                        return None
                     });
                }

                if best_path <= shortest_path - MIN_SAVING {
                    count += 1;
                }
            }
        }
    }
    for saving in savings {
        println!("There are {} cheats saving {} steps", saving.1, saving.0);
    }

    println!("There are {} cheats (out of {}) with better paths", count, total_cheats);

    // let height = map.len() as i32;
    // let width = map[0].len() as i32;

    println!("{}", count);
}

fn get_cheats(maze: &HashMap<(i32, i32), char>, coords: (i32, i32), depth: i32) -> HashSet<Vec<(i32, i32)>> {
    let mut cheats = HashSet::new();
    if depth == 1 {
        cheats.insert(vec![coords]);
        return cheats;
    }

    for dir in DIRECTIONS {
        // let mut cheat = vec![coords];
        let next_coords = (coords.0 + dir.0, coords.1 + dir.1);
        if !maze.contains_key(&next_coords) {
            continue;
        }
        if is_space(maze.get(&next_coords)) {
            cheats.insert(vec![coords, next_coords]);
        } else {
            let onward_coords = get_cheats(maze, next_coords, depth - 1);
            for mut oc in onward_coords {
                let mut cheat = vec![coords];
                cheat.append(&mut oc);
                cheats.insert(cheat);
            }
        }
    }

    return cheats;
}

// fn build_cheat_graph (input: &Vec<Vec<char>>, cheat_coords: &Vec<(usize, usize)>) -> HashMap<CNode, HashMap<CNode, i32>> {
//     let mut new_input = input.clone();
//     for coords in cheat_coords {
//         new_input[coords.1][coords.0] = EMPTY_CHAR;
//     }
//     return no_cheat_graph(&build_graph(&new_input).0);
// }

fn no_cheat_graph (graph: &HashMap<CNode, HashMap<CNode, i32>>) -> HashMap<CNode, HashMap<CNode, i32>> {
    let mut nc_graph = HashMap::new();

    for (node, edges) in graph {
        if node.cheat {
            continue
        }
        let mut nc_edges = HashMap::new();
        for (next_node, weight) in edges {
            if !next_node.cheat {
                nc_edges.insert(next_node.clone(), *weight);
            }
        }
        nc_graph.insert(node.clone(), nc_edges);
    }
    return nc_graph;
}

fn make_cheat_graph (start_node: &CNode, graph: &HashMap<CNode, HashMap<CNode, i32>>) -> HashMap<CheatNode, HashMap<CheatNode, i32>> {
    let mut cheat_graph = HashMap::new();
    let start_cheat_node = CheatNode {
        x: start_node.x,
        y: start_node.y,
        cheat: start_node.cheat,
        cheated: false,
        cheat_level: 0
    };
    cg_ (&start_cheat_node, graph, &mut cheat_graph, false, 0);

    return cheat_graph;
}

fn cg_ (cheat_node: &CheatNode, graph: &HashMap<CNode, HashMap<CNode, i32>>, cheat_graph: &mut HashMap<CheatNode, HashMap<CheatNode, i32>>, cheated: bool, cheat_level: i32) {
    let node = CNode {
        x: cheat_node.x,
        y: cheat_node.y,
        cheat: cheat_node.cheat
    };

    if cheat_graph.contains_key(&cheat_node) {
        return
    }
    println!("{:?}", cheat_node);
    let mut edges = HashMap::new();
    cheat_graph.insert(cheat_node.clone(), HashMap::new());

    let next_nodes = graph.get(&node).unwrap().keys();
    for next_node in next_nodes {
        if cheat_level == 2 && next_node.cheat {
            continue;
        }

        let cl;
        if cheat_level == 0 {
            if next_node.cheat {
                cl = 1
            } else {
                cl = 0
            }
        } else {
            cl = min(cheat_level + 1, 2)
        }
        let new_cheated = cheated || next_node.cheat;

        let next_cheat_node = CheatNode {
            x: next_node.x,
            y: next_node.y,
            cheat: next_node.cheat,
            cheated: new_cheated,
            cheat_level: cl
        };
        cg_(&next_cheat_node, graph, cheat_graph, new_cheated, cl);
        edges.insert(next_cheat_node, 1);
    }
    cheat_graph.insert(cheat_node.clone(), edges);
}

fn make_node (c: char, x: i32, y: i32) -> CNode {
    return CNode {
        x: x,
        y: y,
        cheat: c == WALL_CHAR
    }
}

fn parse_input () -> Vec<Vec<char>> {
    let mut input = vec![];
    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        input.push(line.chars().collect());
    }

    return input;
}

fn dist<T: Coords> (node1: &T, node2: &T) -> i32 {
    return (node1.x() - node2.x()).abs() + (node1.y() - node2.y()).abs()
}

#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
pub struct CNode {
    pub x: i32,
    pub y: i32,
    pub cheat: bool,
}

impl Coords for CNode {
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y
    }
}