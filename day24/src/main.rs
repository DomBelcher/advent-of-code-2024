use std::{collections::{HashMap, HashSet}, fs};

const FILENAME: &str = "./inputs/input.txt";

fn main() {
    let (wires, gates) = parse_input();
    println!("n wires: {}", wires.len());
    println!("n gates: {}", gates.len());

    let mut wire_mapping = HashMap::new();
    for wire in wires.iter() {
        wire_mapping.insert(wire.name.clone(), wire.value);
        // println!("{:?}", wire);
    }

    let mut unsolved_wires = HashSet::new();
    for gate in gates.iter() {
        unsolved_wires.insert(gate.output.clone());
        // println!("{:?}", gate);
    }

    loop {
        if unsolved_wires.len() == 0 {
            break;
        }

        for gate in gates.iter() {
            if !unsolved_wires.contains(&gate.output) {
                continue;
            }
            if !wire_mapping.contains_key(&gate.input1) || !wire_mapping.contains_key(&gate.input2) {
                continue;
            }
            let i1 = wire_mapping.get(&gate.input1).unwrap();
            let i2 = wire_mapping.get(&gate.input2).unwrap();
            wire_mapping.insert(gate.output.clone(), solve_gate(*i1, *i2, gate.op));
            unsolved_wires.remove(&gate.output);
        }
    }

    println!("Solved");

    let mut z_index = 0;
    let mut total = 0;
    loop {
        let z_string = format!("0{}", z_index);
        let z_name = format!("z{}{}", z_string.chars().nth_back(1).unwrap(), z_string.chars().nth_back(0).unwrap());
        if !wire_mapping.contains_key(&z_name) {
            break;
        }
        let wire_val = wire_mapping.get(&z_name).unwrap();
        total += 2_i64.pow(z_index) * *wire_val as i64;

        z_index += 1;
    }
    println!("total: {}", total);

    let x_keys = get_keys('x', &wire_mapping);
    let y_keys = get_keys('y', &wire_mapping);


}

fn get_keys (prefix_char: char, wire_mapping: &HashMap<String, i8>) -> Vec<String> {
    let mut key_index = 0;
    let mut keys = vec![];
    loop {
        let key_string = format!("0{}", key_index);
        let key = format!("{}{}{}", prefix_char, key_string.chars().nth_back(1).unwrap(), key_string.chars().nth_back(0).unwrap());
        if !wire_mapping.contains_key(&key) {
            break;
        }
        keys.push(key);
        key_index += 1;
    }
    return keys;
}

fn parse_input () -> (Vec<Wire>, Vec<Gate>) {
    let mut wires = vec![];
    let mut gates = vec![];

    let mut input_mode = "wires";

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        if input_mode == "wires" && line.len() == 0 {
            input_mode = "gates";
            continue;
        }

        if input_mode == "wires" {
            wires.push(Wire::from_input(line));
        }

        if input_mode == "gates" {
            gates.push(Gate::from_input(line));
        }
    }

    return (wires, gates);
}

fn solve_gate (input1: i8, input2: i8, op: Op) -> i8 {
    match op {
        Op::AND => input1 & input2,
        Op::OR => input1 | input2,
        Op::XOR => input1 ^ input2
    }
}

#[derive(Debug)]
struct Wire {
    name: String,
    value: i8
}

impl Wire {
    fn from_input (input: &str) -> Wire {
        let sections = input.split(": ").collect::<Vec<&str>>();

        return Wire {
            name: sections[0].to_string(),
            value: sections[1].parse().unwrap()
        }
    }
}

#[derive(Debug)]
struct Gate {
    input1: String,
    input2: String,
    op: Op,
    output: String
}

impl Gate {
    fn from_input (input: &str) -> Gate {
        let sections = input.split_whitespace().collect::<Vec<&str>>();

        return Gate {
            input1: sections[0].to_string(),
            input2: sections[2].to_string(),
            op: Op::from_str(sections[1]).unwrap(),
            output: sections[4].to_string()
        }
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
enum Op {
    OR,
    XOR,
    AND
}

impl Op {
    fn from_str (input: &str) -> Option<Op> {
        match input {
            "OR" => Some(Op::OR),
            "XOR" => Some(Op::XOR),
            "AND" => Some(Op::AND),
            _ => {
                println!("[{}] is not a valid op", input);
                None
            }
        }
    }
}

// a b c   a&b a&c b&c   a|b a|c b|c   a^b a^c b^c   
// 0 0 0   0   0   0     0   0   0     0   0   0  
// 0 0 1   0   0   0     0   1   1     0   1   1  
// 0 1 0   0   0   0     1   0   1     1   0   1  
// 0 1 1   0   0   1     1   1   1     1   1   0  
// 1 0 0   0   0   0     1   1   1     1   1   0  
// 1 0 1   0   1   0     1   1   1     1   0   1  
// 1 1 0   1   0   0     1   1   1     0   1   1  
// 1 1 1   1   1   1     0   0   0     0   0   0  