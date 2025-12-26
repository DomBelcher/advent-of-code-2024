use std::fs;

const FILENAME: &str = "./inputs/input.txt";

fn main() {
    let (start_registers, inputs) = parse_input();
    run_program(&start_registers, &inputs);

    let a_vals = find_next_digit(&vec![], &inputs, inputs.len() - 1);
    if a_vals.is_none() {
        panic!()
    }
    let register_a = reduce(&a_vals.unwrap());

    println!("{}", register_a);
    let mut registers = start_registers.clone();
    registers.register_a = register_a;
    run_program(&registers, &inputs);
}

fn reduce (vals: &Vec<u8>) -> i64 {
    let mut register_a = 0;
    for val in vals.iter() {
        register_a <<= 3;
        register_a += *val as i64;
    }
    return register_a;
}

fn find_next_digit (a_prefix_vals: &Vec<u8>, inputs: &Vec<u8>, input_idx: usize) -> Option<Vec<u8>> {
    if input_idx >= inputs.len() {
        println!("end");
        return None;
    }
    let a_prefix = reduce(a_prefix_vals) << 3;

    for j in 0..8 {
        if output_bit(a_prefix, j) == inputs[input_idx] as i64 {
            let mut new_prefix_vals = a_prefix_vals.clone();
            new_prefix_vals.push(j as u8);

            if input_idx == 0 {
                return Some(new_prefix_vals);
            }

            let next_digits = find_next_digit(&new_prefix_vals, inputs, input_idx - 1); 
            if next_digits.is_some() {
                return Some(next_digits.unwrap());
            }
        }
    }

    return None;
}

fn run_program (start_registers: &Registers, inputs: &Vec<u8>) {
    let mut registers = start_registers.clone();

    let mut pointer = 0;
    let mut output = vec![];

    loop {
        if pointer >= inputs.len() {
            break;
        }

        let operator = inputs[pointer];
        let operand = inputs[pointer + 1];

        let (new_pointer, out) = perform_operation(operator, operand, &mut registers);
        if out.is_some() {
            output.push(out.unwrap());
        }

        if new_pointer.is_some() {
            pointer = new_pointer.unwrap();
        } else {
            pointer += 2;
        }
    }

    println!("{}", output.iter().map(|v| format!("{}", v)).collect::<Vec<_>>().join(","));
}

fn perform_operation (operator: u8, operand: u8, registers: &mut Registers) -> (Option<usize>, Option<i64>) {
    match operator {
        0 => {
            registers.register_a = registers.register_a / 2_i64.pow(combo_operand(operand, &registers) as u32);
        },
        1 => {
            registers.register_b = registers.register_b ^ (operand as i64);
        },
        2 => {
            registers.register_b = combo_operand(operand, &registers) % 8;
        },
        3 => {
            if registers.register_a != 0 {
                return (Some(operand as usize), None)
            }
        },
        4 => {
            registers.register_b = registers.register_b ^ registers.register_c;
        },
        5 => {
            return (None, Some(combo_operand(operand, &registers) % 8))
        },
        6 => {
            registers.register_b = registers.register_a / 2_i64.pow(combo_operand(operand, &registers) as u32);
        },
        7 => {
            registers.register_c = registers.register_a / 2_i64.pow(combo_operand(operand, &registers) as u32);
        },
        _ => panic!("invalid operator"),
    }

    return (None, None)
}

fn combo_operand (operand: u8, registers: &Registers) -> i64 {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => registers.register_a,
        5 => registers.register_b,
        6 => registers.register_c,
        7 => panic!("reserved operand 7"),
        _ => panic!("invalid operand"),
    }
}

fn parse_input () -> (Registers, Vec<u8>) {
    let mut register_vals = [0; 3];
    let mut input_vals= None;

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        if line.starts_with("Register") {
            let (reg_idx, reg_val) = parse_register(line);
            register_vals[reg_idx] = reg_val;
        } else if line.starts_with("Program") {
            let sections = line.split_whitespace().collect::<Vec<&str>>();
            input_vals = Some(sections[1].split(",").map(|v| v.parse::<u8>().unwrap()).collect::<Vec<u8>>());
        }
    }

    if input_vals.is_none() {
        panic!();
    }

    return (Registers {
        register_a: register_vals[0],
        register_b: register_vals[1],
        register_c: register_vals[2],
    }, input_vals.unwrap())
}

fn parse_register (register_string: &str) -> (usize, i64) {
    let sections = register_string.split_whitespace().collect::<Vec<_>>();

    let register_idx = match sections[1] {
        "A:" => 0,
        "B:" => 1,
        "C:" => 2,
        _ => {
            println!("Invalid register name: {}", sections[1]);
            panic!()
        }
    };

    return (register_idx, sections[2].parse::<i64>().unwrap())
}

#[derive(Clone)]
#[derive(Hash)]
#[derive(PartialEq, Eq)]
#[derive(Debug)]
struct Registers {
    register_a: i64,
    register_b: i64,
    register_c: i64
}

// this is specific to the individual input
fn output_bit (a_prefix: i64, reg_a: i64) -> i64 {
    let mut reg_b = reg_a;
    reg_b = reg_b ^ 5;
    let reg_c = (a_prefix + reg_a) / (2_i64.pow(reg_b as u32));
    reg_b = reg_b ^ 6;
    reg_b = reg_b ^ reg_c;
    
    return reg_b % 8;
}