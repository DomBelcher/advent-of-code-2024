use std::fmt;
use std::time::Instant;
use std::{collections::{HashMap, HashSet}, fs};
use std::cmp::{min, max};

const FILENAME: &str = "./inputs/input.txt";

fn main() {
    let start_time = Instant::now();

    let (wires, gates) = parse_input();
    println!("n wires: {}", wires.len());
    println!("n gates: {}", gates.len());

    let mut wire_mapping = HashMap::new();
    let mut gate_mapping = HashMap::new();
    
    for wire in wires.iter() {
        wire_mapping.insert(wire.name.clone(), wire.value);
    }

    let mut unsolved_wires = HashSet::new();
    for gate in gates.iter() {
        unsolved_wires.insert(gate.output.clone());

        let mapping_key = order_inputs(&gate.input1, &gate.input2);
        if gate_mapping.contains_key(&mapping_key) {
            let mut gates: &mut Vec<&Gate> = gate_mapping.get_mut(&mapping_key).unwrap();
            gates.push(gate);
        } else {
            gate_mapping.insert(mapping_key, vec![gate; 1]);
        }
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

    let bad_wires = part2(&gate_mapping);
    println!("{:?}", bad_wires.join(","));

    println!("Runtime: {}ms", start_time.elapsed().as_millis());
}

fn part2 (gate_mapping: &HashMap<(String, String), Vec<&Gate>>) -> Vec<String> {
    let mut bad_gates = vec![];

    let x00 = format_key('x', 0);
    let y00 = format_key('y', 0);

    let mut out_bits = vec![];
    let mut carry_bits = vec![];

    let gates = gate_mapping.get(&order_inputs(&x00, &y00)).unwrap();

    for gate in gates {
        if gate.op == Op::XOR {
            out_bits.push(gate.output.clone());
        } else if gate.op == Op::AND {
            carry_bits.push(gate.output.clone());
        }

        if gate.op != Op::XOR && gate.op != Op::AND {
            bad_gates.push(gate.output.clone());
        }
    }

    if out_bits.len() != 1 || carry_bits.len() != 1 {
        println!("out bits:");
        println!("{:?}", out_bits);

        println!("carry bits:");
        println!("{:?}", carry_bits);

        println!("Gates:");
        println!("{:?}", gates);

        panic!();
    }

    println!("Bad gates:");
    println!("{:?}", bad_gates);

    // carry_bits.push()

    let mut key_index = 1;
    loop {
        let x_key = format_key('x', key_index);
        let y_key = format_key('y', key_index);
        let carry_key = carry_bits.last().unwrap();

        if !gate_mapping.contains_key(&order_inputs(&x_key, &y_key)) {
            break;
        }

        let (out_bit, carry_bit, mut bad_outputs) = full_adder(&x_key, &y_key, carry_key, gate_mapping);
        out_bits.push(out_bit);
        carry_bits.push(carry_bit);

        bad_gates.append(&mut bad_outputs);

        key_index += 1;
    }

    println!("{:?}", bad_gates);

    let mut bads = HashSet::new();
    for gate in bad_gates {
        bads.insert(gate.clone());
    }
    let mut bad_wires = bads.into_iter().collect::<Vec<String>>();
    bad_wires.sort();
    return bad_wires;
}

fn expect_gate (input1: &String, input2: &String, op: Op, gate_mapping: &HashMap<(String, String), Vec<&Gate>>) -> Option<String> {
    let input_key = order_inputs(input1, input2);

    if !gate_mapping.contains_key(&input_key) {
        println!("no gate found for: {} {} {}", input1, op, input2);
        return None;
    }

    let gates = gate_mapping.get(&input_key).unwrap();
    if count_gates(gates, op) != 1 {
        println!("bad gate: {} {} {} | count {}", input1, op, input2, count_gates(gates, op));
        return None;
    }
    let out = &get_gate(gates, op).unwrap().output;

    return Some(out.clone())
}

fn full_adder (input1: &String, input2: &String, carry: &String, gate_mapping: &HashMap<(String, String), Vec<&Gate>>) -> (String, String, Vec<String>) {
    let mut bad_outputs = vec![];

    let xored = safe_expect_gate(input1, input2, Op::XOR, gate_mapping, &mut bad_outputs);
    let anded = safe_expect_gate(input1, input2, Op::AND, gate_mapping, &mut bad_outputs);
    let out_bit = safe_expect_gate(&xored, carry, Op::XOR, gate_mapping, &mut bad_outputs);
    let anded_2 = safe_expect_gate(&xored, carry, Op::AND, gate_mapping, &mut bad_outputs);
    let carry_bit = safe_expect_gate(&anded, &anded_2, Op::OR, gate_mapping, &mut bad_outputs);

    return (out_bit, carry_bit, bad_outputs)
}

fn safe_expect_gate (input1: &String, input2: &String, op: Op, gate_mapping: &HashMap<(String, String), Vec<&Gate>>, bad_outputs: &mut Vec<String>) -> String {
    let maybe_out = expect_gate(input1, input2, op, gate_mapping);
    if maybe_out.is_some() {
        return maybe_out.unwrap();
    }
    let (bad_input, corrected_input, out) = fix_bad_input(input1, input2, op, gate_mapping);
    println!("gate fixed | bad input: {}, correct input: {}", bad_input, out);
    bad_outputs.push(bad_input);
    bad_outputs.push(corrected_input);
    return out;
}

fn fix_bad_input (input1: &String, input2: &String, op: Op, gate_mapping: &HashMap<(String, String), Vec<&Gate>>) -> (String, String, String) {
    // one of the inputs is wrong
    // work out which
    println!("Attempting to fix gate {} {} {}", input1, op, input2);
    let gate_for_i1 = find_gate_for_input(input1, op, gate_mapping);
    let gate_for_i2 = find_gate_for_input(input2, op, gate_mapping);

    let corrected_input;
    let bad_input;
    let corrected_output;

    if gate_for_i1.is_some() {
        // input 2 is bad
        bad_input = input2;
        // should be this val
        (corrected_input, corrected_output) = gate_for_i1.unwrap();
    } else if gate_for_i2.is_some() {
        bad_input = input1;
        (corrected_input, corrected_output) = gate_for_i2.unwrap();
    } else {
        println!("Unable to correct gate {} {} {}", input1, op, input2);
        panic!();
    }

    return (bad_input.clone(), corrected_input, corrected_output);
}

fn find_gate_for_input (input: &String, op: Op, gate_mapping: &HashMap<(String, String), Vec<&Gate>>) -> Option<(String, String)> {
    for (input_keys, gates) in gate_mapping {
        if input_keys.0 == *input || input_keys.1 == *input {
            for gate in gates {
                if gate.op == op {
                    let i1 = &gate.input1;
                    let i2 = &gate.input2;
                    if input == i1 {
                        return Some((i2.clone(), gate.output.clone()))
                    }
                    return Some((i1.clone(), gate.output.clone()))
                }
            }
        }
    }

    return None
}

fn count_gates (gates: &Vec<&Gate>, op: Op) -> usize {
    return gates.iter().filter(|g| g.op == op).count();
}

fn get_gate <'a> (gates: &'a Vec<&Gate>, op: Op) -> Option<&'a &'a Gate> {
    return gates.iter().filter(|g| g.op == op).next();
}

fn format_key (prefix_char: char, key_index: usize) -> String {
    let key_string = format!("0{}", key_index);
    return format!("{}{}{}", prefix_char, key_string.chars().nth_back(1).unwrap(), key_string.chars().nth_back(0).unwrap());
}

fn get_keys (prefix_char: char, wire_mapping: &HashMap<String, bool>) -> Vec<String> {
    let mut key_index = 0;
    let mut keys = vec![];
    loop {
        let key = format_key(prefix_char, key_index);
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

fn solve_gate (input1: bool, input2: bool, op: Op) -> bool {
    match op {
        Op::AND => input1 && input2,
        Op::OR => input1 || input2,
        Op::XOR => input1 ^ input2
    }
}

#[derive(Debug)]
struct Wire {
    name: String,
    value: bool
}

impl Wire {
    fn from_input (input: &str) -> Wire {
        let sections = input.split(": ").collect::<Vec<&str>>();

        return Wire {
            name: sections[0].to_string(),
            value: binary_decode(sections[1])
        }
    }
}

#[derive(Clone)]
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

fn order_inputs (input1: &String, input2: &String) -> (String, String) {
    return (
        min(input1, input2).clone(),
        max(input1, input2).clone()
    )
}

fn binary_decode (binary_digit: &str) -> bool {
    match binary_digit {
        "0" => false,
        "1" => true,
        _ => panic!("unable to decode digit {}", binary_digit)
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
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

impl fmt::Display for Op {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::AND => write!(f, "AND"),
            Op::OR => write!(f, "OR"),
            Op::XOR => write!(f, "XOR")
        }
    }
}