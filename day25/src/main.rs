use std::fs;

const FILENAME: &str = "./inputs/input.txt";

const LOCK_HEADER: &str = "#####";
const KEY_HEADER: &str = ".....";

const PIN_CHAR: char = '#';
const EMPTY_CHAR: char = '.';

const LOCK_HEIGHT: usize = 5;
const N_PINS: usize = 5;

fn main() {
    let (locks, keys) = parse_input();

    println!("Locks: {}", locks.len());
    println!("Keys: {}", keys.len());

    let mut total = 0;

    for key in keys.iter() {
        for lock in locks.iter() {
            if match_key_to_lock(key, lock) {
                total += 1;
            }
        }
    }

    println!("Total: {}", total);
}

fn match_key_to_lock (key: &Vec<usize>, lock: &Vec<usize>) -> bool {
    for i in 0..N_PINS {
        if key[i] + lock[i] > LOCK_HEIGHT {
            return false
        }
    }
    return true
}

fn parse_input () -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut keys = vec![];
    let mut locks = vec![];

    let mut pins: Vec<usize> = vec![0; 5];
    let mut input_mode = "";
    let mut height = 0;

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        if input_mode == "" {
            if line == LOCK_HEADER {
                input_mode = "locks";
            } else if line == KEY_HEADER {
                input_mode = "keys";
            } else {
                panic!();
            }
            continue;
        }

        if line.len() == 0 {
            if input_mode == "keys" {
                keys.push(pins);
            } else if input_mode == "locks" {
                locks.push(pins);
            }
            pins = vec![0; 5];

            input_mode = "";
            height = 0;
            continue;
        }

        if height == 5 {
            continue;
        }

        for (pin_idx, pin) in line.chars().enumerate() {
            match pin {
                PIN_CHAR => pins[pin_idx] += 1,
                EMPTY_CHAR => (),
                _ => panic!()
            }
        }

        height += 1;
    }

    if input_mode == "keys" {
        keys.push(pins);
    } else if input_mode == "locks" {
        locks.push(pins);
    }

    return (locks, keys);
}