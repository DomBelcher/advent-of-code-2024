use std::collections::HashMap;

const VOID_CHAR: char = '#';
const START_CHAR: char = 'S';
const END_CHAR: char = 'E';

const DIRECTIONS: [((i32, i32), char); 6] = [
    ((1, 0), '>'),
    ((0, 1), 'v'),
    ((-1, 0), '<'),
    ((0, -1), '^'),
    // ((0, -1), '_'),
    ((0, 0), 'S'),
    ((0, 0), 'E')
];
const ARROWS: [char; 4] = [
    '>',
    'v',
    '<',
    '^',
];


const COSTS: [(char, char, i32); 24] = [
    ('^', '^', 0), ('^', '>', 2), ('^', '<', 2), ('^', 'v', 1), ('^', 'E', 0), ('S', '^', 0), 
    ('>', '>', 0), ('>', '^', 2), ('>', '<', 2), ('>', 'v', 1), ('>', 'E', 0), ('S', '>', 0), 
    ('<', '<', 0), ('<', '>', 2), ('<', '^', 2), ('<', 'v', 1), ('<', 'E', 0), ('S', '<', 0), 
    ('v', 'v', 0), ('v', '>', 1), ('v', '<', 1), ('v', '^', 1), ('v', 'E', 0), ('S', 'v', 0), 
];

fn make_costs_map () -> HashMap<(char, char), i32> {
    let mut map = HashMap::new();
    for c in COSTS {
        map.insert((c.0, c.1), c.2);
    }
    return map
}

pub fn build_directional_graph (keypad: &HashMap<(i32, i32), char>, width: usize, height: usize) -> HashMap<(char, char), HashMap<(char, char), (i32, char)>> {
    let cost_map = make_costs_map();
    let mut graph = HashMap::new();

    for y in 0..height {
        for x in 0..width {
            let coords: (i32, i32) = (x as i32, y as i32);
            if !is_key(keypad.get(&coords)) {
                println!("{:?}", coords);
                println!("{:?}", keypad.get(&coords));
                continue;
            }
            let key = keypad.get(&coords).unwrap();

            for (prev_dir, prev_dir_char) in DIRECTIONS {
                let prev_x = coords.0 - prev_dir.0;
                let prev_y = coords.1 - prev_dir.1;
                let prev_key =  keypad.get(&(prev_x, prev_y));
                if !is_key(prev_key) {
                    continue;
                } else if is_end(prev_dir_char){
                    graph.insert((*key, prev_dir_char), HashMap::new());
                    continue
                }
                // let prev_node = keypad.get(&(prev_x, prev_y)).unwrap();

                let mut edges = HashMap::new();
                for (dir, dir_char) in DIRECTIONS {
                    let next_x = coords.0 + dir.0;
                    let next_y = coords.1 + dir.1;
                    let next_key = keypad.get(&(next_x, next_y));
                    if !is_key(next_key) || is_start(dir_char) {
                        continue;
                    } else if is_start(prev_dir_char) && is_end(dir_char) {
                        // let mut identity_map = HashMap::new();
                        // identity_map.insert((*prev_key.unwrap(), 'E'), (0, 'E'));
                        edges.insert((*prev_key.unwrap(), 'E'), (0, 'E'));
                        continue
                    }
                    let next_node = next_key.unwrap();
                    // println!("{:?}", (prev_dir_char, dir_char));
                    edges.insert((*next_node, dir_char), (*cost_map.get(&(prev_dir_char, dir_char)).unwrap(), dir_char));
                }
                graph.insert((*key, prev_dir_char), edges);
            }
        }
    }

    return graph
}

pub fn build_graph (keypad: &HashMap<(i32, i32), char>, width: usize, height: usize) -> HashMap<char, HashMap<char, (i32, char)>> {
    let mut graph = HashMap::new();

    for y in 0..height {
        for x in 0..width {
            let coords: (i32, i32) = (x as i32, y as i32);
            if !is_key(keypad.get(&coords)) {
                println!("{:?}", coords);
                println!("{:?}", keypad.get(&coords));
                continue;
            }
            let key = keypad.get(&coords).unwrap();

            let mut edges = HashMap::new();
            for (dir, dir_char) in DIRECTIONS {
                let next_x = coords.0 + dir.0;
                let next_y = coords.1 + dir.1;
                if !is_key(keypad.get(&(next_x, next_y))) {
                    continue;
                }
                let next_node = keypad.get(&(next_x, next_y)).unwrap();
                edges.insert(*next_node, (1, dir_char));
            }
            graph.insert(*key, edges);
        }
    }

    return graph;
}

fn is_key(key: Option<&char>) -> bool {
    if key.is_none() || key.unwrap() == &VOID_CHAR {
        return false;
    }
    return true
}

fn is_start (key: char) -> bool {
    return key == START_CHAR
}

fn is_end (key: char) -> bool {
    return key == END_CHAR
}