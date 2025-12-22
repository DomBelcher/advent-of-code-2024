use std::{collections::HashMap, fs, cmp::min};

const FILENAME: &str = "./inputs/input";

const N_BLINKS: usize = 75;

fn main() {
    let input = parse_input();
    let mut stones = input;
    let mut lookups:HashMap<u64, Vec<u64>> = HashMap::new();

    let mut stone_lookups:HashMap<u64, HashMap<usize, u64>> = HashMap::new();
    
    let mut total_stones = 0;
    for stone in stones {
        let (total, _) = blink_with_depth(stone, 0, &mut stone_lookups);
        println!("stone {} becomes {} stones", stone, total);
        total_stones += total;
    }
    println!("{}", stone_lookups.len());
    println!("{}", total_stones);

}

fn blink_with_depth (stone: u64, depth: usize, stone_lookups: &mut HashMap<u64, HashMap<usize, u64>>) -> (u64, Vec<u64>) {
    if depth == N_BLINKS {
        return (1, vec![1]);
    }

    if stone_lookups.contains_key(&stone) {
        if stone_lookups.get_mut(&stone).unwrap().contains_key(&(N_BLINKS - depth)) {
            return (*stone_lookups.get_mut(&stone).unwrap().get(&(N_BLINKS - depth)).unwrap(), vec![1]);
        }
    }

    if stone == 0 {
        let (total, mut cont) = blink_with_depth(1, depth + 1, stone_lookups);
        cont.insert(0, 1);
        extend_lookups (stone, stone_lookups, &cont);
        return (total, cont);
    }
    let n_digits = n_digits(stone);
    if n_digits % 2 == 0 {
        let power_ten = 10_u64.pow((n_digits / 2) as u32);
        let last_digits = stone % power_ten;
        let first_digits = (stone - last_digits) / power_ten;

        let (total1, cont1) = blink_with_depth(first_digits, depth + 1, stone_lookups);
        let (total2, cont2) = blink_with_depth(last_digits, depth + 1, stone_lookups);
        let mut new_cont = vec![];
        let min_len = min(cont1.len(), cont2.len());
        for i in 0..min_len {
            new_cont.push(cont1[i] + cont2[i]);
        }

        new_cont.insert(0, 1);
        extend_lookups(stone, stone_lookups, &new_cont);
        return (total1 + total2, new_cont)
    }
    let (total, mut cont) = blink_with_depth(stone * 2024, depth + 1, stone_lookups);
    cont.insert(0, 1);
    extend_lookups (stone, stone_lookups, &cont);
    return (total, cont);
}

fn extend_lookups (stone: u64, stone_lookups: &mut HashMap<u64, HashMap<usize, u64>>, extension: &Vec<u64>) {
    for (i, step) in extension.iter().enumerate() {
        if stone_lookups.contains_key(&stone) {
            stone_lookups.get_mut(&stone).unwrap().insert(i, *step);
        } else {
            let mut lookup = HashMap::new();
            lookup.insert(i + 1, *step);
            stone_lookups.insert(stone, lookup);
        }
    }
}

fn blink(stones: &Vec<u64>, lookups: &mut HashMap<u64, Vec<u64>>) -> Vec<u64> {
    let mut new_stones = vec![];
    for stone in stones {


        if *stone == 0 {
            new_stones.push(1);
        } else if lookups.contains_key(stone) {
            let mut lookup = lookups.get(stone).unwrap().clone();
            new_stones.append(&mut lookup);
        } else if n_digits(*stone) % 2 == 0 {
            let n_digits = n_digits(*stone);
            let power_ten = 10_u64.pow((n_digits / 2) as u32);
            let last_digits = stone % power_ten;
            let first_digits = (stone - last_digits) / power_ten;
            new_stones.push(first_digits);
            new_stones.push(last_digits);
            lookups.insert(*stone, vec![first_digits, last_digits]);
        } else {
            new_stones.push(stone * 2024)
        }
    }

    return new_stones;
}

fn n_digits (val: u64) -> u64 {
    return (val as f64).log10() as u64 + 1;
}

fn parse_input () -> Vec<u64> {
    let mut input = vec![];
    
    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        let mut nums = line.split_whitespace().map(|num| num.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        input.append(&mut nums);
    }
    return input;
}