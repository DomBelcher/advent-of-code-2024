use std::{collections::{HashMap, HashSet}, fs, hash::Hash, mem::take};

use a_star::a_star;
mod graph_tools;
mod a_star;

const FILENAME: &str = "./example";
const KEYPAD_FILES: [&str; 2] = [
    "./door_keypad",
    "./directional_keypad"
];

fn main() {
    let codes = parse_input();
    // let door_graph = HashMap::new();
    // let door_graph = make_graph(KEYPAD_FILES[0]);    
    // let directional_graph = make_graph(KEYPAD_FILES[1]);
    let door_graph = make_directional_graph(KEYPAD_FILES[0]);    
    let directional_graph = make_directional_graph(KEYPAD_FILES[1]);

    let mut door_moves = HashMap::new();
    let mut directional_moves = HashMap::new();

    println!("{}", door_graph.len());
    println!("{}", directional_graph.len());

    let mut total = 0;

    for code in codes {
        let code_val = code.iter().collect::<String>().split('A').collect::<Vec<&str>>().get(0).unwrap().parse::<i32>().unwrap();
        println!("{}", code_val);

        println!("Code: {}", code.iter().collect::<String>());
        let robot_1_code = resolve_directional_code(&code, &door_graph, &mut door_moves);
        println!("Robot 1 code {}", robot_1_code.iter().collect::<String>());
        let robot_2_code = resolve_directional_code(&robot_1_code, &directional_graph, &mut directional_moves);
        println!("Robot 2 code {}", robot_2_code.iter().collect::<String>());
        let robot_3_code = resolve_directional_code(&robot_2_code, &directional_graph, &mut directional_moves);
        println!("Robot 3 code {}", robot_3_code.iter().collect::<String>());
        println!("{}", robot_3_code.len());

        total += code_val * robot_3_code.len() as i32; 
        // resolve door code to robot 1 code
        // let mut robot_1_code = vec![];
        // for c in code {
        //     let char_pair = (start_char, c);
        //     if door_moves.contains_key(&char_pair) {
        //         robot_1_code.append(&mut door_moves.get(&char_pair).unwrap());
        //     } else {
        //         let moves = a_star(&start_char, &c, &door_graph, fake_dist);
        //         robot_1_code.append(&mut moves);
        //         door_moves.insert(char_pair, moves);
        //     }
        // }
        // resolve robot 1 code to robot 2 code
        // resole robot 2 code to robot 3 code
    }
    println!("Complexity: {}", total);
}

const BASE_COSTS: [(char, char, usize); 20] = [
    ('^', '>', 1), ('^', 'A', 1), ('^', '<', 1), ('^', 'v', 1),
    ('>', '^', 1), ('>', 'A', 1), ('>', '<', 1), ('>', 'v', 1), 
    ('v', '^', 1), ('v', 'A', 1), ('v', '<', 1), ('v', '>', 1), 
    ('<', '^', 1), ('<', 'A', 1), ('<', '>', 1), ('<', 'v', 1), 
    ('A', '^', 1), ('A', '>', 1), ('A', '<', 1), ('A', 'v', 1)
];

fn base_graph () -> () {
    let mut base_cost_map = HashMap::new();
    for cost in BASE_COSTS {
        base_cost_map.insert((cost.0, cost.1), cost.1);
    }
    return base_cost_map;
}



fn resolve_directional_code (code: &Vec<char>, graph: &HashMap<(char, char), HashMap<(char, char), (i32, char)>>, move_set: &mut HashMap<(char, char), Vec<char>>) -> Vec<char> {
    let mut start_char = 'A';
    let mut new_code = vec![];

    for c in code {
        let char_pair = (start_char, *c);
        if move_set.contains_key(&char_pair) {
            // println!("{:?}", char_pair);
            // println!("{:?}", move_set.get(&char_pair));
            new_code.append(&mut move_set.get(&char_pair).unwrap().clone());
        } else {
            // println!("Solving via dijksta");
            let mut moves = a_star(&(start_char, 'S'), &(*c, 'E'), graph, fake_dist);
            if (start_char == '^' && *c == '^') {
                println!("{:?}", graph.get(&('^', 'S')));
                println!("{:?}", graph.get(&('^', 'E')));
            }
            // let mut moves = map_to_code(&a_star(&(start_char, 'S'), &(*c, 'E'), graph, fake_dist));
            move_set.insert(char_pair, moves.clone());
            new_code.append(&mut moves);
        }
        new_code.push('A');
        start_char = *c;
    }
    return new_code;
}

// fn resolve_code (code: &Vec<char>, graph: &HashMap<char, HashMap<char, (i32, char)>>, move_set: &mut HashMap<(char, char), Vec<char>>) -> Vec<char> {
//     let mut start_char = 'A';
//     let mut new_code = vec![];
//     for c in code {
//         let char_pair = (start_char, *c);
//         if move_set.contains_key(&char_pair) {
//             // println!("{:?}", char_pair);
//             // println!("{:?}", move_set.get(&char_pair));
//             new_code.append(&mut move_set.get(&char_pair).unwrap().clone());
//         } else {
//             // println!("Solving via dijksta");
//             let mut moves = map_to_code(&a_star(&start_char, &c, &graph, fake_dist));
//             move_set.insert(char_pair, moves.clone());
//             new_code.append(&mut moves);
//         }
//         new_code.push('A');
//         start_char = *c;
//     }
//     return new_code;
// }

const BEST_CHAR_ORDER: [char; 4] = [
    '>', 'v', '<', '^'
];

fn map_to_code(map: &HashMap<char, i32>) -> Vec<char> {
    let mut code = vec![];
    for c in BEST_CHAR_ORDER {
        let n = map.get(&c).unwrap_or(&0);
        for _ in 0..*n {
            code.push(c);
        }
    }
    // for (c, n) in map {
    //     for _ in 0..*n {
    //         code.push(*c);
    //     }
    // }
    return code;
}

fn fake_dist <T>(_c1: &T, _c2: &T) -> i32 { 0 }



fn make_directional_graph(fname: &str) -> HashMap<(char, char), HashMap<(char, char), (i32, char)>> {
    
    let mut input = HashMap::new();

    let mut width= 0;
    let mut height = 0;

    for (y, line) in fs::read_to_string(fname).unwrap().lines().enumerate() {
        width = line.len();
        height += 1;

        for (x, c) in line.chars().enumerate() {
            input.insert((x as i32, y as i32), c);
        }
    }

    return graph_tools::build_directional_graph(&input, width, height)
}

fn make_graph (fname: &str) -> HashMap<char, HashMap<char, (i32, char)>> {
    let mut input = HashMap::new();

    let mut width= 0;
    let mut height = 0;

    for (y, line) in fs::read_to_string(fname).unwrap().lines().enumerate() {
        width = line.len();
        height += 1;

        for (x, c) in line.chars().enumerate() {
            input.insert((x as i32, y as i32), c);
        }
    }

    return graph_tools::build_graph(&input, width, height)
}

fn parse_input () -> Vec<Vec<char>> {
    let mut input = vec![];

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        input.push(line.chars().collect());
    }

    return input;
}